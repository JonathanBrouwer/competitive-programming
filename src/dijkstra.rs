use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::Entry;
use std::hash::Hash;

fn dijkstra<N: Eq + Hash + Clone + Copy, IN: IntoIterator<Item = (N, usize)>>(
    starts: impl Iterator<Item=(N, usize)>,
    is_end: impl Fn(&N) -> bool,
    nbs: impl Fn(&N) -> IN,
) -> Option<(usize, Vec<N>)> {
    #[derive(Eq, PartialEq)]
    struct State<N: Eq + Clone>(N, usize);
    impl<N: Eq + Clone> PartialOrd<Self> for State<N> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<N: Eq + Clone> Ord for State<N> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.1.cmp(&other.1).reverse()
        }
    }

    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    for (start, startd) in starts {
        distances.insert(start, (startd, None, false));
        heap.push(State(start, startd));
    }

    while let Some(State(n, c)) = heap.pop() {
        //If we found something shorter or equally good, continue
        let (_, _, visited) = distances.get_mut(&n).unwrap();
        if *visited { continue }
        *visited = true;

        //Found goal
        if is_end(&n) {
            // Collect path
            let mut path = vec![n];
            while let Some(next) = distances[path.last().unwrap()].1 {
                path.push(next)
            }
            path.reverse();

            return Some((c, path))
        }

        // Iterate over neighbours
        for (nb, nb_cost) in nbs(&n) {
            let alt = c + nb_cost;

            // Get entry for distance of this neighbour
            match distances.entry(nb) {
                // If node is encountered, update cost if better
                Entry::Occupied(mut cost_old) => {
                    let (cost_old, prev, _) = cost_old.get_mut();
                    if *cost_old > alt {
                        *cost_old = alt;
                        heap.push(State(nb, alt));
                        *prev = Some(n);
                    }
                }
                // If node is not encountered, store it
                Entry::Vacant(cost_old) => {
                    cost_old.insert((alt, Some(n), false));
                    heap.push(State(nb, alt));
                }
            }
        }
    }

    None
}