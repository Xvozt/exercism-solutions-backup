use time::PrimitiveDateTime as DateTime;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let one_billion_seconds = time::Duration::seconds(1_000_000_000);
    start + one_billion_seconds
}
