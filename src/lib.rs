use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;
use uuid::Uuid;

/// Enum representing all available random string formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RandomFormat {
    Uuid,
    UuidV7,
    UrlSafe,
    ApiKey,
    MemorableName,
    HistoricalFigure,
    GeographicName,
    CharacterName,
    PhoneticAlphabet,
    RhymingPair,
    MusicalTerm,
    ScientificElement,
    ConstellationName,
    SportsReference,
    FoodCombination,
}

impl RandomFormat {
    /// Calculate the entropy in bits for this format
    pub fn entropy(&self) -> u32 {
        match self {
            RandomFormat::Uuid => 122, // 128 bits with some version/variant constraints
            RandomFormat::UuidV7 => 122, // 128 bits with time component and some constraints
            RandomFormat::UrlSafe => 95, // 16 chars from 64 possible values: 16 * log2(64) = 96 bits
            RandomFormat::ApiKey => 125, // 24 chars from 36 possible values: 24 * log2(36) = 124.1 bits
            RandomFormat::MemorableName => 30, // 36 adjectives * 35 nouns * 99 numbers = ~17 bits
            RandomFormat::HistoricalFigure => 14, // 40 figures * 999 numbers = ~15 bits
            RandomFormat::GeographicName => 24, // 42 locations * 36^3 suffixes = ~24 bits
            RandomFormat::CharacterName => 22, // 45 characters * 36^3 suffixes = ~22 bits
            RandomFormat::PhoneticAlphabet => 19, // 26^2 or 26^3 combinations * 99 numbers = ~19 bits
            RandomFormat::RhymingPair => 11,      // 21 pairs * 99 numbers = ~11 bits
            RandomFormat::MusicalTerm => 13,      // 30^1 or 30^2 terms * 99 numbers = ~13 bits
            RandomFormat::ScientificElement => 18, // 31^1 or 31^2 elements * 99 numbers = ~18 bits
            RandomFormat::ConstellationName => 21, // 26 constellations * 36^3 suffixes = ~21 bits
            RandomFormat::SportsReference => 15,  // 31 terms * 99 numbers = ~15 bits
            RandomFormat::FoodCombination => 22, // 24 adjectives * 32 foods * 99 numbers = ~22 bits
        }
    }

    /// Short canonical name used for CLI arguments and menu labels.
    pub fn short_name(&self) -> &'static str {
        match self {
            RandomFormat::Uuid => "uuid",
            RandomFormat::UuidV7 => "uuidv7",
            RandomFormat::UrlSafe => "url",
            RandomFormat::ApiKey => "api",
            RandomFormat::MemorableName => "name",
            RandomFormat::HistoricalFigure => "historical",
            RandomFormat::GeographicName => "geo",
            RandomFormat::CharacterName => "character",
            RandomFormat::PhoneticAlphabet => "phonetic",
            RandomFormat::RhymingPair => "rhyme",
            RandomFormat::MusicalTerm => "music",
            RandomFormat::ScientificElement => "element",
            RandomFormat::ConstellationName => "constellation",
            RandomFormat::SportsReference => "sports",
            RandomFormat::FoodCombination => "food",
        }
    }

    pub fn all() -> Vec<RandomFormat> {
        let mut formats = vec![
            RandomFormat::Uuid,
            RandomFormat::UuidV7,
            RandomFormat::UrlSafe,
            RandomFormat::ApiKey,
            RandomFormat::MemorableName,
            RandomFormat::HistoricalFigure,
            RandomFormat::GeographicName,
            RandomFormat::CharacterName,
            RandomFormat::PhoneticAlphabet,
            RandomFormat::RhymingPair,
            RandomFormat::MusicalTerm,
            RandomFormat::ScientificElement,
            RandomFormat::ConstellationName,
            RandomFormat::SportsReference,
            RandomFormat::FoodCombination,
        ];

        // Sort formats by entropy (highest to lowest)
        formats.sort_by(|a, b| b.entropy().cmp(&a.entropy()));

        formats
    }
}

