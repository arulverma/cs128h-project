use textplots::{Chart, Plot, Shape};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fs;
use mappers::word_length_mapper;
use reduce::thread_reducer;
use map::multi_threaded_mapper_generic;

/// This module contains code that takes in a dataset and maps each piece of 
/// data to some value. These functions are used in the MAP stage in MapReduce. 
/// These functions can stand alone and map an entire dataset. They may also be 
/// on a number of distinct distributed nodes to map pieces of an entire dataset. 
/// The mapped data on each node will then be combined to produce a result 
/// equivalent to that of mapping an entire dataset on a single node.
mod mappers {
    use std::collections::HashMap;

    // STEP 1: COMPLETE THIS FUNCTION FIRST

    /// [COMPLETE THIS FUNCTION]
    /// Takes a `Vec` of words and creates a map from the word (the key) to the 
    /// number of occurrences (the value) of that word in the given `Vec`.
    /// 
    /// @param `words` - a `Vec` of `String`s. You can assume that all `String`s 
    ///                  have been formatted and stripped of whitespace, 
    ///                  punctutation, etc.
    /// 
    /// @return a `HashMap` with distinct words as keys and the number of 
    ///         occurrences of the key in given `Vec` of `String`s.
    pub fn word_count_mapper(words: &Vec<String>) -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();

        for word in words {
            let word_string = word.to_string();
            if map.contains_key(word) {
                map.insert(word_string, map.get(word).unwrap() + (1 as usize));
                continue;
            }
            map.insert(word_string, 1);
        }

        map
    }

    // STEP 2: COMPLETE THIS FUNCTION SECOND

    /// [COMPLETE THIS FUNCTION]
    /// Takes a `Vec` of words and creates a map from the length of a word (the
    /// key) to the number of occurrences of words with that length (the value) 
    /// in the given `Vec`.
    /// 
    /// @param `words` - a `Vec` of `String`s. You can assume that all `String`s 
    ///                  have been formatted and stripped of whitespace, 
    ///                  punctutation, etc.
    /// 
    /// @return a `HashMap` with distinct word *LENGTHS* as keys and the number of 
    ///         occurrences of the key in given `Vec` of `String`s.
    pub fn word_length_mapper(words: &Vec<String>) -> HashMap<usize, usize> {
        let mut map: HashMap<usize, usize> = HashMap::new();

        for word in words {
            let len = word.len() as usize;
            if map.contains_key(&len) {
                map.insert(len, map.get(&len).unwrap() + (1 as usize));
                continue;
            }
            map.insert(len, 1);
        }

        map
    }
}

/// This module contains code that assigns work to multiple nodes (threads) and 
/// manages the data mapping on each node. This is the MAP stage of MapReduce. 
mod map {
    use std::sync::{mpsc, mpsc::Receiver};
    use std::collections::HashMap;
    use std::thread::JoinHandle;
    use std::hash::Hash;
    use std::thread;

    use super::mappers::word_length_mapper;
    use super::mappers::word_count_mapper;

    // STEP 3: COMPLETE THIS FUNCTION NEXT

    /// [COMPLETE THIS FUNCTION]
    /// Splits a vector of strings (`items`) into the specified number of 
    /// sub-vectors (`num_chunks`) such that the difference in lengths of each 
    /// sub-vector (chunk) is no more or no less than 1 word.
    /// 
    /// This function helps to distribute the workload of mapping a dataset
    /// to multiple nodes by dividing up the work into chunks. You will use this
    /// function to help assigning work to multiple nodes. 
    /// 
    /// It is up to you how you distribute the workload. You can sequentially 
    /// assign words at an index offset to a chunk of the same index (modulo). 
    /// You could also choose to assign the first `k` pieces of data to the 1st
    /// chunk. We will only be checking the length of each of the chunks you 
    /// form. Make sure that you do not discard any pieces of data. If you do 
    /// discard any data, your final results will differ from what we expect. 
    /// 
    ///     -> abs ( length( result[0] ) - length( result[1] ) ) <= 1
    ///     -> abs ( length( result[1] ) - length( result[2] ) ) <= 1
    ///     -> abs ( length( result[2] ) - length( result[3] ) ) <= 1
    ///     -> abs ( length( result[3] ) - length( result[0] ) ) <= 1
    ///     etc...
    /// 
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

