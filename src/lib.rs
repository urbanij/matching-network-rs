// $ pip install matching_network
// $ matching_network --from 100 --to 43+12j --freq 32e6


#![doc(html_logo_url = 
    "https://raw.githubusercontent.com/urbanij/matching-network-rs/main/favicon.png"
)]

mod reactive_component;
mod tests;
mod matching_network;

pub use crate::matching_network::*;
