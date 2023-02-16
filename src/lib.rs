extern crate chrono;

use chrono::{Date, DateTime, TimeZone, Utc};

const UNIX_EPOCH: JulianDate = JulianDate(2440587.5);
const SECONDS_PER_DAY: u64 = 24 * 60 * 60;
const JAN_2000: JulianDate = JulianDate(2451545.0);
const LEAP_SECONDS: JulianDate = JulianDate(0.0008);

#[derive(Debug, Clone, Copy)]
struct JulianDate(f64);

impl JulianDate {
    fn ceil_days(&self) -> f64 {
        self.0.ceil()
    }

    fn to_datetime(self) -> Option<DateTime<Utc>> {
        match Utc.timestamp_opt(
            ((self - UNIX_EPOCH).0 * SECONDS_PER_DAY as f64).round() as i64,
            0,
        ) {
            chrono::LocalResult::None => None,
            chrono::LocalResult::Single(date) => Some(date),
            chrono::LocalResult::Ambiguous(min, max) => {
                dbg!((min, max));
                None
            }
        }
    }
}

impl From<DateTime<Utc>> for JulianDate {
    fn from(date: DateTime<Utc>) -> Self {
        Self((date.timestamp() as f64 / 86400.0) + 2440587.5)
    }
}

impl std::ops::Sub<JulianDate> for JulianDate {
    type Output = Self;

    fn sub(self, rhs: JulianDate) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Add<JulianDate> for JulianDate {
    type Output = Self;

    fn add(self, rhs: JulianDate) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

/// Calculates the approximate sunset and sunrise times at a given latitude, longitude, and altitude
///
/// # Arguments
///
/// * `date` - The date on which to calculate the sunset and sunrise
/// * `latitude` - The latitude at which to calculate the times. Expressed as degrees
/// * `longitude` - The longitude at which to calculate the times. Expressed as degrees
/// * `elevation` - The elevation at which to calculate the times. Expressed as meters above sea level
///
/// # Return value
///
/// Returns a tuple of `(sunrise,sunset)`
///
/// # Examples
///
/// ```
/// //Calculate the sunset and sunrise times today at Sheffield university's new computer science building
/// let times = sun_times(Utc::today(),53.38,-1.48,100.0);
/// println!("Sunrise: {}, Sunset: {}",times.0,times.1);
/// ```
pub fn sun_times(
    date: Date<Utc>,
    latitude: f64,
    longitude: f64,
    elevation: f64,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    //see https://en.wikipedia.org/wiki/Sunrise_equation

    const ARGUMENT_OF_PERIHELION: f64 = 102.9372;

    let julian_date = JulianDate::from(date.and_hms(0, 0, 0));

    let elevation_correction = -2.076 * (elevation.sqrt()) / 60.0;

    let days_since_2000 = (julian_date - JAN_2000 + LEAP_SECONDS).ceil_days();

    let mean_solar_time = days_since_2000 - (longitude / 360.0);
    let solar_mean_anomaly = (357.5291 + 0.98560028 * mean_solar_time).rem_euclid(360.0);
    let center = 1.9148 * solar_mean_anomaly.to_radians().sin()
        + 0.0200 * (2.0 * solar_mean_anomaly).to_radians().sin()
        + 0.0003 * (3.0 * solar_mean_anomaly).to_radians().sin();
    let ecliptic_longitude =
        (solar_mean_anomaly + center + 180.0 + ARGUMENT_OF_PERIHELION).rem_euclid(360.0);

    let declination =
        (ecliptic_longitude.to_radians().sin() * (23.44f64).to_radians().sin()).asin();
    let hour_angle = (((-0.83 + elevation_correction).to_radians().sin()
        - (latitude.to_radians().sin() * declination.sin()))
        / (latitude.to_radians().cos() * declination.cos()))
    .acos()
    .to_degrees();

    if hour_angle.is_nan() {
        return None;
    }

    let solar_transit =
        JAN_2000.0 + mean_solar_time + 0.0053 * solar_mean_anomaly.to_radians().sin()
            - 0.0069 * (2.0 * ecliptic_longitude).to_radians().sin();
    let solar_transit_julian = JulianDate(solar_transit);

    let julian_rise = JulianDate(solar_transit_julian.0 - hour_angle / 360.0);
    let julian_set = JulianDate(solar_transit_julian.0 + hour_angle / 360.0);
    let rise = julian_rise.to_datetime();
    let set = julian_set.to_datetime();
    if let (Some(rise), Some(set)) = (rise, set) {
        Some((rise, set))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};

    #[test]
    ///Test for https://github.com/Eroc33/sun-times/issues/1
    fn sunrise_and_sunset_land_on_requested_day() {
        let date_range = std::iter::successors(Some(Utc.ymd(2022, 1, 1)), |date| {
            let next = *date + Duration::days(1);
            if next > Utc.ymd(2022, 12, 12) {
                None
            } else {
                Some(next)
            }
        });
        for date in date_range {
            let times = super::sun_times(date, 53.38, -1.48, 0.0);
            assert!(times.is_some());
            let times = times.unwrap();
            assert_eq!(date, times.0.date());
            assert_eq!(date, times.1.date());
        }
    }
}
