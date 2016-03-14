extern crate argparse;
extern crate slack;

pub mod read;
pub mod send;

use std::io::{self, BufRead};
use std::process::exit;

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
        Ok(read::Token(token)) => token,
        Err(e) => {
            if args.verbose { println!("{}", e) }
            return Err(())
        }
    };
    let text = format!("{}{}{}",
                       args.prepend.clone(),
                       input,
                       args.append.clone());
    let msg = send::Msg::new(text, args.channel.clone(), token);
    match send::send(msg) {
        Ok(_) => Ok(()),
        Err(e) => {
            if args.verbose { println!("{}", e) }
            Err(())
        }
    }
}
