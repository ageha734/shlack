extern crate argparse;
extern crate slack;

pub mod msg;
pub mod read;

use std::io::{self, BufRead};
use std::process::exit;

use msg::Msg;
use read::Token;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    if let Ok(_) = handle.read_line(&mut input) {
        if let Ok(_) = run_command(input) {
            return exit(0)
        }
    }
    exit(1)
}

fn run_command(input: String) -> Result<(), ()> {
    let args = read::read_args();
    let token = match read::get_token() {
        Ok(Token(token)) => token,
        Err(e) => {
            if args.verbose { println!("{}", e) }
            return Err(())
        }
    };
    let text = format!("{}{}{}",
                       args.prepend.clone(),
                       input,
                       args.append.clone());
    match Msg::new(text, args.channel.clone(), token).send() {
        Ok(_) => Ok(()),
        Err(e) => {
            if args.verbose { println!("{}", e) }
            Err(())
        }
    }
}
