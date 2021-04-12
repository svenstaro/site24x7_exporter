//! Geolocation data related to the Site24x7 locations.
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GeoLocationInfo {
    pub key: &'static str,
    pub latitude: f64,
    pub longitude: f64,
    pub name: &'static str,
}

/// Initialize a big static list of gep
pub fn get_geolocation_info() -> Vec<GeoLocationInfo> {
    vec![
        GeoLocationInfo {
            key: "Amsterdam - NL",
            name: "Amsterdam - NL",
            latitude: 52.37403,
            longitude: 4.88969,
        },
        GeoLocationInfo {
            key: "Atlanta - US",
            name: "Atlanta - US",
            latitude: 33.749,
            longitude: -84.38798,
        },
        GeoLocationInfo {
            key: "Bangkok - TH",
            name: "Bangkok - TH",
            latitude: 13.75398,
            longitude: 100.50144,
        },
        GeoLocationInfo {
            key: "Barcelona - ES",
            name: "Barcelona - ES",
            latitude: 41.38879,
            longitude: 2.15899,
        },
        GeoLocationInfo {
            key: "Beijing - CHN",
            name: "Beijing - CHN",
            latitude: 39.918722,
            longitude: 116.390186,
        },
        GeoLocationInfo {
            key: "Chengdu - CHN",
            name: "Chengdu - CHN",
            latitude: 30.661116,
            longitude: 104.068254,
        },
        GeoLocationInfo {
            key: "Chennai - IN",
            name: "Chennai - IN",
            latitude: 13.08784,
            longitude: 80.27847,
        },
        GeoLocationInfo {
            key: "Chicago - US",
            name: "Chicago - US",
            latitude: 41.85003,
            longitude: -87.65005,
        },
        GeoLocationInfo {
            key: "Chongqing - CHN",
            name: "Chongqing - CHN",
            latitude: 29.558157,
            longitude: 106.559216,
        },
        GeoLocationInfo {
            key: "Copenhagen - DA",
            name: "Copenhagen - DA",
            latitude: 55.67594,
            longitude: 12.56553,
        },
        GeoLocationInfo {
            key: "Dubai - UAE",
            name: "Dubai - UAE",
            latitude: 25.0657,
            longitude: 55.17128,
        },
        GeoLocationInfo {
            key: "Falkenstein - DE",
            name: "Falkenstein - DE",
            latitude: 50.478056,
            longitude: 12.335641,
        },
        GeoLocationInfo {
            key: "Frankfurt - DE",
            name: "Frankfurt - DE",
            latitude: 50.11552,
            longitude: 8.68417,
        },
        GeoLocationInfo {
            key: "Guangzhou - CHN",
            name: "Guangzhou - CHN",
            latitude: 23.125833,
            longitude: 113.259865,
        },
        GeoLocationInfo {
            key: "Hong Kong - HK",
            name: "Hong Kong - HK",
            latitude: 22.324494,
            longitude: 114.169539,
        },
        GeoLocationInfo {
            key: "Houston - US",
            name: "Houston - US",
            latitude: 29.76328,
            longitude: -95.36327,
        },
        GeoLocationInfo {
            key: "Istanbul - TR",
            name: "Istanbul - TR",
            latitude: 41.01384,
            longitude: 28.94966,
        },
        GeoLocationInfo {
            key: "Johannesburg - ZA",
            name: "Johannesburg - ZA",
            latitude: -26.202477,
            longitude: 28.047010,
        },
        GeoLocationInfo {
            key: "London - UK",
            name: "London - UK",
            latitude: 51.500072,
            longitude: -0.127108,
        },
        GeoLocationInfo {
            key: "Los Angeles - US",
            name: "Los Angeles - US",
            latitude: 34.05223,
            longitude: -118.24368,
        },
        GeoLocationInfo {
            key: "Miami - US",
            name: "Miami - US",
            latitude: 25.77427,
            longitude: -80.19366,
        },
        GeoLocationInfo {
            key: "Moscow - RU",
            name: "Moscow - RU",
            latitude: 55.75222,
            longitude: 37.61556,
        },
        GeoLocationInfo {
            key: "Mumbai - IN",
            name: "Mumbai - IN",
            latitude: 19.07283,
            longitude: 72.88261,
        },         
        GeoLocationInfo {
            key: "New York - US",
            name: "New York - US",
            latitude: 40.725351,
            longitude: -73.998684,
        },
        GeoLocationInfo {
            key: "Paris - FR",
            name: "Paris - FR",
            latitude: 48.85341,
            longitude: 2.3488,
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
            key: "Singapore - SG",
            name: "Singapore - SG",
            latitude: 1.333914,
            longitude: 103.844230,
        },
        GeoLocationInfo {
            key: "Sydney - AUS",
            name: "Sydney - AUS",
            latitude: -33.886836,
            longitude: 151.159892,
        },   
        GeoLocationInfo {
            key: "Taipei - TW",
            name: "Taipei - TW",
            latitude: 25.020797,
            longitude: 121.464569,
        },
        GeoLocationInfo {
            key: "Tokyo - JP",
            name: "Tokyo - JP",
            latitude: 35.6895,
            longitude: 139.69171,
        },
        GeoLocationInfo {
            key: "Vancouver - CA",
            name: "Vancouver - CA",
            latitude: 49.24966,
            longitude: -123.11934,
        },
    ]
}
