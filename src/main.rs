use std::{
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wait-change")]
#[command(author = "Tsubasa Irisawa <lambdasawa@gmail.com>")]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    interval: u64,

    command: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let first_stdout = run_command(&args);

    loop {
        let current_output = run_command(&args);

        println!("first: {:?}, current {:?}", first_stdout, current_output);

        if first_stdout != current_output {
            break;
        }
    }
}

fn run_command(args: &Args) -> String {
    let output = Command::new(&args.command[0])
        .args(&args.command[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    sleep(Duration::from_secs(args.interval));

    let stdout = String::from_utf8(output.stdout).expect("Failed to decode as utf8");

    println!("{}", stdout);

    stdout
}
