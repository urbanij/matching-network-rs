#[cfg(test)]
mod tests {
    use crate::reactive_component::ReactiveComponent;
    use assert_approx_eq::assert_approx_eq;
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
        assert_approx_eq!(x1.get_susceptance(), -71.429, 1e-3f32);
    }

}
