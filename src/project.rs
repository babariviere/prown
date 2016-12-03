use std::path::{Path, PathBuf};

pub struct Project {
    dir: PathBuf,
}

impl Project {
    /// Default constructor
    ///
    /// dir arg is the directory of the project
    pub fn new<P: AsRef<Path>>(dir: P) -> Project {
        Project { dir: dir.as_ref().to_path_buf() }
    }

    /// Check if the project has a prown config file
    pub fn has_prown(&self) -> bool {
        // TODO maybe need to change path
        let path = self.dir.join(".prown.toml");
        path.exists()
    }

    /// Return the prown path if it exist
    pub fn prown(&self) -> Option<PathBuf> {
        let path = self.dir.join(".prown.toml");
        if !self.has_prown() {
            return None;
        }
        Some(path)
    }

    /// Return the dir of the project
    pub fn path(&self) -> &PathBuf {
        &self.dir
    }
}

#[cfg(test)]
mod tests {
    use project::Project;
    use std::path::Path;

    #[test]
    fn pr01() {
        let project = Project::new("tests/pr01");
        assert_eq!(project.has_prown(), false);
        assert_eq!(project.prown(), None);
    }

    #[test]
    fn pr02() {
        let project = Project::new("tests/pr02");
        assert_eq!(project.has_prown(), true);
        assert_eq!(project.prown(),
                   Some(Path::new("tests/pr02/.prown.toml").to_path_buf()));
    }
}
