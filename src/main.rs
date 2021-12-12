extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

trait Challenge {
    fn run(&self);
}

struct DayOneChallenge {

}

impl DayOneChallenge {
    fn from<'a>(args: &'a ArgMatches) -> Self {
        let values = args.values_of("values");

        for v in values {
            println!("Found Value: {:?}", v);
        }

        println!("Initialized");

        Self {

        }
    }
}

impl Challenge for DayOneChallenge {
    fn run(&self) {
        println!("Ran");
    }
}

fn main() {
    let matches = App::new("aocode21")
        .version("2021.0.0")
        .subcommand(SubCommand::with_name("ch01")
            .arg(Arg::with_name("values")
                .multiple(true)))
        .get_matches();

    match matches.subcommand() {
        ("ch01", Some(sub_m)) => {
            DayOneChallenge::from(sub_m).run()
        },
        _ => {}
    }
}
