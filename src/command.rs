use project::Project;
use std::path::Path;

pub fn init<P: AsRef<Path>>(path: P) -> Result<(), ::std::io::Error> {
    let project = Project::init(path);
    // TODO add to config file
    Ok(())
}
