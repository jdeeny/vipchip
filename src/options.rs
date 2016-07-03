use clap::{Arg, App};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Options {
    pub filename: String,
    pub debug: bool
}

pub fn parse_commandline() -> Options {
    let matches = App::new("vipchip")
        .version(VERSION)
        .author("jdeeny")
        .about("Emulates a chip8 system")
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Enables printing of debug information"))
        .arg(Arg::with_name("INPUT")
            .help("The input file to use")
            .required(true)
            .index(1))
        .get_matches();

        println!("{:?}", matches);

        Options {
            filename: matches.value_of("INPUT").unwrap().to_string(),
            debug: matches.is_present("debug")
        }
}
