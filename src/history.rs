#[derive(Debug)]
pub struct History<T> {
    h: std::collections::VecDeque<T>,
    len: usize
}
impl<T> History<T> {
    pub fn new(len: usize) -> Self {
        return Self {
            h: std::collections::VecDeque::with_capacity(len + 1),
            len
        }
    }
    pub fn push(&mut self, value: T) {
        self.h.push_front(value);
        if self.h.len() > self.len {
            self.h.pop_back();
        }
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        match self.h.len() < idx {
            true => Some(&self.h[idx]),
            false => None
        }
    }
    pub fn cur_len(&self) -> usize {
        self.h.len()
    }
}
impl<T> std::ops::Index<usize> for History<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.h[idx]
    }
}