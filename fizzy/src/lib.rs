// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T, S = String> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: S,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString,
    {
        Self {
            matcher: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Default for Fizzy<T> {
    fn default() -> Self {
        Self { matchers: Default::default() }
    }
}

impl<T: ToString + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self::default()
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(self, matcher: Matcher<T>) -> Self {
        let mut matchers = self.matchers;
        matchers.push(matcher);
        Self { matchers }
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |i| {
            let mut res = String::new();
            for m in &self.matchers {
                if (m.matcher)(i) {
                    res.push_str(&m.subs);
                }
            }
            if !res.is_empty() {
                res
            } else {
                i.to_string()
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: ToString + Copy + Rem + From<u8>,
    <T as Rem>::Output: PartialEq<T>,
{
    Fizzy::<T>::new()
        .add_matcher(Matcher::new(|i| i % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|i| i % T::from(5) == T::from(0), "buzz"))
}
