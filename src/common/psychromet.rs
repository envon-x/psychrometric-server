pub trait Psychromet {

  fn calc_TWetBulb_From_TDewPoint(TemperatureDryBulb tdb , TemperatureDewPoint tdp , Pressure P) -> TemperatureWetBulb;

  fn calc_TWetBulb_From_RelHumidity(TemperatureDryBulb tdb , RelHumidity hr , Pressure P) -> TemperatureWetBulb;
  
  fn calc_RelHum_From_TDewPoint(TemperatureDryBulb tdb , TemperatureDewPoint tdp ) -> RelativeHumidity;
  
  fn calc_RelHum_From_TWetBulb(TemperatureDryBulb tdb , TemperatureWetBulb twb , Pressure P) -> RelativeHumidity;
  
  fn calc_TDewPoint_From_RelHumidity(TemperatureDryBulb tdb , RelHumidity hr ) -> TemperatureDewPoint;
  
  fn calc_TDewPoint_From_TWetBulb(TemperatureDryBulb tdb , TemperatureWetBulb twb , Pressure P) -> TemperatureDewPoint;
  
  fn calc_VapPressure_From_RelHumidity(TemperatureDryBulb tdb , RelHumidity hr ) -> VaporPressure;
  
  fn calc_RelHum_From_VapPressure(TemperatureDryBulb tdb , VaporPressure Pv) -> RelativeHumidity;
  
  fn calc_TDewPoint_From_VapPressure(TemperatureDryBulb tdb , VaporPressure Pv) -> TemperatureDewPoint;
  
  fn calc_VapPressure_From_TDewPoint(TemperatureDewPoint tdp ) -> VaporPressure;
  
  fn calc_TWetBulb_From_HumRatio(TemperatureDryBulb tdb , HumidityRatio Y , Pressure P) -> TemperatureWetBulb;
  
  fn calc_HumRatio_From_TWetBulb(TemperatureDryBulb tdb , TemperatureWetBulb twb , Pressure P) -> HumidityRatio;
  
  fn calc_HumRatio_From_RelHumidity(TemperatureDryBulb tdb , RelHumidity hr , Pressure P) -> HumidityRatio;
  
  fn calc_RelHum_From_HumRatio(TemperatureDryBulb tdb , HumidityRatio Y , Pressure P) -> RelativeHumidity;
  
  fn calc_HumRatio_From_TDewPoint(TemperatureDewPoint tdp , Pressure P) -> HumidityRatio;
  
  fn calc_TDewPoint_From_HumRatio(TemperatureDryBulb tdb , HumidityRatio Y , Pressure P) -> TemperatureDewPoint;
  
  fn calc_HumRatio_From_VapPressure(VaporPressure Pv, Pressure P) -> HumidityRatio;
  
  fn calc_VapPressure_From_HumRatio(HumidityRatio Y , Pressure P) -> VaporPressure;
  
  fn calc_SpecificHum_From_HumRatio(HumidityRatio Y ) -> SpecificHumidity;
  
  fn calc_HumRatio_From_SpecificHum(SpecificHumidity y ) -> HumidityRatio;
  
  fn calc_DryGasEnthalpy(TemperatureDryBulb tdb ) -> DryGasEnthalpy;
  
  fn calc_DryGasDensity(TemperatureDryBulb tdb , Pressure P) -> DryGasDensity;
  
  fn calc_DryGasVolume(TemperatureDryBulb tdb , Pressure P) -> DryGasVolume;
  
  fn calc_TDryBulb_From_Enthalpy_And_HumRatio(MoistGasEnthalpy hm, HumidityRatio Y ) -> TemperatureDryBulb;
  
  fn calc_HumRatio_From_Enthalpy_And_TDryBulb(MoistGasEnthalpy hm, TemperatureDryBulb tdb ) -> HumidityRatio;
  
  fn calc_SatVapPressure(TemperatureDryBulb tdb ) -> SatVapPressure;
  
  fn calc_SatHumRatio(TemperatureDryBulb tdb , Pressure P) -> SatHumRatio;
  
  fn calc_SatGasEnthalpy(TemperatureDryBulb tdb , Pressure P) -> SatGasEnthalpy;
  
  fn calc_VaporPressureDeficit(TemperatureDryBulb tdb , HumidityRatio Y , Pressure P) -> VaporPressureDeficit;
  
  fn calc_DegreeOfSaturation(TemperatureDryBulb tdb , HumidityRatio Y , Pressure P) -> DegreeOfSaturation;
  
  fn calc_MoistGasEnthalpy(TemperatureDryBulb tdb , HumidityRatio Y ) -> MoistGasEnthalpy;
  
  fn calc_MoistGasVolume(TemperatureDryBulb tdb , HumidityRatio Y , Pressure P) -> MoistGasVolume;
  
  fn calc_TDryBulb_From_MoistGasVolume_And_HumRatio(MoistGasVolume Va, HumidityRatio Y , Pressure P) -> TemperatureDryBul;
  
  fn calc_MoistGasDensity(TemperatureDryBulb tdb , HumidityRatio Y , Pressure P) -> MoistGasDensity;
  
  fn calc_StandardAtmPressure(Altitude z) -> StandardAtmPressure;
  
  fn calc_StandardAtmTemperature(Altitude z) -> StandardAtmTemperature;
  
  fn calc_SeaLevelPressure(StationPressure Ps, Altitude z, TemperatureDryBulb tdb ) -> SeaLevelPressure;
  
  fn calc_StationPressure(SeaLevelPressure Psea, Altitude z, TemperatureDryBulb tdb ) -> StationPressure;
  
}