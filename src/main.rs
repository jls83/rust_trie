use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};

#[derive(Clone, Eq, PartialEq)]
enum TrieNodeType {
    Final(String),
    Intermediate,
}

#[derive(Clone, Eq, PartialEq)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    node_type: TrieNodeType,
    score: i64,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            node_type: TrieNodeType::Intermediate,
            score: 0,
        }
    }
}

impl Ord for TrieNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for TrieNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    fn _insert(&mut self, word: String, score: i64) {
        let mut current_node = &mut self.root;

        for char in word.chars() {
            let mut next_node = current_node
                .children
                .entry(char)
                .or_insert(TrieNode::new());
            next_node.score += score;
            current_node = next_node;
        }

        current_node.node_type = TrieNodeType::Final(word);
    }

    fn _search(&mut self, word: &String) -> Option<&TrieNode> {
        let mut current_node = &self.root;

        for char in word.chars() {
            match current_node.children.get(&char) {
                Some(next_node) => current_node = next_node,
                None => return None,
            }
        }
        Some(current_node)
    }

    fn get_ranked_results(&mut self, prefix: String) -> Option<Vec<String>> {
        // TODO: Is this ok?
        let furthest_node = self._search(&prefix);

        let initial_children = match furthest_node {
            Some(
                TrieNode {
                    children: local_children,
                    node_type: _,
                    score: _,
                }) => Some(local_children),
            _ => None,
        };

        if initial_children == None {
            return None;
        }

        let mut result: Vec<String> = Vec::new();
        // let mut found_nodes: BinaryHeap<&TrieNode> = BinaryHeap::new();

        let mut heap: BinaryHeap<&TrieNode> = initial_children
            .unwrap()
            .iter()
            .map(|(_, v)| v)
            .collect();

        while let Some(next_node) = heap.pop() {
            // TODO: push TrieNode or String?
            // if let TrieNodeType::Final(_) = &next_node.node_type {
            //     found_nodes.push(next_node);
            // }
            if let TrieNodeType::Final(word) = &next_node.node_type {
                result.push(word.to_string());
            }

            for (_, v) in next_node.children.iter() {
                heap.push(v);
            }
        }

        // let mut result: Vec<String> = Vec::new();

        // while let Some(next_node) = found_nodes.pop() {
        //     if let TrieNodeType::Final(word) = &next_node.node_type {
        //         result.push(word.to_string());
        //     }
        // }

        Some(result)
    }

    fn insert(&mut self, word: String) {
        self._insert(word, 0);
    }

    fn insert_with_score(&mut self, word: String, score: i64) {
        self._insert(word, score);
    }


    fn search(&mut self, word: String) -> Option<String> {
        match self._search(&word) {
            // TODO: I don't love the `to_string` call here.
            Some(
                TrieNode {
                    node_type: TrieNodeType::Final(result),
                    children: _,
                    score: _,
                }) => Some(result.to_string()),
            _ => None,
        }
    }

    fn starts_with(&mut self, prefix: String) -> Option<String> {
        if let Some(_) = self._search(&prefix) {
            Some(prefix)
        } else {
            None
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

        assert_eq!(Some(search_term.to_string()), trie.search(search_term.to_string()));
    }

    #[test]
    fn can_search_for_term_with_score() {
        let search_term = "Foo";
        let mut trie = Trie::new();
        trie.insert_with_score(search_term.to_string(), 10);

        assert_eq!(Some(search_term.to_string()), trie.search(search_term.to_string()));
    }

    #[test]
    fn can_search_for_term_with_similar_entries() {
        let search_term = "Foo";
        let insert_terms = vec!["Foo", "For"];

        let mut trie = Trie::new();
        for term in insert_terms {
            trie.insert(term.to_string());
        }

        assert_eq!(Some(search_term.to_string()), trie.search(search_term.to_string()));
    }

    #[test]
    fn can_find_starts_with_items() {
        let insert_term = "Foo";
        let mut trie = Trie::new();
        trie.insert(insert_term.to_string());

        let prefix = "Fo";

        assert_eq!(Some(prefix.to_string()), trie.starts_with(prefix.to_string()));
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
}

fn main() {
    let mut trie = Trie::new();

    trie.insert_with_score("Foo".to_string(), 100);
    trie.insert_with_score("For".to_string(), 10);
    trie.insert_with_score("Foreign".to_string(), 8);
    trie.insert_with_score("Bar".to_string(), 0);
    trie.insert_with_score("Baz".to_string(), 0);

    let ranked_results = trie.get_ranked_results("Fo".to_string());

    for result in ranked_results.unwrap().iter() {
        println!("{}", result);
    }



}
