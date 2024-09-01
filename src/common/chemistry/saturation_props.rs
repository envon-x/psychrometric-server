pub struct SaturationProps {
    reference: String,
    bib_reference: String,
    correlation: f64, // default value: 1
    antoine_coefficients: Vec<f64>,
    minimum_temperature: Magnitude,
    maximum_temperature: Magnitude,
}