use std::collections::HashMap;
use super::history::History;

type HistoryType = char;
type PredictType = HistoryType;

pub const HISTORY_LEN: usize = 16;
pub fn history_fac(ago: usize) -> u32 {
    2u32.pow((HISTORY_LEN - ago) as u32)
}


#[derive(Clone, Default)]
struct MarkovValue {
    possibilities: HashMap<char, u32>,
}
impl MarkovValue {
    fn new() -> Self {
        return Self {
            possibilities: HashMap::new()
        }
    }
    fn train(&mut self, outcome: PredictType) {
        *self.possibilities.entry(outcome).or_default() += 1;
    }
}
impl std::ops::AddAssign for MarkovValue {
    fn add_assign(&mut self, rhs: Self) {
        for (key, lik) in rhs.possibilities {
            *self.possibilities.entry(key).or_insert(0) += lik;
        }
    }
}
impl std::ops::Add for MarkovValue {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut new = self.clone();
        new += rhs;
        new
    }
}

// #[derive(PartialEq, Eq, Hash)]
// struct MarkovKey {
//     values: [char; HISTORY_LEN],
// }

type MarkovKey = Vec<HistoryType>;

pub struct Markov {
    hist: HashMap<MarkovKey, MarkovValue>,
}
impl Markov {
    pub fn new() -> Self {
        Self {hist: HashMap::new()}
    }
    pub fn train(&mut self, past: History<char>, outcome: char) {
        // TODO: train based on older data (not just last character)
        for i in 0..past.cur_len() {
            let h = past.get_slice(i).to_vec();
            self.hist.entry(h).or_default().train(outcome);
        }
    }
}