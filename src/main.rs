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
    #[arg(short = 'i', long = "interval", default_value_t = 1)]
    interval: u64,

    #[arg(short = 'n', long = "count", default_value_t = 0)]
    count: u64,

    #[arg(short = 's', long = "show", default_value_t = false)]
    show: bool,

    #[arg(short = 'k', long = "stop-on-error", default_value_t = false)]
    stop_on_error: bool,

    #[arg(trailing_var_arg = true)]
    command_string: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    let iterations = if args.count > 0 { args.count } else { u64::MAX };

    if args.command_string.len() == 0 {
        println!("missing command, run --help for info");
        return;
    }

    let actual_command_parts: Vec<String>;
    if args.command_string.len() == 1 {
        actual_command_parts = args.command_string[0]
            .split(' ')
            .map(|s| s.to_string())
            .collect();
    } else {
        actual_command_parts = args.command_string;
    }

    let command = actual_command_parts.first().expect("missing command");
    let arguments: Vec<String> = if actual_command_parts.len() > 1 {
        actual_command_parts[1..].to_vec()
    } else {
        Vec::new()
    };

    'outer: for i in 0..iterations {
        if io::stdout().is_terminal() {
            print!("{esc}c", esc = 27 as char);
        }

        let start_us = std::time::SystemTime::now();
        let output = Command::new(command).args(&arguments).output();
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
