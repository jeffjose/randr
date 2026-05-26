use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;
use uuid::Uuid;

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
    pub fn entropy(&self) -> u32 {
        match self {
            RandomFormat::Uuid => 122,
            RandomFormat::UuidV7 => 122,
            RandomFormat::UrlSafe => 95,
            RandomFormat::ApiKey => 125,
            RandomFormat::MemorableName => 30,
            RandomFormat::HistoricalFigure => 14,
            RandomFormat::GeographicName => 24,
            RandomFormat::CharacterName => 22,
            RandomFormat::PhoneticAlphabet => 19,
            RandomFormat::RhymingPair => 11,
            RandomFormat::MusicalTerm => 13,
            RandomFormat::ScientificElement => 18,
            RandomFormat::ConstellationName => 21,
            RandomFormat::SportsReference => 15,
            RandomFormat::FoodCombination => 22,
        }
    }

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
        formats.sort_by_key(|f| std::cmp::Reverse(f.entropy()));
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

// ---- Wordlists ----------------------------------------------------------

const ADJECTIVES: &[&str] = &[
    "happy", "brave", "clever", "gentle", "wild", "calm", "bright", "swift", "silent",
    "ancient", "bold", "curious", "eager", "fierce", "graceful", "humble", "jolly", "kind",
    "lively", "mighty", "noble", "peaceful", "quick", "radiant", "serene", "tiny", "vibrant",
    "wise", "zealous", "dancing", "flying", "jumping", "running", "sleeping", "smiling",
];

const NOUNS: &[&str] = &[
    "elephant", "tiger", "dolphin", "eagle", "wolf", "bear", "fox", "owl", "river", "mountain",
    "forest", "ocean", "desert", "meadow", "canyon", "star", "moon", "sun", "planet", "comet",
    "galaxy", "universe", "castle", "tower", "bridge", "garden", "village", "city", "island",
    "dragon", "phoenix", "unicorn", "wizard", "knight", "warrior",
];

const FIGURES: &[&str] = &[
    "einstein", "newton", "curie", "darwin", "tesla", "galileo", "aristotle", "hawking",
    "edison", "franklin", "pasteur", "bohr", "faraday", "planck", "mozart", "beethoven",
    "bach", "chopin", "vivaldi", "handel", "debussy", "shakespeare", "dickens", "twain",
    "austen", "tolstoy", "hemingway", "leonardo", "michelangelo", "picasso", "vangogh",
    "rembrandt", "monet", "socrates", "plato", "confucius", "buddha", "gandhi", "mandela",
];

const LOCATIONS: &[&str] = &[
    "paris", "london", "tokyo", "rome", "cairo", "sydney", "moscow", "delhi", "amazon",
    "nile", "danube", "thames", "mississippi", "ganges", "yangtze", "everest", "kilimanjaro",
    "fuji", "alps", "andes", "rockies", "himalayas", "sahara", "gobi", "arctic", "amazon",
    "serengeti", "outback", "tundra", "pacific", "atlantic", "indian", "mediterranean",
    "caribbean", "baltic", "manhattan", "venice", "kyoto", "marrakech", "santorini", "bali",
    "petra",
];

const CHARACTERS: &[&str] = &[
    "gandalf", "frodo", "aragorn", "legolas", "gollum", "bilbo", "sauron", "skywalker",
    "vader", "yoda", "solo", "kenobi", "leia", "chewie", "r2d2", "holmes", "watson",
    "moriarty", "poirot", "marple", "bond", "potter", "superman", "batman", "wonderwoman",
    "spiderman", "hulk", "thor", "ironman", "mario", "luigi", "peach", "bowser", "link",
    "zelda", "pikachu", "sonic", "tarzan", "simba", "nemo", "dory", "woody", "buzz", "elsa",
    "moana",
];

const PHONETICS: &[&str] = &[
    "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india",
    "juliet", "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo",
    "sierra", "tango", "uniform", "victor", "whiskey", "xray", "yankee", "zulu",
];

const RHYME_PAIRS: &[(&str, &str)] = &[
    ("cat", "hat"), ("light", "bright"), ("shake", "bake"), ("fox", "box"),
    ("bear", "chair"), ("boat", "coat"), ("fly", "sky"), ("bee", "tree"),
    ("star", "far"), ("cake", "lake"), ("night", "light"), ("house", "mouse"),
    ("tall", "wall"), ("hook", "book"), ("king", "ring"), ("sun", "fun"),
    ("rock", "clock"), ("blue", "shoe"), ("rice", "nice"), ("car", "far"),
    ("pear", "bear"),
];

const MUSICAL: &[&str] = &[
    "allegro", "forte", "piano", "adagio", "crescendo", "staccato", "legato", "andante",
    "vivace", "presto", "moderato", "largo", "soprano", "tenor", "bass", "alto", "treble",
    "octave", "sonata", "concerto", "symphony", "opera", "quartet", "solo", "major", "minor",
    "sharp", "flat", "chord", "melody",
];

