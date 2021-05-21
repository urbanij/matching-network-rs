use dimensioned as dim;
#[warn(unused_imports)]
use dim::si::{
    OHM, // ohm
    SIE, // siemens
    F,   // farad
    H,   // henry
    HZ,  // Hertz
};

use crate::reactive_component::*;

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn ind1() {
        let x1 = ReactiveComponent::new(19.467, Some(100_000.0));
        assert_eq!(x1.get_value().unwrap(), 0.00003098269);
    }
    #[test]
    fn ind2() {
        let x1 = ReactiveComponent::new(14., Some(13_000_000.0));
        // assert_eq!(x1.value.unwrap(), 171.4*1e-9);
        assert_approx_eq!(x1.get_value().unwrap(), 171.4*1e-9, 1e-3f32);
        assert_approx_eq!(x1.get_susceptance(), -71.429e-3, 1e-3f32);
    }
    */

    #[test]
    fn test_component1() {
        let comp1 = Component::L(Some(32.1 * H));
        assert_eq!(format!("{}", comp1), format!("L = 32.1 m^2*kg*s^-2*A^-2"));
    }

    #[test]
    fn test_ReactiveComponent_1() {
        let comp1 = ReactiveComponent::new(123.4*OHM, Some(32.3e7 * HZ));
        assert_eq!(comp1.get_reactance(), 123.4*OHM);
        assert_eq!(comp1.get_susceptance(), -0.008103727714748784 * SIE);
    }

}
