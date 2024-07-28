use crate::common::physics::magnitude::Magnitude;

use uom::si::pressure::pascal;

pub trait Pressure {
  fn magnitude(&self) -> &Magnitude<pascal>;
}


pub struct AbsolutePressure {
  pub magnitude: Magnitude<pascal>,
}

impl Pressure for AbsolutePressure {
  fn magnitude(&self) -> &Magnitude<pascal> {
      &self.magnitude
  }
}


pub struct AtmosphericPressure {
  magnitude: Magnitude<pascal>,
}

impl Pressure for AtmosphericPressure {
  fn magnitude(&self) -> &Magnitude<pascal> {
      &self.magnitude
  }
}



pub struct VaporPressure {
  magnitude: Magnitude<pascal>,
}

impl VaporPressure {
    fn magnitude(&self) -> &Magnitude<pascal> {
      &self.magnitude
    }
}



pub struct StandardAtmPressure {
  magnitude: Magnitude<pascal>,
}

impl StandardAtmPressure {
    fn magnitude(&self) -> &Magnitude<pascal> {
      &self.magnitude
    }
}




pub struct SaturatedVaporPressure {
  magnitude: Magnitude<pascal>,
}

impl SaturatedVaporPressure {
    fn magnitude(&self) -> &Magnitude<pascal> {
      &self.magnitude
    }
}




pub struct VaporPressureDeficit {
  magnitude: Magnitude<pascal>,
}

impl VaporPressureDeficit {
    fn magnitude(&self) -> &Magnitude<pascal> {
      &self.magnitude
    }
}




pub struct SeaLevelPressure {
  magnitude: Magnitude<pascal>,
}

impl SeaLevelPressure {
    fn magnitude(&self) -> &Magnitude<pascal> {
      &self.magnitude
    }
}




pub struct StationPressure {
  magnitude: Magnitude<pascal>,
}

impl StationPressure {
    fn magnitude(&self) -> &Magnitude<pascal> {
      &self.magnitude
    }
}
