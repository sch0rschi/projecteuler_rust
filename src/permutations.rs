pub struct PermutationPruner<'a, T> {
    items: &'a [T],
    perm: Vec<usize>,
    used: Vec<bool>,
    next: Vec<usize>,
    depth: usize,
}

impl<'a, T> PermutationPruner<'a, T> {
    pub fn new(items: &'a [T]) -> Self {
        let n = items.len();

        Self {
            items,
            perm: vec![0; n],
            used: vec![false; n],
            next: vec![0; n],
            depth: 0,
        }
    }

    /// returns current permutation as references
    pub fn next_permutation(&mut self) -> Option<Vec<&T>> {
        let n = self.items.len();

        loop {
            if self.depth == n {
                let res = self.perm.iter().map(|&i| &self.items[i]).collect();

                self.depth -= 1;
                self.used[self.perm[self.depth]] = false;

                return Some(res);
            }

            let mut i = self.next[self.depth];

            while i < n && self.used[i] {
                i += 1;
            }

            if i < n {
                self.perm[self.depth] = i;
                self.used[i] = true;
                self.next[self.depth] = i + 1;

                self.depth += 1;

                if self.depth < n {
                    self.next[self.depth] = 0;
                }

                continue;
            }

            self.next[self.depth] = 0;

            if self.depth == 0 {
                return None;
            }

            self.depth -= 1;
            self.used[self.perm[self.depth]] = false;
        }
    }

    pub fn prune(&mut self, k: usize) {
        let depth = k + 1;

        while self.depth > depth {
            self.depth -= 1;
            let v = self.perm[self.depth];
            self.used[v] = false;
        }

        if depth < self.next.len() {
            self.next[depth] = self.items.len();
        }
    }
}