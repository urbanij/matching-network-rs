use dim::tarr;
// use quantiphy_rs::Quantity;
use dimensioned as dim;
use std::f64::consts::PI;


fn main() {
    let r1 = 3.2 * dim::si::OHM;
    let f = 32.1 * dim::si::HZ;

    let ans = r1 / (2.0 * PI * f);
    println!("{}", ans);

    println!("{}", 2.1*dim::si::H);




    let x: dim::si::Meter<f64> = 3.0 * dim::si::M;
    let t = 2.0 * dim::si::S;

    let ans = x*t;
    println!("{}", ans);


    let imp1 = 32123.0 * dim::si::OHM;
    let freq1 = 32123.0 * dim::si::HZ;

    println!("{}", imp1);
    println!("{}", freq1);

}