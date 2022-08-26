use std::collections::HashMap;

enum TrieNodeType {
    Final(String),
    Intermediate,
}

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

    trie.insert("Foo".to_string());
    trie.insert("For".to_string());
    trie.insert("Bar".to_string());
    trie.insert("Baz".to_string());

    match trie.search("For".to_string()) {
        Some(result) => println!("Found {}", result),
        None => println!("Search failed"),
    }

    match trie.search("Banana".to_string()) {
        Some(result) => println!("Found {}", result),
        None => println!("Search failed"),
    }

    match trie.starts_with("Ca".to_string()) {
        Some(result) => println!("Found {} as prefix", result),
        None => println!("Prefix search failed"),
    }
}
