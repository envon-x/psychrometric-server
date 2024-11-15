use uom::si::thermodynamic_temperature::kelvin;

use crate::common::physics::magnitude::Magnitude;
use crate::common::physics::mixture::Mixture;
use crate::common::physics::psychrometry::calculator::PsychrometryCalculator;
use crate::common::physics::thermo::temperature::TemperatureDewPoint;
use crate::common::physics::thermo::pressure::{self, AbsolutePressure};
use crate::common::physics::thermo::temperature::TemperatureWetBulb;
use crate::common::physics::psychrometry::relative_humidity::RelativeHumidity;
use crate::common::physics::thermo::pressure::VaporPressure;
use crate::common::physics::thermo::molar_enthalpy::DryGasEnthalpy;
use crate::common::physics::thermo::density::DryGasDensity;
use crate::common::physics::thermo::molar_volume::DryGasVolume;
use crate::common::physics::thermo::temperature::TemperatureDryBulb;
use crate::common::physics::psychrometry::humidity_ratio::HumidityRatio;
use crate::common::physics::psychrometry::specific_humidity::SpecificHumidity;
use crate::common::physics::thermo::molar_enthalpy::MoistureGasEnthalpy;
use crate::common::physics::thermo::pressure::SaturatedVaporPressure;
use crate::common::physics::psychrometry::humidity_ratio_saturated::SaturatedHumidityRatio;
use crate::common::physics::thermo::molar_enthalpy::SaturatedGasEnthalpy;
use crate::common::physics::thermo::pressure::VaporPressureDeficit;
use crate::common::physics::thermo::molar_volume::MoistureGasVolume;
use crate::common::physics::thermo::density::MoistureGasDensity;


impl PsychrometryCalculator for Mixture {
    
    fn calc_TWetBulb_from_TDewPoint(
        &self,
        temperature_dew_point: &TemperatureDewPoint,
        absolute_pressure: &AbsolutePressure
      ) -> TemperatureWetBulb {
        let _ = absolute_pressure;
        let _ = temperature_dew_point;

        let calc_HumidityRatio_from_TDewPoint = self.calc_HumidityRatio_from_TDewPoint(temperature_dew_point, absolute_pressure);
        let wbt = TemperatureWetBulb {
          magnitude: Magnitude {
            name: "Temperatura de rocío".to_string(),
            symbol: String::from("T_db"),
            value: 30.0,
            unit: kelvin,
          },
        };
        wbt
    }
    
    fn calc_TWetBulb_from_RelativeHumidity(
        &self,
        relative_humidity: &RelativeHumidity,
        absolute_pressure: &AbsolutePressure
      ) -> TemperatureWetBulb {
        todo!()
    }
    
    fn calc_RelativeHumidity_from_TDewPoint(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        temperature_dew_point: &TemperatureDewPoint
      ) -> RelativeHumidity {
        todo!()
    }
    
    fn calc_RelativeHumidity_from_TWetBulb(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        temperature_wet_bulb: &TemperatureWetBulb, 
        absolute_pressure:  &AbsolutePressure
      ) -> RelativeHumidity {
        todo!()
    }
    
    fn calc_TDewPoint_from_RelativeHumidity(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        relative_humidity: &RelativeHumidity 
      ) -> TemperatureDewPoint {
        todo!()
    }
    
    fn calc_TDewPoint_from_TWetBulb(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb, 
        temperature_wet_bulb: &TemperatureWetBulb, 
        absolute_pressure: &AbsolutePressure
      ) -> TemperatureDewPoint {
        todo!()
    }
    
    fn calc_VaporPressure_from_RelativeHumidity(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        relative_humidity: &RelativeHumidity 
      ) -> VaporPressure {
        todo!()
    }
    
    fn calc_RelativeHumidity_from_VaporPressure(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        vapor_pressure: &VaporPressure
      ) -> RelativeHumidity {
        todo!()
    }
    
    fn calc_TDewPoint_from_VaporPressure(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        vapor_pressure: &VaporPressure
      ) -> TemperatureDewPoint {
        todo!()
    }
    
    fn calc_VaporPressure_from_TDewPoint(
        &self,
        temperature_dew_point: &TemperatureDewPoint
      ) -> VaporPressure {
        todo!()
    }
    
    fn calc_TWetBulb_from_HumidityRatio(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        humidity_ratio: &HumidityRatio,
        absolute_pressure: &AbsolutePressure
      ) -> TemperatureWetBulb {
        todo!()
    }
    
