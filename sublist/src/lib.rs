#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal
    }
    if first_list.len() == 0 {
        return Comparison::Sublist
    }
    if second_list.len() == 0 {
        return Comparison::Superlist
    }
    if second_list.windows(first_list.len()).any(|a| a == first_list) {
        return Comparison::Sublist;
    }
    if first_list.windows(second_list.len()).any(|a| a == second_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
