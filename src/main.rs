use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_final: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_final: false,
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

    // TODO: template type here?
    fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for char in word.chars() {
            let next_node = current_node
                .children
                .entry(char)
                .or_insert(TrieNode::new());
            current_node = next_node;
        }

        current_node.is_final = true;
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

    fn search(&mut self, word: String) -> Option<String> {
        match self._search(&word) {
            Some(TrieNode { children: _, is_final: true }) => Some(word),
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
