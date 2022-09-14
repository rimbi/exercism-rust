pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let contains_letters = message.chars().any(|c| c.is_alphabetic());
    let is_question = message.ends_with("?");
    let is_yelling = message == message.to_uppercase();
    match (is_question, is_yelling, contains_letters) {
        (true, true, true) => "Calm down, I know what I'm doing!",
        (true, _, _) => "Sure.",
        (_, true, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
