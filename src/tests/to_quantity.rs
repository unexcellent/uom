use crate::si::f64::{Length, Mass};
use crate::si::length::IntoLength;
use crate::si::length::{centimeter, meter, millimeter};
use crate::si::mass::{gram, kilogram};
use crate::si::mass::IntoMass;

mod tests {
    use super::*;

    #[test]
    fn i32_to_length_in_meters() {
        assert_eq!(32.m(), Length::new::<meter>(32.0))
    }

    #[test]
    fn i32_to_length_in_centimeters() {
        assert_eq!(32.cm(), Length::new::<centimeter>(32.0))
    }

    #[test]
    fn i32_to_length_in_millimeters() {
        assert_eq!(32.mm(), Length::new::<millimeter>(32.0))
    }

    #[test]
    fn i32_to_mass_kilograms() {
        assert_eq!(32.kg(), Mass::new::<kilogram>(32.0))
    }

    #[test]
    fn i32_to_mass_grams() {
        assert_eq!(32.g(), Mass::new::<gram>(32.0))
    }

}
