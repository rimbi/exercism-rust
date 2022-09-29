use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    cursor: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: ?Sized + AsRef<[u8]>>(key: &'a Key) -> Xorcism<'a> {
        Self {
            key: key.as_ref(),
            cursor: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let iter = self.munge(data.to_owned());
        for (v, k) in data.iter_mut().zip(iter) {
            *v = k;
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data, I>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        Data: IntoIterator<Item = I>,
        I: Borrow<u8>,
    {
        let collect = data
            .into_iter()
            .zip(self.key.iter().cycle().skip(self.cursor))
            .map(|(v, k)| {
                *v.borrow() ^ *k
            })
            .collect::<Vec<_>>();
        self.cursor = (self.cursor + collect.len()) % self.key.len();
        collect.into_iter()
    }
}
