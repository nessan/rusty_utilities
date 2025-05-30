# `Stopwatch`

A `Stopwatch` measures the time taken by a code block.

## Usage

You might use it like this:

```rust
let sw = rusty_utilities::Stopwatch::new();
do_work();
println!("`do_work` time: {sw}");
```

By default, the time is output as seconds from the creation of the stopwatch.
If the time is less than one second, it is output as milliseconds.
Numbers are printed to two decimal places.

## Note

The primary goal is ease of use, and the type is kept deliberately simple.

Use a `rusty_utilities::Stopwatch` for cheap and cheerful performance measurement.
It is not a replacement for the many more complete but more complex code profiling tools.

A `Stopwatch` measures the elapsed time in seconds from a _zero time_ set at creation or by calling the stopwatch's `reset()` method.
Sticking to seconds as the standard unit makes everything consistent.
In any case, a double number of seconds gives you 15 or 16 places of accuracy, which is enough for any conceivable application.
