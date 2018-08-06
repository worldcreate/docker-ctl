extern crate clap;
use clap::{Arg, App, SubCommand};

use docker_ctl::DockerCtl;

mod docker_ctl;

fn main() {
    DockerCtl::new();

    let matches = App::new("docker ctl")
                          .version("0.1")
                          .about("docker ctl")
                          .arg(Arg::with_name("v")
                               .short("v")
                               .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("test")
                                      .arg(Arg::with_name("INPUT")
                                           .help("Sets the input file to use")
                                           .required(true)
                                           .index(1)))
                          .get_matches();


    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    let is_verbose = matches.is_present("v");

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
        // required we could have used an 'if let' to conditionally get the value)
        println!("arg INPUT = {}", matches.value_of("INPUT").unwrap());
    }

}
