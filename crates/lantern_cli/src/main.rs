use std::path::PathBuf;

use clap::{command, Arg, Command};

mod commands;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("analyze")
                .about("Analyzes the project")
                .arg(Arg::new("path").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("analyze", sub_matches)) => {
            let project_path_raw = sub_matches
                .get_one::<String>("path")
                .expect("path is required");
            let project_path = PathBuf::from(project_path_raw.as_str());
            let _ = commands::analyze::analyze(project_path);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
