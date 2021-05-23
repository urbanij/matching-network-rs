#[cfg(test)]
mod tests {
    use dimensioned as dim;
    use assert_approx_eq::*;
    use crate::reactive_component::*;

    const EPSILON: f64 = 1e-3;
    
    #[test]
    fn test_component1() {
        let comp1 = Component::L(Some(32.1 * dim::si::H));
        assert_eq!(format!("{}", comp1), format!("L = 32.1 H"));
    }

    #[test]
    fn test_reactive_component_1() {
        let comp1 = ReactiveComponent::new(
            123.4 * dim::si::OHM, 
            Some(32.3e7 * dim::si::HZ)
        );

        assert_eq!(comp1.get_reactance(), 123.4 * dim::si::OHM);
        assert_approx_eq!(
            comp1.get_susceptance() / dim::si::SIE, 
            -0.008103727714748784,
            EPSILON);
    }

}

