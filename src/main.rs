use clap::Parser;
use std::{
    process::{Command, Stdio},
    thread,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    cmd: String,
    #[arg(short, long, default_value_t = 2, help = "Amount of threads to spawn")]
    threads: u16,
    #[arg(short, long, default_value_t = 1, help = "Repeat..repeat..repeat")]
    repeat: u16,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.repeat {
        let mut handlers = Vec::new();

        for _ in 0..args.threads {
            let cmd_arg = args.cmd.clone();
            let h = thread::spawn(move || {
                let cmd_parts: Vec<&str> = cmd_arg.split(' ').collect();
                let c = Command::new(cmd_parts[0])
                    .args(&cmd_parts[1..])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("could spawn process");
                let output = c.wait_with_output().expect("process to finish successful");

                let mut log = String::from("");
                if output.status.success() {
                    log = format!("{}{}", log, String::from_utf8_lossy(&output.stdout));
                } else {
                    log = format!("{}{}", log, String::from_utf8_lossy(&output.stderr));
                }
                println!("{}", log);
            });

            handlers.push(h);
        }

        for h in handlers {
            h.join().expect("thread did not die");
        }
    }
}
