use std::env;
use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("input: {}", input);

            let args: Vec<_> = env::args().collect();
            if args.len() > 0 {
                for (i, arg) in args.iter().enumerate().skip(1) {
                    println!("arg {}: {}", i, arg);
                }
            }
        },
        Err(error) => println!("error: {}", error),
    };
}
