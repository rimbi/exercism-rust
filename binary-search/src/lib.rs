use std::array;

pub fn find<T: AsRef<[U]>, U: Ord>(array: T, key: U) -> Option<usize> {
    let array = array.as_ref();
    let mut begin = 0;
    let mut end = array.len();
    while begin != end {
        let i = (end + begin) / 2;
        if key == array[i] {
            return Some(i);
        }
        if key < array[i] {
            end = i;
        } else {
            begin = i + 1;
        }
    }
    None
}
