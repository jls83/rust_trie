
type TreeIndex = usize;

#[derive(Clone, Eq, PartialEq)]
enum ArenaTrieNodeType {
    Final(String),
    Intermediate,
}

struct ArenaTrieNode {
    children: Vec<TreeIndex>,
    node_type: ArenaTrieNodeType,
    word_score: Option<i64>,
    aggregate_score: i64,
}

impl ArenaTrieNode {
    fn new() -> Self {
        ArenaTrieNode {
            children: Vec::new(),
            node_type: ArenaTrieNodeType::Intermediate,
            word_score: None,
            aggregate_score: 0,
        }
    }

    // If we have a `word_score` (i.e. the node represents a `Final` result), use that value as the
    // ranking score. Otherwise, use the aggregate_score value.
    // TODO: Should the word_score be part of the `TrieNodeType`?
    fn get_ranking_score(&self) -> i64 {
        match self.word_score {
            Some(word_score) => word_score,
            _ => self.aggregate_score,
        }
    }

}

pub struct ArenaTrie {
    arena: Vec<ArenaTrieNode>,
    root: Option<TreeIndex>,
}

impl ArenaTrie {
    pub fn new() -> Self {
        ArenaTrie {
            arena: Vec::new(),
            root: ArenaTrieNode::new(),
        }
    }

    fn _new_node(&mut self, data: char) -> TreeIndex {
        let next_index = self.arena.len();
        self.arena.push(ArenaTrieNode {


        });

        next_index

    }

    fn _insert(&mut self, word: String, score: i64) {
        let mut current_node = &mut self.root;

        for char in word.chars() {

        }
    }
}
