#[allow(unused_imports)]

mod cli;
mod build_tool;

use clap::{App, Arg, ArgMatches};
use cli::build_cli_app;

fn main() {
    let app = App::new("Web App Starter Pack CLI")
        .version("1.0")
        .author("Your Name")
        .about("Generates an application starter pack")
        .arg(
            Arg::with_name("application-type")
                .short("t")
                .long("application-type")
                .value_name("APPLICATION-TYPE")
                .help("Sets application type")
                .takes_value(true)
                .possible_values(&["web", "cli"])
                .required(true)
        )
        .arg(
            Arg::with_name("framework")
                .short("f")
                .long("framework")
                .value_name("FRAMEWORK")
                .help("Sets the web application framework")
                .takes_value(true)
                .possible_values(&["springboot", "dropwizard"])
                .required(false),
        )
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("database")
                .value_name("DATABASE")
                .help("Sets the database")
                .takes_value(true)
                .possible_values(&["postgres", "mysql", "mongodb"])
                .required(false),
        )
        .arg(
            Arg::with_name("build-tool")
                .short("b")
                .long("build-tool")
                .value_name("BUILD-TOOL")
                .help("Sets the build tool for the application")
                .takes_value(true)
                .possible_values(&["maven", "gradle"])
                .required(true)
        );

    read(app);
}

fn read(app: App) {
    let matches = app.get_matches();
    let application_type = matches.value_of("application-type");

    match application_type {
        Some(value) => {
            match value {
                "web" => {
                    web_application(matches)
                }
                "cli" => {
                    build_cli_app(matches)
                }
                _ => {
                    println!("Not a valid application-type")
                }
            }
        }
        None => {
            println!("Application type not set");
        }
    }
}

fn web_application(matches: ArgMatches) {
    println!("build web application");
    read_build_tool(matches);
}

fn read_build_tool(matches: ArgMatches) {
    let build_tool = matches.value_of("build-tool");
    match build_tool {
        Some(value) => {
            match value {
                "maven" => {
                    maven_build_tool(matches);
                }
                "gradle" => {
                    gradle_build_tool(matches);
                }
                _ => {
                    println!("{} is a valid tool", value);
                }
            }
        }
        None => {
            println!("No build tool mentioned")
        }
    }
}

fn maven_build_tool(_matches: ArgMatches) {
    println!("build with maven");
}

fn gradle_build_tool(_matches: ArgMatches) {
    println!("build with gradle");
}
