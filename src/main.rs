use ansi_term::Colour;
use clap::Parser;
use std::process;

#[derive(Parser)]
struct Args {
    #[clap(parse(from_os_str), min_values(1))]
    paths: Vec<std::path::PathBuf>,

    /// Minimum size in kilobytes for OH LAWD, IT COMIN'!
    #[clap(short = 'L', long, default_value_t = 50000)]
    lawd: u64,

    /// Minimum size in kilobytes for MEGACHONKER
    #[clap(short = 'M', long, default_value_t = 10000)]
    mega: u64,

    /// Minimum size in kilobytes for HEFTYCHONK
    #[clap(short = 'H', long, default_value_t = 2000)]
    hefty: u64,

    /// Minimum size in kilobytes for A heckin' chonker
    #[clap(short = 'e', long, default_value_t = 500)]
    heck: u64,

    /// Minimum size in kilobytes for It chonk
    #[clap(short = 'c', long, default_value_t = 50)]
    chonk: u64,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.paths.len() < 1 {
        eprintln!("{}", Colour::Red.paint("Wat? No files specified 😿"));
        process::exit(exitcode::USAGE);
    }

    for path in &args.paths {
        let stat = std::fs::metadata(path)?;
        let size = stat.len();
        let chonkiness = get_chonkiness(&args, &size);
        println!("{} {} ({})",
            Colour::White.paint(format!("{}:", path.to_str().unwrap())),
            chonkiness,
            size);
    }
    Ok(())
}

const KB: u64 = 1000;

fn get_chonkiness(args: &Args, size: &u64) -> String {
    return match size {
        s if s > &(args.lawd * KB) => Colour::Red.paint("OH LAWD, IT COMIN'").to_string(),
        s if s > &(args.mega * KB) => Colour::Red.paint("MEGACHONKER").to_string(),
        s if s > &(args.hefty * KB) => Colour::Yellow.paint("HEFTYCHONK").to_string(),
        s if s > &(args.heck * KB) => Colour::Yellow.paint("A heckin' chonker").to_string(),
        s if s > &(args.chonk * KB) => Colour::Cyan.paint("It chonk").to_string(),
        _ => Colour::Green.paint("A fine file").to_string()
    };
}
