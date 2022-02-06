use clap::Parser;
use include_dir::{include_dir, Dir};
use rand::prelude::*;
use rand_distr::Normal;
use soloud::*;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Duration in ms we should delay between shushes
    #[clap(short, long, default_value_t = 1000)]
    delay_ms: u64,

    /// Jitter factor to add to delays (as standard deviations on normal distribution)
    #[clap(short, long, default_value_t = 200)]
    jitter: u64,

    /// Whether to play effect once, without looping.
    #[clap(short, long, parse(from_flag))]
    one_shot: bool,

    /// Print verbose debug output to stdout.
    #[clap(short, long, parse(from_flag))]
    verbose: bool,

    /// Volume (0-10)
    #[clap(short = 'l', long, default_value_t = 10)]
    volume: u64,
}

fn validate_volume_values(level: u64) -> Result<(), &'static str> {
    if level > 10 {
        return Err("Volume level is not between 0 and 10");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    validate_volume_values(args.volume).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });

    let mut sl = Soloud::default()?;
    let mut wav = audio::Wav::default();

    static PROJECT_DIR: Dir<'_> = include_dir!("audio");
    let shh_mp3 = PROJECT_DIR.get_file("shh.mp3").unwrap();

    wav.load_mem(&shh_mp3.contents())?;

    loop {
        if args.verbose {
            println!("Shushing...");
        }

        let poi = Normal::new(args.delay_ms as f64, args.jitter as f64).unwrap();
        let jitter_ms = poi.sample(&mut thread_rng()) as u64;

        sl.set_global_volume(args.volume as f32 / 10.0);
        sl.play(&wav); // calls to play are non-blocking, so we put the thread to sleep
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        if args.one_shot {
            break;
        }

        if args.verbose {
            println!("Jitter: {}", jitter_ms);
        }
        std::thread::sleep(std::time::Duration::from_millis(jitter_ms));
    }
    Ok(())
}
