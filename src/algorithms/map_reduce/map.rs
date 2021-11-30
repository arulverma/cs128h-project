use std::sync::{mpsc, mpsc::Receiver};
use std::collections::HashMap;
use std::thread::JoinHandle;
use std::hash::Hash;
use std::thread;

use crate::algorithms::map_reduce::mappers::word_length_mapper;
use super::mappers::word_count_mapper;

/// @param `items` - a `Vec` of `String`s (the dataset) that must be split 
///                  into smaller chunks of data. 
/// 
/// @param `num_chunks` - an unsigned integer indicating the number of 
///                       chunks to split the data into. You can assume that 
///                       this parameter will be non-zero.
/// 
/// @return a `Vec` of `Vec`s of `String`s. The returned `Vec` should 
///         contain `num_chunks` `Vec`s of `String`s. Each `Vec` of 
///         `String`s should contain similar numbers of words. Refer to the 
///         description above for more details. 
pub fn split_into_chunks(items: &Vec<String>, num_chunks: usize) -> Vec<Vec<String>> {
    let mut chunks = vec![vec![]; num_chunks];

    for (i, item) in items.iter().enumerate() {
        chunks.get_mut(i % num_chunks).unwrap().push(item.clone());
    }

    chunks
}

/// @param `words` - a `Vec` of `String`s. You can assume that all `String`s 
///                  have been formatted and stripped of whitespace, 
///                  punctutation, etc.
/// 
/// @param `num_chunks` - an unsigned integer indicating the number nodes 
///                       that will be working in parallel to map the given 
///                       dataset. You can assume that this parameter will 
///                       be non-zero.
/// 
/// @return a `Vec` of tuples. Each tuple will contain the `JoinHandle` of 
///         each thread spawned and the `mpsc::Receiver` created by the 
///         corresponding thread.
pub fn multi_threaded_mapper(words: &Vec<String>, num_chunks: usize) 
        -> Vec<(JoinHandle<()>, Receiver<HashMap<usize, usize>>)> {
    let chunks = split_into_chunks(words, num_chunks);

    let mut threads: Vec<(JoinHandle<()>, Receiver<HashMap<usize, usize>>)> = vec![];

    for chunk in chunks {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let map = word_length_mapper(&chunk);
            tx.send(map).unwrap();
        });

        let thread = (handle, rx);
        threads.push(thread);
    }

    threads
}

/// @param `words` - a `Vec` of `String`s. You can assume that all `String`s 
///                  have been formatted and stripped of whitespace, 
///                  punctutation, etc.
/// 
/// @param `num_chunks` - an unsigned integer indicating the number nodes 
///                       that will be working in parallel to map the given 
///                       dataset. You can assume that this parameter will 
///                       be non-zero.
/// 
/// @param `func` - a function pointer to a function that takes in a 
///                 borrowed `Vec` of `String`s and returns a `HashMap` with 
///                 generic keys and values of type `usize`. Refer to the 
///                 description above for more information on generics. 
/// 
/// @return a `Vec` of tuples. Each tuple will contain the `JoinHandle` of 
///         each thread spawned and the `mpsc::Receiver` created by the 
///         corresponding thread.
pub fn multi_threaded_mapper_generic<KeyType: 'static + Eq + Hash + Send>(
            words: &Vec<String>, 
            num_chunks: usize, 
            func: fn(&Vec<String>) -> HashMap<KeyType, usize>) 
        -> Vec<(JoinHandle<()>, Receiver<HashMap<KeyType, usize>>)> {
            let chunks = split_into_chunks(words, num_chunks);

            let mut threads: Vec<(JoinHandle<()>, Receiver<HashMap<KeyType, usize>>)> = vec![];
    
            for chunk in chunks {
                let (tx, rx) = mpsc::channel();
    
                let handle = thread::spawn(move || {
                    let map = func(&chunk);
                    tx.send(map).unwrap();
                });
    
                let thread = (handle, rx);
                threads.push(thread);
            }
    
            threads
}