    fn calc_HumidityRatio_from_TWetBulb(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        temperature_wet_bulb: &TemperatureWetBulb,
        absolute_pressure: &AbsolutePressure
      ) -> HumidityRatio {
        todo!()
    }
    
    fn calc_HumidityRatio_from_RelativeHumidity(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        relative_humidity: &RelativeHumidity,
        absolute_pressure: &AbsolutePressure
      ) -> HumidityRatio {
        todo!()
    }
    
    fn calc_RelativeHumidity_from_HumidityRatio(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        humidity_ratio: &HumidityRatio,
        absolute_pressure: &AbsolutePressure
      ) -> RelativeHumidity {
        todo!()
    }
    
    fn calc_HumidityRatio_from_TDewPoint(
        &self,
        temperature_dew_point: &TemperatureDewPoint,
        absolute_pressure: &AbsolutePressure
      ) -> HumidityRatio {
        todo!()
    }
    
    fn calc_TDewPoint_from_HumidityRatio(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        humidity_ratio: &HumidityRatio,
        absolute_pressure: &AbsolutePressure
      ) -> TemperatureDewPoint {
        todo!()
    }
    
    fn calc_HumidityRatio_from_VaporPressure(
        &self,
        vapor_pressure: &VaporPressure,
        absolute_pressure: &AbsolutePressure
      ) -> HumidityRatio {
        todo!()
    }
    
    fn calc_VaporPressure_from_HumidityRatio(
        &self,
        humidity_ratio: &HumidityRatio,
        absolute_pressure: &AbsolutePressure
      ) -> VaporPressure {
        todo!()
    }
    
    fn calc_SpecificHumidity_from_HumidityRatio(
        &self,
        humidity_ratio: &HumidityRatio
      ) -> SpecificHumidity {
        todo!()
    }
    
    fn calc_HumidityRatio_from_SpecificHumidity(
        &self,
        specific_humidity: &SpecificHumidity
      ) -> HumidityRatio {
        todo!()
    }
    
    fn calc_DryGasEnthalpy(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb 
      ) -> DryGasEnthalpy {
        todo!()
    }
    
    fn calc_DryGasDensity(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        absolute_pressure: &AbsolutePressure
      ) -> DryGasDensity {
        todo!()
    }
    
    fn calc_DryGasVolume(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        absolute_pressure: &AbsolutePressure
      ) -> DryGasVolume {
        todo!()
    }
    
    fn calc_TDryBulb_from_Enthalpy_and_HumidityRatio(
        &self,
        moisture_gas_enthalpy: &MoistureGasEnthalpy,
        humidity_ratio: &HumidityRatio 
      ) -> TemperatureDryBulb {
        todo!()
    }
    
    fn calc_HumidityRatio_from_Enthalpy_and_TDryBulb(
        &self,
        moisture_gas_enthalpy: &MoistureGasEnthalpy,
        temperature_dry_bulb: &TemperatureDryBulb 
      ) -> HumidityRatio {
        todo!()
    }
    
    fn calc_SaturatedVaporPressure(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb
      ) -> SaturatedVaporPressure {
        todo!()
    }
    
    fn calc_SaturatedHumidityRatio(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        absolute_pressure: &AbsolutePressure
      ) -> SaturatedHumidityRatio {
        todo!()
    }
    
    fn calc_SaturatedGasEnthalpy(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        absolute_pressure: &AbsolutePressure
      ) -> SaturatedGasEnthalpy {
        todo!()
    }
    
    fn calc_VaporPressureDeficit(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        humidity_ratio: &HumidityRatio,
        absolute_pressure: &AbsolutePressure
      ) -> VaporPressureDeficit {
        todo!()
    }
    
    fn calc_MoistureGasEnthalpy(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        humidity_ratio: &HumidityRatio 
      ) -> MoistureGasEnthalpy {
        todo!()
    }
    
    fn calc_MoistureGasVolume(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        humidity_ratio: &HumidityRatio,
        absolute_pressure: &AbsolutePressure
      ) -> MoistureGasVolume {
        todo!()
    }
    
    fn calc_TDryBulb_from_MoistureGasVolume_and_HumidityRatio(
        &self,
        moisture_gas_volume: &MoistureGasVolume,
        humidity_ratio: &HumidityRatio,
        absolute_pressure: &AbsolutePressure
      ) -> TemperatureDryBulb {
        todo!()
    }
    
    fn calc_MoistureGasDensity(
        &self,
        temperature_dry_bulb: &TemperatureDryBulb,
        humidity_ratio: &HumidityRatio,
        absolute_pressure: &AbsolutePressure
      ) -> MoistureGasDensity {
        todo!()
    }
    
}