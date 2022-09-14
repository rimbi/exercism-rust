fn single_digit(n: u64) -> String {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!("invalid single digit"),
    }
    .to_string()
}

fn double_digit(n: u64) -> String {
    match n {
        0..=9 => single_digit(n),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        21..=29 => "twenty-".to_string() + &single_digit(n % 10),
        31..=39 => "thirty-".to_string() + &single_digit(n % 10),
        41..=49 => "forty-".to_string() + &single_digit(n % 10),
        51..=59 => "fifty-".to_string() + &single_digit(n % 10),
        61..=69 => "sixty-".to_string() + &single_digit(n % 10),
        71..=79 => "seventy-".to_string() + &single_digit(n % 10),
        81..=89 => "eighty-".to_string() + &single_digit(n % 10),
        91..=99 => "ninety-".to_string() + &single_digit(n % 10),
        _ => panic!("invalid double digit"),
    }
}

fn three_digit(n: u64) -> String {
    let mut res = String::new();
    if n / 100 != 0 {
        res.push_str(&single_digit(n / 100));
        res.push_str(" hundred");
    }
    if n % 100 != 0 {
        if !res.is_empty() {
            res.push_str(" ");
        }
        res.push_str(&double_digit(n % 100));
    }
    res
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let mut n = n;
    let mut res = vec![];
    let bases = [
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];
    for base in bases {
        let value = three_digit(n % 1000);
        if !value.is_empty() {
            if !base.is_empty() {
                res.push(base.to_string());
            }
            res.push(value);
        }
        n /= 1000;
        if n == 0 {
            break;
        }
    }
    res.reverse();
    res.join(" ")
}
