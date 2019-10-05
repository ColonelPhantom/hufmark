// use std::collections::HashMap;
use fnv::FnvHashMap as HashMap;
// use std::collections::BTreeMap as HashMap;
// use plain_map::PlainMap as HashMap;

use plain_map::PlainMap;
// use std::collections::HashMap as PlainMap;

use std::hash::Hash;
use super::history::History;

fn len_fac(len: usize) -> u32 {
    // len.pow(2) as u32
    // (len as u32 * 10) + 1
    // len as u32
    // 1
    len as u32 + 1
    
    // These two are equal
    // 2u32.pow(len as u32) as u32
    // 1u32 << len
}


use derivative::Derivative;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct MarkovValue<T: Clone+Eq+Hash+std::fmt::Debug+Copy> {
    possibilities: PlainMap<T, u32>,
    total_occs: u32,
    #[derivative(Debug="ignore")]
    pub full_key: Option<MarkovKey<T>>,
}
impl<T: Clone+Eq+Hash+Copy+std::fmt::Debug> MarkovValue<T> {
    fn new() -> Self {
        return Self {
            possibilities: PlainMap::new(),
            total_occs: 0,
            full_key: None,
        }
    }
    fn train(&mut self, outcome: T) {
        *self.possibilities.entry(outcome).or_default() += 1;
        self.total_occs += 1;
    }

    #[inline(never)]
    fn add_other(&mut self, other: &Self, weight: u32) {
        let mut new_p = other.possibilities.clone();
        for (_, val) in new_p.iter_mut() {
            *val *= weight;
        }
        for (key, lik_self) in self.possibilities.iter() {
            *new_p.entry(*key).or_insert(0) += lik_self;
        }
        self.possibilities = new_p;
        self.total_occs += other.total_occs * weight;
    }
}
impl<T: Clone+Eq+Hash+Copy+std::fmt::Debug> std::default::Default for MarkovValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

// #[derive(PartialEq, Eq, Hash)]
// struct MarkovKey {
//     values: [char; HISTORY_LEN],
// }

type MarkovKey<T> = Box<[T]>;
pub type PredictType<T> = T;


pub type Prediction<T> = Vec<(PredictType<T>, u32)>;

