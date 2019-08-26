use std::collections::HashMap;
use std::hash::Hash;
use super::history::History;

fn len_fac(len: usize) -> u32 {
    len.pow(2) as u32
    // 2u32.pow(len.pow(2) as u32) as u32
}



#[derive(Clone)]
struct MarkovValue<T: Clone+Eq+Hash> {
    possibilities: HashMap<T, u32>,
}
impl<T: Clone+Eq+Hash+Copy> MarkovValue<T> {
    fn new() -> Self {
        return Self {
            possibilities: HashMap::new()
        }
    }
    fn train(&mut self, outcome: T) {
        *self.possibilities.entry(outcome).or_default() += 1;
    }

    #[inline(never)]
    fn add_other(&mut self, other: &Self, weight: u32) {
        if self.possibilities.capacity() < std::cmp::max(self.possibilities.len(), other.possibilities.len()) * 2 {
            self.possibilities.reserve(std::cmp::max(self.possibilities.len(), other.possibilities.len()) * 2 - self.possibilities.len());
        }
        for (key, lik) in &other.possibilities {
            *self.possibilities.entry(*key).or_insert(0) += lik * weight;
        }
    }
}
impl<T: Clone+Eq+Hash+Copy> std::default::Default for MarkovValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

// #[derive(PartialEq, Eq, Hash)]
// struct MarkovKey {
//     values: [char; HISTORY_LEN],
// }

type MarkovKey<T> = Vec<T>;
pub type PredictType<T> = T;


pub type Prediction<T> = Vec<(PredictType<T>, u32)>;

pub struct Markov<T: Clone+Eq+Hash> {
    hist: HashMap<MarkovKey<T>, MarkovValue<T>>,
}
impl<T: Clone+Eq+Hash+Copy+PartialOrd> Markov<T> {
    pub fn new() -> Self {
        Self {hist: HashMap::new()}
    }
    #[inline(never)]
    pub fn train(&mut self, past: &History<T>, outcome: T) {
        // TODO: train based on older data (not just last character)
        for i in 0..past.cur_len() {
            let h = past.get_slice(i).to_vec();
            self.hist.entry(h).or_default().train(outcome);
        }
    }
    #[inline(never)]
    pub fn predict(&self, past: &History<T>) -> Prediction<T> {
        let mut p = &MarkovValue::new();
        // TODO: predict based on stuff thats longer ago
        for i in 0..past.cur_len() {
            let h = past.get_slice(i).to_vec();
            match self.hist.get(&h) {
                // Some(m) => p.add_other(m, len_fac(i)),
                Some(m) => p = m,
                None => {}
            }
        }
        let mut v: Prediction<T> = p.possibilities.iter()
                // .filter(|(_, val)| *val > 0)
                .map(|(a,b)| (*a,*b))
                .collect();
        v.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        // The wrong order is so it sorts descending
        v.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        v
    }
}