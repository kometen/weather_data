// https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x/31193386
// https://stackoverflow.com/questions/37970355/read-xml-file-into-struct
// https://serde.rs/attr-default.html, default values in struct.

#![allow(non_snake_case)]

mod structs;
mod test;

#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

//use std::fs;
use dotenv::dotenv;
use reqwest::header::AUTHORIZATION;
use std::env;
//use std::fs;
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let BASIC_AUTH = env::var("VEGVESEN_BASIC_AUTH").expect("Please add basic authentication");

    let client = reqwest::blocking::Client::new();

    // Check if URL is available.
    let res = client.get("http://localhost:8080").send()?;

    if !res.status().is_success() {
        exit(1);
    }

    let res = client
        .get("https://www.vegvesen.no/ws/no/vegvesen/veg/trafikkpublikasjon/vaer/2/GetMeasuredWeatherData")
        .header(AUTHORIZATION, BASIC_AUTH)
        .send()?;
    let body = res.text().unwrap();
    //    println!("{}", body);

    /*    let filename = "../vejr3.xml";
    let res = fs::read_to_string(filename).expect("Unable to read file");
    let body = res;*/

    let d2LogicalModel: structs::D2LogicalModel = serde_xml_rs::from_str(&body).unwrap();
    let mut measurements: Vec<structs::WeatherMeasurement> = Vec::new();

    for site in &d2LogicalModel.payloadPublication.siteMeasurements {
        // The actual weather data
        let mut readings: Vec<structs::Data> = Vec::new();

        let measurement_time_default = &site.measurementTimeDefault.measurementTimeDefault;
        let id = &site.measurementSiteReference.id;

        for measured_value in &site.measuredValue {
            let index = &measured_value.index;
            let weather_node = &measured_value.measuredValue.basicData;

            // relativeHumidity
            let field_description = &weather_node.humidity.relativeHumidity.field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node.humidity.relativeHumidity.percentage.percentage;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            }

            // precipitationIntensity
            let field_description = &weather_node
                .precipitationDetail
                .precipitationIntensity
                .field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node
                    .precipitationDetail
                    .precipitationIntensity
                    .millimetresPerHourIntensity
                    .millimetresPerHourIntensity;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };

            // roadSurfaceTemperature
            let field_description = &weather_node
                .roadSurfaceConditionMeasurements
                .roadSurfaceTemperature
                .field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node
                    .roadSurfaceConditionMeasurements
                    .roadSurfaceTemperature
                    .temperature
                    .temperature;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };

            // windSpeed
            let field_description = &weather_node.wind.windSpeed.field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node.wind.windSpeed.speed.speed;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };

            // directionBearing
            let field_description = &weather_node.wind.windDirectionBearing.field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node
                    .wind
                    .windDirectionBearing
                    .directionBearing
                    .directionBearing;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };

            // airTemperature
            let field_description = &weather_node.temperature.airTemperature.field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node
                    .temperature
                    .airTemperature
                    .temperature
                    .temperature;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };

            // dewPointTemperature
            let field_description = &weather_node
                .temperature
                .dewPointTemperature
                .field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node
                    .temperature
                    .dewPointTemperature
                    .temperature
                    .temperature;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };

            // visibility
            let field_description = &weather_node
                .visibility
                .minimumVisibilityDistance
                .field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node
                    .visibility
                    .minimumVisibilityDistance
                    .integerMetreDistance
                    .integerMetreDistance;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };

            // roadSurfaceConditionMeasurements
            let field_description = &weather_node
                .roadSurfaceConditionMeasurements
                .roadSurfaceConditionMeasurementsExtension
                .field_description;
            if !field_description.is_empty() {
                let measurement = &weather_node
                    .roadSurfaceConditionMeasurements
                    .roadSurfaceConditionMeasurementsExtension
                    .frictionExtension
                    .friction
                    .coefficientOfFriction
                    .coefficientOfFriction;
                let r = structs::Data {
                    index: *index,
                    field_description: field_description.clone(),
                    measurement: *measurement,
                };
                readings.push(r);
                /*println!("measurement time-default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                measurement_time_default, id, index, field_description, measurement);*/
            };
        }
        let wm = structs::WeatherMeasurement {
            measurement_time_default: measurement_time_default.clone(),
            id: *id,
            data: readings,
        };
        measurements.push(wm);
        // Add final struct here
    }

    let jm = serde_json::to_string(&measurements)?;
    //println!("{:?}", &jm);

    let res = client
        .post("http://localhost:8080/weather_data")
        .body(jm)
        .send()?;

    println!("res: {}", res.status());

    Ok(())
}
