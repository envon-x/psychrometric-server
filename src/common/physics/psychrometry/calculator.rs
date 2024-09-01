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

use super::{
  humidity_ratio::HumidityRatio, 
  humidity_ratio_saturated::SaturatedHumidityRatio, 
  relative_humidity::RelativeHumidity, 
  specific_humidity::SpecificHumidity
};


pub trait PsychrometryCalculator {

  fn calc_TWetBulb_from_TDewPoint(
    &self,
    temperature_dew_point: &TemperatureDewPoint,
    absolute_pressure: &AbsolutePressure
  ) -> TemperatureWetBulb;

  fn calc_TWetBulb_from_RelativeHumidity(
    &self,
    relative_humidity: &RelativeHumidity,
    absolute_pressure: &AbsolutePressure
  ) -> TemperatureWetBulb;

  fn calc_RelativeHumidity_from_TDewPoint(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    temperature_dew_point: &TemperatureDewPoint
  ) -> RelativeHumidity;

  fn calc_RelativeHumidity_from_TWetBulb(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    temperature_wet_bulb: &TemperatureWetBulb, 
    absolute_pressure:  &AbsolutePressure
  ) -> RelativeHumidity;
  
  fn calc_TDewPoint_from_RelativeHumidity(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    relative_humidity: &RelativeHumidity 
  ) -> TemperatureDewPoint;
  
  fn calc_TDewPoint_from_TWetBulb(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb, 
    temperature_wet_bulb: &TemperatureWetBulb, 
    absolute_pressure: &AbsolutePressure
  ) -> TemperatureDewPoint;
  
  fn calc_VaporPressure_from_RelativeHumidity(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    relative_humidity: &RelativeHumidity 
  ) -> VaporPressure;
  
  fn calc_RelativeHumidity_from_VaporPressure(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    vapor_pressure: &VaporPressure
  ) -> RelativeHumidity;
  
  fn calc_TDewPoint_from_VaporPressure(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    vapor_pressure: &VaporPressure
  ) -> TemperatureDewPoint;
  
  fn calc_VaporPressure_from_TDewPoint(
    &self,
    temperature_dew_point: &TemperatureDewPoint
  ) -> VaporPressure;
  
  fn calc_TWetBulb_from_HumidityRatio(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    humidity_ratio: &HumidityRatio,
    absolute_pressure: &AbsolutePressure
  ) -> TemperatureWetBulb;
  
  fn calc_HumidityRatio_from_TWetBulb(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    temperature_wet_bulb: &TemperatureWetBulb,
    absolute_pressure: &AbsolutePressure
  ) -> HumidityRatio;
  
  fn calc_HumidityRatio_from_RelativeHumidity(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    relative_humidity: &RelativeHumidity,
    absolute_pressure: &AbsolutePressure
  ) -> HumidityRatio;
  
  fn calc_RelativeHumidity_from_HumidityRatio(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    humidity_ratio: &HumidityRatio,
    absolute_pressure: &AbsolutePressure
  ) -> RelativeHumidity;
  
  fn calc_HumidityRatio_from_TDewPoint(
    &self,
    temperature_dew_point: &TemperatureDewPoint,
    absolute_pressure: &AbsolutePressure
  ) -> HumidityRatio;
  
  fn calc_TDewPoint_from_HumidityRatio(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    humidity_ratio: &HumidityRatio,
    absolute_pressure: &AbsolutePressure
  ) -> TemperatureDewPoint;
  
  fn calc_HumidityRatio_from_VaporPressure(
    &self,
    vapor_pressure: &VaporPressure,
    absolute_pressure: &AbsolutePressure
  ) -> HumidityRatio;
  
  fn calc_VaporPressure_from_HumidityRatio(
    &self,
    humidity_ratio: &HumidityRatio,
    absolute_pressure: &AbsolutePressure
  ) -> VaporPressure;
  
  fn calc_SpecificHumidity_from_HumidityRatio(
    &self,
    humidity_ratio: &HumidityRatio
  ) -> SpecificHumidity;
  
  fn calc_HumidityRatio_from_SpecificHumidity(
    &self,
    specific_humidity: &SpecificHumidity
  ) -> HumidityRatio;
  
  fn calc_DryGasEnthalpy(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb 
  ) -> DryGasEnthalpy;
  
  fn calc_DryGasDensity(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    absolute_pressure: &AbsolutePressure
  ) -> DryGasDensity;
  
  fn calc_DryGasVolume(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    absolute_pressure: &AbsolutePressure
  ) -> DryGasVolume;
  
  fn calc_TDryBulb_from_Enthalpy_and_HumidityRatio(
    &self,
    moisture_gas_enthalpy: &MoistureGasEnthalpy,
    humidity_ratio: &HumidityRatio 
  ) -> TemperatureDryBulb;
  
  fn calc_HumidityRatio_from_Enthalpy_and_TDryBulb(
    &self,
    moisture_gas_enthalpy: &MoistureGasEnthalpy,
    temperature_dry_bulb: &TemperatureDryBulb 
  ) -> HumidityRatio;
  
  fn calc_SaturatedVaporPressure(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb
  ) -> SaturatedVaporPressure;
  
  fn calc_SaturatedHumidityRatio(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    absolute_pressure: &AbsolutePressure
  ) -> SaturatedHumidityRatio;
  
  fn calc_SaturatedGasEnthalpy(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    absolute_pressure: &AbsolutePressure
  ) -> SaturatedGasEnthalpy;
  
  fn calc_VaporPressureDeficit(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    humidity_ratio: &HumidityRatio,
    absolute_pressure: &AbsolutePressure
  ) -> VaporPressureDeficit;
  
  fn calc_MoistureGasEnthalpy(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    humidity_ratio: &HumidityRatio 
  ) -> MoistureGasEnthalpy;
  
  fn calc_MoistureGasVolume(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    humidity_ratio: &HumidityRatio,
    absolute_pressure: &AbsolutePressure
  ) -> MoistureGasVolume;
  
  fn calc_TDryBulb_from_MoistureGasVolume_and_HumidityRatio(
    &self,
    moisture_gas_volume: &MoistureGasVolume,
    humidity_ratio: &HumidityRatio,
    absolute_pressure: &AbsolutePressure
  ) -> TemperatureDryBulb;
  
  fn calc_MoistureGasDensity(
    &self,
    temperature_dry_bulb: &TemperatureDryBulb,
    humidity_ratio: &HumidityRatio,
    absolute_pressure: &AbsolutePressure
  ) -> MoistureGasDensity;
  

  fn get_SaturatedVapor_Pressure(
    &self, 
    TDryBulb: TemperatureDryBulb
  ) -> SaturatedVaporPressure;
  

  fn get_MoistureGasEnthalpy(
    &self, 
    TDryBulb: TemperatureDryBulb
  ) -> SaturatedVaporPressure;
  

  fn get_MoistureGasVolume(
    &self, 
    TDryBulb: TemperatureDryBulb
  ) -> SaturatedVaporPressure;


}