use matching_network_rs::*;

fn main() {
    let x1 = ReactiveComponent::new(19.467, Some(100_000.0));
    let x2 = ReactiveComponent::new(431.3, None);
    let x3 = ReactiveComponent::new(0.0, None);
    let x4 = ReactiveComponent::new(-430.1, None);
    
    println!("{}", x1);
    println!("{}", x2);
    println!("{}", x3);
    println!("{}", x4);


    match x1.get_freq() {
        Some(f) => println!("{:?}", f),
        None => println!("frequency is not set."),
    }

    match x2.get_freq() {
        Some(f) => println!("{:?}", f),
        None => println!("frequency is not set."),
    }
}
