use crate::common::physics::psychrometry::calculator::PsychrometryCalculator;
use crate::common::physics::thermo::temperature::TemperatureDewPoint;
use crate::common::physics::thermo::pressure::AbsolutePressure;
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

use super::virial_eos::VirialEos;

impl PsychrometryCalculator for VirialEos {
    fn calc_TWetBulb_from_TDewPoint(&self,
      tdp: &TemperatureDewPoint,
      p: &AbsolutePressure,
    ) -> TemperatureWetBulb {
        todo!()
    }

    fn calc_TWetBulb_from_RelativeHumidity(&self,
      hr: &RelativeHumidity,
      p: &AbsolutePressure,
    ) -> TemperatureWetBulb {
        todo!()
    }

    fn calc_RelativeHumidity_from_TDewPoint(&self,
      tdb: &TemperatureDryBulb,
      tdp: &TemperatureDewPoint,
    ) -> RelativeHumidity {
        todo!()
    }

    fn calc_RelativeHumidity_from_TWetBulb(&self,
      tdb: &TemperatureDryBulb,
      twb: &TemperatureWetBulb,
      p:  &AbsolutePressure,
    ) -> RelativeHumidity {
        todo!()
    }

    fn calc_TDewPoint_from_RelativeHumidity(&self,
      tdb: &TemperatureDryBulb,
      hr: &RelativeHumidity,
     ) -> TemperatureDewPoint {
        todo!()
    }

    fn calc_TDewPoint_from_TWetBulb(tdb: &TemperatureDryBulb,
      twb: &TemperatureWetBulb,
      p: &AbsolutePressure,
    ) -> TemperatureDewPoint {
        todo!()
    }

    fn calc_VaporPressure_from_RelativeHumidity(&self,
      tdb: &TemperatureDryBulb,
      hr: &RelativeHumidity,
     ) -> VaporPressure {
        todo!()
    }

    fn calc_RelativeHumidity_from_VaporPressure(&self,
      tdb: &TemperatureDryBulb,
      pv: &VaporPressure,
    ) -> RelativeHumidity {
        todo!()
    }

    fn calc_TDewPoint_from_VaporPressure(&self,
      tdb: &TemperatureDryBulb,
      pv: &VaporPressure,
    ) -> TemperatureDewPoint {
        todo!()
    }

    fn calc_VaporPressure_from_TDewPoint(&self,
      tdp: &TemperatureDewPoint,
    ) -> VaporPressure {
        todo!()
    }

    fn calc_TWetBulb_from_HumidityRatio(&self,
      tdb: &TemperatureDryBulb,
      Y: &HumidityRatio,
      p: &AbsolutePressure,
    ) -> TemperatureWetBulb {
        todo!()
    }

    fn calc_HumidityRatio_from_TWetBulb(&self,
      tdb: &TemperatureDryBulb,
      twb: &TemperatureWetBulb,
      p: &AbsolutePressure,
    ) -> HumidityRatio {
        todo!()
    }

    fn calc_HumidityRatio_from_RelativeHumidity(&self,
      tdb: &TemperatureDryBulb,
      hr: &RelativeHumidity,
      p: &AbsolutePressure,
    ) -> HumidityRatio {
        todo!()
    }

    fn calc_RelativeHumidity_from_HumidityRatio(&self,
      tdb: &TemperatureDryBulb,
      Y: &HumidityRatio,
      p: &AbsolutePressure,
    ) -> RelativeHumidity {
        todo!()
    }

    fn calc_HumidityRatio_from_TDewPoint(&self,
      tdp: &TemperatureDewPoint,
      p: &AbsolutePressure,
    ) -> HumidityRatio {
        todo!()
    }

    fn calc_TDewPoint_from_HumidityRatio(&self,
      tdb: &TemperatureDryBulb,
      Y: &HumidityRatio,
      p: &AbsolutePressure,
    ) -> TemperatureDewPoint {
        todo!()
    }

    fn calc_HumidityRatio_from_VaporPressure(&self,
      p: &VaporPressure,
      p: &AbsolutePressure,
    ) -> HumidityRatio {
        todo!()
    }

    fn calc_VaporPressure_from_HumidityRatio(&self,
      Y: &HumidityRatio,
      p: &AbsolutePressure,
    ) -> VaporPressure {
        todo!()
    }

    fn calc_SpecificHumidity_from_HumidityRatio(&self,
      Y: &HumidityRatio,
    ) -> SpecificHumidity {
        todo!()
    }

    fn calc_HumidityRatio_from_SpecificHumidity(&self,
      y: &SpecificHumidity,
    ) -> HumidityRatio {
        todo!()
    }

    fn calc_DryGasEnthalpy(&self,
      tdb: &TemperatureDryBulb,
     ) -> DryGasEnthalpy {
        todo!()
    }

    fn calc_DryGasDensity(&self,
      tdb: &TemperatureDryBulb,
      p: &AbsolutePressure,
    ) -> DryGasDensity {
        todo!()
    }

    fn calc_DryGasVolume(&self,
      tdb: &TemperatureDryBulb,
      p: &AbsolutePressure,
    ) -> DryGasVolume {
        todo!()
    }

    fn calc_TDryBulb_from_Enthalpy_And_HumidityRatio(&self,
      hm: &MoistureGasEnthalpy,
      Y: &HumidityRatio,
     ) -> TemperatureDryBulb {
        todo!()
    }

    fn calc_HumidityRatio_from_Enthalpy_And_TDryBulb(&self,
      hm: &MoistureGasEnthalpy,
      tdb: &TemperatureDryBulb,
     ) -> HumidityRatio {
        todo!()
    }

    fn calc_SaturatedVaporPressure(&self,
      tdb: &TemperatureDryBulb,
    ) -> SaturatedVaporPressure {
        todo!()
    }

    fn calc_SaturatedHumidityRatio(&self,
      tdb: &TemperatureDryBulb,
      p: &AbsolutePressure,
    ) -> SaturatedHumidityRatio {
        todo!()
    }

    fn calc_SaturatedGasEnthalpy(&self,
      tdb: &TemperatureDryBulb,
      p: &AbsolutePressure,
    ) -> SaturatedGasEnthalpy {
        todo!()
    }

    fn calc_VaporPressureDeficit(&self,
      tdb: &TemperatureDryBulb,
      Y: &HumidityRatio,
      p: &AbsolutePressure,
    ) -> VaporPressureDeficit {
        todo!()
    }

    fn calc_MoistureGasEnthalpy(&self,
      tdb: &TemperatureDryBulb,
      Y: &HumidityRatio,
     ) -> MoistureGasEnthalpy {
        todo!()
    }

    fn calc_MoistureGasVolume(&self,
      tdb: &TemperatureDryBulb,
      Y: &HumidityRatio,
      p: &AbsolutePressure,
    ) -> MoistureGasVolume {
        todo!()
    }

    fn calc_TDryBulb_from_MoistureGasVolume_And_HumidityRatio(&self,
      Va: &MoistureGasVolume,
      Y: &HumidityRatio,
      p: &AbsolutePressure,
    ) -> TemperatureDryBulb {
        todo!()
    }

    fn calc_MoistureGasDensity(&self,
      tdb: &TemperatureDryBulb,
      Y: &HumidityRatio,
      p: &AbsolutePressure,
    ) -> MoistureGasDensity {
        todo!()
    }
}