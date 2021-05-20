use matching_network_rs::*;
use dimensioned as dim;
use dim::si::{
    OHM, // ohm
    SIE, // siemens
    F,   // farad
    H,   // henry
    HZ,  // Hertz
};

fn main() {
    let mut x1;
    
    x1 = ReactiveComponent::new(3123. * OHM);
    println!("{}", x1);
    let x1_resolved = x1.at(100_0000. * HZ);
    println!("{}", x1_resolved);


    // x1 = ReactiveComponent::new(4311.3, Some(13.23e6));
    // println!("{}", x1);    

    // x1 = ReactiveComponent::new(0.0, None);
    // println!("{}", x1);    

    // x1 = ReactiveComponent::new(-430.1, None);
    // println!("{}", x1);    
    


    // let x11 = ReactiveComponent::new(
    //     32123.0 * dim::si::OHM,
    //     100_000_000.0 * dim::si::Hertz,
    // );

    // println!("{:?}", x11);
    
    
    

    // match x1.get_freq() {
    //     Some(f) => println!("{:?}", f),
    //     None => println!("frequency is not set."),
    // }

    // match x2.get_freq() {
    //     Some(f) => println!("{:?}", f),
    //     None => println!("frequency is not set."),
    // }
}
