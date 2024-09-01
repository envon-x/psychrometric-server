//! Mass per energy (base unit kilogram per kilogram, m⁻² · s²).
//!
//! This quantity is typically used to express an emission intensity, also known as carbon
//! intensity. Emission intensity is a measure of how much mass of carbon dioxide (CO2) is emitted
//! per unit of energy.

quantity! {
  /// Mass per energy (base unit kilogram per kilogram, m⁻² · s²).
  quantity: MassPerMass; "mass per mass";
  /// Dimension of mass per energy, L⁻²T² (base unit kilogram per kilogram, m⁻² · s²).
  dimension: ISQ<
      N2,     // length
      Z0,     // mass
      P2,     // time
      Z0,     // electric current
      Z0,     // thermodynamic temperature
      Z0,     // amount of substance
      Z0>;    // luminous intensity
  units {
      @teragram_per_kilogram: prefix!(tera) / prefix!(kilo); "Tg/kg", "teragram per kilogram", "teragrams per kilogram";
      @gigagram_per_kilogram: prefix!(giga) / prefix!(kilo); "Gg/kg", "gigagram per kilogram", "gigagrams per kilogram";
      @megagram_per_kilogram: prefix!(mega) / prefix!(kilo); "Mg/kg", "megagram per kilogram", "megagrams per kilogram";
      /// Derived unit of Mass per energy.
      @kilogram_per_kilogram: prefix!(kilo) / prefix!(kilo); "kg/kg", "kilogram per kilogram", "kilograms per kilogram";
      @hectogram_per_kilogram: prefix!(hecto) / prefix!(kilo); "hg/kg", "hectogram per kilogram", "hectograms per kilogram";
      @decagram_per_kilogram: prefix!(deca) / prefix!(kilo); "dag/kg", "decagram per kilogram", "decagrams per kilogram";
      @gram_per_kilogram: prefix!(none) / prefix!(kilo); "g/kg", "gram per kilogram", "grams per kilogram";
      @decigram_per_kilogram: prefix!(deci) / prefix!(kilo); "dg/kg", "decigram per kilogram", "decigrams per kilogram";
      @centigram_per_kilogram: prefix!(centi) / prefix!(kilo); "cg/kg", "centigram per kilogram", "centigrams per kilogram";
      @milligram_per_kilogram: prefix!(milli) / prefix!(kilo); "mg/kg", "milligram per kilogram", "milligrams per kilogram";
      @microgram_per_kilogram: prefix!(micro) / prefix!(kilo); "µg/kg", "microgram per kilogram", "micrograms per kilogram";

      @pound_per_kilogram: 4.535_924_E-1; "lb/kg", "pound per kilogram", "pounds per kilogram";
      @pound_per_pound: 4.535_924_E-1 / 3.6_E12; "lb/lb", "pound per pound", "pounds per hour";
  }
}

// #[cfg(test)]
// mod tests {
//   storage_types! {
//       use crate::num::One;
//       use crate::si::mass_per_mass as v;
//       use crate::si::energy as e;
//       use crate::si::mass as m;
//       use crate::si::quantities::*;
//       use crate::tests::Test;

//       #[test]
//       fn check_dimension() {
//           let _: MassPerMass<V> = Mass:massnew::<m::kilogram>(V::one())
//               / Mass::new::<m::kilogram>(V::one());
//       }

//       #[test]
//       fn check_units() {
//           test::<m::teragram, e::kilogram, v::teragram_per_kilogram>();
//           test::<m::gigagram, e::kilogram, v::gigagram_per_kilogram>();
//           test::<m::megagram, e::kilogram, v::megagram_per_kilogram>();
//           test::<m::kilogram, e::kilogram, v::kilogram_per_kilogram>();
//           test::<m::hectogram, e::kilogram, v::hectogram_per_kilogram>();
//           test::<m::decagram, e::kilogram, v::decagram_per_kilogram>();
//           test::<m::gram, e::kilogram, v::gram_per_kilogram>();
//           test::<m::decigram, e::kilogram, v::decigram_per_kilogram>();
//           test::<m::centigram, e::kilogram, v::centigram_per_kilogram>();
//           test::<m::milligram, e::kilogram, v::milligram_per_kilogram>();
//           test::<m::microgram, e::kilogram, v::microgram_per_kilogram>();

//           test::<m::pound, e::kilogram, v::pound_per_kilogram>();
//           test::<m::pound, e::gigawatt_hour, v::pound_per_gigawatt_hour>();
//           test::<m::pound, e::megawatt_hour, v::pound_per_megawatt_hour>();
//           test::<m::pound, e::kilowatt_hour, v::pound_per_kilowatt_hour>();
//           test::<m::pound, e::watt_hour, v::pound_per_watt_hour>();

//           test::<m::kilogram, e::gigawatt_hour, v::kilogram_per_gigawatt_hour>();
//           test::<m::kilogram, e::megawatt_hour, v::kilogram_per_megawatt_hour>();
//           test::<m::kilogram, e::kilowatt_hour, v::kilogram_per_kilowatt_hour>();
//           test::<m::kilogram, e::watt_hour, v::kilogram_per_watt_hour>();

//           test::<m::gram, e::gigawatt_hour, v::gram_per_gigawatt_hour>();
//           test::<m::gram, e::megawatt_hour, v::gram_per_megawatt_hour>();
//           test::<m::gram, e::kilowatt_hour, v::gram_per_kilowatt_hour>();
//           test::<m::gram, e::watt_hour, v::gram_per_watt_hour>();

//           fn test<M: m::Conversion<V>, E: e::Conversion<V>, A: v::Conversion<V>>() {
//               Test::assert_approx_eq(&MassPerMass::new::<AmassV::one()),
//                   &(Mass::new::<M>(V::one()) / Energy::new::<E>(V::one())));
//           }
//       }
//   }
// }
