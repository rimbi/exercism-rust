use time::{PrimitiveDateTime as DateTime, ext::NumericalDuration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(1e9.seconds())
}
