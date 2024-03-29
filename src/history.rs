use slice_deque::SliceDeque;

#[derive(Debug)]
pub struct History<T> {
    h: SliceDeque<T>,
    len: usize
}
impl<T> History<T> {
    pub fn new(len: usize) -> Self {
        return Self {
            h: SliceDeque::with_capacity(len + 1),
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
    pub fn get_slice(&self, len: usize) -> &[T] {
        &self.h.as_slice()[..len]
    }
}
impl<T> std::ops::Index<usize> for History<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.h[idx]
    }
}
impl<T: std::fmt::Display + Clone + Into<char>> std::fmt::Display for History<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut v = self.h.as_slice().to_vec();
        v.reverse();
        for item in v {
            let c = item.into();
            if c == '\n' {
                write!(f, " ")?;
                continue;
            }
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}
