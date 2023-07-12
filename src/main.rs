use std::time::Instant;
use lua_vm::number::math::TNumber;

fn main() {
    let mut a = 0.0;
    let mut aa = TNumber::Int(a as i64);
    let mut instant = Instant::now();
    for _ in 0..10000_0000 {
        a = a + 1.1;
    }
    println!("{:?} {} ", instant.elapsed(), a);
    instant = Instant::now();
    for _ in 0..10000_0000 {
        aa = aa.add(TNumber::Float(1.1));
    }
    let e = instant.elapsed();
    println!("{:?} {} {}", e , aa, aa.shr(1.into()));
}
