extern crate argparse;
extern crate slack;

pub mod msg;
pub mod read;

use std::io::{self, BufRead};
use std::process::exit;

use msg::Msg;
use read::{Args, Token};

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    if handle.read_line(&mut input).is_ok() && run_command(input).is_ok() {
        exit(0)
    }
    exit(1)
}

fn run_command(input: String) -> Result<(), ()> {
    let args = Args::read();
    let token = match Token::get() {
        Ok(Token(token)) => token,
        Err(e) => {
            if args.verbose {
                println!("{}", e)
            }
            return Err(());
        }
    };
    let text = format!("{}{}{}", args.prepend.clone(), input, args.append.clone());
    match Msg::new(text, args.channel.clone(), token).send() {
        Ok(_) => Ok(()),
        Err(e) => {
            if args.verbose {
                println!("{}", e)
            }
            Err(())
        }
    }
}
