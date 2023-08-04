# Collatz-optimized
#### Finds number N which took the largest steps to reach 1 following the collatz conjecture
### _~130x faster than original C collatz conjecture programs!_



```rust
pub fn compute(mut input: u64) -> u64 {
    let start: u64 = input - 1;
    input *= input & 1;
    
    let mut count: u64 = 0;

    while input > 1 {
        count += 1;
        input *= start.wrapping_sub(input) >> 63;
        let m: u64 = (input & 1) ^ 1;
        input = m * (input >> 1) + (m ^ 1) * ((input * 3 + 1) / 2);
    }

    count
}   
```

Traditional C program to check stopping time
```bash
Time taken: 47.14 seconds.
Found largest: 949 (#63728127)
```

Collatz-Optimized
```bash
>>> Found largest: 949 (#63728127)
>>> Took 0.355 s to compute
```

Simply run with 
```bash
cargo run --release {num}
```

Inspired and checked with [this article](https://www.sciencedirect.com/science/article/pii/089812219290034F)