const ELEMENTS: &[&str] = &[
    "hydrogen", "helium", "lithium", "beryllium", "boron", "carbon", "nitrogen", "oxygen",
    "fluorine", "neon", "sodium", "magnesium", "aluminum", "silicon", "phosphorus", "sulfur",
    "chlorine", "argon", "potassium", "calcium", "titanium", "iron", "cobalt", "nickel",
    "copper", "zinc", "silver", "gold", "mercury", "lead", "uranium",
];

const CONSTELLATIONS: &[&str] = &[
    "orion", "pegasus", "cassiopeia", "andromeda", "aquarius", "aries", "cancer", "capricorn",
    "gemini", "leo", "libra", "pisces", "sagittarius", "scorpio", "taurus", "virgo", "ursa",
    "draco", "cygnus", "lyra", "phoenix", "hydra", "centaurus", "perseus", "hercules",
    "aquila",
];

const SPORTS: &[&str] = &[
    "touchdown", "homerun", "slamdunk", "goal", "strike", "birdie", "penalty", "foul",
    "offside", "serve", "ace", "backhand", "forehand", "knockout", "sprint", "marathon",
    "hurdle", "javelin", "vault", "basket", "rebound", "assist", "block", "tackle",
    "interception", "pitcher", "batter", "goalie", "forward", "defense", "midfielder",
];

const FOOD_ADJECTIVES: &[&str] = &[
    "spicy", "sweet", "sour", "bitter", "salty", "tangy", "creamy", "crunchy", "hot", "cold",
    "fresh", "roasted", "baked", "fried", "grilled", "steamed", "juicy", "ripe", "zesty",
    "savory", "rich", "light", "hearty", "crispy",
];

const FOODS: &[&str] = &[
    "taco", "pizza", "burger", "pasta", "sushi", "curry", "salad", "soup", "apple", "banana",
    "orange", "grape", "cherry", "lemon", "peach", "mango", "cookie", "cake", "pie", "donut",
    "brownie", "muffin", "bread", "pastry", "cheese", "yogurt", "butter", "cream", "sauce",
    "syrup", "honey", "jam",
];

// ---- Primitive helpers --------------------------------------------------

fn from_charset(length: usize, charset: &str) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

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

fn leetify_str<R: Rng>(s: &str, rng: &mut R) -> String {
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

// ---- Style --------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum Case {
    Lower,
    Title,
    Upper,
}

#[derive(Debug, Clone, Copy)]
enum TagKind {
    SmallInt,
    Alnum(usize),
    Alpha(usize),
    Hex(usize),
    Digits(usize),
}

#[derive(Debug, Clone, Copy)]
enum TagPosition {
    Prefix,
    Middle,
    Suffix,
}

/// A locked rendering style for wordlist-based formats. Lock one of these per
/// "group" of outputs to get a uniform feel within the group (many candidates
/// all with the same separator/case/tag-shape) while rolling a fresh style per
/// group for between-group diversity.
#[derive(Debug, Clone)]
pub struct Style {
    case: Case,
    body_sep: &'static str,
    tag_sep: &'static str,
    tag_kind: TagKind,
    tag_position: TagPosition,
    leetify: bool,
}

impl Style {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let case = match rng.gen_range(0..3) {
            0 => Case::Lower,
            1 => Case::Title,
            _ => Case::Upper,
        };
        let body_sep_opts: &[&str] = if matches!(case, Case::Lower) {
            &["-", "_", "."]
        } else {
            &["-", "_", ".", ""]
        };
        let body_sep = *body_sep_opts.choose(&mut rng).unwrap();
        let tag_sep = *["-", "_", ".", ""].choose(&mut rng).unwrap();
        let tag_kind = match rng.gen_range(0..5) {
            0 => TagKind::SmallInt,
            1 => TagKind::Alnum(rng.gen_range(2..=5)),
            2 => TagKind::Alpha(rng.gen_range(2..=4)),
            3 => TagKind::Hex(rng.gen_range(3..=6)),
            _ => TagKind::Digits(rng.gen_range(2..=5)),
        };
        let tag_position = match rng.gen_range(0..3) {
            0 => TagPosition::Prefix,
            1 => TagPosition::Middle,
            _ => TagPosition::Suffix,
        };
        let leetify = rng.gen_bool(0.25);
        Style {
            case,
            body_sep,
            tag_sep,
            tag_kind,
            tag_position,
            leetify,
        }
    }
}

