use chrono::{Datelike, Duration, Month, NaiveDate, TimeZone, Utc};
use num_traits::FromPrimitive;

/// uses the [sun_times::altitude] function to show a plot of sun up/sun down times
fn main() {
    let date_range =
        std::iter::successors(Some(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()), |date| {
            let next = *date + Duration::days(1);
            if next > NaiveDate::from_ymd_opt(2022, 12, 31).unwrap() {
                None
            } else {
                Some(next)
            }
        });

    let mut sun_up = [[false; 24]; 365];
    for (x, date) in date_range.enumerate() {
        for (y, hour) in (0..24).enumerate() {
            let date = date
                .and_hms_opt(hour, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .single()
                .unwrap();
            let latitude = 80.0;
            let altitude = sun_times::altitude(date, latitude, 0.0);

            let is_visible = (-0.0..=90.0).contains(&altitude);

            sun_up[x][y] = is_visible
        }
    }
    plot(sun_up);
}

fn plot(bitmap: [[bool; 24]; 365]) {
    let index = |x, y| {
        bitmap
            .get(x)
            .and_then(|inner: &[bool; 24]| inner.get(y))
            .copied()
            .unwrap_or(false)
    };
    print!("  ");
    for x in (0..365).step_by(2) {
        let date = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .unwrap()
            .with_ordinal(x + 1)
            .unwrap();
        let month = Month::from_u32(date.month()).unwrap();
        let month_char = month.name().chars().next().unwrap();
        print!("{month_char}")
    }
    println!();
    for x in (0..365).step_by(2) {
        let date = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .unwrap()
            .with_ordinal(x + 1)
            .unwrap();
        let day_char = date.day().to_string().chars().last().unwrap();
        print!("{day_char}")
    }
    println!();
    for y in (0..24).step_by(2) {
        print!("{:>2}", y);
        for x in (0..365).step_by(2) {
            let tl = index(x, y);
            let bl = index(x, y + 1);
            let tr = index(x + 1, y);
            let br = index(x + 1, y + 1);
            let char = match (tl, bl, tr, br) {
                (true, true, true, true) => '█',
                (true, true, true, false) => '▛',
                (true, true, false, true) => '▙',
                (true, true, false, false) => '▌',
                (true, false, true, true) => '▜',
                (true, false, true, false) => '▀',
                (true, false, false, true) => '▚',
                (true, false, false, false) => '▘',
                (false, true, true, true) => '▟',
                (false, true, true, false) => '▞',
                (false, true, false, true) => '▄',
                (false, true, false, false) => '▖',
                (false, false, true, true) => '▐',
                (false, false, true, false) => '▝',
                (false, false, false, true) => '▗',
                (false, false, false, false) => '░',
            };
            print!("{char}");
        }
        println!();
    }
}
