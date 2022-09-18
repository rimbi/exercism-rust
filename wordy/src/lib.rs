pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    let command = &command[..command.len() - 1];
    let words = command.split_ascii_whitespace().skip(2).collect::<Vec<_>>();
    calculate(words.first()?.parse().ok()?, &words[1..])
}

fn calculate(res: i32, words: &[&str]) -> Option<i32> {
    let (res, words) = match words {
        [] => return Some(res),
        ["plus", num2, rest @ ..] => (res + num2.parse::<i32>().ok()?, rest),
        ["minus", num2, rest @ ..] => (res - num2.parse::<i32>().ok()?, rest),
        ["multiplied", "by", num2, rest @ ..] => (res * num2.parse::<i32>().ok()?, rest),
        ["divided", "by", num2, rest @ ..] => (res / num2.parse::<i32>().ok()?, rest),
        ["raised", "to", "the", num2, "power", rest @ ..] => {
            let num2: String = num2.chars().take_while(char::is_ascii_digit).collect();            
            (res.pow(num2.parse::<u32>().ok()?), rest)
        },
        _ => return None,
    };
    calculate(res, words)
}
