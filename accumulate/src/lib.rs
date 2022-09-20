use std::vec;

/// What should the type of _function be?
pub fn map<F, T, U>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut res = vec![];
    for value in input {
        res.push(function(value));
    }
    res
}
