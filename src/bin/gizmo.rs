extern crate gizmo;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand, AppSettings};

fn main() {
    let args = App::new("Gizmo")
                   .about("A static site generator written in Rust.")
                   .version(crate_version!())
                   .settings(&[AppSettings::SubcommandRequiredElseHelp,
                               AppSettings::VersionlessSubcommands])
                   .subcommand(SubCommand::with_name("new")
                                   .about("Scaffold a new site at the given path")
                                   .arg(Arg::with_name("name")
                                            .takes_value(true)
                                            .value_name("PATH")
                                            .required(true)
                                            .help("The location of the scaffolded site")))
                   .subcommand(SubCommand::with_name("build").about("Build the site"))
                   .subcommand(SubCommand::with_name("serve")
                                   .about("Serve the site locally")
                                   .arg(Arg::with_name("port")
                                            .long("port")
                                            .takes_value(true)
                                            .value_name("PORT")
                                            .help("Set the port for the server"))
                                   .arg(Arg::with_name("no-watch")
                                            .long("no-watch")
                                            .help("Don't watch source files for changes")))
                   .get_matches();

    match args.subcommand() {
        ("new", Some(subcmd)) => println!("Subcommand: {}", "new"),
        ("build", Some(subcmd)) => println!("Subcommand: {}", "build"),
        ("serve", Some(subcmd)) => println!("Subcommand: {}", "serve"),
        _ => panic!(),
    }
}
