#![feature(start)]

const fn sum_n_natural(n : u64) -> u64 {
    let mut sum = 0u64;
    let mut i = 0u64;
    while i <= n {
        sum += i;
        i+=1;
    }
    sum
}

const fn factorial(n : u64) -> u64 {
    match n {
        0u64 | 1u64 => 1u64,
        _ => n*factorial(n-1)
    }
}


#[start]
pub fn start(argc: isize, _: *const *const u8) -> isize {
    sum_n_natural(5) as isize
    // let num : u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    // factorial(argc as u64) as isize
}