    // STEP 4: COMPLETE THIS FUNCTION NEXT

    /// [COMPLETE THIS FUNCTION]
    /// Takes a `Vec` of `String`s and spawns the specified number of threads 
    /// (`num_chunks`). Each thread will call the `word_count_mapper mapper` 
    /// function using chunks of data of roughly-equal length (see the 
    /// description of `split_into_chunks`).
    /// 
    /// This is the MAP stage in MapReduce. This function will break up the 
    /// dataset into smaller chunks, distribute each chunk to a node, and 
    /// monitor the progress of each node.
    /// 
    /// HINT: use the `split_into_chunks` function!
    /// 
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

    // STEP 5: COMPLETE THIS FUNCTION NEXT
    //         COMPLETE THIS FUNCTION ONLY AFTER HAVING COMPLETED STEP 4
    //         THIS IMPLEMENTATION WILL BE SIMILAR TO THAT IN STEP 4

    /// [COMPLETE THIS FUNCTION]
    /// This function is a `GENERIC` version of the `multi_threaded_mapper`
    /// function. It is identical to the function above apart from the fact that
    /// there is an additional parameter that takes in a `function pointer`. The 
    /// function pointer is, well, a pointer to a function. The thing to note is 
    /// that this  function pointer is a pointer to a generic function (more on 
    /// this in a bit). 
    /// 
    /// This function takes in a `Vec` of `String`s and returns a `HashMap` with 
    /// keys of *ALMOST* any type and values of type `usize`. The key values are 
    /// bounded by certain parameters, as indicated here in the function 
    /// signature: (`<KeyType: 'static + Eq + Hash + Send>`). The first bound is 
    /// a `LIFETIME`. The `'static` lifetime indicates that the variable will 
    /// last the *ENTIRE* runtime of the *FUNCTION*. The remaining bounds are 
    /// `Trait`s that `KeyType` must implement. 
    /// 
    ///     -> `Eq`: `KeyType` can be compared for equality
    ///     -> `Hash`: `KeyType` can be hashed 
    ///     -> `Send`: `KeyType` can be sent safely between threads 
    /// 
    /// Refer to the description of `multi_threaded_mapper` for a detailed 
    /// explanation of the functionality of this method. The only difference is 
    /// that this function will call the function passed in by the parameter 
    /// `func` (a function pointer) in each thread on the chunk of data assigned
    /// to the corresponding thread. 
    /// 
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
}

/// This module contains code that combines the results generated by all of the 
/// nodes (threads). This is the REDUCE stage of MapReduce. 
mod reduce {
    use std::collections::HashMap;
    use std::sync::mpsc::Receiver;
    use std::thread::JoinHandle;
    use std::hash::Hash;

    // STEP 6: COMPLETE THIS FUNCTION NEXT

    /// [COMPLETE THIS FUNCTION]
    /// Receives the results contained in the `Vec` of tuples of `JoinHandle`s 
    /// and `mpsc::Receiver`s and then combines the received results into a 
    /// single `HashMap` from `KeyType` to a `usize` such that the values mapped 
    /// to the same key in different received `HashMaps` will be added to each 
    /// other and stored under that same key in the resulting `HashMap`.
    /// 
    /// This function will be able to consume the `Vec` returned by either the 
    /// `multi_threaded_mapper_generic` function or the `multi_threaded_mapper`
    /// function. 
    /// 
    /// Refer to the description of the `multi_threaded_mapper_generic` function
    /// for more information on `GENERICS`. Refer to the `map` module and its
    /// functions for more information on how the given `Vec` is generated.
    /// 
    /// @param `receivers` - a `Vec` of tuples. Each tuple will contain the 
    ///                      `JoinHandle` of each thread spawned and the 
    ///                      `mpsc::Receiver` created by the corresponding 
    ///                      thread.
    /// 
    /// @return a `HashMap` from `KeyType` to a `usize` that is the result of 
    ///         combining the received `HashMaps` by adding values with the 
    ///         same key and storing under the same key.
    pub fn thread_reducer<KeyType: Eq + Hash>(
                receivers: Vec<(JoinHandle<()>, Receiver<HashMap<KeyType, usize>>)>) 
            -> HashMap<KeyType, usize> {
        let mut map: HashMap<KeyType, usize> = HashMap::new();
        let mut received = 0;
        let len_receivers = receivers.len();
        while received < len_receivers {
            for receiver in &receivers {
                let (_, rx) = receiver;
                if let Ok(count_map) = rx.try_recv() {
                    for (key, val) in count_map {
                        let value = map.entry(key).or_insert(0);
                        *value += val;
                    }
                    received += 1;
                }
            }
        }
        map
    }
}

