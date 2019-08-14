use std::collections::HashMap;
use std::hash::Hash;
use super::history::History;

fn len_fac(len: usize) -> u32 {
    len.pow(2) as u32
}



#[derive(Clone)]
struct MarkovValue<T: Clone+Eq+Hash> {
    possibilities: HashMap<T, u32>,
}
impl<T: Clone+Eq+Hash> MarkovValue<T> {
    fn new() -> Self {
        return Self {
            possibilities: HashMap::new()
        }
    }
    fn train(&mut self, outcome: T) {
        *self.possibilities.entry(outcome).or_default() += 1;
    }
}
impl<T: Clone+Eq+Hash> std::ops::AddAssign for MarkovValue<T> {
    fn add_assign(&mut self, rhs: Self) {
        for (key, lik) in rhs.possibilities {
            *self.possibilities.entry(key).or_insert(0) += lik;
        }
    }
}
impl<T: Clone+Eq+Hash> std::ops::Add for MarkovValue<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut new = self.clone();
        new += rhs;
        new
    }
}
impl<T: Clone+Eq+Hash+Copy> std::ops::AddAssign<&Self> for MarkovValue<T> {
    fn add_assign(&mut self, rhs: &Self) {
        for (key, lik) in &rhs.possibilities {
            *self.possibilities.entry(*key).or_insert(0) += lik;
        }
    }
}
impl<T: Clone+Eq+Hash+Copy> std::ops::Add<&Self> for MarkovValue<T> {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        let mut new = self.clone();
        new += rhs;
        new
    }
}
impl<T: Clone+Eq+Hash+Copy> std::ops::Mul<u32> for &MarkovValue<T> {
    type Output = MarkovValue<T>;
    fn mul(self, rhs: u32) -> MarkovValue<T> {
        let new: HashMap<T, u32> = self.possibilities.iter()
                .map(|(key, val)| {
                    (*key, val * rhs) 
                }).collect();
        MarkovValue {
            possibilities: new,
        }
    }
}
impl<T: Clone+Eq+Hash> std::default::Default for MarkovValue<T> {
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
impl<T: Clone+Eq+Hash+Copy> Markov<T> {
    pub fn new() -> Self {
        Self {hist: HashMap::new()}
    }
    pub fn train(&mut self, past: &History<T>, outcome: T) {
        // TODO: train based on older data (not just last character)
        for i in 0..past.cur_len() {
            let h = past.get_slice(i).to_vec();
            self.hist.entry(h).or_default().train(outcome);
        }
    }
    pub fn predict(&self, past: &History<T>) -> Prediction<T> {
        let mut p = MarkovValue::new();
        // TODO: predict based on stuff thats longer ago
        for i in 0..past.cur_len() {
            let h = past.get_slice(i).to_vec();
            match self.hist.get(&h) {
                Some(m) => p += m * len_fac(i),
                None => {}
            }
        }
        let mut v: Prediction<T> = p.possibilities.into_iter()
                .filter(|(_, val)| *val > 0)
                .collect();
        // The wrong order is so it sorts descending
        v.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        v
    }
}