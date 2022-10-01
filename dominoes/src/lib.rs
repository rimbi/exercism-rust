use core::fmt::Debug;

#[derive(Clone, Copy)]
struct Node {
    id: usize,
    n1: u8,
    n2: u8,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Node {
    pub fn can_chain_to(&self, other: &Self) -> bool {
        self.id != other.id && self.n2 == other.n1
    }

    pub fn reverse(&self) -> Node {
        Self {
            id: self.id,
            n1: self.n2,
            n2: self.n1,
        }
    }
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let len = input.len();
    if len == 0 {
        return Some(vec![]);
    }
    if len == 1 {
        let d = input[0];
        if d.1 == d.0 {
            return Some(vec![d]);
        }
        return None;
    }
    let input = input
        .iter()
        .enumerate()
        .map(|(id, &(n1, n2))| Node { id, n1, n2 })
        .flat_map(|n| [n, n.reverse()]);
    let mut chains = vec![];
    for node in input.cycle().take(2 * len * len) {
        if chains.len() < 2 * len {
            // Fill the first items
            chains.push(vec![node]);
            continue;
        }
        let mut old_chains = chains.clone();
        while let Some(mut chain) = old_chains.pop() {
            if chain.contains(&node) {
                continue;
            }
            let last_node = &chain.last().unwrap().clone();
            if last_node.can_chain_to(&node) {
                chain.push(node);
                if chain.len() == len && chain.first().unwrap().n1 == chain.last().unwrap().n2 {
                    return Some(chain.iter().map(|n| (n.n1, n.n2)).collect());
                }
                if chain.len() < len {
                    chains.push(chain);
                }
            }
        }
    }
    None
}
