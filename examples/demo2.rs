use dimensioned as dim;
use std::f64::consts::PI;


fn main() {
    let r1 = 3.2 * dim::si::OHM;
    let f = 32.1 * dim::si::HZ;

    let ans = r1 / (2.0 * PI * f);
    println!("{}", ans);

}