impl fmt::Display for RandomFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandomFormat::Uuid => write!(f, "UUID ({})", self.entropy()),
            RandomFormat::UuidV7 => write!(f, "UUIDv7 ({})", self.entropy()),
            RandomFormat::UrlSafe => write!(f, "URL ({})", self.entropy()),
            RandomFormat::ApiKey => write!(f, "API ({})", self.entropy()),
            RandomFormat::MemorableName => write!(f, "NAME ({})", self.entropy()),
            RandomFormat::HistoricalFigure => write!(f, "HISTORICAL ({})", self.entropy()),
            RandomFormat::GeographicName => write!(f, "GEO ({})", self.entropy()),
            RandomFormat::CharacterName => write!(f, "CHARACTER ({})", self.entropy()),
            RandomFormat::PhoneticAlphabet => write!(f, "PHONETIC ({})", self.entropy()),
            RandomFormat::RhymingPair => write!(f, "RHYME ({})", self.entropy()),
            RandomFormat::MusicalTerm => write!(f, "MUSIC ({})", self.entropy()),
            RandomFormat::ScientificElement => write!(f, "ELEMENT ({})", self.entropy()),
            RandomFormat::ConstellationName => write!(f, "CONSTELLATION ({})", self.entropy()),
            RandomFormat::SportsReference => write!(f, "SPORTS ({})", self.entropy()),
            RandomFormat::FoodCombination => write!(f, "FOOD ({})", self.entropy()),
        }
    }
}

/// Generate a random string from a custom character set
fn from_charset(length: usize, charset: &str) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

/// Pick a random item from a slice
fn random_item<T: Clone>(items: &[T]) -> T {
    let mut rng = rand::thread_rng();
    items[rng.gen_range(0..items.len())].clone()
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// ~1-in-4 calls swaps some letters for visually similar digits (leetspeak).
/// When it fires, each eligible letter has a 50% chance of substitution, so
/// the result varies from "barely touched" to "heavily leeted".
fn maybe_leetify<R: Rng>(s: &str, rng: &mut R) -> String {
    if !rng.gen_bool(0.25) {
        return s.to_string();
    }
    s.chars()
        .map(|c| {
            let sub = match c.to_ascii_lowercase() {
                'a' => Some('4'),
                'e' => Some('3'),
                'i' => Some('1'),
                'o' => Some('0'),
                's' => Some('5'),
                't' => Some('7'),
                'g' => Some('9'),
                'l' => Some('1'),
                'b' => Some('8'),
                _ => None,
            };
            match sub {
                Some(d) if rng.gen_bool(0.5) => d,
                _ => c,
            }
        })
        .collect()
}

/// Render a wordlist-based identifier with randomized separator, per-word
/// capitalization, a random tag placed at a random position, optional
/// mixed-separator between body and tag, and optional leetspeak — so the
/// structural shape itself isn't a fingerprint.
fn render_with_words(words: &[&str]) -> String {
    let mut rng = rand::thread_rng();

    // Per-word case (each word independently lower / Title / UPPER)
    let cased: Vec<String> = words
        .iter()
        .map(|w| match rng.gen_range(0..3) {
            1 => capitalize(w),
            2 => w.to_uppercase(),
            _ => w.to_lowercase(),
        })
        .collect();

    // Body separator: empty only with mixed case, otherwise "boldgarden" blurs.
    let has_caps = cased
        .iter()
        .any(|w| w.chars().any(|c| c.is_uppercase()));
    let body_sep_options: &[&str] = if has_caps {
        &["-", "_", ".", ""]
    } else {
        &["-", "_", "."]
    };
    let body_sep = *body_sep_options.choose(&mut rng).unwrap();

    // Tag separator: independently picked, can differ from body sep — this is
    // what produces strings like "Bold_garden42" (body "_", tag glue "").
    let tag_sep = *["-", "_", ".", ""].choose(&mut rng).unwrap();

    // Random tag, varied charset + length.
    let alnum = "abcdefghijklmnopqrstuvwxyz0123456789";
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let digits = "0123456789";
    let hex = "0123456789abcdef";
    let tag = match rng.gen_range(0..5) {
        0 => {
            let max = [9u32, 99, 999, 9999][rng.gen_range(0..4)];
            rng.gen_range(1..=max).to_string()
        }
        1 => from_charset(rng.gen_range(2..=5), alnum),
        2 => from_charset(rng.gen_range(2..=4), alpha),
        3 => from_charset(rng.gen_range(3..=6), hex),
        _ => from_charset(rng.gen_range(2..=5), digits),
    };

    // Place the tag: 0 = prefix, cased.len() = suffix, anything between = wedged.
    let pos = rng.gen_range(0..=cased.len());
    let assembled = if cased.is_empty() {
        tag
    } else if pos == 0 {
        format!("{}{}{}", tag, tag_sep, cased.join(body_sep))
    } else if pos == cased.len() {
        format!("{}{}{}", cased.join(body_sep), tag_sep, tag)
    } else {
        let left = cased[..pos].join(body_sep);
        let right = cased[pos..].join(body_sep);
        format!("{}{}{}{}{}", left, tag_sep, tag, tag_sep, right)
    };

    maybe_leetify(&assembled, &mut rng)
}

/// Generate a UUID (Universally Unique Identifier)
pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Generate a UUIDv7 (Time-based Universally Unique Identifier)
pub fn uuidv7() -> String {
    Uuid::now_v7().to_string()
}

/// Generate a URL-safe random string
pub fn url_safe(length: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";
    from_charset(length, charset)
}

/// Generate a random string suitable for API keys
pub fn api_key(length: usize) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    from_charset(length, charset)
}

