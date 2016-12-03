extern crate clap;
extern crate prown;

use clap::{App, Arg, SubCommand};
use prown::project::Project;
use std::env;
use std::path::Path;

fn main() {
    let app = App::new("prown")
        .about("CLI app to manage project and watch files to exec command")
        .author("notkild<notkild@gmail.com>")
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
        .subcommand(SubCommand::with_name("build")
            .about("Build the current project")
            .arg(Arg::with_name("all")
                .help("Build all project")
                .short("a")
                .long("all")))
        .subcommand(SubCommand::with_name("goto")
            .about("Goto the project directory")
            .arg(Arg::with_name("name")
                .help("Name of the project")
                .required(true)
                .takes_value(true)))
        .get_matches();

    let current_dir = env::current_dir().unwrap();

    match app.subcommand() {
        ("init", Some(arg)) => {
            let path = if let Some(p) = arg.value_of("path") {
                Path::new(p).to_path_buf()
            } else {
                current_dir
            };
            Project::init(path);
        }
        ("watch", Some(_arg)) => {}
        ("build", Some(_arg)) => {}
        ("goto", Some(_arg)) => {}
        _ => {
            println!("You must run a subcommand.\nRun `prown --help` for more info.\n");
            println!("{}", app.usage());
            ::std::process::exit(1);
        }
    }
}
