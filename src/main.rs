extern crate clap;
extern crate prown;

use clap::{App, AppSettings, Arg, SubCommand};
use prown::project::Project;
use std::env;
use std::path::Path;

fn main() {
    let app = App::new("prown")
        .about("CLI app to manage project and watch files to exec command")
        .author("notkild<notkild@gmail.com>")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("init")
            .about("Init project with prown")
            .arg(Arg::with_name("path")
                .help("Define the project path to init")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("watch")
            .about("Watch project")
            .arg(Arg::with_name("all")
                .help("Watch all project")
                .short("a")
                .long("all")))
        .subcommand(SubCommand::with_name("goto")
            .about("Goto the project directory")
            .arg(Arg::with_name("name")
                .help("Name of the project")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("run")
            .about("Run a command declared in toml file")
            .arg(Arg::with_name("command")
                .help("Command to execute")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("all")
                .help("Run all project with command")
                .short("a")
                .long("all")))
        .get_matches();

    let current_dir = env::current_dir().unwrap();

    match app.subcommand() {
        ("init", Some(arg)) => {
            let path = if let Some(p) = arg.value_of("path") {
                Path::new(p).to_path_buf()
            } else {
                current_dir
            };
            Project::init(path).unwrap();
        }
        ("watch", Some(_arg)) => {
            let mut project = Project::open(current_dir).unwrap();
            let error = project.watch().err();
            if error.is_some() {
                println!("{}", error.unwrap());
            }
        }
        ("goto", Some(_arg)) => {}
        ("run", Some(arg)) => {
            let command = arg.value_of("command").unwrap();
            let mut project = match Project::open(current_dir) {
                Ok(p) => p,
                Err(e) => {
                    println!("{}", e);
                    ::std::process::exit(1);
                }
            };
            println!("Running command {}", command);
            let status = project.run(&command);
            match status {
                Ok(s) => println!("Command {} exited with code", s),
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        _ => unreachable!(),
    }
}
