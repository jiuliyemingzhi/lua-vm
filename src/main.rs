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
    let ac = TNumber::Float(1.1);
    instant = Instant::now();
    for _ in 0..10000_0000 {
        aa = aa.add(ac);
    }
    let e = instant.elapsed();
    println!("{:?} {} {}", e , aa, aa.to_integer().shr((-10).into()));
}
