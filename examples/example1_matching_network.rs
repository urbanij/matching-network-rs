use matching_network_rs::*;
use dimensioned as dim;
use num::complex::*;


fn main() {
    
    let z1 = Complex::new(
        100.0 * dim::si::OHM,
        0.0 * dim::si::OHM);

    let z2 = Complex::new(
            43.0 * dim::si::OHM,
            12.0 * dim::si::OHM);
    
    
    let mut mn1 = MatchingNetwork::new(
        z1,
        z2
    );
    let mn2 = mn1.clone();


    println!("{}", mn1);

    println!("solutions: {:?}", 
        mn1.solve()
    );

    

    
    mn2.solve()
       .eval_at(32e6 * dim::si::HZ);

    

    


    /*

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
    
    */

}
