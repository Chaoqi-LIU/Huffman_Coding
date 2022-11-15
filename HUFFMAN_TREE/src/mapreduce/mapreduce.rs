pub mod mappers {
    use std::{collections::HashMap};
    
    pub fn char_count_mapper(chars: String) -> HashMap<char, i32> {
        let mut ret = HashMap::new();
        for c in chars.chars() {
            if ret.contains_key(&c) {
                ret.insert(c, ret[&c] + 1);
            } else {
                ret.insert(c, 1);
            }
        }
        return ret;
    }
}

pub mod map {
    use std::sync::{mpsc, mpsc::Receiver};
    use std::collections::HashMap;
    use std::thread::JoinHandle;
    use std::hash::Hash;
    use std::thread;

    pub fn split_into_chunks(items: String, num_chunks: usize) -> Vec<String> {
        let mut items : Vec<&str> = items.as_str().split("").collect();
        items.remove(0); items.pop();
        let mut res : Vec<String> = Vec::new();
        for _i in 0..num_chunks { res.push(String::new()); }
        for i in 0..items.len() {  res[i % num_chunks].push_str(items[i]);  }
        return res;
    }
    
    pub fn multi_threaded_mapper_generic<KeyType: 'static + Eq + Hash + Send>(
                words: String, 
                num_chunks: usize, 
                func: fn(String) -> HashMap<KeyType, i32>) 
            -> (Vec<JoinHandle<()>>, Receiver<HashMap<KeyType, i32>>) {
        let mut ret:Vec<JoinHandle<()>> = Vec::new();
        let (tx, rx) = mpsc::channel();
        for i in split_into_chunks(words, num_chunks) {
            let tx_clone = tx.clone();
            ret.push(thread::spawn(move || {
                tx_clone.send(func(i)).unwrap();
            }));
        }
        return (ret, rx);
    }
}

pub mod reduce {
    use std::collections::HashMap;
    use std::sync::mpsc::Receiver;
    use std::thread::JoinHandle;
    use std::hash::Hash;

    pub fn thread_reducer<KeyType: Eq + Hash>(
                receivers: (Vec<JoinHandle<()>>, Receiver<HashMap<KeyType, i32>>)) 
                -> HashMap<KeyType, i32> {
        let mut ret:HashMap<KeyType, i32> = HashMap::new();
        let (a, b) = receivers;
        for i in a {
            i.join().unwrap();
        }
        while let Ok(x) = b.recv() {
            for (k, v) in x {
                if ret.contains_key(&k) {
                    *ret.get_mut(&k).unwrap() += v;
                } else {
                    ret.insert(k, v);
                }
            }
        }
        return ret;
    }
}
