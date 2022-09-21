pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed_float = speed as f64;
    let cars_per_hour: f64 = 221.;

    if (1..5).contains(&speed) {
        speed_float * cars_per_hour
    } else if (5..9).contains(&speed) {
        speed_float * cars_per_hour * 0.9
    } else if (9..11).contains(&speed) {
        speed_float * cars_per_hour * 0.77
    } else {
        0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.) as u32
}