/// Generate a memorable name like "happy-elephant-42"
pub fn memorable_name() -> String {
    let adjectives = [
        "happy", "brave", "clever", "gentle", "wild", "calm", "bright", "swift", "silent",
        "ancient", "bold", "curious", "eager", "fierce", "graceful", "humble", "jolly", "kind",
        "lively", "mighty", "noble", "peaceful", "quick", "radiant", "serene", "tiny", "vibrant",
        "wise", "zealous", "dancing", "flying", "jumping", "running", "sleeping", "smiling",
    ];

    let nouns = [
        "elephant", "tiger", "dolphin", "eagle", "wolf", "bear", "fox", "owl", "river", "mountain",
        "forest", "ocean", "desert", "meadow", "canyon", "star", "moon", "sun", "planet", "comet",
        "galaxy", "universe", "castle", "tower", "bridge", "garden", "village", "city", "island",
        "dragon", "phoenix", "unicorn", "wizard", "knight", "warrior",
    ];

    render_with_words(&[random_item(&adjectives), random_item(&nouns)])
}

/// Generate a historical figure name like "einstein-42"
pub fn historical_figure() -> String {
    let figures = [
        "einstein",
        "newton",
        "curie",
        "darwin",
        "tesla",
        "galileo",
        "aristotle",
        "hawking",
        "edison",
        "franklin",
        "pasteur",
        "bohr",
        "faraday",
        "planck",
        "mozart",
        "beethoven",
        "bach",
        "chopin",
        "vivaldi",
        "handel",
        "debussy",
        "shakespeare",
        "dickens",
        "twain",
        "austen",
        "tolstoy",
        "hemingway",
        "leonardo",
        "michelangelo",
        "picasso",
        "vangogh",
        "rembrandt",
        "monet",
        "socrates",
        "plato",
        "confucius",
        "buddha",
        "gandhi",
        "mandela",
    ];

    render_with_words(&[random_item(&figures)])
}

/// Generate a geographic name like "paris-xj29"
pub fn geographic_name() -> String {
    let locations = [
        "paris",
        "london",
        "tokyo",
        "rome",
        "cairo",
        "sydney",
        "moscow",
        "delhi",
        "amazon",
        "nile",
        "danube",
        "thames",
        "mississippi",
        "ganges",
        "yangtze",
        "everest",
        "kilimanjaro",
        "fuji",
        "alps",
        "andes",
        "rockies",
        "himalayas",
        "sahara",
        "gobi",
        "arctic",
        "amazon",
        "serengeti",
        "outback",
        "tundra",
        "pacific",
        "atlantic",
        "indian",
        "mediterranean",
        "caribbean",
        "baltic",
        "manhattan",
        "venice",
        "kyoto",
        "marrakech",
        "santorini",
        "bali",
        "petra",
    ];

    render_with_words(&[random_item(&locations)])
}

