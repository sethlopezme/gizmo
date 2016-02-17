extern crate gizmo;

#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    App::new("Gizmo")
        .about("A static site generator written in Rust.")
        .version(crate_version!())
        .subcommand(SubCommand::with_name("build")
            .about("Build the site"))
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

    println!("Hello from Gizmo!");
}
