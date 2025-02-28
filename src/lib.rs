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

/// Generate a random number as a string
fn random_number(min: u32, max: u32) -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max).to_string()
}

/// Pick a random item from a slice
fn random_item<T: Clone>(items: &[T]) -> T {
    let mut rng = rand::thread_rng();
    items[rng.gen_range(0..items.len())].clone()
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

    format!(
        "{}-{}-{}",
        random_item(&adjectives),
        random_item(&nouns),
        random_number(1, 99)
    )
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

    format!("{}-{}", random_item(&figures), random_number(1, 999))
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

    let suffix = from_charset(3, "abcdefghijklmnopqrstuvwxyz0123456789");

    format!("{}-{}", random_item(&locations), suffix)
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

    let suffix = from_charset(3, "abcdefghijklmnopqrstuvwxyz0123456789");

    format!("{}-{}", random_item(&characters), suffix)
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

    let mut words = Vec::with_capacity(count);
    for _ in 0..count {
        words.push(random_item(&phonetics));
    }

    format!("{}-{}", words.join("-"), random_number(1, 99))
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

    format!("{}-{}-{}", first, second, random_number(1, 99))
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

    let mut words = Vec::with_capacity(count);
    for _ in 0..count {
        words.push(random_item(&terms));
    }

    format!("{}-{}", words.join("-"), random_number(1, 99))
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

    let mut words = Vec::with_capacity(count);
    for _ in 0..count {
        words.push(random_item(&elements));
    }

    format!("{}-{}", words.join("-"), random_number(1, 99))
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

    let suffix = from_charset(3, "abcdefghijklmnopqrstuvwxyz0123456789");

    format!("{}-{}", random_item(&constellations), suffix)
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

    format!("{}-{}", random_item(&terms), random_number(1, 99))
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

    format!(
        "{}-{}-{}",
        random_item(&adjectives),
        random_item(&foods),
        random_number(1, 99)
    )
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
