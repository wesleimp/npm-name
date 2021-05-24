extern crate npm_name as lib;
extern crate quicli;

use lib::Availability;
use quicli::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Args {
  /// The name of the package you're targeting.
  name: String,
}

fn main() {
  let args = Args::from_args();
  match lib::get(&args.name).unwrap() {
    Availability::Unavailable => println!("Unavailable."),
    Availability::Available => println!("Available."),
    Availability::Unknown => println!("Unknown status code returned."),
  };
}
