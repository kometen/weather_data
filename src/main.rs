// https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x/31193386
// https://stackoverflow.com/questions/37970355/read-xml-file-into-struct
// https://serde.rs/attr-default.html, default values in struct.

#![allow(non_snake_case)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_xml_rs;
extern crate reqwest;
extern crate dotenv;

//use std::fs;
use reqwest::header::AUTHORIZATION;
use dotenv::dotenv;
use std::env;

// XML-root
#[derive(Deserialize, Debug)]
struct D2LogicalModel {
    modelBaseVersion: u8,
    exchange: Exchange,
    payloadPublication: PayloadPublication,
}

// exchange
#[derive(Deserialize, Debug)]
struct Exchange {
    supplierIdentification: SupplierIdentification,
}

#[derive(Deserialize, Debug)]
struct SupplierIdentification {
    country: Country,
    nationalIdentifier: NationalIdentifier,
}

#[derive(Deserialize, Debug)]
struct Country {
    #[serde(rename = "$value")]
    country: String,
}

#[derive(Deserialize, Debug)]
struct NationalIdentifier {
    #[serde(rename = "$value")]
    identifier: String,
}

// payloadPublication
#[derive(Deserialize, Debug)]
struct PayloadPublication {
    lang: String,
    publicationTime: PublicationTime,
    publicationCreator: PublicationCreator,
    measurementSiteTableReference: MeasurementSiteTableReference,
    headerInformation: HeaderInformation,
    siteMeasurements: Vec<SiteMeasurements>,
}

#[derive(Deserialize, Debug)]
struct PublicationTime {
    #[serde(rename = "$value")]
    publicationTime: String,
}

#[derive(Deserialize, Debug)]
struct PublicationCreator {
    country: Country,
    nationalIdentifier: NationalIdentifier,
}

#[derive(Deserialize, Debug)]
struct MeasurementSiteTableReference {
    id: String,
    targetClass: String,
    version: u64,
}

#[derive(Deserialize, Debug)]
struct HeaderInformation {
    confidentiality: Confidentiality,
    informationStatus: InformationStatus
}

#[derive(Deserialize, Debug)]
struct Confidentiality {
    #[serde(rename = "$value")]
    confidentiality: String,
}

#[derive(Deserialize, Debug)]
struct InformationStatus {
    #[serde(rename = "$value")]
    informationStatus: String,
}

// // siteMeasurements, the various weather measurements are below (sub-root).
#[derive(Deserialize, Debug)]
struct SiteMeasurements {
    measurementSiteReference: MeasurementSiteReference,
    measurementTimeDefault: MeasurementTimeDefault,
    #[serde(default)]
    measuredValue: Vec<MeasuredValue_>,
}

#[derive(Deserialize, Debug)]
struct MeasurementSiteReference {
    id: u16,
    targetClass: String,
    version: u16,
}

#[derive(Deserialize, Debug)]
struct MeasurementTimeDefault {
    #[serde(rename = "$value")]
    measurementTimeDefault: String,
}

// Common for all measurements.
#[derive(Deserialize, Debug)]
struct MeasuredValue_ {
    index: u16,
    measuredValue: MeasuredValue,
}

#[derive(Deserialize, Debug)]
struct MeasuredValue {
    basicData: BasicData,
}

// Split based on type of measurement. Below this point the tree is different.
#[derive(Deserialize, Debug, Default)]
struct BasicData {
    #[serde(default)]
    humidity: Humidity,
    #[serde(default)]
    precipitationDetail: PrecipitationDetail,
    #[serde(default)]
    roadSurfaceConditionMeasurements: RoadSurfaceConditionMeasurements,
    #[serde(default)]
    wind: Wind,
    #[serde(default)]
    temperature: Temperature_,  // Add underscore since this collides with another struct
    #[serde(default)]
    visibility: Visibility,
}

// relativeHumidity
#[derive(Deserialize, Debug, Default)]
struct Humidity {
    #[serde(default)]
    relativeHumidity: RelativeHumidity,
}

