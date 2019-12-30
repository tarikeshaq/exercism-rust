use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
// Panics if it overflows
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start.checked_add_signed(Duration::seconds(1000000000)).unwrap()
}