pub struct Markov<T: Clone+Eq+Hash+std::fmt::Debug+Copy> {
    hist: HashMap<MarkovKey<T>, MarkovValue<T>>,
}
impl<T: Clone+Eq+Hash+Copy+PartialOrd +Ord+std::fmt::Display+std::fmt::Debug> Markov<T> {
    pub fn new() -> Self {
        Self {hist: HashMap::default()}
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {hist: HashMap::with_capacity_and_hasher(cap, Default::default())}
    }
    #[inline(never)]
    pub fn train(&mut self, past: &History<T>, outcome: T) {
        // TODO: train based on older data (not just last character)

        'inclen: for i in 0..=past.cur_len() {
            let h = past.get_slice(i);
            // self.hist.entry(h).or_default().train(outcome);

            let mut to_insert = None;

            match self.hist.entry(Box::from(h)) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    let mv = e.get_mut();
                    // mv.train(outcome);

                    match &mv.full_key {
                        Some(k) => {
                            // The entry is a 'shorthand' entry (ie. it is for ello but also represents hello)
                            if k.as_ref() == past.get_slice(past.cur_len()) {
                                // The shorthand stays valid
                                mv.train(outcome);
                                // Since we do not need any longer values to represent this, break.
                                break 'inclen;
                            } else {
                                // The shorthand is no longer valid as there will be a second value represented
                                // Eg ello now not only represents hello but also jello
                                // Therefore, this entry will be 'split'.

                                if mv.possibilities.len() == 1 && mv.possibilities.iter().next().unwrap().0 == outcome {
                                    // The prediction is the same as for the current value we are training on
                                    // As such, we do not have to split(?)
                                    mv.train(outcome);
                                    break 'inclen;
                                }

                                // First, copy it out with an additional character (if possible)
                                if i < k.len() {
                                    // Yes, it is possible. Continue.
                                    to_insert = Some((k[..i+1].to_vec(), mv.clone()));
                                }

                                // Then, train the one that will stay in this slot
                                mv.train(outcome);
                                // Remove the full key as this has multiple 'children' now and is not a shorthand anymore.
                                mv.full_key = None;
                            }
                        }
                        None => {
                            // The entry is not a shorthand: train it regularly like a branch node
                            mv.train(outcome);
                    }
                }
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    // We will create a new entry
                    let mut new_mv: MarkovValue<T> = MarkovValue::default();
                    if i < past.cur_len() {
                        // We can create a shorthand entry to save memory.
                        new_mv.full_key = Some(Box::from(past.get_slice(past.cur_len())));
                    }
                    // Make sure the new entry is properly trained
                    new_mv.train(outcome);
                    // and finally insert it.
                    e.insert(new_mv);

                    // Since this has to be a leaf node, we do not need to store any longer nodes.
                    break 'inclen;
                }
            }
            
            // We could not store the extended shorthand in the match, so do it now.
            if let Some((k,v)) = to_insert {
                self.hist.insert(Box::from(k.as_slice()),v);
            }
        }
    }
    #[inline(never)]
    pub fn predict(&self, past: &History<T>) -> Prediction<T> {
        // TODO: predict based on stuff thats longer ago

        // Find the appropriate history entries, cache them in a Vec to reduce hashtable retrieves
        let mut hists = Vec::with_capacity(past.cur_len());
        'inclen: for i in 0..=past.cur_len() {
            let past_slice = past.get_slice(i);
            match self.hist.get(past_slice) {
                Some(h) => hists.push(Some(h)),
                None => {
                    // There is no matching entry found: try to see if the last entry was a shorthand and if so, reuse it.
                    if let Some(l) = hists.last() {
                        if let Some(entry) = l {
                            if let Some(full_key) = &entry.full_key {
                                if full_key.len() >= i && &full_key[..i] == past_slice {
                                    // The previous hist entry is also applicable for this one
                                    hists.push(*hists.last().unwrap());
                                    continue 'inclen;
                                }
                            }
                        }
                    }
                    // The last history entry deviates from our current entry. As such just push an empty one
                    // (break would also be a possibility)
                    hists.push(None);
                }
            }
            // hists.push(self.hist.get(&past.get_slice(i).to_vec()));
        }

        // println!("\nPREDICT HISTS");
        // for (i,h) in hists.iter().enumerate() {
        //     match h {
        //         Some(h) => println!("{:?} {:?}", &past.get_slice(i), h),
        //         None => println!("NONE")
        //     };
        // }
        
        // Find max total_occs
        let mut max_occs = 0;
        for h in hists.iter() {
            match h {
                // Some(m) => max_occs = std::cmp::max(max_occs, m.total_occs),
                Some(m) => max_occs += m.total_occs,
                None => {}
            }
        }

        // Create a prediction for the next value
        let mut p = &mut MarkovValue::new();
        for (i,h) in hists.iter().enumerate().rev() {
            match h {
                Some(m) => p.add_other(m, len_fac(i)
                       * ((max_occs) / (m.total_occs))
                    //    * ((max_occs as f64) / (m.total_occs as f64)) as u32
                ),
                // Some(m) => p = m.clone(),
                None => {}
            }
        }
        
        let vec: Vec<_> = p.possibilities.iter().filter(|(_,v)| *v > 0).collect();
        // if vec.is_empty() {
        //     let empty_markov = MarkovValue::new();
        //     let p = self.hist.get(&vec![]).unwrap_or(&empty_markov);
        //     let mut v: Prediction<T> = p.possibilities.iter()
        //         .filter(|(_, val)| *val > 0)
        //         .map(|(a,b)| (*a,*b))
        //         .collect();
        //     v.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        //     // The wrong order is so it sorts descending
        //     v.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        //     return v;
        // }
        let mut v: Prediction<T> = p.possibilities.iter()
                .filter(|(_, val)| *val > 0)
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
    pub fn get_capacity(&self) -> usize {
        self.hist.capacity()
    }
    pub fn get_len(&self) -> usize {
        self.hist.len()
    }
}