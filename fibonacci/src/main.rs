extern crate rpassword;

use num_bigint::BigUint;
use num_traits::One;
use std::time::Instant;

const ITERATIONS: i32 = 200_000;

fn main() {
    fibonacci_prim()
    // println!("Hello, world!");
    // let mut cur: BigUint = One::one();
    // let mut next: BigUint =  One::one();
    // fibonacci( cur.clone(), next.clone());
    // fibonacci2( cur.clone(), next.clone());
    // fibonacci3(&mut cur, &mut next);
}

fn fibonacci_prim(){

    let before = Instant::now();
    for n in 0..ITERATIONS {
        is_prime(n as u32);
    }
    println!("Prim time: {:.2?}", before.elapsed())
}
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}

fn fibonacci(mut cur: BigUint, mut next: BigUint) {
    let mut tmp = One::one();
    let before = Instant::now();
    for _ in 0..ITERATIONS {
        tmp = &cur + &next;
        cur = next;
        next = tmp;
    }
    println!("Move ref time: {:.2?}", before.elapsed())
}

fn fibonacci2(mut cur: BigUint, mut next: BigUint) {
    let before = Instant::now();
    for _ in 0..ITERATIONS {
        (cur, next) = (next.clone(), cur + next)
    }
    println!("Move/Clone ref time: {:.2?}", before.elapsed())
}

fn fibonacci3(cur: &mut BigUint, next: &mut BigUint) {
    let mut tmp = One::one();
    let before = Instant::now();
    for _ in 0..ITERATIONS {
        tmp = cur.clone() + next.clone();
        *cur = next.clone();
        *next = tmp;
    }
    println!("Borrow ref time: {:.2?}", before.elapsed())
}