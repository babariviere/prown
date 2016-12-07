use APP_INFO;
use app_dirs::{AppDataType, app_root};
use error::*;
use prown::Prown;
use std::ffi::CString;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct ProjectManager {
    projects: Vec<Project>,
}

impl ProjectManager {
    pub fn new(projects: Vec<Project>) -> ProjectManager {
        ProjectManager { projects: projects }
    }

    /// Load projects list from default configuration file
    pub fn load() -> Result<ProjectManager> {
        let path = app_root(AppDataType::UserConfig, &APP_INFO)?.join("projects_list.txt");
        let projects = read_projects_file(&path)?;
        Ok(ProjectManager::new(projects))
    }

    /// Goto the project directory
    pub fn goto(&self, project_name: &str) -> Option<PathBuf> {
        println!("Not working yet");
        for project in &self.projects {
            // FIXME
        }
        None
    }

    /// Add project to the list and write it to config path
    pub fn add_project(&mut self, project: Project) -> Result<()> {
        self.projects.push(project);
        self.save()
    }

    /// Save project list
    pub fn save(&self) -> Result<()> {
        let mut buf = String::new();
        for project in &self.projects {
            let path = project.path().canonicalize()?;
            buf.push_str(&format!("{}\n", path.display()));
        }
        let path = app_root(AppDataType::UserConfig, &APP_INFO)?.join("projects_list.txt");
        let mut file = File::create(&path)?;
        file.write_all(buf.as_bytes())?;
        Ok(())
    }
}

/// Read all projects from file
fn read_projects_file<P: AsRef<Path>>(path: P) -> Result<Vec<Project>> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Err(PError::MissingProjectList),
    };
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let mut projects = Vec::new();
    for s in buf.split_whitespace() {
        match Project::open(s) {
            Ok(p) => projects.push(p),
            Err(_) => continue,
        }
    }
    Ok(projects)
}

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

    /// Return the project name
    pub fn name(&self) -> &str {
        &self.dir.file_name().unwrap().to_str().unwrap()
    }

    /// Run command if there is a prown
    pub fn run<S: AsRef<str>>(&mut self, command: S) -> Result<i32> {
        if self.prown.is_none() {
            return Err(PError::MissingPrown(self.dir.clone()));
        }
        self.prown.as_mut().unwrap().run(command)
    }

    /// Watch file change
    pub fn watch(&mut self) -> Result<()> {
        if self.prown.is_none() {
            return Err(PError::MissingPrown(self.dir.clone()));
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
