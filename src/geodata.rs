use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GeoLocationInfo {
    pub key: &'static str,
    pub latitude: f64,
    pub longitude: f64,
    pub name: &'static str,
}

/// Initialize a big static list of gep
pub fn get_geo_location_info() -> Vec<GeoLocationInfo> {
    vec![
        GeoLocationInfo {
            key: "Falkenstein - DE",
            name: "Falkenstein - DE",
            latitude: 50.478056,
            longitude: 12.335641,
        },
        GeoLocationInfo {
            key: "London - UK",
            name: "London - UK",
            latitude: 51.500072,
            longitude: -0.127108,
        },
        GeoLocationInfo {
            key: "New York - US",
            name: "New York - US",
            latitude: 40.725351,
            longitude: -73.998684,
        },
        GeoLocationInfo {
            key: "Rio de Janeiro - BR",
            name: "Rio de Janeiro - BR",
            latitude: -22.877932,
            longitude: -43.317430,
        },
        GeoLocationInfo {
            key: "Seattle - US",
            name: "Seattle - US",
            latitude: 47.604262,
            longitude: -122.334683,
        },
        GeoLocationInfo {
            key: "Shanghai - CHN",
            name: "Shanghai - CHN",
            latitude: 31.214492,
            longitude: 121.481223,
        },
        GeoLocationInfo {
            key: "Shenzhen - CHN",
            name: "Shenzhen - CHN",
            latitude: 22.546685,
            longitude: 113.945502,
        },
        GeoLocationInfo {
            key: "Sydney - AUS",
            name: "Sydney - AUS",
            latitude: -33.886836,
            longitude: 151.159892,
        },
    ]
}