/// Generate a character name like "gandalf-7h2"
pub fn character_name() -> String {
    let characters = [
        "gandalf",
        "frodo",
        "aragorn",
        "legolas",
        "gollum",
        "bilbo",
        "sauron",
        "skywalker",
        "vader",
        "yoda",
        "solo",
        "kenobi",
        "leia",
        "chewie",
        "r2d2",
        "holmes",
        "watson",
        "moriarty",
        "poirot",
        "marple",
        "bond",
        "potter",
        "superman",
        "batman",
        "wonderwoman",
        "spiderman",
        "hulk",
        "thor",
        "ironman",
        "mario",
        "luigi",
        "peach",
        "bowser",
        "link",
        "zelda",
        "pikachu",
        "sonic",
        "tarzan",
        "simba",
        "nemo",
        "dory",
        "woody",
        "buzz",
        "elsa",
        "moana",
    ];

    render_with_words(&[random_item(&characters)])
}

/// Generate a phonetic alphabet ID like "alpha-bravo-charlie-42"
pub fn phonetic_alphabet() -> String {
    let phonetics = [
        "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india",
        "juliet", "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo", "sierra",
        "tango", "uniform", "victor", "whiskey", "xray", "yankee", "zulu",
    ];

    let mut rng = rand::thread_rng();
    let count = rng.gen_range(2..=3);
    let words: Vec<&str> = (0..count).map(|_| random_item(&phonetics)).collect();
    render_with_words(&words)
}

/// Generate a rhyming pair like "cat-hat-42"
pub fn rhyming_pair() -> String {
    let pairs = [
        ("cat", "hat"),
        ("light", "bright"),
        ("shake", "bake"),
        ("fox", "box"),
        ("bear", "chair"),
        ("boat", "coat"),
        ("fly", "sky"),
        ("bee", "tree"),
        ("star", "far"),
        ("cake", "lake"),
        ("night", "light"),
        ("house", "mouse"),
        ("tall", "wall"),
        ("hook", "book"),
        ("king", "ring"),
        ("sun", "fun"),
        ("rock", "clock"),
        ("blue", "shoe"),
        ("rice", "nice"),
        ("car", "far"),
        ("pear", "bear"),
    ];

    let (first, second) = random_item(&pairs);
    render_with_words(&[first, second])
}

/// Generate a musical term ID like "allegro-forte-56"
pub fn musical_term() -> String {
    let terms = [
        "allegro",
        "forte",
        "piano",
        "adagio",
        "crescendo",
        "staccato",
        "legato",
        "andante",
        "vivace",
        "presto",
        "moderato",
        "largo",
        "soprano",
        "tenor",
        "bass",
        "alto",
        "treble",
        "octave",
        "sonata",
        "concerto",
        "symphony",
        "opera",
        "quartet",
        "solo",
        "major",
        "minor",
        "sharp",
        "flat",
        "chord",
        "melody",
    ];

    let mut rng = rand::thread_rng();
    let count = rng.gen_range(1..=2);
    let words: Vec<&str> = (0..count).map(|_| random_item(&terms)).collect();
    render_with_words(&words)
}

/// Generate a scientific element ID like "carbon-oxygen-42"
pub fn scientific_element() -> String {
    let elements = [
        "hydrogen",
        "helium",
        "lithium",
        "beryllium",
        "boron",
        "carbon",
        "nitrogen",
        "oxygen",
        "fluorine",
        "neon",
        "sodium",
        "magnesium",
        "aluminum",
        "silicon",
        "phosphorus",
        "sulfur",
        "chlorine",
        "argon",
        "potassium",
        "calcium",
        "titanium",
        "iron",
        "cobalt",
        "nickel",
        "copper",
        "zinc",
        "silver",
        "gold",
        "mercury",
        "lead",
        "uranium",
    ];

    let mut rng = rand::thread_rng();
    let count = rng.gen_range(1..=2);
    let words: Vec<&str> = (0..count).map(|_| random_item(&elements)).collect();
    render_with_words(&words)
}

