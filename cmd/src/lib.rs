extern crate sniffer;

use clap::{App, Arg, ArgMatches};
use std::process;
mod vars;

fn get_new_snyfy_cmd() -> ArgMatches {
    App::new("Snyfy")
        .version("1.0")
        .author("Mayank Shah <hello@mayankshah.dev>")
        .about("A simple port sniffer built using rust")
        .arg(
            Arg::with_name("IP")
                .help("Sets the IPv4 Address of the target machine")
                .required(true)
                .index(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("num")
                .short('n')
                .long("num")
                .takes_value(true)
                .help("Sets the number of threads to use. Default is 1000."),
        )
        .arg(
            Arg::with_name("min")
                .short('l')
                .long("min")
                .takes_value(true)
                .help("Sets a lower bound on the range of ports to scan. Default is 0"),
        )
        .arg(
            Arg::with_name("max")
                .short('u')
                .long("max")
                .takes_value(true)
                .help("Sets a lower bound on the range of ports to scan. Default is 65535"),
        )
        .get_matches()
}

pub fn run_snyfy_cmd() {
    let cmd = get_new_snyfy_cmd();

    match vars::Opts::from(&cmd) {
        Ok(opts) => {
            let threads = opts.n;
            let ip = opts.ip;
            let lower = opts.lower;
            let upper = opts.upper;
            sniffer::sniff(ip, threads, lower, upper);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(0);
        }
    };
}
