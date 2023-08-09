use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub trait AlphabetVec {
    fn from_word(word : &str, char_in : &HashMap<char, usize>) -> Option<Self>
        where Self: Sized;

    fn pangram(char_in: &HashMap<char, usize>) -> Self;
    fn empty() -> Self;

    fn top(&self) -> Self;

    fn contains(&self, other : &Self) -> bool;
    fn sub(&self, other : &Self) -> Self;
}


impl AlphabetVec for u32 {
    fn from_word(word: &str, char_in: &HashMap<char, usize>) -> Option<Self> {
        let mut vec = 0u32;
        for c in word.chars() {
            let ind = char_in.get(&c);
            if ind.is_none() || *ind.unwrap() > 32 {
                unreachable!("Alphabet with too many characters");
            }
            let bit = 1u32 << (32-*ind.unwrap());
            if bit & vec != 0u32 {
                return None;
            }
            vec |= bit;
        }
        Some(vec)
    }
    fn empty() -> Self {
        0u32
    }
    fn pangram(char_in: &HashMap<char, usize>) -> Self {
        let mut vec = 0u32;
        for index in char_in.values() {
            let bit = 1u32 << (32 -*index);
            vec |= bit;
        }
        vec
    }
    fn top(&self) -> Self {
        let l = self.leading_zeros();
        1u32 << (32-l)
    }
    fn contains(&self, other: &Self) -> bool {
        *self & *other == *other
    }
    fn sub(&self, other: &Self) -> Self {
        *self & (!other)
    }
}

pub fn count_pangram_words<V>(words : &Vec<String>, alphabet : &HashMap<char, usize>) -> usize
  where V : Eq + Ord + Hash + Clone + AlphabetVec
{
    let mut words_map : BTreeMap<V, usize> = BTreeMap::new();
    let mut words_prog : HashMap<V, &str> = HashMap::new();
    for word in words {
        if let Some(word_vec) = V::from_word(word, alphabet) {
            if word_vec != V::empty() {
                *words_map.entry(word_vec.clone()).or_insert(0) += 1;
                words_prog.insert(word_vec, word);
            }
        }
    }
    let pangram= V::pangram(alphabet);
    collects_count(&pangram, &words_map, &pangram)
}

fn collects_count<V>(
    collect : &V,
    word_map : &BTreeMap<V, usize>,
    word_max : &V,
) -> usize
    where V : Eq + Ord + Hash + Clone + AlphabetVec
{
    if *collect == V::empty() {
        return 1;
    }
    let mut count: usize = 0;
    let top_bit = collect.top();
    for (word_v, word_count) in word_map.iter().rev() {
        if word_v >= word_max {
            continue;
        }
        if !word_v.contains(&top_bit) {
            continue;
        }
        if collect.contains(word_v) {
            let reduce = collect.sub(word_v);
            let reduce_count = collects_count(
                &reduce,
                word_map,
                &word_v
            );
            count += reduce_count * word_count;
        }
    }
    count
}