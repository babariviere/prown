use error::*;
use glob::Pattern;
use notify::{DebouncedEvent, Watcher, RecursiveMode, watcher};
use parser::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

const DEFAULT_PROWN: &'static str = include_str!("../default.prown.toml");

#[derive(Debug)]
pub struct Prown {
    path: PathBuf,
    modules: Vec<Module>,
    commands: BTreeMap<String, String>,
}

impl Prown {
    pub fn new(path: PathBuf, modules: Vec<Module>, commands: BTreeMap<String, String>) -> Prown {
        Prown {
            path: path,
            modules: modules,
            commands: commands,
        }
    }

    /// Create the .prown.toml file to the path
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Prown> {
        let path = path.as_ref();
        if !path.exists() {
            let mut file = File::create(path)?;
            file.write_all(DEFAULT_PROWN.as_bytes())?;
        } else {
            println!("Prown file already exist: {}", path.display());
        }

        Ok(Prown::new(path.to_path_buf(), Vec::new(), BTreeMap::new()))
    }

    /// Parse the .prown.toml file
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Prown> {
        let mut file = File::open(&path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        parse_prown(&buf, path.as_ref().to_path_buf())
    }

    /// Run command
    pub fn run<S: AsRef<str>>(&mut self, command: S) -> Result<i32> {
        let command = command.as_ref();
        match self.commands.get(command) {
            Some(c) => {
                println!("Running command {}", c);
                let mut c = parse_command(&c);
                let output = c.output()?;
                Ok(output.status.code().unwrap())
            }
            None => Err(Error::MissingCommand(command.to_string(), self.path.clone())),
        }
    }

    /// Watch all modules
    pub fn watch(&mut self, path: PathBuf) {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

        watcher.watch(path, RecursiveMode::Recursive).unwrap();
        while let Ok(event) = rx.recv() {
            match event {
                DebouncedEvent::Write(p) |
                DebouncedEvent::NoticeWrite(p) |
                DebouncedEvent::Create(p) => {
                    for module in self.modules.iter_mut() {
                        if module.match_change(&p) {
                            println!("Running commands from module \'{}\', file matched {}",
                                     module.get_name(),
                                     p.display());
                            let commands = module.get_run();
                            for command in commands {
                                println!("  - Running command {} ...", command);
                                let mut command = parse_command(&command);
                                if let Err(e) = command.status() {
                                    println!("Error {}", e);
                                }
                            }
                            println!("");
                        }
                    }
                }
                _ => {}
            }
        }
    }
}


#[derive(Debug)]
pub struct Module {
    name: String,
    change: Vec<Pattern>,
    run: Vec<String>,
}

impl Module {
    pub fn new<S: Into<String>>(name: S) -> Module {
        Module {
            name: name.into(),
            change: Vec::new(),
            run: Vec::new(),
        }
    }

    /// Get the name of the module
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Set the change pattern to watch file
    pub fn change(&mut self, change: Vec<String>) -> Result<()> {
        for c in change {
            let pattern = Pattern::new(&format!("**/{}", c))?;
            self.change.push(pattern);
        }
        Ok(())
    }

    /// On change check if file is in module
    pub fn match_change<P: AsRef<Path>>(&self, path: P) -> bool {
        let path_str = path.as_ref().to_str().unwrap();
        for pattern in &self.change {
            if pattern.matches(path_str) {
                return true;
            }
        }
        false
    }

    /// Get all command to run
    pub fn get_run(&self) -> &Vec<String> {
        &self.run
    }

    /// Set the command to run when condition are fullfilled
    pub fn run(&mut self, run: Vec<String>) {
        self.run = run;
    }
}
