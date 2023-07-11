use std::time::Instant;
use lua_vm::number::math::TNumber;

fn main() {
    let mut a = 1000.0;
    let mut aa = TNumber::Float(a);
    let mut instant = Instant::now();
    for _ in 0..1000_0000 {
        a = a - -1.0;
        if a < 1000.0 {
            a = 0.0;
        }
    }
    println!("{:?} {} ", instant.elapsed(), a);
    instant = Instant::now();
    for _ in 0..1000_0000 {
        aa = aa - -TNumber::Int(1);
        if aa < TNumber::Float(1000.0) {
            aa = TNumber::Float(0.0)
        }
        // !TNumber::Int(1).shl(10.into());
        // TNumber::Float(3.1) > TNumber::Float(1.0);
        // TNumber::Float(3.1) == TNumber::Float(1.0);
        // TNumber::Int(1) == TNumber::Float(1.0);
        // TNumber::Int(1) > -TNumber::Float(-f64::INFINITY);
    }
    println!("{:?} {}", instant.elapsed(), aa);
}
