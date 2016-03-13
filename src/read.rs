use argparse::{ArgumentParser, StoreTrue, Store};

use std::env;
use std::io;

pub struct Args {
    pub verbose: bool,
    pub channel: String,
    pub prepend: String,
    pub append: String,
}

pub struct Token(pub String);

pub fn get_or_set_token() -> Result<Token, String> {
    let key = "SLACK_TOKEN";
    match env::var(&key) {
        Ok(token) => Ok(Token(token)),
        Err(_) => {
            println!("slack API token: ");
            let mut token = String::new();
            try!(io::stdin()
                 .read_line(&mut token)
                 .map_err(|_| "error reading input"));
            env::set_var(&key, &token);
            Ok(Token(token))
        },
    }
}

pub fn read_args() -> Args {
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

    Args {
        verbose: verbose,
        channel: channel,
        prepend: prepend,
        append: append,
    }
}
