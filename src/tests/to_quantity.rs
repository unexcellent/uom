use crate::si::f64::Length;
use crate::si::length::IntoLength;
use crate::si::length::meter;

mod tests {
    use super::*;
    #[test]
    fn i32_to_length_in_meters() {
        assert_eq!(32.m(), Length::new::<meter>(32.0))
    }
}
