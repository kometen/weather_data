#[cfg(test)]
mod tests {
    use crate::structs;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parse_xml() {
        let body = r#"<d2LogicalModel modelBaseVersion="2" xmlns="http://datex2.eu/schema/2/2_0">
    <exchange>
        <supplierIdentification>
            <country>no</country>
            <nationalIdentifier>Norwegian Public Roads Administration</nationalIdentifier>
        </supplierIdentification>
    </exchange>
    <payloadPublication lang="nob" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="MeasuredDataPublication">
        <publicationTime>2021-03-24T21:02:28.762+01:00</publicationTime>
        <publicationCreator>
            <country>no</country>
            <nationalIdentifier>Norwegian Public Roads Administration</nationalIdentifier>
        </publicationCreator>
        <measurementSiteTableReference id="WOST" targetClass="MeasurementSiteTable" version="20210324102826000"/>
        <headerInformation>
            <confidentiality>noRestriction</confidentiality>
            <informationStatus>real</informationStatus>
        </headerInformation>
        <siteMeasurements>
            <measurementSiteReference id="228" targetClass="MeasurementSiteRecord" version="3576"/>
            <measurementTimeDefault>2021-03-24T20:50:00+01:00</measurementTimeDefault>
            <measuredValue index="201">
                <measuredValue>
                    <basicData xsi:type="HumidityInformation">
                        <humidity>
                            <relativeHumidity>
                                <percentage>77.5</percentage>
                            </relativeHumidity>
                        </humidity>
                    </basicData>
                </measuredValue>
            </measuredValue>
            <measuredValue index="2501">
                <measuredValue>
                    <basicData xsi:type="PrecipitationInformation">
                        <precipitationDetail>
                            <precipitationIntensity>
                                <millimetresPerHourIntensity>0.0</millimetresPerHourIntensity>
                            </precipitationIntensity>
                        </precipitationDetail>
                    </basicData>
                </measuredValue>
            </measuredValue>
            <measuredValue index="801">
                <measuredValue>
                    <basicData xsi:type="RoadSurfaceConditionInformation">
                        <roadSurfaceConditionMeasurements>
                            <roadSurfaceTemperature>
                                <temperature>-3.0</temperature>
                            </roadSurfaceTemperature>
                        </roadSurfaceConditionMeasurements>
                    </basicData>
                </measuredValue>
            </measuredValue>
            <measuredValue index="901">
                <measuredValue>
                    <basicData xsi:type="WindInformation">
                        <wind>
                            <windSpeed>
                                <speed>21.24</speed>
                            </windSpeed>
                        </wind>
                    </basicData>
                </measuredValue>
            </measuredValue>
            <measuredValue index="1001">
                <measuredValue>
                    <basicData xsi:type="WindInformation">
                        <wind>
                            <windDirectionBearing>
                                <directionBearing>176</directionBearing>
                            </windDirectionBearing>
                        </wind>
                    </basicData>
                </measuredValue>
            </measuredValue>
            <measuredValue index="101">
                <measuredValue>
                    <basicData xsi:type="TemperatureInformation">
                        <temperature>
                            <airTemperature>
                                <temperature>0.2</temperature>
                            </airTemperature>
                        </temperature>
                    </basicData>
                </measuredValue>
            </measuredValue>
            <measuredValue index="301">
                <measuredValue>
                    <basicData xsi:type="TemperatureInformation">
                        <temperature>
                            <dewPointTemperature>
                                <temperature>-3.3</temperature>
                            </dewPointTemperature>
                        </temperature>
                    </basicData>
                </measuredValue>
            </measuredValue>
            <measuredValue index="1401">
                <measuredValue>
                    <basicData xsi:type="VisibilityInformation">
                        <visibility>
                            <minimumVisibilityDistance>
                                <integerMetreDistance>9999</integerMetreDistance>
                            </minimumVisibilityDistance>
                        </visibility>
                    </basicData>
                </measuredValue>
            </measuredValue>
            <measuredValue index="5401">
                <measuredValue>
                    <basicData xsi:type="RoadSurfaceConditionInformation">
                        <roadSurfaceConditionMeasurements>
                            <roadSurfaceConditionMeasurementsExtension>
                                <frictionExtension>
                                    <friction>
                                        <coefficientOfFriction>0.82</coefficientOfFriction>
                                    </friction>
                                </frictionExtension>
                            </roadSurfaceConditionMeasurementsExtension>
                        </roadSurfaceConditionMeasurements>
                    </basicData>
                </measuredValue>
            </measuredValue>
        </siteMeasurements>
        <siteMeasurements>
            <measurementSiteReference id="1760" targetClass="MeasurementSiteRecord" version="3560"/>
            <measurementTimeDefault>2021-03-24T19:00:00+01:00</measurementTimeDefault>
        </siteMeasurements>
    </payloadPublication>
</d2LogicalModel>"#;

        let json = r#"[{"measurement_time_default":"2021-03-24T20:50:00+01:00","id":228,"index":201,"field_description":"relative_humidity","measurement":77.5},{"measurement_time_default":"2021-03-24T20:50:00+01:00","id":228,"index":2501,"field_description":"precipitation_intensity","measurement":0.0},{"measurement_time_default":"2021-03-24T20:50:00+01:00","id":228,"index":801,"field_description":"road_surface_temperature","measurement":-3.0},{"measurement_time_default":"2021-03-24T20:50:00+01:00","id":228,"index":901,"field_description":"wind_speed","measurement":21.24},{"measurement_time_default":"2021-03-24T20:50:00+01:00","id":228,"index":1001,"field_description":"wind_direction_bearing","measurement":176.0},{"measurement_time_default":"2021-03-24T20:50:00+01:00","id":228,"index":101,"field_description":"air_temperature","measurement":0.2},{"measurement_time_default":"2021-03-24T20:50:00+01:00","id":228,"index":301,"field_description":"dew_point_temperature","measurement":-3.3},{"measurement_time_default":"2021-03-24T20:50:00+01:00","id":228,"index":1401,"field_description":"minimum_visibility_distance","measurement":9999.0},{"measurement_time_default":"2021-03-24T20:50:00+01:00","id":228,"index":5401,"field_description":"road_surface_condition_measurements_extension","measurement":0.82}]"#.to_string();

        let mut measurements: Vec<structs::WeatherMeasurement> = Vec::new();

        let d2LogicalModel: structs::D2LogicalModel = serde_xml_rs::from_str(&body).unwrap();
        for site in &d2LogicalModel.payloadPublication.siteMeasurements {
            let id = &site.measurementSiteReference.id;
            let measurement_time_default = &site.measurementTimeDefault.measurementTimeDefault;
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
                        measurement_time_default: String::from(measurement_time_default.clone()),
                        id: u16::from(id.clone()),
                        index: u16::from(index.clone()),
                        field_description: String::from(field_description.clone()),
                        measurement: f32::from(measurement.clone()),
                    };
                    assert_eq!(measurement_time_default, "2021-03-24T20:50:00+01:00");
                    assert_eq!(id.clone(), 228);
                    assert_eq!(index.clone(), 201);
                    assert_eq!(field_description, "relative_humidity");
                    assert_eq!(measurement.clone(), f32::from(77.5));
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
                        measurement_time_default: String::from(measurement_time_default.clone()),
                        id: u16::from(id.clone()),
                        index: u16::from(index.clone()),
                        field_description: String::from(field_description.clone()),
                        measurement: f32::from(measurement.clone()),
                    };
                    assert_eq!(measurement_time_default, "2021-03-24T20:50:00+01:00");
                    assert_eq!(id.clone(), 228);
                    assert_eq!(index.clone(), 2501);
                    assert_eq!(field_description, "precipitation_intensity");
                    assert_eq!(measurement.clone(), f32::from(0.0));
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
                        measurement_time_default: String::from(measurement_time_default.clone()),
                        id: u16::from(id.clone()),
                        index: u16::from(index.clone()),
                        field_description: String::from(field_description.clone()),
                        measurement: f32::from(measurement.clone()),
                    };
                    assert_eq!(measurement_time_default, "2021-03-24T20:50:00+01:00");
                    assert_eq!(id.clone(), 228);
                    assert_eq!(index.clone(), 801);
                    assert_eq!(field_description, "road_surface_temperature");
                    assert_eq!(measurement.clone(), f32::from(-3.0));
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
                        measurement_time_default: String::from(measurement_time_default.clone()),
                        id: u16::from(id.clone()),
                        index: u16::from(index.clone()),
                        field_description: String::from(field_description.clone()),
                        measurement: f32::from(measurement.clone()),
                    };
                    assert_eq!(measurement_time_default, "2021-03-24T20:50:00+01:00");
                    assert_eq!(id.clone(), 228);
                    assert_eq!(index.clone(), 901);
                    assert_eq!(field_description, "wind_speed");
                    assert_eq!(measurement.clone(), f32::from(21.24));
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
                        measurement_time_default: String::from(measurement_time_default.clone()),
                        id: u16::from(id.clone()),
                        index: u16::from(index.clone()),
                        field_description: String::from(field_description.clone()),
                        measurement: f32::from(measurement.clone()),
                    };
                    assert_eq!(measurement_time_default, "2021-03-24T20:50:00+01:00");
                    assert_eq!(id.clone(), 228);
                    assert_eq!(index.clone(), 1001);
                    assert_eq!(field_description, "wind_direction_bearing");
                    assert_eq!(measurement.clone(), f32::from(176.0));
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
                        measurement_time_default: String::from(measurement_time_default.clone()),
                        id: u16::from(id.clone()),
                        index: u16::from(index.clone()),
                        field_description: String::from(field_description.clone()),
                        measurement: f32::from(measurement.clone()),
                    };
                    assert_eq!(measurement_time_default, "2021-03-24T20:50:00+01:00");
                    assert_eq!(id.clone(), 228);
                    assert_eq!(index.clone(), 101);
                    assert_eq!(field_description, "air_temperature");
                    assert_eq!(measurement.clone(), f32::from(0.2));
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
                        measurement_time_default: String::from(measurement_time_default.clone()),
                        id: u16::from(id.clone()),
                        index: u16::from(index.clone()),
                        field_description: String::from(field_description.clone()),
                        measurement: f32::from(measurement.clone()),
                    };
                    assert_eq!(measurement_time_default, "2021-03-24T20:50:00+01:00");
                    assert_eq!(id.clone(), 228);
                    assert_eq!(index.clone(), 301);
                    assert_eq!(field_description, "dew_point_temperature");
                    assert_eq!(measurement.clone(), f32::from(-3.3));
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
                        measurement_time_default: String::from(measurement_time_default.clone()),
                        id: u16::from(id.clone()),
                        index: u16::from(index.clone()),
                        field_description: String::from(field_description.clone()),
                        measurement: f32::from(measurement.clone()),
                    };
                    assert_eq!(measurement_time_default, "2021-03-24T20:50:00+01:00");
                    assert_eq!(id.clone(), 228);
                    assert_eq!(index.clone(), 1401);
                    assert_eq!(field_description, "minimum_visibility_distance");
                    assert_eq!(measurement.clone(), f32::from(9999.0));
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
                        measurement_time_default: String::from(measurement_time_default.clone()),
                        id: u16::from(id.clone()),
                        index: u16::from(index.clone()),
                        field_description: String::from(field_description.clone()),
                        measurement: f32::from(measurement.clone()),
                    };
                    assert_eq!(measurement_time_default, "2021-03-24T20:50:00+01:00");
                    assert_eq!(id.clone(), 228);
                    assert_eq!(index.clone(), 5401);
                    assert_eq!(field_description, "road_surface_condition_measurements_extension");
                    assert_eq!(measurement.clone(), f32::from(0.82));
                    measurements.push(wm);
                    /*println!("publication time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                             publication_time, id, index, field_description, measurement);*/

                    let optional = Some(serde_json::to_string(&measurements));
                    match optional {
                        Some(jm) => {
                            assert_eq!(jm.unwrap(), json);
                        },
                        _ => {println!("test")},
                    }
                };
            }
        }
    }
}
