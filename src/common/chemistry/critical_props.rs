/**
* Propiedades físico-químicas  críticas de una sustancia
 */
pub struct CriticalProps {
    reference: String,
    bib_reference: String,
    temperature: f64,
    pressure: f64,
    compressibility: f64
}