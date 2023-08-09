mod pangram;

use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::Read;
use crate::pangram::count_pangram_words;

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

fn strip(word : &str) -> String {
    String::from_iter(word.chars().filter(
        |c| c.is_alphabetic()
    ).map(
        |c| c.to_uppercase()
    ).flatten())
}

fn load_words(path : &str) -> Option<Vec<String>> {
    let mut file = File::open(path).ok()?;
    let mut words_buff = String::new();
    file.read_to_string(&mut words_buff).ok()?;
    Some(
        words_buff.split('\n').skip_while(
            |line| *line != "---"
        ).skip(1).map(
            |word| strip(word)
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
    let alphabet_map = HashMap::from_iter(
        alpha.iter().enumerate().map(|(i, c)| (*c, i))
    );
    let count = count_pangram_words::<u32>(&words, &alphabet_map);
    println!("Number possible pangrams word sets is {}", count)
}
