use std::{thread, sync::{Arc, RwLock}};

use rust_trie::trie::Trie;

fn main() {
    let trie = Arc::new(RwLock::new(Trie::new()));

    // Insert a few values with a weight; higher values show higher in the results.
    let vals = vec![
        ("Foo".to_string(), 0),
        ("For".to_string(), 8),
        ("Foreign".to_string(), 10),
        // Add some values that won't match just to be sure :)
        ("Bar".to_string(), 0),
        ("Baz".to_string(), 0),
    ];

    let handles: Vec<_> =  vals.into_iter()
        .map(|(val, score)| {
            let trie = Arc::clone(&trie);
            thread::spawn(move || {
                (*trie).write().unwrap().insert_with_score(val, score);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let trie = trie.read().unwrap();

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
