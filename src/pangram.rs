use std::collections::HashMap;

trait AlphabetVec {
    fn from_word(word : &str, char_in : &HashMap<char, usize>) -> Option<Self>;

    fn contains(&self, other : &Self) -> bool;
    fn sub(&self, other : &Self) -> Self;
}


impl AlphabetVec for u32 {
    fn from_word(word: &str, char_in: &HashMap<char, usize>) -> Option<Self> {
        let mut vec = 0u32;
        for c in word.chars() {
            let ind = char_in.get(&c);
            if ind.is_none() || ind.unwrap() > 32 {
                unreachable!("Alphabet with too many characters");
            }
            let bit = 1u32 << ind.unwrap();
            if bit & vec {
                return None;
            }
            vec |= bit;
        }
        vec
    }
    fn contains(&self, other: &Self) -> bool {
        *self & other == other
    }
    fn sub(&self, other: &Self) -> Self {
        *self & (!other)
    }
}