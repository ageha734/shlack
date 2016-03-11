extern crate argparse;
extern crate slack;

pub mod send;

use std::io;
use std::process::exit;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match run_command(input) {
                Ok(_) => exit(0),
                Err(_) => exit(1),
            }
        },
        Err(_) => {
            exit(1)
        }
    }
}

fn read_args() -> (bool, String, String, String) {
    let mut verbose = false;
    let mut channel = "slackbot".to_string();
    let mut prepend = "".to_string();
    let mut append = "".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Pipe text into Slack.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
                        "Be verbose");
        ap.refer(&mut channel)
            .add_option(&["-c", "--channel"], Store,
                        "Channel to send message to");
        ap.refer(&mut prepend)
            .add_option(&["-p", "--prepend"], Store,
                        "Text to prepend to input on message");
        ap.refer(&mut append)
            .add_option(&["-a", "--append"], Store,
                        "Text to append to input on message");
        ap.parse_args_or_exit();
    }

    (verbose, channel, prepend, append)
}

fn run_command(input: String) -> Result<(), ()> {
    let (verbose, channel, prepend, append) = read_args();

    let text = format!("{}{}{}", prepend, input, append);
    if verbose {
        println!("{}: {}", &channel, &text);
    }

    match send::send(&text, &channel) {
        Ok(_) => Ok(()),
        Err(e) => {
            if verbose { println!("{}", e) }
            Err(())
        }
    }
}
