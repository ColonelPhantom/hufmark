use std::collections::HashMap;
// use plain_map::PlainMap as HashMap;

use plain_map::PlainMap;
// use std::collections::HashMap as PlainMap;

use std::hash::Hash;
use super::history::History;

fn len_fac(len: usize) -> u32 {
    // len.pow(2) as u32
    len as u32
    // 2u32.pow(len as u32) as u32
}



#[derive(Clone)]
pub struct MarkovValue<T: Clone+Eq+Hash> {
    possibilities: PlainMap<T, u32>,
    total_occs: u32,
}
impl<T: Clone+Eq+Hash+Copy> MarkovValue<T> {
    fn new() -> Self {
        return Self {
            possibilities: PlainMap::new(),
            total_occs: 0,
        }
    }
    fn train(&mut self, outcome: T) {
        *self.possibilities.entry(outcome).or_default() += 1;
        self.total_occs += 1;
    }

    #[inline(never)]
    fn add_other(&mut self, other: &Self, weight: f64) {
        for (key, lik) in other.possibilities.iter() {
            *self.possibilities.entry(*key).or_insert(0) += (*lik as f64 * weight) as u32;
            self.total_occs += (*lik as f64 * weight) as u32;
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
        for i in 1..=past.cur_len() {
            let h = past.get_slice(i).to_vec();
            self.hist.entry(h).or_default().train(outcome);
        }
    }
    #[inline(never)]
    pub fn predict(&self, past: &History<T>) -> Prediction<T> {
        // TODO: predict based on stuff thats longer ago

        // Find max total_occs
        let mut max_occs = 0;
        for i in 1..=past.cur_len() {
            match self.hist.get(&past.get_slice(i).to_vec()) {
                Some(m) => max_occs = std::cmp::max(max_occs, m.total_occs),
                None => {}
            }
        }

        // Create a prediction for the next value
        let mut p = MarkovValue::new();
        for i in 1..=past.cur_len() {
            let h = past.get_slice(i).to_vec();
            match self.hist.get(&h) {
                Some(m) => p.add_other(m, len_fac(i) as f64
                      * (max_occs as f64 / m.total_occs as f64)
                ),
                // Some(m) => p = m.clone(),
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
    pub fn get_entry_occs(&self) -> std::collections::BTreeMap<u32, u32> {
        let mut result = std::collections::BTreeMap::new();
        for (_,h) in self.hist.iter() {
            *result.entry(h.total_occs).or_default() += 1;
        }
        result
    }
    pub fn get_entry_lens(&self) -> std::collections::BTreeMap<usize, u32> {
        let mut result = std::collections::BTreeMap::new();
        for (_,h) in self.hist.iter() {
            *result.entry(h.possibilities.len()).or_default() += 1;
        }
        result
    }
}