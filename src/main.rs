use clap::Parser;
use rand::seq::SliceRandom;
use randr::{self, RandomFormat, Style};
use terminal_size::{terminal_size, Width};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Specific format to generate. Omit to get the themed menu.
    format: Option<String>,

    /// Number of items (only applies to format-specific mode; default fills
    /// about 5 rows at your terminal width).
    #[arg(short = 'n', long)]
    count: Option<usize>,
}

/// Fixed themes shown on every menu run. Each section picks one format from
/// its pool, picks one Style, and generates many items using that locked
/// style — so within a section you scan uniform candidates, but each section
/// looks distinctly different from the others.
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

/// Cap how tall any one section can get. Items per section = cols × this,
/// where cols is whatever fits the format at the terminal width — so wide
/// formats (UUID) get a few candidates, narrow ones get more, but both
/// stay short vertically.
const MAX_ROWS_PER_SECTION: usize = 2;
const COL_GAP: usize = 2;

fn term_width() -> usize {
    terminal_size().map(|(Width(w), _)| w as usize).unwrap_or(80)
}

/// Print a heading + grid of items. Items are assumed to share roughly the
/// same width (because the same Style was used to produce them).
fn print_grid(items: &[String], indent: usize, width: usize) {
    let max = items.iter().map(|s| s.chars().count()).max().unwrap_or(0);
    let cols = std::cmp::max(1, width.saturating_sub(indent) / (max + COL_GAP));
    let pad = " ".repeat(indent);
    for chunk in items.chunks(cols) {
        let line: Vec<String> = chunk
            .iter()
            .map(|s| format!("{:<w$}", s, w = max))
            .collect();
        println!("{}{}", pad, line.join(&" ".repeat(COL_GAP)));
    }
}

fn generate_batch(format: RandomFormat, style: &Style, n: usize) -> Vec<String> {
    (0..n).map(|_| randr::generate_with_style(format, style)).collect()
}

/// Rough estimate of an item's width so we can size sections before
/// generating. Slight under/over-estimation is fine — the grid uses the
/// actual measured width when laying out.
fn estimated_width(format: RandomFormat) -> usize {
    match format {
        RandomFormat::Uuid | RandomFormat::UuidV7 => 36,
        RandomFormat::ApiKey => 24,
        RandomFormat::UrlSafe => 16,
        _ => 22,
    }
}

fn print_themed_menu() {
    let mut rng = rand::thread_rng();
    let width = term_width();
    let indent = 2;

    for (theme, pool) in THEMES {
        let format = *pool.choose(&mut rng).unwrap();
        let style = Style::random();

        // Generate a starter batch using the format's estimated width, then
        // re-measure actual content and grow the batch to fill the row at
        // that real width. Keeps narrow formats from showing a half-empty row.
        let est = estimated_width(format);
        let est_cols = std::cmp::max(1, width.saturating_sub(indent) / (est + COL_GAP));
        let mut items = generate_batch(format, &style, est_cols * MAX_ROWS_PER_SECTION);

        let actual_max = items.iter().map(|s| s.chars().count()).max().unwrap_or(est);
        let actual_cols =
            std::cmp::max(1, width.saturating_sub(indent) / (actual_max + COL_GAP));
        let target = actual_cols * MAX_ROWS_PER_SECTION;
        if target > items.len() {
            items.extend(
                (items.len()..target).map(|_| randr::generate_with_style(format, &style)),
            );
        } else {
            items.truncate(target);
        }

        println!("\n{} ({})", theme, format.short_name());
        print_grid(&items, indent, width);
    }
}

fn print_format(format: RandomFormat, count: Option<usize>) {
    let style = Style::random();
    let width = term_width();
    let est = estimated_width(format);
    let cols = std::cmp::max(1, width / (est + COL_GAP));
    let n = count.unwrap_or(cols * 3);
    let items = generate_batch(format, &style, n);
    print_grid(&items, 0, width);
}

fn main() {
    let cli = Cli::parse();

    if let Some(format) = &cli.format {
        let wanted = format.to_lowercase();
        let selected = RandomFormat::all()
            .into_iter()
            .find(|f| f.short_name() == wanted);

        match selected {
            Some(fmt) => print_format(fmt, cli.count),
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
