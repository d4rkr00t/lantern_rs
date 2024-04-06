use std::path::PathBuf;

use clap::{command, Arg, Command};

mod commands;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("unused_exports")
                .about("Find unused exports in a project")
                .arg(
                    Arg::new("path")
                        .required(true)
                        .num_args(1..)
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("depgraph")
                .about("Build a dependency graph for a project")
                .arg(
                    Arg::new("path")
                        .required(true)
                        .num_args(1..)
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("affected")
                .about("Find affected files in a project")
                .arg(
                    Arg::new("entries")
                        .long("entries")
                        .required(true)
                        .num_args(1..)
                        .value_parser(clap::value_parser!(PathBuf)),
                )
                .arg(
                    Arg::new("changed")
                        .long("changed")
                        .required(true)
                        .num_args(1..)
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("unused_exports", sub_matches)) => {
            let entry_points = sub_matches
                .get_many::<PathBuf>("path")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            commands::unused_exports::analyze(entry_points).unwrap();
        }
        Some(("depgraph", sub_matches)) => {
            let entry_points = sub_matches
                .get_many::<PathBuf>("path")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            commands::depgraph::build(entry_points).unwrap();
        }
        Some(("affected", sub_matches)) => {
            let entry_points = sub_matches
                .get_many::<PathBuf>("entries")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            let changed = sub_matches
                .get_many::<PathBuf>("changed")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Entries: {:?}, Changed: {:?}", entry_points, changed);
            commands::affected::analyze(entry_points, changed).unwrap();
        }
        _ => {
            unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
        }
    }
}
