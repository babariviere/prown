use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use toml::{Parser, Value};

const DEFAULT_PROWN: &'static str = include_str!("../default.prown.toml");

#[derive(Debug)]
pub struct Prown {
    modules: Vec<Module>,
}

impl Prown {
    /// Create the .prown.toml file to the path
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Prown, ::std::io::Error> {
        let path = path.as_ref();
        if !path.exists() {
            let mut file = File::create(path)?;
            file.write_all(DEFAULT_PROWN.as_bytes())?;
        } else {
            println!("Prown file already exist: {}", path.display());
        }

        Ok(Prown { modules: Vec::new() })
    }

    /// Parse the .prown.toml file
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Prown, ::std::io::Error> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let modules = parse_modules(&buf);
        Ok(Prown { modules: modules })
    }
}

/// Parse modules from a TOML file
// TODO return Err on error
fn parse_modules(toml: &str) -> Vec<Module> {
    let mut modules = Vec::new();
    let values = Parser::new(toml).parse().unwrap();
    for name in values {
        println!("{}:", name.0);
        let mut module = Module::new(name.0.clone());
        let table = name.1.as_table().expect(&format!("{} is not a table", name.0));
        for value in table {
            let content = match *value.1 {
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
            };
            // Only for debug
            println!("  {}:", value.0);
            for s in &content {
                println!("    - {}", s);
            }
            match value.0.as_str() {
                "change" | "changes" => module.change(content),
                "run" => module.run(content),
                v => panic!("Command {} is not supported yet", v),
            }
        }
        modules.push(module);
    }
    Vec::new()
}

#[derive(Debug)]
struct Module {
    name: String,
    change: Vec<String>,
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

    pub fn change(&mut self, change: Vec<String>) {
        self.change = change;
    }

    pub fn run(&mut self, run: Vec<String>) {
        let mut commands = Vec::new();
        for r in run {
            let mut splitted = r.split_whitespace();
            let mut command = Command::new(splitted.next().unwrap());
            for s in splitted {
                command.arg(s);
            }
            commands.push(command);
        }
        self.run = commands;
    }
}