#[derive(Deserialize, Debug, Default)]
struct RelativeHumidity {
    #[serde(default = "relative_humidity")]
    field_description: String,
    #[serde(default)]
    percentage: Percentage,
}

#[derive(Deserialize, Debug, Default)]
struct Percentage {
    #[serde(default)]
    #[serde(rename = "$value")]
    percentage: f32,
}

// precipitationIntensity
#[derive(Deserialize, Debug, Default)]
struct PrecipitationDetail {
    #[serde(default)]
    precipitationIntensity: PrecipitationIntensity,
}

#[derive(Deserialize, Debug, Default)]
struct PrecipitationIntensity {
    #[serde(default = "precipitation_intensity")]
    field_description: String,
    #[serde(default)]
    millimetresPerHourIntensity: MillimetresPerHourIntensity,
}

#[derive(Deserialize, Debug, Default)]
struct MillimetresPerHourIntensity {
    #[serde(rename = "$value")]
    millimetresPerHourIntensity: f32,
}

// roadSurfaceTemperature
#[derive(Deserialize, Debug, Default)]
struct RoadSurfaceConditionMeasurements {
    #[serde(default)]
    roadSurfaceTemperature: RoadSurfaceTemperature,
    #[serde(default)]
    roadSurfaceConditionMeasurementsExtension: RoadSurfaceConditionMeasurementsExtension,
}

#[derive(Deserialize, Debug, Default)]
struct RoadSurfaceTemperature {
    #[serde(default = "road_surface_temperature")]
    field_description: String,
    #[serde(default)]
    temperature: Temperature,
}

#[derive(Deserialize, Debug, Default)]
struct Temperature {
    #[serde(rename = "$value")]
    temperature: f32,
}

// roadSurfaceConditionMeasurements
#[derive(Deserialize, Debug, Default)]
struct RoadSurfaceConditionMeasurementsExtension {
    #[serde(default = "road_surface_condition_measurements_extension")]
    field_description: String,
    #[serde(default)]
    frictionExtension: FrictionExtension,
}

#[derive(Deserialize, Debug, Default)]
struct FrictionExtension {
    #[serde(default)]
    friction: Friction,
}

#[derive(Deserialize, Debug, Default)]
struct Friction {
    #[serde(default)]
    coefficientOfFriction: CoefficientOfFriction,
}

#[derive(Deserialize, Debug, Default)]
struct CoefficientOfFriction {
    #[serde(rename = "$value")]
    coefficientOfFriction: f32,
}

// windSpeed
#[derive(Deserialize, Debug, Default)]
struct Wind {
    #[serde(default)]
    windSpeed: WindSpeed,
    #[serde(default)]
    windDirectionBearing: WindDirectionBearing,
}

#[derive(Deserialize, Debug, Default)]
struct WindSpeed {
    #[serde(default = "wind_speed")]
    field_description: String,
    #[serde(default)]
    speed: Speed,
}

#[derive(Deserialize, Debug, Default)]
struct Speed {
    #[serde(rename = "$value")]
    speed: f32,
}

// windDirectionBearing
#[derive(Deserialize, Debug, Default)]
struct WindDirectionBearing {
    #[serde(default = "wind_direction_bearing")]
    field_description: String,
    #[serde(default)]
    directionBearing: DirectionBearing,
}

#[derive(Deserialize, Debug, Default)]
struct DirectionBearing {
    #[serde(rename = "$value")]
    directionBearing: f32,
}

// airTemperature
#[derive(Deserialize, Debug, Default)]
struct Temperature_ {
    #[serde(default)]
    airTemperature: AirTemperature,
    #[serde(default)]
    dewPointTemperature: DewPointTemperature,
}

#[derive(Deserialize, Debug, Default)]
struct AirTemperature {
    #[serde(default = "air_temperature")]
    field_description: String,
    #[serde(default)]
    temperature: Temperature,
}

