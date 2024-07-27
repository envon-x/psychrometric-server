mod magnitude;

enum Pressure {
  Atmospheric(Magnitude),
  Vapor(Magnitude),
  SeaLevel(Magnitude),
  Station(Magnitude),
}