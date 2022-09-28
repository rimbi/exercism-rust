use std::{collections::HashMap, sync::mpsc, thread};

#[derive(Debug, Clone)]
enum Msg {
    Process(String),
    Exit,
}
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let workers = (0..worker_count)
        .map(|_| {
            let (tx, rx) = mpsc::channel();
            let handler = thread::spawn(move || {
                let mut map = HashMap::new();
                for s in rx {
                    match s {
                        Msg::Process(s) => {
                            for c in s.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
                                *map.entry(c).or_insert(0) += 1;
                            }
                        }
                        Msg::Exit => return map,
                    }
                }
                map
            });
            (tx, handler)
        })
        .collect::<Vec<_>>();
    for (msg, (tx, _)) in input
        .iter()
        .map(|s| Msg::Process(s.to_string()))
        .zip(workers.iter().cycle())
    {
        tx.send(msg).unwrap();
    }
    for (tx, _) in &workers {
        tx.send(Msg::Exit).unwrap();
    }

    let mut acc = HashMap::new();
    for (_, handler) in workers {
        let m = handler.join().unwrap();
        for (k, v) in m {
            *acc.entry(k).or_insert(0) += v;
        }
    }
    acc
}
