use clap::Parser;

mod machine;

use machine::Machine;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    machine_string: String,
}

fn main() {
    let args = Args::parse();
    let mut machine = Machine::new(&args.machine_string);
    let mut i = 1;
    while machine.step() {
        i += 1;
    }
    println!("Halted after {} steps", i);
}
