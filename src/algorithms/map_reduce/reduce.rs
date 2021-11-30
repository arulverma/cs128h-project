use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use std::hash::Hash;

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
