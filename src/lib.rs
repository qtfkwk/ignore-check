#![doc = include_str!("../README.md")]

use {
    anyhow::{Result, anyhow},
    ignore::gitignore::{Gitignore, GitignoreBuilder, gitconfig_excludes_path},
    std::{
        collections::BTreeSet,
        path::{Path, PathBuf},
    },
};

/**
Check if a path is ignored

# Panics

Panics if the given path has no parent

# Errors

Returns an error if the given path fails to be processed
*/
pub fn ignored(path: impl AsRef<Path>) -> Result<bool> {
    Ok(Ignore::new(path.as_ref().parent().unwrap())?.check(path))
}

pub struct Ignore {
    matcher: Gitignore,
}

impl Default for Ignore {
    fn default() -> Ignore {
        Ignore::new("").unwrap()
    }
}

impl Ignore {
    /**
    # Errors

    Returns an error if the given path fails to be processed
    */
    pub fn new(root: impl AsRef<Path>) -> Result<Ignore> {
        let mut builder = GitignoreBuilder::new(&root);

        // Add local `.gitignore` file(s)
        let mut added = BTreeSet::new();
        let mut dir = root.as_ref().to_path_buf();
        loop {
            add_path(dir.join(".gitignore"), &mut builder, &mut added)?;

            if let Some(parent) = dir.parent() {
                dir = parent.to_path_buf();
            } else {
                break;
            }
        }

        // Add global (user) excludes path (`~/.gitignore`)
        if let Some(path) = gitconfig_excludes_path() {
            add_path(path, &mut builder, &mut added)?;
        }

        Ok(Ignore {
            matcher: builder.build()?,
        })
    }

    pub fn check(&self, path: impl AsRef<Path>) -> bool {
        self.matcher
            .matched_path_or_any_parents(&path, path.as_ref().is_dir())
            .is_ignore()
    }
}

fn add_path(
    path: PathBuf,
    builder: &mut GitignoreBuilder,
    added: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    if path.exists() && !added.contains(&path) {
        if let Some(e) = builder.add(&path) {
            Err(anyhow!("Failed to add {:?}: {e}", path.display()))
        } else {
            added.insert(path);
            Ok(())
        }
    } else {
        Ok(())
    }
}
