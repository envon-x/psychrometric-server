use crate::common::physics::thermo::{
  density::{
    DryGasDensity, 
    MoistureGasDensity
  }, 
  molar_enthalpy::{
    DryGasEnthalpy, 
    MoistureGasEnthalpy, 
    SaturatedGasEnthalpy
  }, 
  molar_volume::{
    DryGasVolume, 
    MoistureGasVolume
  }, 
  pressure::{
    AbsolutePressure, 
    SaturatedVaporPressure, 
    VaporPressure, 
    VaporPressureDeficit
  }, 
  temperature::{
    TemperatureDewPoint, 
    TemperatureDryBulb, 
    TemperatureWetBulb
  }
};

use super::{humidity_ratio::{HumidityRatio, SaturatedHumidityRatio}, relative_humidity::RelativeHumidity, specific_humidity::SpecificHumidity};


pub trait PsychrometryCalculator {

  fn calc_TWetBulb_from_TDewPoint(&self, tdp: &TemperatureDewPoint, p: &AbsolutePressure) -> TemperatureWetBulb;

  fn calc_TWetBulb_from_RelativeHumidity(&self, hr: &RelativeHumidity, p: &AbsolutePressure) -> TemperatureWetBulb;

  fn calc_RelativeHumidity_from_TDewPoint(&self, tdb: &TemperatureDryBulb, tdp: &TemperatureDewPoint) -> RelativeHumidity;

  fn calc_RelativeHumidityfrom_TWetBulb(&self, tdb: &TemperatureDryBulb, twb: &TemperatureWetBulb, p:  &AbsolutePressure) -> RelativeHumidity;
  
  fn calc_TDewPoint_from_RelativeHumidity(&self, tdb: &TemperatureDryBulb, hr: &RelativeHumidity ) -> TemperatureDewPoint;
  
  fn calc_TDewPoint_from_TWetBulb(tdb: &TemperatureDryBulb, twb: &TemperatureWetBulb, p: &AbsolutePressure) -> TemperatureDewPoint;
  
  fn calc_VaporPressure_from_RelativeHumidity(&self, tdb: &TemperatureDryBulb, hr: &RelativeHumidity ) -> VaporPressure;
  
  fn calc_RelativeHumidityfrom_VaporPressure(&self, tdb: &TemperatureDryBulb, pv: &VaporPressure) -> RelativeHumidity;
  
  fn calc_TDewPoint_from_VaporPressure(&self, tdb: &TemperatureDryBulb, pv: &VaporPressure) -> TemperatureDewPoint;
  
  fn calc_VaporPressure_from_TDewPoint(&self, tdp: &TemperatureDewPoint) -> VaporPressure;
  
  fn calc_TWetBulb_from_HumidityRatio(&self, tdb: &TemperatureDryBulb, Y: &HumidityRatio, p: &AbsolutePressure) -> TemperatureWetBulb;
  
  fn calc_HumidityRatio_from_TWetBulb(&self, tdb: &TemperatureDryBulb, twb: &TemperatureWetBulb, p: &AbsolutePressure) -> HumidityRatio;
  
  fn calc_HumidityRatio_from_RelativeHumidity(&self, tdb: &TemperatureDryBulb, hr: &RelativeHumidity, p: &AbsolutePressure) -> HumidityRatio;
  
  fn calc_RelativeHumidity_from_HumidityRatio(&self, tdb: &TemperatureDryBulb, Y: &HumidityRatio, p: &AbsolutePressure) -> RelativeHumidity;
  
  fn calc_HumidityRatio_from_TDewPoint(&self, tdp: &TemperatureDewPoint, p: &AbsolutePressure) -> HumidityRatio;
  
  fn calc_TDewPoint_from_HumidityRatio(&self, tdb: &TemperatureDryBulb, Y: &HumidityRatio, p: &AbsolutePressure) -> TemperatureDewPoint;
  
  fn calc_HumidityRatio_from_VaporPressure(&self, p: &VaporPressure, p: &AbsolutePressure) -> HumidityRatio;
  
  fn calc_VaporPressure_from_HumidityRatio(&self, Y: &HumidityRatio, p: &AbsolutePressure) -> VaporPressure;
  
  fn calc_SpecificHumidity_from_HumidityRatio(&self, Y: &HumidityRatio) -> SpecificHumidity;
  
  fn calc_HumidityRatio_from_SpecificHumidity(&self, y: &SpecificHumidity) -> HumidityRatio;
  
  fn calc_DryGasEnthalpy(&self, tdb: &TemperatureDryBulb ) -> DryGasEnthalpy;
  
  fn calc_DryGasDensity(&self, tdb: &TemperatureDryBulb, p: &AbsolutePressure) -> DryGasDensity;
  
  fn calc_DryGasVolume(&self, tdb: &TemperatureDryBulb, p: &AbsolutePressure) -> DryGasVolume;
  
  fn calc_TDryBulb_from_Enthalpy_And_HumidityRatio(&self, hm: &MoistureGasEnthalpy, Y: &HumidityRatio ) -> TemperatureDryBulb;
  
  fn calc_HumidityRatio_from_Enthalpy_And_TDryBulb(&self, hm: &MoistureGasEnthalpy, tdb: &TemperatureDryBulb ) -> HumidityRatio;
  
  fn calc_SaturatedVaporPressure(&self, tdb: &TemperatureDryBulb) -> SaturatedVaporPressure;
  
  fn calc_SaturatedHumidityRatio(&self, tdb: &TemperatureDryBulb, p: &AbsolutePressure) -> SaturatedHumidityRatio;
  
  fn calc_SaturatedGasEnthalpy(&self, tdb: &TemperatureDryBulb, p: &AbsolutePressure) -> SaturatedGasEnthalpy;
  
  fn calc_VaporPressureDeficit(&self, tdb: &TemperatureDryBulb, Y: &HumidityRatio, p: &AbsolutePressure) -> VaporPressureDeficit;
  
  fn calc_MoistureGasEnthalpy(&self, tdb: &TemperatureDryBulb, Y: &HumidityRatio ) -> MoistureGasEnthalpy;
  
  fn calc_MoistureGasVolume(&self, tdb: &TemperatureDryBulb, Y: &HumidityRatio, p: &AbsolutePressure) -> MoistureGasVolume;
  
  fn calc_TDryBulb_from_MoistureGasVolume_And_HumidityRatio(&self, Va: &MoistureGasVolume, Y: &HumidityRatio, p: &AbsolutePressure) -> TemperatureDryBulb;
  
  fn calc_MoistureGasDensity(&self, tdb: &TemperatureDryBulb, Y: &HumidityRatio, p: &AbsolutePressure) -> MoistureGasDensity;
  
}