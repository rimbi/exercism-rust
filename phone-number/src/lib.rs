pub fn number(user_number: &str) -> Option<String> {
    let un = user_number
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    let un = match un[..] {
        [x, n1, _, _, n2, ..] if un.len() == 11 && (x != 1 || n1 < 2 || n2 < 2) => return None,
        [n1, _, _, n2, ..] if un.len() == 10 && (n1 < 2 || n2 < 2) => return None,
        _ if un.len() == 11 => &un[1..],
        _ if un.len() == 10 => &un[..],
        _ => return None,
    };
    let res = un.iter().map(|d| d.to_string()).collect::<String>();
    Some(res)
}
