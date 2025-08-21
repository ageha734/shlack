use argparse::{ArgumentParser, Store, StoreTrue};

use std::env;

pub struct Args {
    pub verbose: bool,
    pub channel: String,
    pub prepend: String,
    pub append: String,
}

impl Args {
    pub fn read() -> Args {
        let mut verbose = false;
        let mut channel = "slackbot".to_string();
        let mut prepend = "".to_string();
        let mut append = "".to_string();

        {
            let mut ap = ArgumentParser::new();
            ap.set_description("Pipe text into Slack.");
            ap.refer(&mut channel)
                .add_argument("channel", Store, "Channel to send to");
            ap.refer(&mut verbose)
                .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
            ap.refer(&mut prepend).add_option(
                &["-p", "--prepend"],
                Store,
                "Text to prepend to input on message",
            );
            ap.refer(&mut append).add_option(
                &["-a", "--append"],
                Store,
                "Text to append to input on message",
            );
            ap.parse_args_or_exit();
        }

        Args {
            verbose,
            channel,
            prepend,
            append,
        }
    }
}

pub struct Token(pub String);

impl Token {
    pub fn get() -> Result<Token, String> {
        let token_var = "SLACK_TOKEN";
        env::var(token_var)
            .map(Token)
            .map_err(|_| format!("environment variable {} not found", token_var))
    }
}
