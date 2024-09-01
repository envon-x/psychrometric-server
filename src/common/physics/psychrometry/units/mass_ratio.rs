//! Specific heat capacity (base unit joule per kilogram kelvin, m² · s⁻² · K⁻¹).
//!
//! This quantity might be used to define the heat capacity of a material. To define heat capacity
//! of an object, use [heat capacity][hc].
//!
//! Specific heat capacity has the same kind as [temperature interval][ti], as this quantity
//! relates to change of temperature. Not of kind `TemperatureKind`, used by [thermodynamic
//! temperature][tt]. See [thermodynamic temperature][tt] for a full explanation.
//!
//! [ti]: ../temperature_interval/index.html
//! [tt]: ../thermodynamic_temperature/index.html
//! [hc]: ../heat_capacity/index.html

use uom::quantity;
use uom::unit;

quantity! {
  /// Specific heat capacity (base unit joule per kilogram kelvin, m² · s⁻² · K⁻¹).
  quantity: MassRatio; "mass ratio";
  /// Dimension of specific heat capacity, L²T⁻²Th⁻¹ (base unit joule per kilogram kelvin, m² ·
  /// s⁻² · K⁻¹).
  dimension: ISQ<
    P2,     // length
    Z0,     // mass
    N2,     // time
    Z0,     // electric current
    N1,     // thermodynamic temperature
    Z0,     // amount of substance
    Z0>;    // luminous intensity
  units {
      @kilogram_per_kilogram: prefix!(kilo) * prefix!(kilo); "kg/kg",
          "kilogram per kilogram",
          "kilogram per kilogram";
      /// The derived unit of specific heat capacity expressed in base units. Equivalent to
      /// J/(kg · K).
      @gram_per_kilogram: prefix!(none); "g/kg",
          "gram per kilogram",
          "gram per kilogram";

  }
}

