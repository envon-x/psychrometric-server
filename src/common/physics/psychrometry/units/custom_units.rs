use uom::quantity;

quantity! {
    /// Mass (base unit kilogram, kg).
    quantity: MassRatio; "mass_ratio";
    /// Mass dimension, M (base unit kilogram, kg).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        // ... existing units ...
        @gram_per_kilogram: 1.0_E-3; "g/kg", "gram per kilogram", "grams per kilogram";
    }
}
