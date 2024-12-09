use clap::Parser;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

#[derive(Parser)]
struct Args {
    day: u32,
    part: u32,
    input: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!(
        "Executing day {:?}, part {:?}, from input in {:?}",
        args.day, args.part, args.input
    );

    let file = std::fs::File::open(args.input)?;
    let reader = std::io::BufReader::new(file);

    match args.day {
        1 => day1::doit(args.part, reader),
        2 => day2::doit(args.part, reader),
        3 => day3::doit(args.part, reader),
        4 => day4::doit(args.part, reader),
        5 => day5::doit(args.part, reader),
        6 => day6::doit(args.part, reader),
        _ => todo!(),
    }
}
