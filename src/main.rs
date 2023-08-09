use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;

// Alphabet used for the words sorted in descending order of usage
fn alphabet(words : &Vec<String>) -> Vec<char> {
    let mut counts : HashMap<char, usize> = HashMap::new();
    for word in words.iter() {
        for c in word.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
    }
    let mut alphabet_counts : Vec<(char, usize)> = counts.into_iter().collect();
    alphabet_counts.sort_by(
        |(_, size1), (_, size2) | size1.cmp(size2)
    );
    alphabet_counts.into_iter().rev().map(
        |(c, _)| c
    ).collect()
}

fn load_words(path : &str) -> Option<Vec<String>> {
    let mut file = File::open(path).ok()?;
    let mut words_buff = String::new();
    file.read_to_string(&mut words_buff).ok()?;
    Some(
        words_buff.split('\n').map(
            |word| word.to_string()
        ).collect()
    )
}

fn main() {
    let words_file = args().nth(1).unwrap_or("words.txt".to_string());
    let words = load_words(&words_file).expect("Could not open words file");
    println!("{} words loaded", words.len());
    let alpha = alphabet(&words);
    println!("Alphabet of {} characters found:", alpha.len());
    println!("{}", String::from_iter(alpha.iter()));
}
