use std::fs;
use std::process::exit;
use std::{thread, time};
use clap::{Arg, App, ArgMatches};
use toml::Value;

mod snaps;

static ONE_MINUTE: u64 = 60;
static ONE_HOUR: u64 = 60 * 60;

fn parse_args() -> ArgMatches<'static> {
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("config")
             .long("config")
             .takes_value(true)
             .help("Use this config file"))
        .get_matches();
    return args;
}

fn read_config(config: &str) -> Value {
    let contents = fs::read_to_string(config);
    let config_str = match contents {
        Ok(x)   => x,
        Err(x)  => {
            println!("Could not open file {}: {}", config, x);
            exit(1);
        }
    };
    let conf = config_str.parse::<Value>().unwrap();
    return conf;
}

fn main() {
    let args = parse_args();
    let conf = read_config(args.value_of("config").unwrap_or("/etc/autosnap.toml"));
    println!("{}", conf["main"]["uris"].as_str().expect("config has no value for hosts"));

    let interval = time::Duration::from_secs(1);
    loop {
        let start = time::Instant::now();
        snaps::do_snaps(&conf);
        thread::sleep(interval - start.elapsed());
    }
}
