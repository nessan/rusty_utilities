#![doc = include_str!("../doc/stopwatch.md")]

use std::{
    fmt::Display,
    time::Instant,
};

#[doc = include_str!("../doc/stopwatch.md")]
pub struct Stopwatch {
    zero: Instant,
}

// Implement the `Default` trait for the `Stopwatch` type.
impl Default for Stopwatch {
    /// Creates a stopwatch with the zero time set to the current time.
    fn default() -> Self { Self::new() }
}

// Implement the `Stopwatch` type.
impl Stopwatch {
    /// Creates a stopwatch with the zero time set to the current time.
    pub fn new() -> Self { Self { zero: Instant::now() } }

    /// Sets the stopwatch's zero time to the current time.
    pub fn reset(&mut self) { self.zero = Instant::now(); }

    /// Returns the elapsed time in seconds from the stopwatch's zero time to the call time.
    pub fn elapsed(&self) -> f64 { Instant::now().duration_since(self.zero).as_secs_f64() }

    /// **Associated Method** <br>
    /// Returns a "pretty" string for a time in seconds. <br>
    /// If the time is less than 1 second, it is formatted as a number of milliseconds. <br>
    /// If the time is greater than 1 second, it is formatted as a number of seconds. <br>
    /// Numbers are formatted to 2 decimal places and output with a unit suffix (`ms` or `s`).
    ///
    /// # Examples
    /// ```
    /// use rusty_utilities::Stopwatch;
    /// assert_eq!(Stopwatch::format_seconds(0.0001), "0.10ms");
    /// assert_eq!(Stopwatch::format_seconds(0.011), "11.00ms");
    /// assert_eq!(Stopwatch::format_seconds(1.0), "1.00s");
    /// assert_eq!(Stopwatch::format_seconds(25.23456789), "25.23s");
    /// ```
    pub fn format_seconds(seconds: f64) -> String {
        if seconds < 1.0 { format!("{:.2}ms", seconds * 1000.0) } else { format!("{seconds:.2}s") }
    }
}

/// Implements the `Display` trait for the `Stopwatch` type.
///
/// # Examples
/// ```
/// use rusty_utilities::Stopwatch;
/// let sw = Stopwatch::new();
/// do_something_expensive();
/// println!("Time taken: {sw}");
/// ```
impl Display for Stopwatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::format_seconds(self.elapsed()))
    }
}
