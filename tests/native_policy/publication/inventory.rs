use super::fixture::{self, FIRST, LIMITS, SECOND};
use super::types::Seat;
use macroonz::compiler::stamp::{PublicationGround, PublishedStamp, planned};
use macroonz::native_publication::{
    InventoryError, LandingBinding, Publication, PublicationBinding, PublicationLimits,
    PublicationPath, published_digest,
};

#[test]
fn sealed_units_and_complete_landings_keep_original_order_and_material() -> Result<(), String> {
    let (expansion, stamp) = fixture::expanded([7, 9, 42])?;
    let identity = expansion.identity();
    let original = expansion
        .published()
        .map(macroonz::compiler::RenderedUnit::digest)
        .collect::<Vec<_>>();
    let publication = Publication::declared(
        expansion,
        &fixture::bindings("generated/definition.rs", "generated/other.rs")?,
        LIMITS,
    )
    .map_err(|error| format!("{error:?}"))?
    .with_stamp(stamp, &fixture::landings()?)
    .map_err(|error| format!("{error:?}"))?;
    let files = publication.files().collect::<Vec<_>>();
    assert_eq!(
        files
            .iter()
            .map(|file| file.path().spelling())
            .collect::<Vec<_>>(),
        [
            "generated/definition.rs",
            "generated/alpha.rs",
            "generated/beta.rs",
            "generated/other.rs"
        ]
    );
    assert_eq!(
        files.iter().map(|file| file.site()).collect::<Vec<_>>(),
        [None, Some("alpha"), Some("beta"), None]
    );
    assert_eq!(publication.expansion().identity(), identity);
    assert_eq!(
        publication
            .expansion()
            .published()
            .map(macroonz::compiler::RenderedUnit::digest)
            .collect::<Vec<_>>(),
        original
    );
    let other = files.last().ok_or("other file absent")?;
    assert_eq!(other.source(), "pub const OTHER : u8 = 42 ; \n");
    assert_eq!(other.unit().address(), Some(SECOND));
    let alpha = files
        .iter()
        .find(|file| file.site() == Some("alpha"))
        .ok_or("alpha absent")?;
    assert!(alpha.source().contains('7'));
    assert_eq!(alpha.unit().address(), Some(FIRST));
    assert_ne!(
        other.canonical_digest().as_bytes(),
        published_digest(&other.tokens().canonical_bytes()).as_bytes()
    );
    assert_ne!(
        published_digest(b"pub const OTHER:u8=42;"),
        published_digest(b"pub const OTHER: u8 = 42;\n")
    );
    Ok(())
}

#[test]
fn destination_binding_refuses_missing_foreign_duplicate_and_aliasing_paths() -> Result<(), String>
{
    let invalid = [
        vec![],
        vec![PublicationBinding {
            address: FIRST,
            path: fixture::path("one.rs")?,
        }],
        vec![
            PublicationBinding {
                address: FIRST,
                path: fixture::path("one.rs")?,
            },
            PublicationBinding {
                address: FIRST,
                path: fixture::path("two.rs")?,
            },
        ],
    ];
    for bindings in invalid {
        assert_eq!(
            Publication::declared(fixture::expanded([7, 9, 42])?.0, &bindings, LIMITS).err(),
            Some(InventoryError::Binding)
        );
    }
    let mut foreign = fixture::bindings("one.rs", "two.rs")?;
    foreign.first_mut().ok_or("binding absent")?.address.bytes = [3; 32];
    assert_eq!(
        Publication::declared(fixture::expanded([7, 9, 42])?.0, &foreign, LIMITS).err(),
        Some(InventoryError::Binding)
    );
    for (first, second) in [
        ("one.rs", "ONE.rs"),
        ("file", "file/child.rs"),
        ("file/child.rs", "FILE"),
    ] {
        assert_eq!(
            Publication::declared(
                fixture::expanded([7, 9, 42])?.0,
                &fixture::bindings(first, second)?,
                LIMITS
            )
            .err(),
            Some(InventoryError::PathCollision)
        );
    }
    for path in [
        "../x.rs",
        "/x.rs",
        "a\\b.rs",
        "C:x.rs",
        "a//b.rs",
        "a/./b.rs",
        "NUL.rs",
        "COM2.txt",
        "x.",
        "a/space name.rs",
        ".macroonz-publication/state",
        "x\0.rs",
    ] {
        assert_eq!(
            PublicationPath::informed(path),
            Err(InventoryError::Path),
            "{path:?}"
        );
    }
    assert_eq!(
        fixture::path("Upper_name-1.rs")?.spelling(),
        "Upper_name-1.rs"
    );
    Ok(())
}