pub fn run(text: String) -> HashMap<usize, usize> {
    // This function is only here so that main() is not cluttered.
    // Feel free to delete this function or change it to play around and experiment :)

    let start: Instant = Instant::now();
    let cleaned_contents: String = text;

    let words: Vec<String> = cleaned_contents.split(" ")
        .into_iter()
        .map(|s: &str| s.to_owned())
        .collect();

    println!("Num words: {}", words.len());

    // Run and print the duration of a single function by passing a function pointer.
    let _word_counts: HashMap<usize, usize> = 
        run_and_print_duration("word_length_mapper()", &words, word_length_mapper);

    // Run and print the duration of multiple functions by passing a closure.
    let word_counts_mt: HashMap<usize, usize> = 
        run_and_print_duration(
            &format!("multi_threaded_generics for {} at {} threads", "word_length_mapper()", 10)[..],
            &words,
            move |w: &Vec<String>| {
                // you must pass in literals (unless you are borrowing) inside 
                // this closure so the closure will be interpreted as a function pointer
                // NOTE: compare the word vector borrow above ("&words") 
                // to the number of threads literal and function pointer that are consumed.
                let results = multi_threaded_mapper_generic(&w, 10, word_length_mapper.clone());
                thread_reducer(results)
            }
        );

    // assert!(word_counts_mt == _word_counts); // if you want a sanity check :)

    // Print the duration of the runtime.
    let duration: Duration = start.elapsed();
    println!("Time elapsed was: {:?}", duration);

    word_counts_mt
}

// [COMPLETED FUNCTION - DO NOT EDIT - USE THIS AS YOU SEE FIT]
fn load_file(filepath: &str) -> String {
    // Reads a file from the specified path to a String.
    // This function has already been implemented. You can use this in your own testing.
    fs::read_to_string(filepath).expect("Something went wrong reading the file")
}

// [COMPLETED FUNCTION - DO NOT EDIT - USE THIS AS YOU SEE FIT]
pub fn graph_results(results: HashMap<usize, usize>) {
    // Graph a histogram depicting the relative frequency of words by word length.
    // This function has already been implemented. You can use this in your own testing.
    let word_count = results.iter().fold(0f32, |sum, (_key, val)| sum + *val as f32);

    let mut points: Vec<(f32, f32)> = results.into_iter()
        .map(|(a, b): (usize, usize)| (a as f32, 100.0 * (b as f32 / word_count)))
        .collect();

    points.sort_by_key(|k| k.0 as i32);

    let (min, max): (f32, f32) = (0.0, 15.0);
    println!("Relative Frequency of Words by Word Length");
    println!("\ny = percent occurence of word length, x = word length");
    Chart::new(180, 100, min, max)
        .lineplot(&Shape::Bars(&points))
        .nice();
}

// [COMPLETED FUNCTION - DO NOT EDIT - USE THIS AS YOU SEE FIT]
fn run_and_print_duration<InputType, OutputType>(func_name: &str, input: InputType, func: fn(InputType) -> OutputType) -> OutputType {
    // This function prints the time taken by a a function and returns the result of that function.
    // The function that is run must take in an input parameter and return some output parameter.
    // HINT: Use this function to find the duration of your implementations of 
    //          1. word_count_mapper
    //          2. word_length_mapper
    //          3. multi_threaded_mapper
    //          4. multi_threaded_mapper_generic using the "word_count_mapper" function
    //          5. multi_threaded_mapper_generic using the "word_length_mapper" function
    // This function has already been implemented. You can use this in your own testing.
    
    let start: Instant = Instant::now();
    let result: OutputType = func(input);
    let duration: Duration = start.elapsed();

    println!("Time elapsed in implementation of {} was: {:?}", func_name, duration);

    result
}
