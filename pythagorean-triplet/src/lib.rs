use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut common_triplets = vec![
        (3u32, 4u32, 5u32),
        (5, 12, 13),
        (8, 15, 17),
        (7, 24, 25),
        (9, 40, 41),
        (11, 60, 61),
        (12, 35, 37),
        (28, 45, 53),
        (20, 21, 29),
        (15, 36, 39),
        (16, 63, 65),
        (40, 399, 401),
        (48, 575, 577),
        (15, 112, 113),
        (56, 390, 394),
    ];
    common_triplets.sort_by_key(|(a, b, c)| a + b + c);
    common_triplets
        .iter()
        .flat_map(|(a, b, c)| {
            (1u32..)
                .map(move |i| (a * i, b * i, c * i))
                .skip_while(|(a, b, c)| a + b + c < sum)
                .take_while(|(a, b, c)| a + b + c == sum)
        })
        .map(|(a, b, c)| [a, b, c])
        .collect()
}
