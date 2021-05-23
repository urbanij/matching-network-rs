use matching_network_rs::*;
use dimensioned as dim;


fn main() {
    let mut x1;
    
    // x1 = ReactiveComponent::new(3123. * OHM, None);
    // println!("{:?}", x1);

    x1 = ReactiveComponent::new(4311.3 * dim::si::OHM, Some(13.23e6 * dim::si::HZ));
    println!("{:?}", x1);
    println!("{}", x1);


    x1 = ReactiveComponent::new(0.0 * dim::si::OHM, None);
    println!("{}", x1);  

    x1 = ReactiveComponent::new(-430.1 * dim::si::OHM, None);
    println!("{}", x1);    
    
    x1 = ReactiveComponent::new(-430.1 * dim::si::OHM, Some(120_000.0 * dim::si::HZ));
    println!("{}", x1);    
    

}
