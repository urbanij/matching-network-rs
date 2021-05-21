#[cfg(test)]
mod tests {

    use crate::reactive_component::*;
    use assert_approx_eq::assert_approx_eq;
    use dimensioned as dim;

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
    fn test1() {
        let comp1 = Component::L(Some(32.1 * dim::si::H));
        assert_eq!(format!("{}", comp1), format!("L = 32.1 m^2*kg*s^-2*A^-2"));
    }

}
