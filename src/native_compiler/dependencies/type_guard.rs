use super::{DependencyError, DependencyInfo, DependencyLimits};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

impl DependencyInfo {
    pub(in crate::native_compiler) fn captured(
        bytes: Vec<u8>,
        root: &Path,
        artifacts: &[PathBuf],
        limits: DependencyLimits,
    ) -> Result<Self, DependencyError> {
        let source = std::str::from_utf8(&bytes)
            .map_err(|error| DependencyError::Representation(error.to_string()))?;
        if !source.ends_with('\n') || source.contains('\0') {
            return Err(DependencyError::Representation(
                "incomplete dependency text".to_owned(),
            ));
        }
        let mut selected = None;
        for line in source
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
        {
            let Some((left, right)) = rule(line)? else {
                continue;
            };
            let target = crate::native_compiler::source::physical(root, &left.replace("\\ ", " "))
                .map_err(|error| DependencyError::Representation(format!("{error:?}")))?;
            if !artifacts.contains(&target) {
                continue;
            }
            if selected.is_some() {
                return Err(DependencyError::Artifact);
            }
            selected = Some((target, paths(right, root, limits.files)?));
        }
        let (artifact, files) = selected.ok_or(DependencyError::Artifact)?;
        if files.is_empty() {
            return Err(DependencyError::Artifact);
        }
        Ok(Self {
            bytes,
            artifact,
            files,
        })
    }

    /// The exact retained dependency-file bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The link artifact named by both compiler output and this dependency rule.
    #[must_use]
    pub fn artifact(&self) -> &Path {
        &self.artifact
    }

    /// The observed source paths resolved against the invocation's declared working directory.
    #[must_use]
    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }
}

fn rule(line: &str) -> Result<Option<(&str, &str)>, DependencyError> {
    if let Some(parts) = line.split_once(": ") {
        return Ok(Some(parts));
    }
    if line.ends_with(':') {
        return Ok(None);
    }
    Err(DependencyError::Representation(
        "unsupported dependency rule".to_owned(),
    ))
}

fn paths(source: &str, root: &Path, maximum: usize) -> Result<Vec<PathBuf>, DependencyError> {
    let mut paths = BTreeSet::new();
    let mut word = String::new();
    let mut characters = source.chars().peekable();
    while let Some(character) = characters.next() {
        if character == '\\' && characters.peek().is_some_and(|next| *next == ' ') {
            let _escaped_space = characters.next();
            word.push(' ');
        } else if character == ' ' {
            insert(&mut paths, &mut word, root, maximum)?;
        } else if character.is_control() {
            return Err(DependencyError::Representation(
                "control character in dependency path".to_owned(),
            ));
        } else {
            word.push(character);
        }
    }
    insert(&mut paths, &mut word, root, maximum)?;
    Ok(paths.into_iter().collect())
}

fn insert(
    paths: &mut BTreeSet<PathBuf>,
    word: &mut String,
    root: &Path,
    maximum: usize,
) -> Result<(), DependencyError> {
    if word.is_empty() {
        return Ok(());
    }
    if paths.len() >= maximum {
        return Err(DependencyError::FileBound);
    }
    let path = crate::native_compiler::source::physical(root, word)
        .map_err(|error| DependencyError::Representation(format!("{error:?}")))?;
    if !paths.insert(path) {
        return Err(DependencyError::Representation(
            "duplicate source path".to_owned(),
        ));
    }
    word.clear();
    Ok(())
}
