#[macro_use]
mod mass_ratio {
    quantity! {
        /// Length (base unit meter, m).
        quantity: MassRatio; "mass_ratio";
        /// Length dimension, m.
        dimension: Q<
            P1,  // ratio
            Z0,  // mass
            Z0>; // time
        units {
            @kg_per_kg: 1.0E0; "kg_per_kg", "kg_per_kg", "kilogram_per_kilograms";
            @g_per_kg: 1.08E3; "g_per_kg", "g_per_kg", "gram_per_kilograms";
        }
    }
}

#[macro_use]
mod mass {
    quantity! {
        /// Mass (base unit kilogram, kg).
        quantity: Mass; "mass";
        /// Mass dimension, kg.
        dimension: Q<
            Z0,  // length
            P1,  // mass
            Z0>; // time
        units {
            @kilogram: 1.0; "kg", "kilogram", "kilograms";
        }
    }
}

#[macro_use]
mod time {
    quantity! {
        /// Time (base unit second, s).
        quantity: Time; "time";
        /// Time dimension, s.
        dimension: Q<
            Z0,  // length
            Z0,  // mass
            P1>; // time
        units {
            @second: 1.0; "s", "second", "seconds";
        }
    }
}

system! {
    quantities: Q {
        mass_ratio: kg_per_kg, L;
        mass: kilogram, M;
        time: second, T;
    }

    units: U {
        mod mass_ratio::MassRatio,
        mod mass::Mass,
        mod time::Time,
    }
}

mod f32 {
    mod mks {
        pub use super::super::*;
    }

    Q!(self::mks, f32);
}
