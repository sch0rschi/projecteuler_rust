use std::collections::HashMap;
use std::hash::Hash;

pub struct MultisetCombinations<T> {
    items: Vec<(T, usize)>,
    counts: Vec<usize>,
    k: usize,
    done: bool,
}

impl<T: Clone + Eq + Hash> MultisetCombinations<T> {
    pub fn new(data: impl IntoIterator<Item=T>, k: usize) -> Self {
        let mut map = HashMap::new();
        for x in data {
            *map.entry(x).or_insert(0) += 1;
        }

        let items: Vec<(T, usize)> = map.into_iter().collect();
        let counts = vec![0; items.len()];

        Self {
            items,
            counts,
            k,
            done: false,
        }
    }

    fn current_sum(&self) -> usize {
        self.counts.iter().sum()
    }

    fn build_output(&self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.k);
        for ((val, _), &c) in self.items.iter().zip(self.counts.iter()) {
            for _ in 0..c {
                out.push(val.clone());
            }
        }
        out
    }

    fn advance(&mut self) -> bool {
        for i in 0..self.counts.len() {
            if self.counts[i] < self.items[i].1 {
                self.counts[i] += 1;
                for j in 0..i {
                    self.counts[j] = 0;
                }
                return true;
            }
        }
        false
    }
}

impl<T: Clone + Eq + Hash> Iterator for MultisetCombinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        loop {
            let sum = self.current_sum();

            if sum == self.k {
                let out = self.build_output();

                if !self.advance() {
                    self.done = true;
                }

                return Some(out);
            }

            if !self.advance() {
                self.done = true;
                return None;
            }
        }
    }
}
