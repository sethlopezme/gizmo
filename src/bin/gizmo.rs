extern crate gizmo;
use gizmo::scaffolder;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand, AppSettings};

fn main() {
    let args = App::new("Gizmo")
                   .about("A static site generator written in Rust.")
                   .version(crate_version!())
                   .settings(&[AppSettings::SubcommandRequiredElseHelp,
                               AppSettings::VersionlessSubcommands])
                   .usage("gizmo [SUBCOMMAND] [FLAGS]")
                   .subcommand(SubCommand::with_name("init")
                                   .about("Scaffold a new site at the given path")
                                   .arg(Arg::with_name("path")
                                            .required(true)
                                            .value_name("PATH")
                                            .default_value(".")
                                            .help("The path for the new site"))
                                   .arg(Arg::with_name("force")
                                            .long("force")
                                            .help("Force file creation and overwrite existing \
                                                   files at the given path")))
                   .subcommand(SubCommand::with_name("generate")
                                   .about("Generate the site from source files")
                                   .arg(Arg::with_name("path")
                                            .required(true)
                                            .value_name("PATH")
                                            .default_value(".")
                                            .help("The path containing a gizmo config file")))
                   .subcommand(SubCommand::with_name("serve")
                                   .about("Serve the site locally")
                                   .arg(Arg::with_name("port")
                                            .takes_value(true)
                                            .value_name("PORT")
                                            .long("port")
                                            .help("Set the port for the server"))
                                   .arg(Arg::with_name("no-regenerate")
                                            .long("no-regenerate")
                                            .help("Don't regenerate when source files change")))
                   .get_matches();

    match args.subcommand() {
        ("init", Some(subcmd)) => {
            let path = subcmd.value_of("path").unwrap();
            let force = subcmd.is_present("force");

            println!("Beep boop. Scaffolding your new site.");

            match scaffolder::scaffold(path, force) {
                Ok(_) => println!("Beep boop. Scaffolding complete!"),
                Err(e) => panic!("ERROR: {}", e),
            }
        }
        ("generate", Some(subcmd)) => println!("Subcommand: {}", "generate"),
        ("serve", Some(subcmd)) => println!("Subcommand: {}", "serve"),
        _ => panic!("Unknown subcommand."),
    };
}
