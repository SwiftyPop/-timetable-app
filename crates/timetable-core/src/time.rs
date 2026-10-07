use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use std::sync::Mutex;

/// Malaysia Standard Time (MYT) is UTC+8 with no daylight saving time.
pub fn malaysia_offset() -> FixedOffset {
    FixedOffset::east_opt(8 * 3600).expect("Valid +08:00 offset")
}

/// Abstract clock trait allowing time injection and deterministic testing.
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<FixedOffset>;
}

/// System clock using true local/UTC time mapped to UTC+8.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<FixedOffset> {
        Utc::now().with_timezone(&malaysia_offset())
    }
}

/// Fake clock for unit testing schedules at arbitrary times,
/// including weekends, midnight crossovers, and semester boundaries.
pub struct FakeClock {
    current_time: Mutex<DateTime<FixedOffset>>,
}

impl FakeClock {
    pub fn new(time: DateTime<FixedOffset>) -> Self {
        Self {
            current_time: Mutex::new(time),
        }
    }

    pub fn from_ymd_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> Self {
        let naive = NaiveDate::from_ymd_opt(year, month, day)
            .expect("Valid date")
            .and_hms_opt(hour, min, sec)
            .expect("Valid time");
        let dt = DateTime::<FixedOffset>::from_naive_utc_and_offset(
            naive - chrono::Duration::hours(8),
            malaysia_offset(),
        );
        Self::new(dt)
    }

    pub fn set_time(&self, time: DateTime<FixedOffset>) {
        let mut lock = self.current_time.lock().unwrap();
        *lock = time;
    }

    pub fn advance_minutes(&self, minutes: i64) {
        let mut lock = self.current_time.lock().unwrap();
        *lock = *lock + chrono::Duration::minutes(minutes);
    }

    pub fn advance_days(&self, days: i64) {
        let mut lock = self.current_time.lock().unwrap();
        *lock = *lock + chrono::Duration::days(days);
    }
}

impl Clock for FakeClock {
    fn now(&self) -> DateTime<FixedOffset> {
        *self.current_time.lock().unwrap()
    }
}
