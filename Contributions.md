# Future Ideas

Issues to implement soon:

- better panic function, which provides crash info like file name, line number, and a backtrace
- string formatting macros, similar to `format!` in Rust's standard library but in a heapless context
- exception handling mechanism for non driver interrupts

Issues to implement some day when I will have more OS features:

- Each IRQ handles multiple interrupt sources and runs multiple handlers. Log cases where multiple handlers run for the same interrupt. When I will have a file system.