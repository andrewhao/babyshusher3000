use clap::Parser;
use include_dir::{include_dir, Dir};
use rand::prelude::*;
use rand_distr::Normal;
use soloud::*;

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let sl = Soloud::default()?;
    let mut wav = audio::Wav::default();

    static PROJECT_DIR: Dir<'_> = include_dir!("audio");
    let shh_mp3 = PROJECT_DIR.get_file("shh.mp3").unwrap();

    wav.load_mem(&shh_mp3.contents())?;

    loop {
        if args.verbose {
            println!("Shushing...");
        }

        // let jitter_ms = thread_rng().gen_range(1..JITTER_MAX);
        let poi = Normal::new(args.delay_ms as f64, args.jitter as f64).unwrap();
        let jitter_ms = poi.sample(&mut thread_rng()) as u64;

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
