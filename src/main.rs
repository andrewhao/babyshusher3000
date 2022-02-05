use include_dir::{include_dir, Dir};
use rand::prelude::*;
use rand_distr::Normal;
use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const JITTER_MEAN: f64 = 1000.0;
    let sl = Soloud::default()?;
    let mut wav = audio::Wav::default();

    static PROJECT_DIR: Dir<'_> = include_dir!("audio");
    let shh_mp3 = PROJECT_DIR.get_file("shh.mp3").unwrap();

    // wav.load(&std::path::Path::new("shh.mp3"))?;
    wav.load_mem(&shh_mp3.contents())?;

    loop {
        println!("Shushing...");
        // let jitter_ms = thread_rng().gen_range(1..JITTER_MAX);
        let poi = Normal::new(JITTER_MEAN, 200.0).unwrap();
        let jitter = poi.sample(&mut thread_rng()) as u64;

        sl.play(&wav); // calls to play are non-blocking, so we put the thread to sleep
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        println!("Jitter: {}", jitter);
        std::thread::sleep(std::time::Duration::from_millis(jitter));
    }
}
