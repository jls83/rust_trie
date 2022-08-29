use std::collections::HashMap;

type ArenaTrieIndex = usize;
type ArenaTrieNodeValue = char;  // TODO: generic over type T

#[derive(Clone, Eq, PartialEq)]
enum ArenaTrieNodeType {
    Final(String),
    Intermediate,
}

struct ArenaTrieNode {
    children: HashMap<ArenaTrieNodeValue, ArenaTrieIndex>,
    node_type: ArenaTrieNodeType,
    word_score: Option<i64>,
    aggregate_score: i64,
}

impl ArenaTrieNode {
    fn new() -> Self {
        ArenaTrieNode {
            children: HashMap::new(),
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
}

impl ArenaTrie {
    pub fn new() -> Self {
        ArenaTrie {
            arena: vec![ArenaTrieNode::new()],
            // root: ArenaTrieNode::new(),
        }
    }

    fn _insert(&mut self, word: String, score: i64) {
        let mut current_node_index = 0;

        for char in word.chars() {
            if let Some(next_idx) = self.arena[current_node_index].children.get(&char) {
                current_node_index = *next_idx;
            } else {
                let next_idx = self.arena.len();
                self.arena.push(ArenaTrieNode::new());
                self.arena[current_node_index].children.insert(char, next_idx);
                current_node_index = next_idx;
            }
        }

        let mut current_node = &mut self.arena[current_node_index];
        current_node.node_type = ArenaTrieNodeType::Final(word);
        current_node.word_score = Some(score);
    }

    fn _search(&self, word: &String) -> Option<&ArenaTrieNode> {
        let mut current_node_index = 0;

        for char in word.chars() {
            match self.arena[current_node_index].children.get(&char) {
                Some(next_idx) => current_node_index = *next_idx,
                None => return None,
            }
        }

        Some(&self.arena[current_node_index])
    }

    pub fn insert(&mut self, word: String) {
        self._insert(word, 0);
    }


    pub fn insert_with_score(&mut self, word: String, score: i64) {
        self._insert(word, score);
    }

    pub fn search(&self, word: String) -> Option<String> {
        match self._search(&word) {
            Some(ArenaTrieNode {
                node_type: ArenaTrieNodeType::Final(result),
                ..
            }) => Some(result.to_string()),
            _ => None,
        }
    }

    pub fn starts_with(&self, prefix: String) -> Option<String> {
        match self._search(&prefix) {
            Some(_) => Some(prefix),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ArenaTrie;

    #[test]
    fn can_search_for_term() {
        let search_term = "Foo";
        let mut trie = ArenaTrie::new();
        trie.insert(search_term.to_string());

        assert_eq!(
            Some(search_term.to_string()),
            trie.search(search_term.to_string())
        );
    }

    #[test]
    fn can_search_for_term_with_score() {
        let search_term = "Foo";
        let mut trie = ArenaTrie::new();
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

        let mut trie = ArenaTrie::new();
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
        let mut trie = ArenaTrie::new();
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

        let mut trie = ArenaTrie::new();
        trie.insert(insert_term.to_string());

        assert_eq!(None, trie.search(search_term.to_string()));
    }

    #[test]
    fn missing_starts_with_prefix_returns_none() {
        let insert_term = "Foo";
        let prefix = "Ba";

        let mut trie = ArenaTrie::new();
        trie.insert(insert_term.to_string());

        assert_eq!(None, trie.starts_with(prefix.to_string()));
    }

    // #[test]
    // fn get_ranked_results_uses_score_ordering() {
    //     let words_and_scores = vec![("Foreign", 10), ("For", 8), ("Foo", 0)];

    //     let expected_words: Vec<String> = words_and_scores
    //         .iter()
    //         .map(|(word, _)| word.to_string())
    //         .collect();

    //     let mut trie = ArenaTrie::new();

    //     for (word, score) in words_and_scores.iter() {
    //         trie.insert_with_score(word.to_string(), *score);
    //     }

    //     let ranked_results = trie.get_ranked_results("Fo".to_string()).unwrap();

    //     assert_eq!(expected_words, ranked_results);
    // }
}
