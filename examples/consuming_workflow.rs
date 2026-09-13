//! A consuming recipe whose caller owns the resource, runtime authority and effects.

pub(crate) struct Document {
    owner: u32,
    revision: u32,
    text: String,
}

pub(crate) struct Editor {
    owner: u32,
    revision: u32,
    phase: workflow::Phase,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Refusal {
    Owner,
    Revision,
    Phase,
    Empty,
}

pub(crate) fn inspect(
    editor: &mut Editor,
    document: &Document,
    expected: workflow::Phase,
) -> Result<(), Refusal> {
    if editor.owner != document.owner {
        return Err(Refusal::Owner);
    }
    if editor.revision != document.revision {
        return Err(Refusal::Revision);
    }
    if editor.phase != expected {
        return Err(Refusal::Phase);
    }
    Ok(())
}

macroonz::recipe! {
    pub(crate) mod workflow {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub(crate) enum Phase { Draft, Published }
        pub(crate) enum Event { Publish }
        bake! {
            vocabularies { Phase; Event; };
            transitions(Phase, Event) {
                (Draft, Publish) => Published with(target) {
                    if text.is_empty() { return Err(crate::Refusal::Empty); }
                    document.text = text;
                    editor.revision = editor.revision.saturating_add(1);
                    document.revision = editor.revision;
                    editor.phase = target;
                    Ok(())
                };
            };
            absence(refused);
            projections {
                companions;
                typestate(Phase) {
                    wrapper(Article);
                    resource(document: crate::Document);
                    runtime(editor: &mut crate::Editor);
                    refusal(crate::Refusal);
                    validate(crate::inspect);
                    methods { Publish => publish(text: String); };
                };
            };
        }
    }
}

fn main() -> Result<(), Refusal> {
    use workflow::baked::typestate::{Article, Draft, Published};
    let mut editor = Editor {
        owner: 3,
        revision: 0,
        phase: workflow::Phase::Draft,
    };
    let document = Document {
        owner: 3,
        revision: 0,
        text: String::new(),
    };
    let draft = Article::<Draft>::restore(&mut editor, document).map_err(|(_, error)| error)?;
    let Err((unchanged, empty_error)) = draft.publish(&mut editor, String::new()) else {
        return Err(Refusal::Empty);
    };
    assert_eq!(empty_error, Refusal::Empty);
    let restored_draft =
        Article::<Draft>::restore(&mut editor, unchanged).map_err(|(_, error)| error)?;
    let published = restored_draft
        .publish(&mut editor, String::from("ready to read"))
        .map_err(|(_, error)| error)?;
    assert_eq!(published.resource().text, "ready to read");
    let published_document = published.into_resource();
    let Err((current_document, phase_error)) =
        Article::<Draft>::restore(&mut editor, published_document)
    else {
        return Err(Refusal::Phase);
    };
    assert_eq!(phase_error, Refusal::Phase);
    let restored_published =
        Article::<Published>::restore(&mut editor, current_document).map_err(|(_, error)| error)?;
    assert_eq!(restored_published.resource().revision, 1);
    assert_eq!(workflow::baked::EVENT_VARIANTS.len(), 1);
    Ok(())
}
