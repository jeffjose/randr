# Randr - Random String Generator

A Rust library for generating various types of random strings.

## Features

Randr provides functions to generate the following types of random strings:

1. **UUID** - Universally Unique Identifiers (v4)
2. **URL** - URL-safe random strings
3. **API** - Random strings suitable for API keys
4. **NAME** - Easy to remember names like "happy-elephant-42"
5. **HISTORICAL** - Names based on historical figures like "einstein-42"
6. **GEO** - Location-based names like "paris-xj29"
7. **CHARACTER** - Fictional character names like "gandalf-7h2"
8. **PHONETIC** - NATO phonetic alphabet like "alpha-bravo-charlie-42"
9. **RHYME** - Rhyming word pairs like "cat-hat-42"
10. **MUSIC** - Musical terminology like "allegro-forte-56"
11. **ELEMENT** - Chemical elements like "carbon-oxygen-42"
12. **CONSTELLATION** - Star constellations like "orion-a7b"
13. **SPORTS** - Sports terminology like "touchdown-95"
14. **FOOD** - Food-based names like "spicy-taco-45"

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
randr = "0.1.0"
```

### Command Line Usage

```bash
# Show all random string formats in a tabular layout
cargo run

# Generate a specific format (3 examples)
cargo run -- uuid
cargo run -- name
cargo run -- food
cargo run -- sports
```

### Example Code

```rust
use randr;
use randr::RandomFormat;

fn main() {
    // Generate a UUID
    let uuid = randr::uuid();
    println!("UUID: {}", uuid);

    // Generate a URL-safe random string
    let url_safe = randr::url_safe(16);
    println!("URL: {}", url_safe);

    // Generate an API key
    let api_key = randr::api_key(24);
    println!("API: {}", api_key);

    // Generate a memorable name
    let name = randr::memorable_name();
    println!("NAME: {}", name);

    // Generate using the enum
    let element = randr::generate(RandomFormat::ScientificElement);
    println!("ELEMENT: {}", element);
}
```

## API Reference

### Basic Functions

- `uuid() -> String` - Generate a UUID (Universally Unique Identifier)
- `url_safe(length: usize) -> String` - Generate a URL-safe random string
- `api_key(length: usize) -> String` - Generate a random string suitable for API keys

### Named Formats

- `memorable_name() -> String` - Generate a memorable name like "happy-elephant-42"
- `historical_figure() -> String` - Generate a historical figure name like "einstein-42"
- `geographic_name() -> String` - Generate a geographic name like "paris-xj29"
- `character_name() -> String` - Generate a character name like "gandalf-7h2"
- `phonetic_alphabet() -> String` - Generate a phonetic alphabet ID like "alpha-bravo-charlie-42"
- `rhyming_pair() -> String` - Generate a rhyming pair like "cat-hat-42"
- `musical_term() -> String` - Generate a musical term ID like "allegro-forte-56"
- `scientific_element() -> String` - Generate a scientific element ID like "carbon-oxygen-42"
- `constellation_name() -> String` - Generate a constellation name like "orion-a7b"
- `sports_reference() -> String` - Generate a sports reference like "touchdown-95"
- `food_combination() -> String` - Generate a food combination like "spicy-taco-45"

### Generic Function

- `generate(format: RandomFormat) -> String` - Generate a random string based on the specified format

## License

This project is licensed under the MIT License - see the LICENSE file for details.