// dewPointTemperature
#[derive(Deserialize, Debug, Default)]
struct DewPointTemperature {
    #[serde(default = "dew_point_temperature")]
    field_description: String,
    #[serde(default)]
    temperature: Temperature,
}

// visibility
#[derive(Deserialize, Debug, Default)]
struct Visibility {
    #[serde(default)]
    minimumVisibilityDistance: MinimumVisibilityDistance,
}

#[derive(Deserialize, Debug, Default)]
struct MinimumVisibilityDistance {
    #[serde(default = "minimum_visibility_distance")]
    field_description: String,
    #[serde(default)]
    integerMetreDistance: IntegerMetreDistance,
}

#[derive(Deserialize, Debug, Default)]
struct IntegerMetreDistance {
    #[serde(default)]
    #[serde(rename = "$value")]
    integerMetreDistance: f32,
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
struct WeatherMeasurement {
    publication_time: String,
    id: u16,
    index: u16,
    field_description: String,
    measurement: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
/*    let filename = "../vejr.xml";
    let content= fs::read_to_string(filename).expect("Unable to read file");*/

    dotenv().ok();

    let BASIC_AUTH = env::var("VEGVESEN_BASIC_AUTH").expect("Please add basic authentication");

    let client = reqwest::blocking::Client::new();
    let res = client
        .get("https://www.vegvesen.no/ws/no/vegvesen/veg/trafikkpublikasjon/vaer/2/GetMeasuredWeatherData")
        .header(AUTHORIZATION, BASIC_AUTH)
        .send()?;

    let body = res.text().unwrap();

    let mut measurements: Vec<WeatherMeasurement> = Vec::new();

