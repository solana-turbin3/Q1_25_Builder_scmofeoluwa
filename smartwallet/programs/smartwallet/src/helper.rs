pub fn calculate_next_reset(current_ts: i64) -> i64 {
    const SECONDS_PER_DAY: i64 = 86_400;
    let days_since_epoch = current_ts / SECONDS_PER_DAY;
    (days_since_epoch + 1) * SECONDS_PER_DAY
}
