pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();
    while left != right {
        let i = (left + right) / 2;
        if array[i] == key {
            return Some(i);
        }
        if array[i] > key {
            right = i;
        } else {
            left = i;
        }
    }
    None
}
