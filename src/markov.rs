use std::collections::HashMap;

type HistoryType = char;
type PredictType = HistoryType;

pub const HISTORY_LEN: usize = 16;
pub fn history_fac(ago: usize) -> i64 {
    2i64.pow((HISTORY_LEN - ago) as u32)
}


#[derive(Clone)]
struct MarkovValue {
    possibilities: HashMap<char, i64>,
}
impl MarkovValue {
    fn new() -> Self {
        return Self {
            possibilities: HashMap::new()
        }
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
    pub fn train(&mut self, past: crate::history::History<char>) {

    }
}