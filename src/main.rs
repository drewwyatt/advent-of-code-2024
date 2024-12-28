use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(index = 1, value_name = "DAY")]
    day: u8,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

fn main() {
    let args = Args::parse();

    println!("Day: {}", args.day);
    println!("Part: {}", args.part);
}
