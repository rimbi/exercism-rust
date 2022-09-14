pub fn build_proverb(list: &[&str]) -> String {
    if list.len() < 1 {
        return "".to_string();
    }
    let mut proverbs = list
        .windows(2)
        .map(|words| format!("For want of a {} the {} was lost.", words[0], words[1]))
        .collect::<Vec<_>>();
    proverbs.push(format!(
        "And all for the want of a {}.",
        list.first().unwrap()
    ));
    proverbs.join("\n")
}
