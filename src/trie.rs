use std::cmp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Eq, PartialEq)]
enum TrieNodeType {
    Final,
    Intermediate,
}

#[derive(Clone, Eq, PartialEq)]
struct TrieNode {
    value: Option<char>,
    children: HashMap<char, TrieNode>,
    node_type: TrieNodeType,
    word_score: Option<i64>,
    node_score: i64,
}

impl TrieNode {
    fn new(value: Option<char>) -> Self {
        TrieNode {
            value,
            children: HashMap::new(),
            node_type: TrieNodeType::Intermediate,
            word_score: None,
            node_score: 0,
        }
    }
}

impl Ord for TrieNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.node_score.cmp(&other.node_score)
    }
}

impl PartialOrd for TrieNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Trie {
    root: TrieNode,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct QueueWrapper<'a> {
    node: &'a TrieNode,
    nodes_previous: Vec<&'a TrieNode>,
}

#[derive(Clone, Eq, PartialEq)]
struct OutputWrapper<'a> {
    node: &'a TrieNode,
    nodes_previous: Vec<&'a TrieNode>,
}

impl<'a> OutputWrapper<'a> {
    fn join(&self) -> String {
        self.nodes_previous
            .iter()
            .map(|n| n.value.unwrap_or_default())
            .collect::<String>()
    }

    fn last(&self) -> Option<&&'a TrieNode> {
        self.nodes_previous.last()
    }

    fn output_score(&self) -> i64 {
        match self.last() {
            Some(node) => match node.word_score {
                Some(score) => score,
                _ => 0,
            },
            _ => 0,
        }
    }
}

impl Ord for OutputWrapper<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.output_score().cmp(&other.output_score())
    }
}

impl PartialOrd for OutputWrapper<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(None),
        }
    }

    fn _insert(&mut self, word: String, score: i64) {
        let mut current_node = &mut self.root;

        for char in word.chars() {
            let mut next_node = current_node
                .children
                .entry(char)
                .or_insert(TrieNode::new(Some(char)));
            next_node.node_score = cmp::max(next_node.node_score, score);
            current_node = next_node;
        }

        // Set some properties on the last node so that it can be used as a representation of the
        // incoming `word`.
        current_node.node_type = TrieNodeType::Final;
        current_node.word_score = Some(score);
    }

    fn _search(&self, word: &String) -> Option<OutputWrapper> {
        // NOTE: We do not include the root of the trie when returning results, as it only contains
        // an empty char, plus references to its children.
        let mut node = &self.root;
        let mut nodes_previous: Vec<&TrieNode> = vec![];

        for char in word.chars() {
            if let Some(next_node) = node.children.get(&char) {
                nodes_previous.push(next_node);
                node = next_node;
            } else {
                return None;
            }
        }
        Some(OutputWrapper {
            node,
            nodes_previous,
        })
    }

    fn _get_ranked_results(&self, prefix: String, k: usize) -> Option<Vec<String>> {
        // Our collection of "found" items is represented by `OutputWrapper`
        // instances so that we can use a specific `Ord` trait implementation
        // to order by the underlying word's score before returning.
        let mut found_nodes: BinaryHeap<OutputWrapper> = BinaryHeap::new();
        let mut max_word_score: i64 = 0;

        let mut heap: BinaryHeap<QueueWrapper>;

        if let Some(output_wrapper) = self._search(&prefix) {
            let OutputWrapper { node, nodes_previous } = output_wrapper;
            heap = BinaryHeap::from(vec![QueueWrapper { node, nodes_previous }]);
        } else {
            return None;
        }

        // TODO: breaking this out with an extra let to allow for some shenanigans
        while let Some(queue_wrapper) = heap.pop() {
            let QueueWrapper { node, nodes_previous } = queue_wrapper;
            // TODO: Some(max_word_score) is weird...
            if (k != 0 && node.word_score < Some(max_word_score)) && found_nodes.len() >= k {
                break;
            }
            if let TrieNodeType::Final = &node.node_type {
                found_nodes.push(OutputWrapper {
                    node,
                    nodes_previous: nodes_previous.to_owned(),
                });
                max_word_score = cmp::max(max_word_score, node.word_score.unwrap());
            }
            for child in node.children.values() {
                let mut blah = nodes_previous.to_owned();
                blah.push(child);
                heap.push(QueueWrapper {
                    node: child,
                    nodes_previous: blah,
                });
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
            match output_wrapper.node.node_type {
                TrieNodeType::Final => Some(output_wrapper.join()),
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
    use crate::Trie;

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
