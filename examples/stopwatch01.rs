//! This example shows how we can use a `Stopwatch` to measure the actual time taken by a code block. <br>
//! We put a thread to sleep for a range of durations and measure the actual time taken as measured by the stopwatch.

use std::{
    thread::sleep,
    time::Duration,
};

fn main() {
    // Iterate over a range of sleep durations and measure the actual time taken as measured by a stopwatch.
    let sw = rusty_utilities::Stopwatch::new();
    for sleep_ms in (200..=2000).step_by(200) {
        let t0 = sw.elapsed();
        sleep(Duration::from_millis(sleep_ms));
        let t1 = sw.elapsed();

        // Calculate some timing statistics & print them.
        let elapsed_ms = 1000.0 * (t1 - t0);
        let expected_ms = sleep_ms as f64;
        let diff_ms = elapsed_ms - expected_ms;
        let diff_pct = (diff_ms / expected_ms).abs() * 100.0;
        println!(
            "Requested: {sleep_ms:4}ms, Elapsed: {elapsed_ms:6.1}ms => Overhead: {diff_ms:3.1}ms ({diff_pct:2.1}%)"
        );
    }

    // Print the total time taken by the program.
    println!("Total time taken: {sw}");
}
