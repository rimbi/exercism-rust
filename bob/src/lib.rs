pub fn reply(message: &str) -> &str {
    let is_question = |m: &str| m.ends_with("?");
    let is_yelling = |m: &str| m.to_uppercase() == m;
    let is_alpha = |m: &str| m.chars().any(|c| c.is_alphabetic());
    match message.trim() {
        "" => "Fine. Be that way!",
        m if is_alpha(m) && is_question(m) && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if is_alpha(m) && is_yelling(m) => "Whoa, chill out!",
        m if is_question(m) => "Sure.",
        _ => "Whatever.",
    }
}
