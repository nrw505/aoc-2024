use clap::Parser;

pub mod day1;

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
        1 => day1::day1(args.part, reader),
        _ => todo!(),
    }
}
