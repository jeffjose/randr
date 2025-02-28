# Randr - Random String Generator

A command-line tool for generating various types of random strings.

## Features

Randr provides the following random string formats:

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

### Building

```bash
# Clone the repository
git clone https://github.com/yourusername/randr.git
cd randr

# Build the project
cargo build --release

# Run the application
cargo run
```

### Command Line Usage

```bash
# Show all random string formats in a tabular layout
randr

# Generate 30 examples of a specific format
randr uuid
randr name
randr food
randr sports
```

## Examples

### UUID Format

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│ 32d76965-7e57-42f4-8881-fd40fb010856   9896453c-1609-47fc-9743-d7b5cc39e6d8   695bc83b-bfdd-4010-8ee9-061f159160ac │
│ 563ad6b7-e280-4419-9b9d-d43d72b22234   914140ce-2bd1-4b6b-96d1-0117085e2b80   bb3dc8a5-94d3-4438-a266-2905df88a254 │
│ 00dd88f8-246a-4c49-811e-5cce40f9cbc1   47c168f5-b3f1-4727-af62-367efd8b2736   0e0c53c0-8aed-4bea-b7e3-ac70f91bd89f │
└──────────────────────────────────────────────────────────────────────────────────────────┘
```

### NAME Format

```
┌─────────────────────────────────────────────────────────────┐
│ fierce-village-24    jolly-wolf-80       smiling-unicorn-51 │
│ bold-city-28         swift-tower-62      bright-garden-8    │
│ jolly-phoenix-98     humble-garden-3     curious-wolf-96    │
└─────────────────────────────────────────────────────────────┘
```
