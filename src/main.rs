mod surahs;
mod cli;

use rand::Rng;
use surahs::SURAHS;
use cli::Cli;

use clap::Parser;

fn main() {
    let args = Cli::parse();
    match args.command {
        cli::Cmd::Config(config) => {
            println!("Config Command:");
            let sfrom = match config.from_surah {
                Some(num) => num,
                None => 1,
            };
            let afrom = match config.from_ayah {
                Some(num) => num,
                None => 1,
            };
            let supto = match config.upto_surah {
                Some(num) => num,
                None => SURAHS.len(),
            };
            let aupto = match config.upto_ayah {
                Some(num) => num,
                None => SURAHS[supto - 1].1,
            };
            // Now we need to write these to a config.json file
            let file = std::fs::File::create("config.json").expect("Unable to create config file");
            let config_data = serde_json::json!({
                "from_surah": sfrom,
                "from_ayah": afrom,
                "upto_surah": supto,
                "upto_ayah": aupto,
            });
            serde_json::to_writer_pretty(file, &config_data).expect("Unable to write config data");
            println!("Configuration saved to config.json");
        },
        cli::Cmd::Generate(generate) => {
            println!("Generate Command:");
            let n = match generate.number {
                Some(num) => num,
                None => 1,
            };
            println!("Generating {} questions...", n);
            let mut rng = rand::rng();
            let config = std::fs::read_to_string("config.json").expect("Unable to read config file, maybe you need to run the config command first");
            let config_data: serde_json::Value = serde_json::from_str(&config).expect("Unable to parse config data");
            let sfrom = config_data["from_surah"].as_u64().unwrap() as usize;
            let afrom = config_data["from_ayah"].as_u64().unwrap() as usize;
            let supto = config_data["upto_surah"].as_u64().unwrap() as usize;
            let aupto = config_data["upto_ayah"].as_u64().unwrap() as usize;
            let srand = rng.random_range(sfrom..=supto);
            let arand = if srand == sfrom && srand == supto {
                rng.random_range(afrom..=aupto)
            } else if srand == sfrom {
                rng.random_range(afrom..=SURAHS[srand - 1].1)
            } else if srand == supto {
                rng.random_range(1..=aupto)
            } else {
                rng.random_range(1..=SURAHS[srand - 1].1)
            };
            println!("Randomly selected: Surah {} Ayah {}", srand, arand);
        },
    }
}
