use clap::Parser;
use prettytable::{format, row, Table};
use randr::{self, RandomFormat};
use std::collections::HashMap;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The format to generate (if not specified, all formats will be shown)
    format: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(format) = &cli.format {
        // Find the format by name (case-insensitive)
        let format_lower = format.to_lowercase();
        let selected_format = RandomFormat::all()
            .into_iter()
            .find(|f| format!("{}", f).to_lowercase().replace(' ', "") == format_lower);

        if let Some(format) = selected_format {
            println!("{}:", format);
            for _ in 0..3 {
                println!("  {}", randr::generate(format));
            }
        } else {
            println!("Unknown format: {}", format);
            println!("Available formats:");
            for format in RandomFormat::all() {
                println!("  - {}", format);
            }
        }
    } else {
        // Generate 3 random strings for each format and display in a compact format
        println!("Random String Generator Examples");

        // Generate all the strings first
        let mut format_strings: HashMap<RandomFormat, Vec<String>> = HashMap::new();
        for format in RandomFormat::all() {
            let mut strings = Vec::new();
            for _ in 0..3 {
                strings.push(randr::generate(format));
            }
            format_strings.insert(format, strings);
        }

        // Create a table for displaying the formats
        let mut table = Table::new();

        // Use a clean format without borders for a compact display
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        // Add header row
        table.add_row(row!["FORMAT", "EXAMPLES"]);

        // Add data rows
        for format in RandomFormat::all() {
            if let Some(strings) = format_strings.get(&format) {
                table.add_row(row![format.to_string(), strings.join(", ")]);
            }
        }

        // Print the table
        table.printstd();
    }
}
