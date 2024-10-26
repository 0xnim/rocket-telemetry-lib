
// Telemetry
#[derive(Debug)]
pub struct TelemetryData {
    pub altitude: Option<f32>,
    pub pressure: Option<f32>,
    pub rotation: Option<(f32, f32, f32)>,
    pub acceleration: Option<(f32, f32, f32)>,
    pub temperature: Option<f32>,
    pub gps: Option<GpsData>,
    pub time: Option<u32>,
    // ... other fields
}

#[derive(Debug)]
pub struct GpsData {
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub altitude: Option<f32>,
    pub ground_speed: Option<f32>,
    pub heading: Option<f32>,
}

#[derive(Debug)]
pub enum TelemetryField {
    Altitude,
    Pressure,
    RotationX,
    RotationY,
    RotationZ,
    AccelerationX,
    AccelerationY,
    AccelerationZ,
    Temperature,
    GpsLatitude,
    GpsLongitude,
    GpsAltitude,
    GpsGroundSpeed,
    GpsHeading,
    Time,
}

