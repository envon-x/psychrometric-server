pub fn calc_RelHumidity_From_TDewPoint(tdb: TemperatureDryBulb, tdp: TemperatureDewPoint) -> RelativeHumidity {
  // implementation of the calculation
  let rel_hum = (tdp.magnitude.value - tdb.magnitude.value) / (tdb.magnitude.value - 273.15);
  RelativeHumidity::new(rel_hum)
}