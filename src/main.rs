extern crate argparse;
extern crate slack;

pub mod read;
pub mod send;

use std::io;
use std::process::exit;

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        if let Ok(_) = run_command(input) {
            return exit(1)
        }
    }
    exit(1)
}

fn run_command(input: String) -> Result<(), ()> {
    let args = read::read_args();

    let text = format!("{}{}{}",
                       args.prepend.clone(),
                       input,
                       args.append.clone());

    match send::send(&text, &args.channel) {
        Ok(_) => Ok(()),
        Err(e) => {
            if args.verbose { println!("{}", e) }
            Err(())
        }
    }
}
