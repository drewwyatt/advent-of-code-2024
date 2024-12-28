use anyhow::Result;
use clap::Parser;
use lib::{input::read_input_for_day, solution::get_solution};

pub mod lib;
pub mod solutions;

#[derive(Parser)]
struct Args {
    #[arg(index = 1, value_name = "DAY")]
    day: u8,

    #[arg(short, long, default_value_t = 0)]
    part: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match get_solution(&args.day) {
        Some(solution) => {
            let input = read_input_for_day(args.day as usize)?;

            match args.part {
                1 => println!("[day {}][part 1]: {}", args.day, solution.part_1(&input)?),
                2 => println!("[day {}][part 2]: {}", args.day, solution.part_2(&input)?),
                _ => {
                    println!("[day {}][part 1]: {}", args.day, solution.part_1(&input)?);
                    println!("[day {}][part 2]: {}", args.day, solution.part_2(&input)?);
                }
            };
        }
        None => println!("No solution for day {}", args.day),
    };

    Ok(())
}
