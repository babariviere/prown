use error::*;
use prown::{Module, Prown};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use toml::{Parser, Table, Value};

/// Parse prown from a TOML file
pub fn parse_prown(toml: &str, path: PathBuf) -> Result<Prown> {
    let mut modules = Vec::new();
    let mut commands = BTreeMap::new();
    let values = Parser::new(toml).parse().unwrap();

    for value in values {
        let value1 = match value.1.as_table() {
            Some(t) => t,
            None => return Err(Error::NotATable(value.0.to_string())),
        };
        if (value.0 == "commands" || value.0 == "command") && commands.is_empty() {
            commands = parse_commands(&value1);
        } else {
            modules.push(parse_module(&value.0, &value1)?);
        }
    }
    Ok(Prown::new(path, modules, commands))
}

/// Parse a single module
fn parse_module(name: &str, table: &Table) -> Result<Module> {
    let mut module = Module::new(name.clone());
    for value in table {
        let content = parse_content(value.1);
        match value.0.as_str() {
            "change" | "changes" => module.change(content)?,
            "run" => module.run(content),
            v => return Err(Error::CommandNotImplemented(v.to_string())),
        }
    }
    Ok(module)
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
pub fn parse_command<S: AsRef<str>>(command: S) -> Command {
    let command = command.as_ref();
    let mut splitted = command.split_whitespace();
    let mut command = Command::new(splitted.next().unwrap_or_default());
    for s in splitted {
        command.arg(s);
    }
    command
}

/// Parse all commands specified by the user
fn parse_commands(table: &Table) -> BTreeMap<String, String> {
    let mut tree = BTreeMap::new();
    for value in table {
        let command = value.1.as_str().unwrap();
        tree.insert(value.0.to_string(), command.to_string());
    }
    tree
}
