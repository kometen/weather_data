// XML-root
#[derive(Deserialize, Debug)]
pub(crate) struct D2LogicalModel {
    modelBaseVersion: u8,
    exchange: Exchange,
    pub(crate) payloadPublication: PayloadPublication,
}

// exchange
#[derive(Deserialize, Debug)]
pub(crate) struct Exchange {
    supplierIdentification: SupplierIdentification,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SupplierIdentification {
    country: Country,
    nationalIdentifier: NationalIdentifier,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Country {
    #[serde(rename = "$value")]
    country: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct NationalIdentifier {
    #[serde(rename = "$value")]
    identifier: String,
}

// payloadPublication
#[derive(Deserialize, Debug)]
pub(crate) struct PayloadPublication {
    lang: String,
    pub(crate) publicationTime: PublicationTime,
    publicationCreator: PublicationCreator,
    measurementSiteTableReference: MeasurementSiteTableReference,
    headerInformation: HeaderInformation,
    pub(crate) siteMeasurements: Vec<SiteMeasurements>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PublicationTime {
    #[serde(rename = "$value")]
    pub(crate) publicationTime: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PublicationCreator {
    country: Country,
    nationalIdentifier: NationalIdentifier,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MeasurementSiteTableReference {
    id: String,
    targetClass: String,
    version: u64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct HeaderInformation {
    confidentiality: Confidentiality,
    informationStatus: InformationStatus
}

#[derive(Deserialize, Debug)]
pub(crate) struct Confidentiality {
    #[serde(rename = "$value")]
    confidentiality: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct InformationStatus {
    #[serde(rename = "$value")]
    informationStatus: String,
}

// // siteMeasurements, the various weather measurements are below (sub-root).
#[derive(Deserialize, Debug)]
pub(crate) struct SiteMeasurements {
    pub(crate) measurementSiteReference: MeasurementSiteReference,
    measurementTimeDefault: MeasurementTimeDefault,
    #[serde(default)]
    pub(crate) measuredValue: Vec<MeasuredValue_>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MeasurementSiteReference {
    pub(crate) id: u16,
    targetClass: String,
    version: u16,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MeasurementTimeDefault {
    #[serde(rename = "$value")]
    measurementTimeDefault: String,
}

// Common for all measurements.
#[derive(Deserialize, Debug)]
pub(crate) struct MeasuredValue_ {
    pub(crate) index: u16,
    pub(crate) measuredValue: MeasuredValue,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MeasuredValue {
    pub(crate) basicData: BasicData,
}

// Split based on type of measurement. Below this point the tree is different.
#[derive(Deserialize, Debug, Default)]
pub(crate) struct BasicData {
    #[serde(default)]
    pub(crate) humidity: Humidity,
    #[serde(default)]
    pub(crate) precipitationDetail: PrecipitationDetail,
    #[serde(default)]
    pub(crate) roadSurfaceConditionMeasurements: RoadSurfaceConditionMeasurements,
    #[serde(default)]
    pub(crate) wind: Wind,
    #[serde(default)]
    pub(crate) temperature: Temperature_,  // Add underscore since this collides with another struct
    #[serde(default)]
    pub(crate) visibility: Visibility,
}

// relativeHumidity
#[derive(Deserialize, Debug, Default)]
pub(crate) struct Humidity {
    #[serde(default)]
    pub(crate) relativeHumidity: RelativeHumidity,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct RelativeHumidity {
    #[serde(default = "relative_humidity")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) percentage: Percentage,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Percentage {
    #[serde(default)]
    #[serde(rename = "$value")]
    pub(crate) percentage: f32,
}

// precipitationIntensity
#[derive(Deserialize, Debug, Default)]
pub(crate) struct PrecipitationDetail {
    #[serde(default)]
    pub(crate) precipitationIntensity: PrecipitationIntensity,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct PrecipitationIntensity {
    #[serde(default = "precipitation_intensity")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) millimetresPerHourIntensity: MillimetresPerHourIntensity,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct MillimetresPerHourIntensity {
    #[serde(rename = "$value")]
    pub(crate) millimetresPerHourIntensity: f32,
}

// roadSurfaceTemperature
#[derive(Deserialize, Debug, Default)]
pub(crate) struct RoadSurfaceConditionMeasurements {
    #[serde(default)]
    pub(crate) roadSurfaceTemperature: RoadSurfaceTemperature,
    #[serde(default)]
    pub(crate) roadSurfaceConditionMeasurementsExtension: RoadSurfaceConditionMeasurementsExtension,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct RoadSurfaceTemperature {
    #[serde(default = "road_surface_temperature")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) temperature: Temperature,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Temperature {
    #[serde(rename = "$value")]
    pub(crate) temperature: f32,
}

// roadSurfaceConditionMeasurements
#[derive(Deserialize, Debug, Default)]
pub(crate) struct RoadSurfaceConditionMeasurementsExtension {
    #[serde(default = "road_surface_condition_measurements_extension")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) frictionExtension: FrictionExtension,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct FrictionExtension {
    #[serde(default)]
    pub(crate) friction: Friction,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Friction {
    #[serde(default)]
    pub(crate) coefficientOfFriction: CoefficientOfFriction,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct CoefficientOfFriction {
    #[serde(rename = "$value")]
    pub(crate) coefficientOfFriction: f32,
}

// windSpeed
#[derive(Deserialize, Debug, Default)]
pub(crate) struct Wind {
    #[serde(default)]
    pub(crate) windSpeed: WindSpeed,
    #[serde(default)]
    pub(crate) windDirectionBearing: WindDirectionBearing,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct WindSpeed {
    #[serde(default = "wind_speed")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) speed: Speed,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct Speed {
    #[serde(rename = "$value")]
    pub(crate) speed: f32,
}

// windDirectionBearing
#[derive(Deserialize, Debug, Default)]
pub(crate) struct WindDirectionBearing {
    #[serde(default = "wind_direction_bearing")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) directionBearing: DirectionBearing,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct DirectionBearing {
    #[serde(rename = "$value")]
    pub(crate) directionBearing: f32,
}

// airTemperature
#[derive(Deserialize, Debug, Default)]
pub(crate) struct Temperature_ {
    #[serde(default)]
    pub(crate) airTemperature: AirTemperature,
    #[serde(default)]
    pub(crate) dewPointTemperature: DewPointTemperature,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct AirTemperature {
    #[serde(default = "air_temperature")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) temperature: Temperature,
}

// dewPointTemperature
#[derive(Deserialize, Debug, Default)]
pub(crate) struct DewPointTemperature {
    #[serde(default = "dew_point_temperature")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) temperature: Temperature,
}

// visibility
#[derive(Deserialize, Debug, Default)]
pub(crate) struct Visibility {
    #[serde(default)]
    pub(crate) minimumVisibilityDistance: MinimumVisibilityDistance,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct MinimumVisibilityDistance {
    #[serde(default = "minimum_visibility_distance")]
    pub(crate) field_description: String,
    #[serde(default)]
    pub(crate) integerMetreDistance: IntegerMetreDistance,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct IntegerMetreDistance {
    #[serde(default)]
    #[serde(rename = "$value")]
    pub(crate) integerMetreDistance: f32,
}

// Add default values in serde.
fn relative_humidity() -> String {
    "relative_humidity".to_string()
}

fn precipitation_intensity() -> String {
    "precipitation_intensity".to_string()
}

fn road_surface_temperature() -> String {
    "road_surface_temperature".to_string()
}

fn wind_speed() -> String {
    "wind_speed".to_string()
}

fn wind_direction_bearing() -> String {
    "wind_direction_bearing".to_string()
}

fn air_temperature() -> String {
    "air_temperature".to_string()
}

fn dew_point_temperature() -> String {
    "dew_point_temperature".to_string()
}

fn minimum_visibility_distance() -> String {
    "minimum_visibility_distance".to_string()
}

fn road_surface_condition_measurements_extension() -> String {
    "road_surface_condition_measurements_extension".to_string()
}

#[derive(Serialize)]
pub(crate) struct WeatherMeasurement {
    pub(crate) publication_time: String,
    pub(crate) id: u16,
    pub(crate) index: u16,
    pub(crate) field_description: String,
    pub(crate) measurement: f32,
}