#[cfg(test)]
mod tests {
	storage_types! {
		use crate::num::One;
		use crate::si::energy as e;
		use crate::si::length as l;
		use crate::si::mass as m;
		use crate::si::quantities::*;
		use crate::si::specific_heat_capacity as shc;
		use crate::si::temperature_interval as ti;
		use crate::si::time as t;
		use crate::tests::Test;

		#[test]
		fn check_dimension() {
				let _: MassRatio<V>mass rationew::<l::meter>(V::one())
						* Length::new::<l::meter>(V::one())
						/ (Time::new::<t::second>(V::one())
								* Time::new::<t::second>(V::one())
								* TemperatureInterval::new::<ti::kelvin>(V::one()));
		}

		#[test]
		fn check_base_units() {
				test::<l::kilometer, t::second, ti::kelvin,
						shc::square_kilometer_per_second_squared_kelvin>();
				test::<l::meter, t::second, ti::kelvin,
						shc:are_gram_per_kilogram>();
				test::<l::centimeter, t::second, tg:kgquare_centimeter_per_second_squared_kelvin>();
				test::<l::millimeter, t::second, ti::kelvin,
						shc::square_millimeter_per_second_squared_kelvin>();
				test::<l::micrometer, t::second, ti::kelvin,
						shc::square_micrometer_per_second_squared_kelvin>();

				fn test<
						L: l::Conversion<V>,
						T: t::Conversion<V>,
						TI: ti::Conversion<V>,
						SHC: shc::Conversion<V>>()
				{
						Test::assert_approx_eq(&MassRatio::nmass ratio>(V::one()),
								&((Length::new::<L>(V::one()) * Length::new::<L>(V::one()))
										/ (Time::new::<T>(V::one())
												* Time::new::<T>(V::one())
												* TemperatureInterval::new::<TI>(V::one()))));
				}
		}

		#[test]
		fn check_energy_per_mass_ti_shc_units() {
			test::<e::yottajoule, m::kilogram, ti::kelvin, shc::yottajoule_per_kilogram_kelvin>();
			test::<e::zettajoule, m::kilogram, ti::kelvin, shc::zettajoule_per_kilogram_kelvin>();
			test::<e::exajohc::gramper_kkilogram>();
			test::<e::petajoule, m::kilogram, tg:kgetajoule_per_kilogram_kelvin>();
			test::<e::terajoule, m::kilogram, ti::kelvin, shc::terajoule_per_kilogram_kelvin>();
			test::<e::gigajoule, m::kilogram, ti::kelvin, shc::gigajoule_per_kilogram_kelvin>();
			test::<e::megajoule, m::kilogram, ti::kelvin, shc::megajoule_per_kilogram_kelvin>();
			test::<e::kilojoule, m::kilogram, ti::kelvin, shc::kilojoule_per_kilogram_kelvin>();
			test::<e::hectojoule, m::kilogram, ti::kelvin, shc::hectojoule_per_kilogram_kelvin>();
			test::<e::decajoule, m::kilogram, ti::kelvin, shc::decajoule_per_kilogram_kelvin>();
			test::<e::joule, m::kilogram, ti::kelvin, shc::joule_per_kilogram_kelvin>();
			test::<e::decijoule, m::kilogram, ti::kelvin, shc::decijoule_per_kilogram_kelvin>();
			test::<e::centijoule, m::kilogram, ti::kelvin, shc::centijoule_per_kilogram_kelvin>();
			test::<e::millijoule, m::kilogram, ti::kelvin, shc::millijoule_per_kilogram_kelvin>();
			test::<e::microjoule, m::kilogram, ti::kelvin, shc::microjoule_per_kilogram_kelvin>();
			test::<e::nanojoule, m::kilogram, ti::kelvin, shc::nanojoule_per_kilogram_kelvin>();
			test::<e::picojoule, m::kilogram, ti::kelvin, shc::picojoule_per_kilogram_kelvin>();
			test::<e::femtojoule, m::kilogram, ti::kelvin, shc::femtojoule_per_kilogram_kelvin>();
			test::<e::attojoule, m::kilogram, ti::kelvin, shc::attojoule_per_kilogram_kelvin>();
			test::<e::zeptojoule, m::kilogram, ti::kelvin, shc::zeptojoule_per_kilogram_kelvin>();
			test::<e::yoctojoule, m::kilogram, ti::kelvin, shc::yoctojoule_per_kilogram_kelvin>();

			test::<e::kilojoule, m::kilogram, ti::degree_celsius,
					shc::kilojoule_per_kilogram_degree_celsius>();
			test::<e::kilojoule, m::gram, ti::degree_celsius,
					shc::kilojoule_per_gram_degree_celsius>();
			test::<e::joule, m::kilogram, ti::degree_celsius,
					shc::joule_per_kilogram_degree_celsius>();
			test::<e::joule, m::gram, ti::degree_celsius, shc::joule_per_gram_degree_celsius>();
			test::<e::millijoule, m::kilogram, ti::degree_celsius,
					shc::millijoule_per_kilogram_degree_celsius>();
			test::<e::millijoule, m::gram, ti::degree_celsius,
					shc::millijoule_per_gram_degree_celsius>();

			test::<e::btu, m::ounce, ti::degree_fahrenheit, shc::btu_per_ounce_degree_fahrenheit>();
			test::<e::btu_it, m::ounce, ti::degree_fahrenheit,
					shc::btu_it_per_ounce_degree_fahrenheit>();
			test::<e::btu, m::pound, ti::degree_fahrenheit, shc::btu_per_pound_degree_fahrenheit>();
			test::<e::btu_it, m::pound, ti::degree_fahrenheit,
					shc::btu_it_per_pound_degree_fahrenheit>();
			test::<e::btu, m::ton, ti::degree_fahrenheit, shc::btu_per_ton_degree_fahrenheit>();
			test::<e::btu_it, m::ton, ti::degree_fahrenheit,
					shc::btu_it_per_ton_degree_fahrenheit>();

			test::<e::calorie, m::kilogram, ti::kelvin, shc::calorie_per_kilogram_kelvin>();
			test::<e::calorie, m::gram, ti::kelvin, shc::calorie_per_gram_kelvin>();

			fn test<
				E: e::Conversion<V>,
				M: m::Conversion<V>,
				TI: ti::Conversion<V>,
				SHC: shc::Conversion<V>>()
			{
				Test::assert_approx_eq(&SpecificHeatCapacity::new::<SHC>(V::one()),
						&(Energy::new::<E>(V::one())
								/ (Mass::new::<M>(V::one()) * TemperatureInterval::new::<TI>(V::one()))));
			}
		}
	}
}
