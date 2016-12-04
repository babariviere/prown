use error::*;
use prown::Prown;
use std::path::{Path, PathBuf};

pub struct Project {
    dir: PathBuf,
    prown: Option<Prown>,
}

impl Project {
    pub fn new(dir: PathBuf, prown: Option<Prown>) -> Project {
        Project {
            dir: dir,
            prown: prown,
        }
    }

    /// Open a project directory
    ///
    /// dir arg is the directory of the project
    pub fn open<P: AsRef<Path>>(dir: P) -> Result<Project> {
        let prown_path = dir.as_ref().join(".prown.toml");
        let prown = if !prown_path.exists() {
            None
        } else {
            Some(Prown::parse(&prown_path)?)
        };
        Ok(Project::new(dir.as_ref().to_path_buf(), prown))
    }

    /// Init project with a prown file
    pub fn init<P: AsRef<Path>>(dir: P) -> Result<Project> {
        let prown_path = dir.as_ref().join(".prown.toml");
        let prown = Prown::init(prown_path)?;
        Ok(Project::new(dir.as_ref().to_path_buf(), Some(prown)))
    }

    /// Check if the project has a prown config file
    pub fn has_prown(&self) -> bool {
        self.prown.is_some()
    }

    /// Return the prown path if it exist
    pub fn prown(&self) -> &Option<Prown> {
        &self.prown
    }

    /// Return the dir of the project
    pub fn path(&self) -> &PathBuf {
        &self.dir
    }

    /// Run command if there is a prown
    pub fn run<S: AsRef<str>>(&mut self, command: S) -> Result<i32> {
        if self.prown.is_none() {
            return Err(Error::MissingPrown(self.dir.clone()));
        }
        self.prown.as_mut().unwrap().run(command)
    }

    /// Watch file change
    pub fn watch(&mut self) -> Result<()> {
        if self.prown.is_none() {
            return Err(Error::MissingPrown(self.dir.clone()));
        }
        self.prown.as_mut().unwrap().watch(self.dir.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use project::Project;

    #[test]
    fn pr01() {
        let project = Project::new("tests/pr01");
        assert_eq!(project.has_prown(), false);
    }

    #[test]
    fn pr02() {
        let project = Project::new("tests/pr02");
        assert_eq!(project.has_prown(), true);
    }
}
