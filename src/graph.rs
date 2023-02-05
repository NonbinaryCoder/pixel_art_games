use std::{collections::VecDeque, hash::Hash, ops::ControlFlow};

use bevy::utils::HashSet;

pub fn bfs<N, O>(
    queue: &mut VecDeque<(N, u32)>,
    marked: &mut impl Set<N>,
    mut visit: impl FnMut(N, u32) -> ControlFlow<O, N>,
    mut neighbors: impl NeighborsFn<N>,
) -> Option<O> {
    while let Some((node, dist)) = queue.pop_front() {
        if marked.insert(&node) {
            let node = match visit(node, dist) {
                ControlFlow::Continue(node) => node,
                ControlFlow::Break(result) => return Some(result),
            };
            for neighbor in neighbors.get(node) {
                if !marked.contains(&neighbor) {
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
    }
    None
}

pub trait Set<N> {
    fn insert(&mut self, node: &N) -> bool;

    fn contains(&self, node: &N) -> bool;
}

impl<N> Set<N> for HashSet<N>
where
    N: Copy + Eq + Hash,
{
    fn insert(&mut self, node: &N) -> bool {
        HashSet::insert(self, *node)
    }

    fn contains(&self, node: &N) -> bool {
        HashSet::contains(self, node)
    }
}

pub trait NeighborsFn<N> {
    type Iter: IntoIterator<Item = N>;

    fn get(&mut self, node: N) -> Self::Iter;
}

impl<T, I, N> NeighborsFn<N> for T
where
    T: FnMut(N) -> I,
    I: IntoIterator<Item = N>,
{
    type Iter = I;

    fn get(&mut self, node: N) -> Self::Iter {
        self(node)
    }
}
