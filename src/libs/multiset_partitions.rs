use std::cmp::Ord;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Partition<T> {
    pub blocks: Vec<Vec<T>>,
}

pub struct MultisetPartitions<T: Clone + Ord> {
    stack: Vec<State<T>>,
}

#[derive(Clone)]
struct State<T> {
    elems: Vec<(T, usize)>,
    index: usize,
    blocks: Vec<Vec<T>>,
}

impl<T: Clone + Ord> MultisetPartitions<T> {
    pub fn new(mut input: Vec<T>) -> Self {
        input.sort();

        // compress
        let mut elems = Vec::new();
        for x in input {
            if let Some((v, c)) = elems.last_mut()
                && *v == x
            {
                *c += 1;
                continue;
            }
            elems.push((x, 1));
        }

        Self {
            stack: vec![State {
                elems,
                index: 0,
                blocks: Vec::new(),
            }],
        }
    }
}

impl<T: Clone + Ord> Iterator for MultisetPartitions<T> {
    type Item = Partition<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.stack.pop() {
            if state.index == state.elems.len() {
                let mut blocks = state.blocks;

                // canonicalize
                for b in &mut blocks {
                    b.sort();
                }
                blocks.sort();

                return Some(Partition { blocks });
            }

            let (ref val, count) = state.elems[state.index];

            fn distribute<T: Clone + Ord>(
                val: &T,
                k: usize,
                start: usize,
                blocks: &mut Vec<Vec<T>>,
                out: &mut BTreeSet<Vec<Vec<T>>>,
            ) {
                if k == 0 {
                    let mut b = blocks.clone();

                    // canonicalize immediately
                    for x in &mut b {
                        x.sort();
                    }
                    b.sort();

                    out.insert(b);
                    return;
                }

                for i in start..blocks.len() {
                    blocks[i].push(val.clone());
                    distribute(val, k - 1, i, blocks, out);
                    blocks[i].pop();
                }

                blocks.push(vec![val.clone()]);
                distribute(val, k - 1, blocks.len() - 1, blocks, out);
                blocks.pop();
            }

            let mut next_blocks = BTreeSet::new();
            let mut base = state.blocks.clone();

            distribute(val, count, 0, &mut base, &mut next_blocks);

            for b in next_blocks {
                self.stack.push(State {
                    elems: state.elems.clone(),
                    index: state.index + 1,
                    blocks: b,
                });
            }
        }

        None
    }
}
