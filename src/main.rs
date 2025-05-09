use clap::{Parser, arg};
use retry::pretty_print;
use std::{
    io::{self, IsTerminal},
    process::Command,
    thread::sleep,
    time::Duration,
};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'i', long = "interval", default_value_t = 0)]
    interval: u64,

    #[arg(short = 'n', long = "count", default_value_t = 1)]
    count: u64,

    #[arg(short = 's', long = "show", default_value_t = false)]
    show: bool,

    #[arg(short = 'k', long = "stop-on-error", default_value_t = false)]
    stop_on_error: bool,

    #[arg(long = "command")]
    command_string: String,
}

fn main() {
    let args = Cli::parse();
    let parts: Vec<&str> = args.command_string.split_whitespace().collect();

    'outer: for i in 0..args.count {
        if let Some(command) = parts.first() {
            let arguments = &parts[1..];

            if io::stdout().is_terminal() {
                print!("{esc}c", esc = 27 as char);
            }

            let start_us = std::time::SystemTime::now();
            let output = Command::new(command).args(arguments).output();
            let end = std::time::SystemTime::now()
                .duration_since(start_us)
                .expect("to get current time");

            if args.show {
                println!(
                    "iter: {}, every: {}s, last duration: {}, command: {} {}\n",
                    i + 1,
                    args.interval,
                    pretty_print::duration(end),
                    command,
                    arguments.join(" "),
                )
            }

            match output {
                Ok(o) => {
                    println!("{}", String::from_utf8_lossy(&o.stdout));
                    eprintln!("{}", String::from_utf8_lossy(&o.stderr));
                    if let Some(code) = o.status.code() {
                        if code != 0 && args.stop_on_error {
                            println!("Stopping further iterations due to error.");
                            break 'outer;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error executing command: {}", e);
                    if args.stop_on_error {
                        println!("Stopping further iterations due to error.");
                        break 'outer;
                    }
                }
            }
            sleep(Duration::from_secs(args.interval));
        }
    }
}