    let d2LogicalModel: D2LogicalModel = serde_xml_rs::from_str(&body).unwrap();
    let publication_time = &d2LogicalModel.payloadPublication.publicationTime.publicationTime;
    for site in &d2LogicalModel.payloadPublication.siteMeasurements {
        let id = &site.measurementSiteReference.id;
        for measured_value in &site.measuredValue {
            let index = &measured_value.index;

            let weather_node = &measured_value.measuredValue.basicData;

            // relativeHumidity
            let field_description = &weather_node
                .humidity
                .relativeHumidity
                .field_description;
            if field_description != "" {
                let measurement = &weather_node
                    .humidity
                    .relativeHumidity
                    .percentage
                    .percentage;
                let wm = WeatherMeasurement {
                    publication_time: publication_time.clone(),
                    id: id.clone(),
                    index: index.clone(),
                    field_description: field_description.clone(),
                    measurement: measurement.clone(),
                };
                measurements.push(wm);
                /*println!("publication time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                         publication_time, id, index, field_description, measurement);*/
            }

            // precipitationIntensity
            let field_description = &weather_node
                .precipitationDetail
                .precipitationIntensity
                .field_description;
            if field_description != "" {
                let measurement = &weather_node
                    .precipitationDetail
                    .precipitationIntensity
                    .millimetresPerHourIntensity
                    .millimetresPerHourIntensity;
                let wm = WeatherMeasurement {
                    publication_time: publication_time.clone(),
                    id: id.clone(),
                    index: index.clone(),
                    field_description: field_description.clone(),
                    measurement: measurement.clone(),
                };
                measurements.push(wm);
                /*println!("publication time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                         publication_time, id, index, field_description, measurement);*/
            };

            // roadSurfaceTemperature
            let field_description = &weather_node
                .roadSurfaceConditionMeasurements
                .roadSurfaceTemperature
                .field_description;
            if field_description != "" {
                let measurement = &weather_node
                    .roadSurfaceConditionMeasurements
                    .roadSurfaceTemperature
                    .temperature
                    .temperature;
                let wm = WeatherMeasurement {
                    publication_time: publication_time.clone(),
                    id: id.clone(),
                    index: index.clone(),
                    field_description: field_description.clone(),
                    measurement: measurement.clone(),
                };
                measurements.push(wm);
                /*println!("publication time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                         publication_time, id, index, field_description, measurement);*/
            };

            // windSpeed
            let field_description = &weather_node
                .wind
                .windSpeed
                .field_description;
            if field_description != "" {
                let measurement = &weather_node
                    .wind
                    .windSpeed
                    .speed
                    .speed;
                let wm = WeatherMeasurement {
                    publication_time: publication_time.clone(),
                    id: id.clone(),
                    index: index.clone(),
                    field_description: field_description.clone(),
                    measurement: measurement.clone(),
                };
                measurements.push(wm);
                /*println!("publication time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                         publication_time, id, index, field_description, measurement);*/
            };

            // directionBearing
            let field_description = &weather_node
                .wind
                .windDirectionBearing
                .field_description;
            if field_description != "" {
                let measurement = &weather_node
                    .wind
                    .windDirectionBearing
                    .directionBearing
                    .directionBearing;
                let wm = WeatherMeasurement {
                    publication_time: publication_time.clone(),
                    id: id.clone(),
                    index: index.clone(),
                    field_description: field_description.clone(),
                    measurement: measurement.clone(),
                };
                measurements.push(wm);
                /*println!("publication time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                         publication_time, id, index, field_description, measurement);*/
            };

            // airTemperature
            let field_description = &weather_node
                .temperature
                .airTemperature
                .field_description;
            if field_description != "" {
                let measurement = &weather_node
                    .temperature
                    .airTemperature
                    .temperature
                    .temperature;
                let wm = WeatherMeasurement {
                    publication_time: publication_time.clone(),
                    id: id.clone(),
                    index: index.clone(),
                    field_description: field_description.clone(),
                    measurement: measurement.clone(),
                };
                measurements.push(wm);
                /*println!("publication time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                         publication_time, id, index, field_description, measurement);*/
            };

            // dewPointTemperature
            let field_description = &weather_node
                .temperature
                .dewPointTemperature
                .field_description;
            if field_description != "" {
                let measurement = &weather_node
                    .temperature
                    .dewPointTemperature
                    .temperature
                    .temperature;
                let wm = WeatherMeasurement {
                    publication_time: publication_time.clone(),
                    id: id.clone(),
                    index: index.clone(),
                    field_description: field_description.clone(),
                    measurement: measurement.clone(),
                };
                measurements.push(wm);
                /*println!("publication time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                         publication_time, id, index, field_description, measurement);*/
            };

            // visibility
            let field_description = &weather_node
                .visibility
                .minimumVisibilityDistance
                .field_description;
            if field_description != "" {
                let measurement = &weather_node
                    .visibility
                    .minimumVisibilityDistance
                    .integerMetreDistance
                    .integerMetreDistance;
                let wm = WeatherMeasurement {
                    publication_time: publication_time.clone(),
                    id: id.clone(),
                    index: index.clone(),
                    field_description: field_description.clone(),
                    measurement: measurement.clone(),
                };
                measurements.push(wm);
                /*println!("publication time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                         publication_time, id, index, field_description, measurement);*/
            };

            // roadSurfaceConditionMeasurements
            let field_description = &weather_node
                .roadSurfaceConditionMeasurements
                .roadSurfaceConditionMeasurementsExtension
                .field_description;
            if field_description != "" {
                let measurement = &weather_node
                    .roadSurfaceConditionMeasurements
                    .roadSurfaceConditionMeasurementsExtension
                    .frictionExtension
                    .friction
                    .coefficientOfFriction
                    .coefficientOfFriction;
                let wm = WeatherMeasurement {
                    publication_time: publication_time.clone(),
                    id: id.clone(),
                    index: index.clone(),
                    field_description: field_description.clone(),
                    measurement: measurement.clone(),
                };
                measurements.push(wm);
                /*println!("publication time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                         publication_time, id, index, field_description, measurement);*/
            };
        }
    }

    let jm = serde_json::to_string(&measurements)?;
//    println!("{:?}", &jm);

    let res = client
        .post("http://localhost:8080/weather_data")
        .body(jm)
        .send()?;

    println!("res: {}", res.status());

    Ok(())
}
