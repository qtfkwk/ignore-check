#![doc = include_str!("../README.md")]

use {
    anyhow::{anyhow, Result},
    ignore::gitignore::{gitconfig_excludes_path, Gitignore, GitignoreBuilder},
    std::{
        collections::BTreeSet,
        path::{Path, PathBuf},
    },
};

/// Check if a path is ignored
pub fn ignored(path: impl AsRef<Path>) -> Result<bool> {
    Ok(Ignore::new(path.as_ref().parent().unwrap())?.check(path))
}

struct Ignore {
    matcher: Gitignore,
}

impl Default for Ignore {
    fn default() -> Ignore {
        Ignore::new("").unwrap()
    }
}

impl Ignore {
    fn new(dir: impl AsRef<Path>) -> Result<Ignore> {
        let mut builder = GitignoreBuilder::new(&dir);

        // Add local `.gitignore` file(s)
        let mut added = BTreeSet::new();
        let mut dir = dir.as_ref().to_path_buf();
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

    fn check(&self, path: impl AsRef<Path>) -> bool {
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
        match builder.add(&path) {
            Some(e) => Err(anyhow!("Failed to add {path:?}: {e}")),
            None => {
                added.insert(path);
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}
