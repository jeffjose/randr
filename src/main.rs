use clap::Parser;
use rand::seq::SliceRandom;
use rand::Rng;
use randr::{self, RandomFormat};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Specific format to generate. Omit to get the themed menu.
    format: Option<String>,

    /// Number of strings to generate (only applies when a format is specified).
    #[arg(short = 'n', long, default_value_t = 10)]
    count: usize,
}

/// Fixed themes shown on every menu run. Each theme groups related formats so
/// the picks under one theme share a vibe but differ in concrete shape. Pool
/// is sampled so each distinct format appears at least once per row (when
/// possible), then remaining slots are random.
const THEMES: &[(&str, &[RandomFormat])] = &[
    ("id", &[RandomFormat::UuidV7, RandomFormat::Uuid]),
    ("token", &[RandomFormat::UrlSafe, RandomFormat::ApiKey]),
    (
        "memorable",
        &[RandomFormat::MemorableName, RandomFormat::FoodCombination],
    ),
    (
        "place",
        &[RandomFormat::GeographicName, RandomFormat::ConstellationName],
    ),
    (
        "character",
        &[RandomFormat::CharacterName, RandomFormat::HistoricalFigure],
    ),
];

const PICKS_PER_THEME: usize = 3;

fn pick_formats<R: Rng>(pool: &[RandomFormat], n: usize, rng: &mut R) -> Vec<RandomFormat> {
    let mut shuffled = pool.to_vec();
    shuffled.shuffle(rng);
    let mut picks: Vec<RandomFormat> = shuffled.into_iter().take(n).collect();
    while picks.len() < n {
        picks.push(*pool.choose(rng).unwrap());
    }
    picks.shuffle(rng);
    picks
}

fn print_themed_menu() {
    let mut rng = rand::thread_rng();
    let label_w = THEMES.iter().map(|(name, _)| name.len()).max().unwrap_or(0);

    for (label, pool) in THEMES {
        let picks = pick_formats(pool, PICKS_PER_THEME, &mut rng);
        let values: Vec<String> = picks.iter().map(|&f| randr::generate(f)).collect();
        println!(
            "{:<width$}   {}",
            label,
            values.join("   "),
            width = label_w
        );
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(format) = &cli.format {
        let wanted = format.to_lowercase();
        let selected = RandomFormat::all()
            .into_iter()
            .find(|f| f.short_name() == wanted);

        match selected {
            Some(fmt) => {
                for _ in 0..cli.count {
                    println!("{}", randr::generate(fmt));
                }
            }
            None => {
                eprintln!("unknown format: {}", format);
                eprintln!("available:");
                for fmt in RandomFormat::all() {
                    eprintln!("  {}", fmt.short_name());
                }
                std::process::exit(1);
            }
        }
    } else {
        print_themed_menu();
    }
}
