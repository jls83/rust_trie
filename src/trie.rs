use std::cmp;
use std::collections::BinaryHeap;
use std::sync::{Arc, RwLock};

use crate::helpers::output_wrapper::OutputWrapper;
use crate::helpers::queue_wrapper::QueueWrapper;
use crate::trie_node::{TrieNode, TrieNodeType};

#[derive(Clone)]
pub struct Trie {
    root: TrieNode,
    root_index: usize,
    arena: Arc<RwLock<Vec<Arc<RwLock<TrieNode>>>>>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(None),
            root_index: 0,
            arena: Arc::new(RwLock::new(vec![Arc::new(RwLock::new(TrieNode::new(
                None,
            )))])),
        }
    }

    fn _insert(&mut self, word: String, score: i64) {
        let mut current_node = &mut self.root;

        let mut arena = self.arena.write().expect("RwLock poisoned");

        let mut current_node_index: usize = 0;

        for char in word.chars() {
            let mut next_node = current_node
                .children
                .entry(char)
                .or_insert(TrieNode::new(Some(char)));
            next_node.node_score = cmp::max(next_node.node_score, score);
            current_node = next_node;

            if let Some(next_idx) = arena
                .get(current_node_index)
                .unwrap()
                .read()
                .ok()
                .unwrap()
                .children_new
                .get(&char)
            {
                current_node_index = *next_idx;
                continue;
            }
            let next_idx = arena.len();
            arena.push(Arc::new(RwLock::new(TrieNode::new(Some(char)))));

            let mut node_to_mod = arena[current_node_index].write().unwrap();
            node_to_mod.children_new.insert(char, next_idx);

            current_node_index = next_idx;
        }

        // Set some properties on the last node so that it can be used as a representation of the
        // incoming `word`.
        current_node.node_type = TrieNodeType::Final;
        current_node.word_score = Some(score);

        let mut node_to_mod = arena[current_node_index].write().unwrap();
        node_to_mod.node_type = TrieNodeType::Final;
        node_to_mod.word_score = Some(score);
    }

    fn _search(&self, word: &String) -> Option<OutputWrapper> {
        // NOTE: We do not include the root of the trie when returning results, as it only contains
        // an empty char, plus references to its children.
        let mut node = &self.root;
        let mut nodes: Vec<&TrieNode> = vec![];

        let asdf = self.arena.read().unwrap();
        let mut node_new = asdf.get(self.root_index).unwrap();
        let mut node_idxs: Vec<usize> = vec![];

        for char in word.chars() {
            if let Some(next_node) = node.children.get(&char) {
                nodes.push(next_node);
                node = next_node;
            } else {
                return None;
            }

            if let Some(next_idx) = node_new.read().unwrap().children_new.get(&char) {
                node_idxs.push(*next_idx);
                node_new = asdf.get(*next_idx).unwrap();
            }
        }

        let foo: String = node_idxs
            .iter()
            .map(|idx| asdf[*idx].read().unwrap().value.unwrap())
            .collect();

        println!("WHAT {}", foo);

        Some(OutputWrapper { nodes })
    }

    fn _get_ranked_results(&self, prefix: String, k: usize) -> Option<Vec<String>> {
        // Our collection of "found" items is represented by `OutputWrapper`
        // instances so that we can use a specific `Ord` trait implementation
        // to order by the underlying word's score before returning.
        let mut found_nodes: BinaryHeap<OutputWrapper> = BinaryHeap::new();
        let mut max_word_score: i64 = 0;

        let mut heap: BinaryHeap<QueueWrapper>;

        if let Some(output_wrapper) = self._search(&prefix) {
            heap = BinaryHeap::from(vec![output_wrapper.to_queue_wrapper()]);
        } else {
            return None;
        }

        while let Some(queue_wrapper) = heap.pop() {
            if (k != 0 && queue_wrapper.output_score() < max_word_score) && found_nodes.len() >= k {
                break;
            }
            if let Some(TrieNodeType::Final) = queue_wrapper.leaf_type() {
                found_nodes.push(queue_wrapper.to_output_wrapper());
                max_word_score = cmp::max(max_word_score, queue_wrapper.output_score());
            }
            if let Some(children) = queue_wrapper.children() {
                for child in children {
                    heap.push(queue_wrapper.new_with_node(child));
                }
            }
        }

        // NOTE: It's a bit convoluted to turn a `BinaryHeap` into a `Vec` with the values in heap
        // order. `BinaryHeap.into_iter_sorted` will do what we need, but it is not yet stable (see
        // https://github.com/rust-lang/rust/issues/59278).
        let result: Vec<String> = found_nodes
            .into_sorted_vec()
            .iter()
            .rev()
            .map(|t| t.join())
            .collect();

        Some(result)
    }

    pub fn get_ranked_results(&self, prefix: String) -> Option<Vec<String>> {
        self._get_ranked_results(prefix, 0)
    }

    pub fn get_k_ranked_results(&self, prefix: String, k: usize) -> Option<Vec<String>> {
        self._get_ranked_results(prefix, k)
    }

    pub fn insert(&mut self, word: String) {
        self._insert(word, 0);
    }

    pub fn insert_with_score(&mut self, word: String, score: i64) {
        self._insert(word, score);
    }

    pub fn search(&self, word: String) -> Option<String> {
        if let Some(output_wrapper) = self._search(&word) {
            match output_wrapper.leaf_type() {
                Some(TrieNodeType::Final) => Some(output_wrapper.join()),
                _ => None,
            }
        } else {
            return None;
        }
    }

    pub fn starts_with(&self, prefix: String) -> Option<String> {
        match self._search(&prefix) {
            Some(output_wrapper) => Some(output_wrapper.join()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::Trie;

    #[test]
    fn can_search_for_term() {
        let search_term = "Foo";
        let mut trie = Trie::new();
        trie.insert(search_term.to_string());

        assert_eq!(
            Some(search_term.to_string()),
            trie.search(search_term.to_string())
        );
    }

    #[test]
    fn can_search_for_term_with_score() {
        let search_term = "Foo";
        let mut trie = Trie::new();
        trie.insert_with_score(search_term.to_string(), 10);

        assert_eq!(
            Some(search_term.to_string()),
            trie.search(search_term.to_string())
        );
    }

    #[test]
    fn can_search_for_term_with_similar_entries() {
        let search_term = "Foo";
        let insert_terms = vec!["Foo", "For"];

        let mut trie = Trie::new();
        for term in insert_terms {
            trie.insert(term.to_string());
        }

        assert_eq!(
            Some(search_term.to_string()),
            trie.search(search_term.to_string())
        );
    }

    #[test]
    fn can_find_starts_with_items() {
        let insert_term = "Foo";
        let mut trie = Trie::new();
        trie.insert(insert_term.to_string());

        let prefix = "Fo";

        assert_eq!(
            Some(prefix.to_string()),
            trie.starts_with(prefix.to_string())
        );
    }

    #[test]
    fn missing_search_term_returns_none() {
        let insert_term = "Foo";
        let search_term = "Bar";

        let mut trie = Trie::new();
        trie.insert(insert_term.to_string());

        assert_eq!(None, trie.search(search_term.to_string()));
    }

    #[test]
    fn missing_starts_with_prefix_returns_none() {
        let insert_term = "Foo";
        let prefix = "Ba";

        let mut trie = Trie::new();
        trie.insert(insert_term.to_string());

        assert_eq!(None, trie.starts_with(prefix.to_string()));
    }

    #[test]
    fn get_ranked_results_uses_score_ordering() {
        let words_and_scores = vec![("Foreign", 10), ("For", 8), ("Foo", 0)];

        let expected_words: Vec<String> = words_and_scores
            .iter()
            .map(|(word, _)| word.to_string())
            .collect();

        let mut trie = Trie::new();

        for (word, score) in words_and_scores.iter() {
            trie.insert_with_score(word.to_string(), *score);
        }

        let ranked_results = trie.get_ranked_results("Fo".to_string()).unwrap();

        assert_eq!(expected_words, ranked_results);
    }

    #[test]
    fn get_k_ranked_results_returns_correct_count() {
        let words_and_scores = vec![("Foreign", 10), ("For", 8), ("Foo", 0)];

        // TODO: This seems like a silly way to construct this.
        let expected_words: Vec<String> = words_and_scores
            .iter()
            .map(|(word, _)| word.to_string())
            .collect::<Vec<String>>()[..2]
            .to_vec();

        let mut trie = Trie::new();

        for (word, score) in words_and_scores.iter() {
            trie.insert_with_score(word.to_string(), *score);
        }

        let ranked_results = trie.get_k_ranked_results("Fo".to_string(), 2).unwrap();

        assert_eq!(expected_words[..2], ranked_results);
    }
}
