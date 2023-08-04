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

pub fn compute_full(mut input: u64) -> u64 {
    let mut count: u64 = 0;

    while input > 1 {
        
        count += 1;
        let m: u64 = (input & 1) ^ 1;
        input = m * (input >> 1) + (m ^ 1) * (input * 3 + 1);
    }

    count
}