#[test]
fn stamp_binding_refuses_another_record_definition_or_landing_set() -> Result<(), String> {
    let (record_expansion, _) = fixture::expanded([7, 9, 42])?;
    let (_, other_stamp) = fixture::expanded([8, 9, 42])?;
    let record_publication = Publication::declared(
        record_expansion,
        &fixture::bindings("definition.rs", "other.rs")?,
        LIMITS,
    )
    .map_err(|error| format!("{error:?}"))?;
    assert_eq!(
        record_publication
            .with_stamp(other_stamp, &fixture::landings()?)
            .err(),
        Some(InventoryError::Stamp)
    );
    let (definition_expansion, _) = fixture::expanded([7, 9, 42])?;
    let decision = planned(definition_expansion.plan(), Seat::Definition)
        .map_err(|error| error.to_string())?;
    let changed = PublishedStamp::rendered(
        &decision,
        &fixture::stamp("different_definition", [7, 9])?,
        PublicationGround::CrossFileArtifact,
    )
    .map_err(|error| error.to_string())?;
    let definition_publication = Publication::declared(
        definition_expansion,
        &fixture::bindings("definition.rs", "other.rs")?,
        LIMITS,
    )
    .map_err(|error| format!("{error:?}"))?;
    assert_eq!(
        definition_publication
            .with_stamp(changed, &fixture::landings()?)
            .err(),
        Some(InventoryError::Stamp)
    );
    for names in [
        vec![],
        vec!["alpha"],
        vec!["alpha", "alpha"],
        vec!["alpha", "foreign"],
    ] {
        let (expansion, stamp) = fixture::expanded([7, 9, 42])?;
        let publication = Publication::declared(
            expansion,
            &fixture::bindings("definition.rs", "other.rs")?,
            LIMITS,
        )
        .map_err(|error| format!("{error:?}"))?;
        let bindings = names
            .into_iter()
            .map(|site| {
                Ok(LandingBinding {
                    site: site.to_owned(),
                    path: fixture::path(&format!("{site}.rs"))?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        assert_eq!(
            publication.with_stamp(stamp, &bindings).err(),
            Some(InventoryError::Landing)
        );
    }
    Ok(())
}

#[test]
fn landings_share_total_limits_and_cannot_overwrite_another_destination() -> Result<(), String> {
    let (expansion, stamp) = fixture::expanded([7, 9, 42])?;
    let publication = Publication::declared(
        expansion,
        &fixture::bindings("definition.rs", "other.rs")?,
        PublicationLimits {
            files: 3,
            bytes: 65536,
        },
    )
    .map_err(|error| format!("{error:?}"))?;
    assert_eq!(
        publication.with_stamp(stamp, &fixture::landings()?).err(),
        Some(InventoryError::FileBound)
    );
    assert_eq!(
        Publication::declared(
            fixture::expanded([7, 9, 42])?.0,
            &fixture::bindings("definition.rs", "other.rs")?,
            PublicationLimits { files: 8, bytes: 1 }
        )
        .err(),
        Some(InventoryError::ByteBound)
    );
    let (collision_expansion, collision_stamp) = fixture::expanded([7, 9, 42])?;
    let mut bindings = fixture::landings()?;
    bindings.first_mut().ok_or("landing absent")?.path = fixture::path("OTHER.rs")?;
    let collision_publication = Publication::declared(
        collision_expansion,
        &fixture::bindings("definition.rs", "other.rs")?,
        LIMITS,
    )
    .map_err(|error| format!("{error:?}"))?;
    assert_eq!(
        collision_publication
            .with_stamp(collision_stamp, &bindings)
            .err(),
        Some(InventoryError::PathCollision)
    );
    Ok(())
}
