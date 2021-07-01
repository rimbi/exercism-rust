pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-', '_', ':'][..])
        .flat_map(|w| {
            let mut it = w.chars();
            it.take(1).chain(
                w.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
