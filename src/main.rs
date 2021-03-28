// https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x/31193386
// https://stackoverflow.com/questions/37970355/read-xml-file-into-struct
// https://serde.rs/attr-default.html, default values in struct.

#![allow(non_snake_case)]

mod structs;

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
use std::process::exit;
use crate::structs::D2LogicalModel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
/*    let filename = "../vejr.xml";
    let content= fs::read_to_string(filename).expect("Unable to read file");*/

    dotenv().ok();

    let BASIC_AUTH = env::var("VEGVESEN_BASIC_AUTH").expect("Please add basic authentication");

    let client = reqwest::blocking::Client::new();

    // Check if URL is available.
    let res = client
        .get("http://localhost:8080")
        .send()?;

    if !res.status().is_success() {
        exit(1);
    }

    let res = client
        .get("https://www.vegvesen.no/ws/no/vegvesen/veg/trafikkpublikasjon/vaer/2/GetMeasuredWeatherData")
        .header(AUTHORIZATION, BASIC_AUTH)
        .send()?;

    let body = res.text().unwrap();

    let mut measurements: Vec<structs::WeatherMeasurement> = Vec::new();

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
                let wm = structs::WeatherMeasurement {
                    publication_time: String::from(publication_time.clone()),
                    id: u16::from(id.clone()),
                    index: u16::from(index.clone()),
                    field_description: String::from(field_description.clone()),
                    measurement: f32::from(measurement.clone()),
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
                let wm = structs::WeatherMeasurement {
                    publication_time: String::from(publication_time.clone()),
                    id: u16::from(id.clone()),
                    index: u16::from(index.clone()),
                    field_description: String::from(field_description.clone()),
                    measurement: f32::from(measurement.clone()),
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
                let wm = structs::WeatherMeasurement {
                    publication_time: String::from(publication_time.clone()),
                    id: u16::from(id.clone()),
                    index: u16::from(index.clone()),
                    field_description: String::from(field_description.clone()),
                    measurement: f32::from(measurement.clone()),
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
                let wm = structs::WeatherMeasurement {
                    publication_time: String::from(publication_time.clone()),
                    id: u16::from(id.clone()),
                    index: u16::from(index.clone()),
                    field_description: String::from(field_description.clone()),
                    measurement: f32::from(measurement.clone()),
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
                let wm = structs::WeatherMeasurement {
                    publication_time: String::from(publication_time.clone()),
                    id: u16::from(id.clone()),
                    index: u16::from(index.clone()),
                    field_description: String::from(field_description.clone()),
                    measurement: f32::from(measurement.clone()),
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
                let wm = structs::WeatherMeasurement {
                    publication_time: String::from(publication_time.clone()),
                    id: u16::from(id.clone()),
                    index: u16::from(index.clone()),
                    field_description: String::from(field_description.clone()),
                    measurement: f32::from(measurement.clone()),
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
                let wm = structs::WeatherMeasurement {
                    publication_time: String::from(publication_time.clone()),
                    id: u16::from(id.clone()),
                    index: u16::from(index.clone()),
                    field_description: String::from(field_description.clone()),
                    measurement: f32::from(measurement.clone()),
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
                let wm = structs::WeatherMeasurement {
                    publication_time: String::from(publication_time.clone()),
                    id: u16::from(id.clone()),
                    index: u16::from(index.clone()),
                    field_description: String::from(field_description.clone()),
                    measurement: f32::from(measurement.clone()),
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
                let wm = structs::WeatherMeasurement {
                    publication_time: String::from(publication_time.clone()),
                    id: u16::from(id.clone()),
                    index: u16::from(index.clone()),
                    field_description: String::from(field_description.clone()),
                    measurement: f32::from(measurement.clone()),
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
