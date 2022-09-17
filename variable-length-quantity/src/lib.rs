const MASK_SIZE: u8 = 7;
const VLQ_MASK: u8 = 0x7F;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res = vec![];
    for &value in values.iter().rev() {
        let mut value = value;
        let b = (value & VLQ_MASK as u32) as u8;
        res.push(b);
        value >>= MASK_SIZE;
        while value != 0 {
            let b = (value & VLQ_MASK as u32) as u8;
            // Set bit
            res.push(b | 0x80);
            value >>= MASK_SIZE;
        }
    }
    res.reverse();
    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut value = 0;
    let mut bytes = bytes;
    if bytes.len() == 5 {
        if bytes[0] > 0x8f {
            return Err(Error::Overflow);
        }
        // Take 4 bits from the first byte and skip the byte
        value = (bytes[0] & 0xf) as u32;
        bytes = &bytes[1..];
    }
    let mut res = vec![];
    for &b in bytes {
        value <<= MASK_SIZE;
        let vlq_byte = b & VLQ_MASK;
        value |= vlq_byte as u32;
        if b & 0x80 == 0 {
            res.push(value);
            // Reset values for the next number
            value = 0;
        }
    }
    if res.is_empty() {
        return Err(Error::IncompleteNumber);
    }
    Ok(res)
}
