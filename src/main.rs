use clap::{Parser, arg};
use std::{process::Command, thread::sleep, time::Duration};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'i', long = "interval", default_value_t = 0)]
    interval: u64,

    #[arg(short = 'n', long = "count", default_value_t = 1)]
    count: u64,

    #[arg(short = 's', long = "show", default_value_t = false)]
    show: bool,

    #[arg(long = "command")]
    command_string: String,
}

fn main() {
    let args = Cli::parse();
    let parts: Vec<&str> = args.command_string.split_whitespace().collect();

    for i in 0..args.count {
        if let Some(command) = parts.first() {
            let arguments = &parts[1..];

            print!("{esc}c", esc = 27 as char);
            let output = Command::new(command).args(arguments).output();

            if args.show {
                println!(
                    "iter: {}, every: {}s, command: {}\n",
                    i + 1,
                    args.interval,
                    command
                )
            }

            match output {
                Ok(o) => {
                    if o.status.success() {
                        println!("{}", String::from_utf8_lossy(&o.stdout));
                    } else {
                        eprintln!("{}", String::from_utf8_lossy(&o.stderr));
                    }
                }
                Err(e) => {
                    eprintln!("Error executing command: {}", e);
                }
            }
            sleep(Duration::from_secs(args.interval));
        }
    }
}
