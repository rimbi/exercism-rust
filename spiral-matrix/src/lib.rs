pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    get_inner_spiral_matrix(size, 1)
}

fn get_inner_spiral_matrix(size: u32, start: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![];
    if size == 0 {
        return matrix;
    }
    let mut next = start + size - 1;
    // First row
    matrix.push((start..=next).collect());
    if size == 1 {
        return matrix;
    }
    // Middle rows
    for _ in 1..(size - 1) {
        matrix.push(vec![0; size as usize]);
    }
    let x = start + 2 * size - 2;
    // Last row
    matrix.push((x..=x + size - 1).rev().collect());
    if size == 2 {
        return matrix;
    }
    // Last column
    for i in (size - 1..size.pow(2)).step_by(size as usize) {
        matrix[(i / size) as usize][(i % size) as usize] = next;
        next += 1;
    }
    // First column
    next = start + 3 * size - 2;
    for i in (size..size * (size - 2) + 1).step_by(size as usize).rev() {
        matrix[(i / size) as usize][(i % size) as usize] = next;
        next += 1;
    }
    // Get and fill inner matrix
    let inner = get_inner_spiral_matrix(size - 2, next);
    for i in 1..(size - 1) {
        let replace_with = inner[(i - 1) as usize].clone();
        matrix.get_mut(i as usize)
            .unwrap()
            .splice(1..((size - 1) as usize), replace_with);
    }
    matrix
}
