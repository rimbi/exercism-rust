use std::{cmp::min, collections::HashMap};

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut book_counts = HashMap::new();
    for book in books {
        *book_counts.entry(book).or_insert(0) += 1;
    }
    let mut groups = HashMap::new();
    loop {
        let mut different_books = 0;
        for (_book, count) in &mut book_counts {
            if *count > 0 {
                *count -= 1;
                different_books += 1;
            }
        }
        if different_books == 0 {
            break;
        }
        *groups.entry(different_books).or_insert(0) += 1;
    }
    let mut three_books = *groups.entry(3).or_default();
    let mut five_books = *groups.entry(5).or_default();
    let mut four_books = *groups.entry(4).or_default();
    let min = min(three_books, five_books);
    four_books += 2 * min;
    three_books -= min;
    five_books -= min;
    *groups.entry(3).or_default() = three_books;
    *groups.entry(4).or_default() = four_books;
    *groups.entry(5).or_default() = five_books;
    groups
        .iter()
        .map(|(k, v)| {
            v * match k {
                1 => 800,
                2 => 2 * 8 * 95,
                3 => 3 * 8 * 90,
                4 => 4 * 8 * 80,
                5 => 5 * 8 * 75,
                _ => panic!("invalid state!"),
            }
        })
        .sum()
}
