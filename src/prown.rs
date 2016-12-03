use glob::Pattern;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use toml::{Parser, Table, Value};

const DEFAULT_PROWN: &'static str = include_str!("../default.prown.toml");

#[derive(Debug)]
pub struct Prown {
    modules: Vec<Module>,
    build: Option<Command>,
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
            build: None,
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
    pub fn build(&mut self) {
        if self.build.is_none() {
            println!("There is no build command, add `build = \"<command>\" to the .prown.toml \
                      file");
            return;
        }
        let mut build = self.build.as_mut().unwrap();
        build.spawn().unwrap().wait();
    }
}

/// Parse modules from a TOML file
fn parse_prown(toml: &str) -> Prown {
    let mut modules = Vec::new();
    let mut build = None;
    let values = Parser::new(toml).parse().unwrap();

    for value in values {
        match value.1 {
            Value::Table(ref t) => modules.push(parse_module(&value.0, t)),
            Value::String(s) => {
                if value.0 != "build" {
                    panic!("Unknown param {}", value.0);
                }
                build = Some(parse_command(s));
            }
            v => panic!("Unexpected {:?}", v),
        }
    }
    Prown {
        modules: modules,
        build: build,
    }
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

    /// Set the change pattern to watch file
    pub fn change(&mut self, change: Vec<String>) {
        self.change = change.iter().map(|s| Pattern::new(s).unwrap()).collect()
    }

    pub fn match_change<P: AsRef<Path>>(&self, path: P) -> bool {
        let path_str = path.as_ref().to_str().unwrap();
        for pattern in &self.change {
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
}
