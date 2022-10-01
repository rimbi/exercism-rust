pub fn count(lines: &[&str]) -> u32 {
    let height = lines.len();
    if height == 0 {
        return 0;
    }
    let width = lines[0].len();
    let char_at = |row: usize, col: usize| lines[row].chars().nth(col).unwrap();
    let mut corners = vec![];
    let mut rectangles = vec![];
    for h in 0..height - 1 {
        for w in 0..width {
            match char_at(h, w) {
                ' ' => corners.clear(),
                '+' => {
                    if !corners.is_empty() {
                        // Find the corner below
                        for r in h + 1..height {
                            if char_at(r, w) == '+' {
                                for c0 in &corners {
                                    let corners = (*c0, (r, w));
                                    if is_rectangle(lines, &corners) {
                                        rectangles.push(corners);
                                    }
                                }
                            }
                        }
                    }
                    corners.push((h, w));
                }
                '-' => {}
                '|' => {}
                _ => panic!("Invalid character!"),
            }
        }
        corners.clear();
    }
    rectangles.len() as u32
}

/// Given 2 opposite corner points, returns true if the region depicts 
/// a rectangle, false otherwise.
fn is_rectangle(lines: &[&str], corners: &((usize, usize), (usize, usize))) -> bool {
    let &((r0, c0), (r1, c1)) = corners;
    let char_at = |row: usize, col: usize| lines[row].chars().nth(col).unwrap();
    if char_at(r1, c0) != '+' || char_at(r0, c1) != '+' {
        return false;
    }
    let upper_edge = &lines[r0][c0..c1];
    if upper_edge.contains(|c| c != '-' && c != '+') {
        return false;
    }
    let lower_edge = &lines[r1][c0..c1];
    if lower_edge.contains(|c| c != '-' && c != '+') {
        return false;
    }
    if lines[r0..r1].iter().any(|l| {
        let c = l.as_bytes()[c0];
        c != b'+' && c != b'|'
    }) {
        return false;
    }
    if lines[r0..r1].iter().any(|l| {
        let c = l.as_bytes()[c1];
        c != b'+' && c != b'|'
    }) {
        return false;
    }
    true
}
