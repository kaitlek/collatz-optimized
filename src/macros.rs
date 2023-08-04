#[macro_export]
macro_rules! billions { ($n: expr) => { $n * 1_000_000_000 } }

#[macro_export]
macro_rules! millions { ($n: expr) => { $n * 1_000_000 } }
