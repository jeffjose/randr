use clap::Parser;
use prettytable::{format, row, Table};
use randr::{self, RandomFormat};
use terminal_size::{terminal_size, Width};

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
            // Generate 30 strings
            let mut strings = Vec::new();
            for _ in 0..30 {
                strings.push(randr::generate(format));
            }

            // Create a table for the grid display
            let mut table = Table::new();

            // Use a format with only outer borders for the grid
            let table_format = format::FormatBuilder::new()
                .column_separator(' ')
                .borders('│')
                .separators(
                    &[format::LinePosition::Top],
                    format::LineSeparator::new('─', '─', '┌', '┐'),
                )
                .separators(
                    &[format::LinePosition::Bottom],
                    format::LineSeparator::new('─', '─', '└', '┘'),
                )
                .padding(1, 1)
                .build();

            table.set_format(table_format);

            // Add rows to the table (3 columns by 10 rows)
            let columns = 3;
            let rows = (strings.len() + columns - 1) / columns; // Ceiling division

            for i in 0..rows {
                let mut row_cells = Vec::new();
                for j in 0..columns {
                    let index = i * columns + j;
                    if index < strings.len() {
                        row_cells.push(strings[index].clone());
                    } else {
                        row_cells.push(String::new());
                    }
                }
                table.add_row(row![row_cells[0], row_cells[1], row_cells[2]]);
            }

            // Print the table
            table.printstd();
        } else {
            println!("Unknown format: {}", format);
            println!("Available formats:");
            for format in RandomFormat::all() {
                println!("  - {}", format);
            }
        }
    } else {
        // Get terminal width
        let width = terminal_size().map(|(Width(w), _)| w).unwrap_or(80);

        // Determine number of columns based on terminal width
        // Assume each mini-table needs about 40 characters of width
        let columns = std::cmp::max(1, width as usize / 40);

        // Create the main table
        let mut main_table = Table::new();

        // Use a format with borders all around but no internal separators
        let main_format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders('│')
            .separators(
                &[format::LinePosition::Top],
                format::LineSeparator::new('─', '─', '┌', '┐'),
            )
            .separators(
                &[format::LinePosition::Bottom],
                format::LineSeparator::new('─', '─', '└', '┘'),
            )
            .padding(1, 1)
            .build();

        main_table.set_format(main_format);

        // Format for the inner tables
        let inner_format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders('│')
            .separators(
                &[format::LinePosition::Top],
                format::LineSeparator::new('─', '─', '┌', '┐'),
            )
            .separators(
                &[format::LinePosition::Bottom],
                format::LineSeparator::new('─', '─', '└', '┘'),
            )
            .padding(1, 1)
            .build();

        // Get all formats
        let all_formats = RandomFormat::all();
        let rows = (all_formats.len() + columns - 1) / columns; // Ceiling division

        // Add rows to the main table
        for i in 0..rows {
            let mut row_cells = Vec::new();

            for j in 0..columns {
                let index = i * columns + j;

                if index < all_formats.len() {
                    let format = all_formats[index];

                    // Generate 3 strings for this format
                    let mut strings = Vec::new();
                    for _ in 0..3 {
                        strings.push(randr::generate(format));
                    }

                    // Create inner table for this format
                    let mut inner_table = Table::new();
                    inner_table.set_format(inner_format.clone());

                    // Add format name as header
                    inner_table.add_row(row![format.to_string()]);

                    // Add each string as a row
                    for s in &strings {
                        inner_table.add_row(row![s]);
                    }

                    row_cells.push(inner_table.to_string());
                } else {
                    row_cells.push(String::new());
                }
            }

            // Add the row to the main table
            if columns == 1 {
                main_table.add_row(row![row_cells[0]]);
            } else if columns == 2 {
                if row_cells.len() > 1 {
                    main_table.add_row(row![row_cells[0], row_cells[1]]);
                } else {
                    main_table.add_row(row![row_cells[0], ""]);
                }
            } else if columns == 3 {
                if row_cells.len() > 2 {
                    main_table.add_row(row![row_cells[0], row_cells[1], row_cells[2]]);
                } else if row_cells.len() > 1 {
                    main_table.add_row(row![row_cells[0], row_cells[1], ""]);
                } else {
                    main_table.add_row(row![row_cells[0], "", ""]);
                }
            } else {
                // For 4 or more columns, we need to build the row dynamically
                let row_string = row_cells.join(" ");
                main_table.add_row(row![row_string]);
            }
        }

        // Print the main table
        main_table.printstd();
    }
}