fn render_styled(words: &[&str], style: &Style) -> String {
    let mut rng = rand::thread_rng();

    let cased: Vec<String> = words
        .iter()
        .map(|w| match style.case {
            Case::Lower => w.to_lowercase(),
            Case::Title => capitalize(w),
            Case::Upper => w.to_uppercase(),
        })
        .collect();

    let tag = match style.tag_kind {
        TagKind::SmallInt => {
            let max = [9u32, 99, 999, 9999][rng.gen_range(0..4)];
            rng.gen_range(1..=max).to_string()
        }
        TagKind::Alnum(len) => from_charset(len, "abcdefghijklmnopqrstuvwxyz0123456789"),
        TagKind::Alpha(len) => from_charset(len, "abcdefghijklmnopqrstuvwxyz"),
        TagKind::Hex(len) => from_charset(len, "0123456789abcdef"),
        TagKind::Digits(len) => from_charset(len, "0123456789"),
    };

    let assembled = if cased.is_empty() {
        tag
    } else {
        let pos = match style.tag_position {
            TagPosition::Prefix => 0,
            TagPosition::Middle if cased.len() <= 1 => cased.len(),
            TagPosition::Middle => rng.gen_range(1..cased.len()),
            TagPosition::Suffix => cased.len(),
        };
        if pos == 0 {
            format!("{}{}{}", tag, style.tag_sep, cased.join(style.body_sep))
        } else if pos == cased.len() {
            format!("{}{}{}", cased.join(style.body_sep), style.tag_sep, tag)
        } else {
            let left = cased[..pos].join(style.body_sep);
            let right = cased[pos..].join(style.body_sep);
            format!(
                "{}{}{}{}{}",
                left, style.tag_sep, tag, style.tag_sep, right
            )
        }
    };

    if style.leetify {
        leetify_str(&assembled, &mut rng)
    } else {
        assembled
    }
}

// ---- Generators (public: random style each call) ------------------------

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn uuidv7() -> String {
    Uuid::now_v7().to_string()
}

pub fn url_safe(length: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";
    from_charset(length, charset)
}

pub fn api_key(length: usize) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    from_charset(length, charset)
}

fn pick_words(format: RandomFormat) -> Vec<&'static str> {
    let mut rng = rand::thread_rng();
    match format {
        RandomFormat::MemorableName => vec![random_item(ADJECTIVES), random_item(NOUNS)],
        RandomFormat::HistoricalFigure => vec![random_item(FIGURES)],
        RandomFormat::GeographicName => vec![random_item(LOCATIONS)],
        RandomFormat::CharacterName => vec![random_item(CHARACTERS)],
        RandomFormat::PhoneticAlphabet => {
            let count = rng.gen_range(2..=3);
            (0..count).map(|_| random_item(PHONETICS)).collect()
        }
        RandomFormat::RhymingPair => {
            let (a, b) = random_item(RHYME_PAIRS);
            vec![a, b]
        }
        RandomFormat::MusicalTerm => {
            let count = rng.gen_range(1..=2);
            (0..count).map(|_| random_item(MUSICAL)).collect()
        }
        RandomFormat::ScientificElement => {
            let count = rng.gen_range(1..=2);
            (0..count).map(|_| random_item(ELEMENTS)).collect()
        }
        RandomFormat::ConstellationName => vec![random_item(CONSTELLATIONS)],
        RandomFormat::SportsReference => vec![random_item(SPORTS)],
        RandomFormat::FoodCombination => vec![random_item(FOOD_ADJECTIVES), random_item(FOODS)],
        // Non-wordlist formats: caller shouldn't reach here.
        _ => vec![],
    }
}

pub fn memorable_name() -> String {
    render_styled(&pick_words(RandomFormat::MemorableName), &Style::random())
}
pub fn historical_figure() -> String {
    render_styled(&pick_words(RandomFormat::HistoricalFigure), &Style::random())
}
pub fn geographic_name() -> String {
    render_styled(&pick_words(RandomFormat::GeographicName), &Style::random())
}
pub fn character_name() -> String {
    render_styled(&pick_words(RandomFormat::CharacterName), &Style::random())
}
pub fn phonetic_alphabet() -> String {
    render_styled(&pick_words(RandomFormat::PhoneticAlphabet), &Style::random())
}
pub fn rhyming_pair() -> String {
    render_styled(&pick_words(RandomFormat::RhymingPair), &Style::random())
}
pub fn musical_term() -> String {
    render_styled(&pick_words(RandomFormat::MusicalTerm), &Style::random())
}
pub fn scientific_element() -> String {
    render_styled(&pick_words(RandomFormat::ScientificElement), &Style::random())
}
pub fn constellation_name() -> String {
    render_styled(&pick_words(RandomFormat::ConstellationName), &Style::random())
}
pub fn sports_reference() -> String {
    render_styled(&pick_words(RandomFormat::SportsReference), &Style::random())
}
pub fn food_combination() -> String {
    render_styled(&pick_words(RandomFormat::FoodCombination), &Style::random())
}

// ---- Top-level entry points --------------------------------------------

pub fn generate(format: RandomFormat) -> String {
    generate_with_style(format, &Style::random())
}

/// Generate using a caller-provided style — use this when producing a batch
/// of items that should share visual shape (one section of the menu, or a
/// `randr <format>` listing).
pub fn generate_with_style(format: RandomFormat, style: &Style) -> String {
    match format {
        RandomFormat::Uuid => uuid(),
        RandomFormat::UuidV7 => uuidv7(),
        RandomFormat::UrlSafe => url_safe(16),
        RandomFormat::ApiKey => api_key(24),
        _ => render_styled(&pick_words(format), style),
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
