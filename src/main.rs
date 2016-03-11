extern crate argparse;
extern crate slack;

pub mod send;

use std::io;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
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

        let complete_text = format!("{}{}{}", prepend, input.trim(), append);

        if verbose {
            println!("{}: {}", &channel, &complete_text);
        }

        let msg = send::Msg {
            text: complete_text,
            channel: channel,
        };

        if let Err(error) = send::send(msg) {
            if verbose {
                println!("{}", error);
            }
        }
    };
}
