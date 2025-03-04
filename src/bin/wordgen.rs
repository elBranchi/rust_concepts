use rand::prelude::*;
use rand::rngs::StdRng;

#[derive(Debug)]
struct WordGenParts {
    #[allow(unused)]
    language: String,
    syl_begin: Vec<String>,
    vowel: Vec<String>,
    syl_end: Vec<String>,
}

impl WordGenParts {
    fn new(lang: &str, syllabes: &[&str], vowels: &[&str]) -> WordGenParts {
        WordGenParts {
            language: String::from(lang),
            syl_begin: syllabes.iter().map(|s| String::from(*s)).collect(),
            vowel: vowels.iter().map(|s| String::from(*s)).collect(),
            syl_end: Vec::<String>::new(),
        }
    }
    fn add_ends(&mut self, ends: &[&str]) {
        ends.iter()
            .for_each(|s| self.syl_end.push(String::from(*s)));
    }
    fn generate_word<T: Rng>(&self, max_syl_count: u8, rng: &mut T) -> String {
        let mut word = String::new();

        let len = 1 + rng.random_range(0..max_syl_count);

        // let get_rand_word = |words: &'a Vec<String>| -> &'a str {
        fn get_rand_part<'a, T: Rng>(parts: &'a Vec<String>, rng: &mut T) -> &'a str {
            let idx = rng.random_range(..parts.len());
            let part = &parts[idx];

            part
        }

        let vowel_start = rng.random_bool(0.30);

        if vowel_start {
            word.push_str(get_rand_part(&self.vowel, rng));
        }

        let mut i = 0;
        while i < len {
            word.push_str(get_rand_part(&self.syl_begin, rng));
            word.push_str(get_rand_part(&self.vowel, rng));

            i += 1;
        }
        let has_end = rng.random_bool(0.45);
        if has_end {
            word.push_str(get_rand_part(&self.syl_end, rng));
        }

        word
    }
}

fn main() {
    let mut seed_rng = rand::rng();
    let seed: u64 = seed_rng.random();

    println!("Using {seed} as rng seed");
    let mut actual_rng = StdRng::seed_from_u64(seed);

    let mut spanish: WordGenParts = WordGenParts::new(
        "spanish",
        &["b", "c", "d", "f", "g", "h", "j", "fr", "cr"],
        &["a", "e", "i", "o", "u"],
    );
    spanish.add_ends(&["s", "n"]);

    let mut catalan: WordGenParts = WordGenParts::new(
        "spanish",
        &["b", "c", "d", "f", "g", "h", "j", "fr", "cr", "qu", "ç"],
        &["a", "e", "i", "o", "u"],
    );
    catalan.add_ends(&["s", "n", "c", "ç"]);

    // println!("spanish {spanish:?}\ncatalan {catalan:?}");

    for c in 0..3 {
        let _w_sp = spanish.generate_word(2 + c, &mut actual_rng);
        let _w_cat = catalan.generate_word(1 + c, &mut actual_rng);

        println!("sp:{}\ncat:{}", &_w_sp, &_w_cat);
    }
}