/// Generate a constellation name like "orion-a7b"
pub fn constellation_name() -> String {
    let constellations = [
        "orion",
        "pegasus",
        "cassiopeia",
        "andromeda",
        "aquarius",
        "aries",
        "cancer",
        "capricorn",
        "gemini",
        "leo",
        "libra",
        "pisces",
        "sagittarius",
        "scorpio",
        "taurus",
        "virgo",
        "ursa",
        "draco",
        "cygnus",
        "lyra",
        "phoenix",
        "hydra",
        "centaurus",
        "perseus",
        "hercules",
        "aquila",
    ];

    render_with_words(&[random_item(&constellations)])
}

/// Generate a sports reference like "touchdown-95"
pub fn sports_reference() -> String {
    let terms = [
        "touchdown",
        "homerun",
        "slam-dunk",
        "goal",
        "strike",
        "birdie",
        "penalty",
        "foul",
        "offside",
        "serve",
        "ace",
        "backhand",
        "forehand",
        "knockout",
        "sprint",
        "marathon",
        "hurdle",
        "javelin",
        "vault",
        "basket",
        "rebound",
        "assist",
        "block",
        "tackle",
        "interception",
        "pitcher",
        "batter",
        "goalie",
        "forward",
        "defense",
        "midfielder",
    ];

    render_with_words(&[random_item(&terms)])
}

/// Generate a food combination like "spicy-taco-45"
pub fn food_combination() -> String {
    let adjectives = [
        "spicy", "sweet", "sour", "bitter", "salty", "tangy", "creamy", "crunchy", "hot", "cold",
        "fresh", "roasted", "baked", "fried", "grilled", "steamed", "juicy", "ripe", "zesty",
        "savory", "rich", "light", "hearty", "crispy",
    ];

    let foods = [
        "taco", "pizza", "burger", "pasta", "sushi", "curry", "salad", "soup", "apple", "banana",
        "orange", "grape", "cherry", "lemon", "peach", "mango", "cookie", "cake", "pie", "donut",
        "brownie", "muffin", "bread", "pastry", "cheese", "yogurt", "butter", "cream", "sauce",
        "syrup", "honey", "jam",
    ];

    render_with_words(&[random_item(&adjectives), random_item(&foods)])
}

/// Generate a random string based on the specified format
pub fn generate(format: RandomFormat) -> String {
    match format {
        RandomFormat::Uuid => uuid(),
        RandomFormat::UuidV7 => uuidv7(),
        RandomFormat::UrlSafe => url_safe(16),
        RandomFormat::ApiKey => api_key(24),
        RandomFormat::MemorableName => memorable_name(),
        RandomFormat::HistoricalFigure => historical_figure(),
        RandomFormat::GeographicName => geographic_name(),
        RandomFormat::CharacterName => character_name(),
        RandomFormat::PhoneticAlphabet => phonetic_alphabet(),
        RandomFormat::RhymingPair => rhyming_pair(),
        RandomFormat::MusicalTerm => musical_term(),
        RandomFormat::ScientificElement => scientific_element(),
        RandomFormat::ConstellationName => constellation_name(),
        RandomFormat::SportsReference => sports_reference(),
        RandomFormat::FoodCombination => food_combination(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid() {
        let s = uuid();
        assert_eq!(s.len(), 36);
        assert!(s.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    }

    #[test]
    fn test_uuidv7() {
        let s = uuidv7();
        assert_eq!(s.len(), 36);
        assert!(s.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    }

    #[test]
    fn test_url_safe() {
        let s = url_safe(10);
        assert_eq!(s.len(), 10);
        assert!(s
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_'));
    }

    #[test]
    fn test_api_key() {
        let s = api_key(10);
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c.is_uppercase() || c.is_numeric()));
    }
}
