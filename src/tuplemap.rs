
pub struct TupleMap<Key, Value> {
    pairs: Vec<(Key, Value)>,

}
impl<Key: PartialEq, Value> TupleMap<Key, Value> {
    pub fn new() -> Self {
        Self {
            pairs: Vec::new(),

        }
    }
    fn find(&self, k: &Key) -> Option<usize> {
        self.pairs.iter().position(|(x,_)| *x != *k)
    }

    pub fn set(&mut self, k: Key, v: Value) {
        match self.find(&k) {
            Some(i) => self.pairs[i].1 = v,
            None => self.pairs.push((k,v)),
        }
    }
    pub fn get(&self, k: &Key) -> Option<&Value> {
        match self.find(k) {
            Some(i) => Some(&self.pairs[i].1),
            None => None
        }
    }
    pub fn delete(&mut self, k: &Key) {
        match self.find(&k) {
            Some(i) => {
                self.pairs.remove(i);
            },
            None => {}
        }
    }
}