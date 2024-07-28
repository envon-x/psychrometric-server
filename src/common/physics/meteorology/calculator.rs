use crate::common::physics::{
  altitude::SeaLevelAltitude, 
  thermo::{
    pressure::{
      SeaLevelPressure, 
      StandardAtmPressure, 
      StationPressure
    }, 
    temperature::{
      StandardAtmTemperature, 
      TemperatureDryBulb
    }
  }
};

pub trait Calculator {
  
  fn calc_StandardAtmPressure(&self, z: SeaLevelAltitude) -> StandardAtmPressure;
  
  fn calc_StandardAtmTemperature(&self, z: SeaLevelAltitude) -> StandardAtmTemperature;
  
  fn calc_SeaLevelPressure(&self,  p_s: &StationPressure, z: &SeaLevelAltitude, tdb: &TemperatureDryBulb ) -> SeaLevelPressure;
  
  fn calc_StationPressure(&self, p_sea: &SeaLevelPressure, z: &SeaLevelAltitude, tdb: &TemperatureDryBulb ) -> StationPressure;
  
  // fn calc_DegreeOfSaturation(&self, tdb: &TemperatureDryBulb, Y: &HumidityRatio, p: &AbsolutePressure) -> DegreeOfSaturation;
}