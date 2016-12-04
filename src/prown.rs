use glob::Pattern;
use notify::{DebouncedEvent, Watcher, RecursiveMode, watcher};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
use toml::{Parser, Table, Value};

const DEFAULT_PROWN: &'static str = include_str!("../default.prown.toml");

#[derive(Debug)]
pub struct Prown {
    modules: Vec<Module>,
    commands: BTreeMap<String, Command>,
}

impl Prown {
    /// Create the .prown.toml file to the path
    pub fn init<P: AsRef<Path>>(path: P) -> Prown {
        let path = path.as_ref();
        if !path.exists() {
            let mut file = File::create(path).expect("Error when creating init file");
            file.write_all(DEFAULT_PROWN.as_bytes())
                .expect("Error when writing default prown template");
        } else {
            println!("Prown file already exist: {}", path.display());
        }

        Prown {
            modules: Vec::new(),
            commands: BTreeMap::new(),
        }
    }

    /// Parse the .prown.toml file
    pub fn parse<P: AsRef<Path>>(path: P) -> Prown {
        let mut file = File::open(path).expect("Error when opening prown file");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Error when reading prown file");
        parse_prown(&buf)
    }

    /// Run the build command
    pub fn run<S: AsRef<str>>(&mut self, command: S) -> Option<i32> {
        let command = command.as_ref();
        match self.commands.get_mut(command) {
            Some(c) => {
                let output = c.output().expect("Failed to execute command");
                output.status.code()
            }
            None => {
                println!("There is no {0} command, add `{0} = \"<command>\" to the .prown.toml \
                          file in [commands]",
                         command);
                None
            }
        }
    }

    /// Watch all modules
    pub fn watch(&mut self, path: PathBuf) {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

        watcher.watch(path, RecursiveMode::Recursive).unwrap();
        loop {
            match rx.recv() {
                Ok(event) => {
                    match event {
                        DebouncedEvent::Write(p) |
                        DebouncedEvent::NoticeWrite(p) |
                        DebouncedEvent::NoticeRemove(p) |
                        DebouncedEvent::Create(p) => {
                            for module in self.modules.iter_mut() {
                                if module.match_change(&p) {
                                    println!("File match for module {}", module.get_name());
                                    module.run_commands();
                                }
                            }
                        }
                        e => {
                            println!("Event {:?}", e);
                        }
                    }
                }
                Err(e) => panic!("Error {}", e),

            }
        }
    }
}

/// Parse modules from a TOML file
fn parse_prown(toml: &str) -> Prown {
    let mut modules = Vec::new();
    let mut commands = BTreeMap::new();
    let values = Parser::new(toml).parse().unwrap();

    for value in values {
        if (value.0 == "commands" || value.0 == "command") && commands.is_empty() {
            commands = parse_commands(&value.1.as_table().unwrap());
        } else {
            modules.push(parse_module(&value.0, &value.1.as_table().unwrap()));
        }
    }
    Prown {
        modules: modules,
        commands: commands,
    }
}

/// Parse all commands specified by the user
fn parse_commands(table: &Table) -> BTreeMap<String, Command> {
    let mut tree = BTreeMap::new();
    for value in table {
        let string = value.1.as_str().unwrap();
        let command = parse_command(string.to_string());
        tree.insert(value.0.to_string(), command);
    }
    tree
}

/// Parse a single module
fn parse_module(name: &str, table: &Table) -> Module {
    let mut module = Module::new(name.clone());
    for value in table {
        let content = parse_content(value.1);
        match value.0.as_str() {
            "change" | "changes" => module.change(content),
            "run" => module.run(content),
            v => panic!("Command {} is not supported yet", v),
        }
    }
    module
}

/// Parse content from a table
fn parse_content(value: &Value) -> Vec<String> {
    match *value {
        Value::String(ref s) => {
            let mut vec = Vec::new();
            vec.push(s.to_string());
            vec
        }
        Value::Array(ref vec) => {
            vec.iter()
                .map(|value| value.as_str().expect("Value should be a string").to_string())
                .collect()
        }
        ref v => panic!("Value {} is not supported", v),
    }
}

/// Parse a command from string
fn parse_command(command: String) -> Command {
    let mut splitted = command.split_whitespace();
    let mut command = Command::new(splitted.next().unwrap_or_default());
    for s in splitted {
        command.arg(s);
    }
    command
}

#[derive(Debug)]
struct Module {
    name: String,
    change: Vec<Pattern>,
    run: Vec<Command>,
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
    pub fn change(&mut self, change: Vec<String>) {
        self.change = change.iter().map(|s| Pattern::new(&format!("**/{}", s)).unwrap()).collect()
    }

    /// On change check if file is in module
    pub fn match_change<P: AsRef<Path>>(&self, path: P) -> bool {
        let path_str = path.as_ref().to_str().unwrap();
        for pattern in &self.change {
            println!("Matching {} with {}", pattern, path_str);
            if pattern.matches(path_str) {
                return true;
            }
        }
        false
    }

    /// Set the command to run when condition are fullfilled
    pub fn run(&mut self, run: Vec<String>) {
        let mut commands = Vec::new();
        for r in run {
            commands.push(parse_command(r));
        }
        self.run = commands;
    }

    /// Run commands
    pub fn run_commands(&mut self) {
        for command in self.run.iter_mut() {
            command.output().unwrap();
        }
    }
}
