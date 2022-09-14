#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let factors: Vec<_> = (1..=(num / 2)).filter(|x| num % x == 0).collect();
    let sum: u64 = factors.iter().sum();
    if sum > num {
        return Some(Classification::Abundant);
    }
    if sum == num {
        return Some(Classification::Perfect);
    }
    Some(Classification::Deficient)
}
