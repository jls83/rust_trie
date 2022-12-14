use rust_trie::trie::Trie;

fn main() {
    let mut trie = Trie::new();

    // Insert a few values with a weight; higher values show higher in the results.
    trie.insert_with_score("Foo".to_string(), 0);
    trie.insert_with_score("For".to_string(), 8);
    trie.insert_with_score("Foreign".to_string(), 10);

    // Add some values that won't match just to be sure :)
    trie.insert_with_score("Bar".to_string(), 0);
    trie.insert_with_score("Baz".to_string(), 0);

    // Should print values in the expected order ("Foreign", "For", "Foo").
    for result in trie.get_ranked_results("Fo".to_string()).unwrap().iter() {
        println!("{}", result);
    }

    // Testing for membership works as expected as well.
    match trie.search("For".to_string()) {
        Some(found_word) => println!("Found {}", found_word),
        _ => println!("Did not find word"),
    }

    match trie.search("Bar".to_string()) {
        Some(found_word) => println!("Found {}", found_word),
        _ => println!("Did not find word"),
    }

    match trie.search("Banana".to_string()) {
        Some(found_word) => println!("Found {}", found_word),
        _ => println!("Did not find word"),
    }
}
