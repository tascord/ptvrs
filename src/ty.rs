//! Some types are marked as 'Value' and have a note, TODO: T.
//! This is because the API documentation does not specify the type of the value.
//! I'll do my best to fill in what appears to be correct, but it's not guaranteed to be correct.
//!
//! I appreciate any work done to fill in the TODO: T types.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Display, str::FromStr};
use to_and_fro::{output_case, ToAndFro};

use crate::{
    de_rfc3339,
    helpers::{de_iso_8601, de_service_time, ser_iso_8601, ser_touch_utc},
    opt_de_rfc3339,
};

pub struct I32ButSilly(pub i32);
impl<'de> Deserialize<'de> for I32ButSilly {
    fn deserialize<D>(deserializer: D) -> Result<I32ButSilly, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(I32ButSilly(
            i32::from_str(&value).map_err(|e| serde::de::Error::custom(format!("{e:?}")))?,
        ))
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(i8)]
pub enum RouteType {
    Train = 0,
    Tram = 1,
    Bus = 2,
    VLine = 3,
    NightBus = 4,
    Other(i8),
}

impl From<i8> for RouteType {
    fn from(value: i8) -> Self {
        match value {
            0 => RouteType::Train,
            1 => RouteType::Tram,
            2 => RouteType::Bus,
            3 => RouteType::VLine,
            4 => RouteType::NightBus,
            _ => RouteType::Other(value),
        }
    }
}

impl Into<i8> for RouteType {
    fn into(self) -> i8 {
        match self {
            RouteType::Train => 0,
            RouteType::Tram => 1,
            RouteType::Bus => 2,
            RouteType::VLine => 3,
            RouteType::NightBus => 4,
            RouteType::Other(value) => value,
        }
    }
}

impl Serialize for RouteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i8((*self).into())
    }
}

impl<'de> Deserialize<'de> for RouteType {
    fn deserialize<D>(deserializer: D) -> Result<RouteType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i8::deserialize(deserializer)?;
        Ok(value.into())
    }
}

impl Display for RouteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Into::<i8>::into(*self).fmt(f)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DisruptionModes {}

impl From<i8> for DisruptionModes {
    fn from(_value: i8) -> Self {
        todo!();
    }
}

impl Into<i8> for DisruptionModes {
    fn into(self) -> i8 {
        todo!();
    }
}

impl Serialize for DisruptionModes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i8((*self).into())
    }
}

impl<'de> Deserialize<'de> for DisruptionModes {
    fn deserialize<D>(deserializer: D) -> Result<DisruptionModes, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i8::deserialize(deserializer)?;
        Ok(value.into())
    }
}

impl DisruptionModes {
    pub fn as_number(&self) -> i8 {
        Into::<i8>::into(*self)
    }
}

//

#[derive(Deserialize, Debug)]
pub struct Status {
    /// API Version number
    pub version: String,
    /// API system health status (0=offline, 1=online)
    pub health: i8,
}

//

#[derive(Serialize, Default)]
pub struct DeparturesStopOpts {
    /// Filter by platform number at stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_numbers: Option<Vec<i32>>,
    /// Filter by identifier of direction of travel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_id: Option<i32>,
    /// Indicates that stop_id parameter will accept "GTFS stop_id" data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtfs: Option<bool>,
    /// Filter by the date and time of the request (default = current date and time)
    #[serde(serialize_with = "ser_iso_8601")]
    #[serde(rename = "date_utc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDateTime>,
    /// Maximum number of results returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// Indicates if cancelled services are included in results.
    /// Metro Trains only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cancelled: Option<bool>,
    /// Indicates if filtering runs to those that arrive at destination before date_urc
    /// (default = false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_backwards: Option<bool>,
    /// Last of objects to be returned in full
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<ExpandOptions>>,
    ///Indicates if the route geopath should be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_geopath: Option<bool>,
}

#[derive(Serialize, Default)]
pub struct DeparturesStopRouteOpts {
    /// Filter by identifier of direction of travel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_id: Option<i32>,
    /// Indicates that stop_id parameter will accept "GTFS stop_id" data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtfs: Option<bool>,
    /// Filter by the date and time of the request (default = current date and time)
    #[serde(serialize_with = "ser_iso_8601")]
    #[serde(rename = "date_utc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDateTime>,
    /// Maximum number of results returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// Indicates if cancelled services are included in results.
    /// Metro Trains only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cancelled: Option<bool>,
    /// Indicates if filtering runs to those that arrive at destination before date_urc
    /// (default = false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_backwards: Option<bool>,
    /// Last of objects to be returned in full
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<ExpandOptions>>,
    ///Indicates if the route geopaath should be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_geopath: Option<bool>,
}

#[derive(Debug, ToAndFro)]
pub enum ExpandOptions {
    All,
    Stop,
    Router,
    Run,
    Direction,
    Disruption,
    VehiclePosition,
    VehicleDescriptor,
    None,
}

impl Serialize for ExpandOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ExpandOptions {
    fn deserialize<D>(deserializer: D) -> Result<ExpandOptions, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::from_str(&value).map_err(|e| serde::de::Error::custom(e))
    }
}
#[derive(Deserialize, Debug)]
pub struct DeparturesResponse {
    /// Timetabled and real-time service departures
    pub departures: Vec<Departure>,
    /// A train station, tram stop, bus stop, regional coach stop or Night Bus stop
    pub stops: HashMap<String, DepartureStop>,
    /// Train lines, tram routes, bus routes, regional coach routes, Night Bus routes
    pub routes: HashMap<String, DepartureRoute>,
    /// Individual trips/services of a route
    pub runs: HashMap<String, Run>,
    /// Directions of travel of route
    pub directions: HashMap<String, Direction>,
    /// Disruption information applicable to relevant routes or stops
    pub disruptions: HashMap<String, Disruption>,
    // API Status / Metadata
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct Departure {
    /// Stop identifier
    pub stop_id: i32,
    /// Route identifier
    pub route_id: i32,
    /// Numeric trip/service run identifier. Defaults to -1 when run identifier is Alphanumeric
    pub run_id: i32,
    /// Alphanumeric trip/service run identifier
    pub run_ref: String,
    /// Direction of travel identifier
    pub direction_id: i32,
    /// Disruption information identifier(s)
    pub disruption_ids: Vec<i32>,
    /// Scheduled (i.e. timetabled) departure time and date
    #[serde(deserialize_with = "opt_de_rfc3339")]
    #[serde(rename = "scheduled_departure_utc")]
    pub scheduled_departure: Option<NaiveDateTime>, // TODO: Seems to always be Some
    /// Real-time estimate of departure time and date
    #[serde(deserialize_with = "opt_de_rfc3339")]
    #[serde(rename = "estimated_departure_utc")]
    pub estimated_departure: Option<NaiveDateTime>,
    /// Indicates if the metropolitan train service is at the platform at the time of query.
    /// false for other modes
    pub at_platform: bool,
    /// Platform number at stop (metropolitan train only.
    /// None for other modes
    pub platform_number: Option<String>,
    /// Flag indicating special condition for run
    pub flags: String,
    /// Chronological sequence for the departures in a run.
    pub departure_sequence: i32,
}

/// TODO: Should we rename fields here?
#[derive(Deserialize, Debug)]
pub struct DepartureStop {
    #[serde(rename = "stop_distance")]
    pub distance: f32,
    #[serde(rename = "stop_suburb")]
    pub suburb: String,
    #[serde(rename = "stop_name")]
    pub name: String,
    #[serde(rename = "stop_id")]
    pub id: i32,
    pub route_type: RouteType,
    #[serde(rename = "stop_latitude")]
    pub latitude: f64,
    #[serde(rename = "stop_longitude")]
    pub longitude: f64,
    /// Seems to sometimes be empty
    #[serde(rename = "stop_landmark")]
    pub landmark: String,
    #[serde(rename = "stop_sequence")]
    pub sequence: i32,
}

#[derive(Deserialize, Debug)]
pub struct DepartureRoute {
    #[serde(rename = "route_type")]
    pub route_type: RouteType,
    #[serde(rename = "route_id")]
    pub id: i32,
    #[serde(rename = "route_name")]
    pub name: String,
    #[serde(rename = "route_number")]
    pub number: String,
    #[serde(rename = "route_gtfs_id")]
    pub gtfs_id: String,
    /// TODO: T
    pub geopath: Vec<Value>,
}

#[derive(Deserialize, Debug)]
pub struct Direction {
    #[serde(rename = "direction_id")]
    pub id: i32,
    #[serde(rename = "direction_name")]
    pub name: String,
    pub route_id: i32,
    pub route_type: RouteType,
}

//

#[derive(Deserialize, Debug)]
pub struct DirectionsResponse {
    /// Directions of travel of route
    pub directions: Vec<DirectionWithDescription>,
    /// API Status / Metadata
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct DirectionWithDescription {
    /// Description
    #[serde(rename = "route_direction_description")]
    pub description: String,
    /// Direction of travel identifier
    #[serde(rename = "direction_id")]
    pub id: i32,
    /// Name of direction of travel
    #[serde(rename = "direction_name")]
    pub name: String,
    /// Route identifier
    pub route_id: i32,
    /// Transport mode identifier
    pub route_type: RouteType,
}

//

#[derive(Serialize, Default)]
pub struct DisruptionsOpts {
    /// Filter by route type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_types: Option<Vec<RouteType>>,
    /// Filter by disruption_modes
    #[serde(rename = "disruption_modes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<DisruptionModes>>,
    /// Filter by status of disruption
    #[serde(rename = "disruption_status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DisruptionStatus>,
}

#[derive(Serialize, Default)]
pub struct DisruptionsSpecificOpts {
    /// Filter by status of disruption
    #[serde(rename = "disruption_status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DisruptionStatus>,
}

#[derive(Debug, ToAndFro)]
#[output_case("lower")]
pub enum DisruptionStatus {
    Current,
    Planned,
}

impl Serialize for DisruptionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DisruptionStatus {
    fn deserialize<D>(deserializer: D) -> Result<DisruptionStatus, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::from_str(&value).map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(Deserialize, Debug)]
pub struct DisruptionsResponse {
    /// Disruption information applicable to relavenet route, run, stop, direction
    pub disruptions: Disruptions,
    /// API Status / Metadata
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct Disruptions {
    /// Subset of disruption information applicable to multiple route_types
    pub general: Vec<Disruption>,
    /// Subset of disruption information applicable to metropolitan train
    pub metro_train: Vec<Disruption>,
    /// Subset of disruption information applicable to metropolitan tram
    pub metro_tram: Vec<Disruption>,
    /// Subset of disruption information applicable to metropolitan bus
    pub metro_bus: Vec<Disruption>,
    /// Subset of disruption information applicable to V/Line train
    pub regional_train: Vec<Disruption>,
    /// Subset of disruption information applicable to V/Line coach
    pub regional_coach: Vec<Disruption>,
    /// Subset of disruption information applicable to regional bus
    pub regional_bus: Vec<Disruption>,
    /// Subset of disruption information applicable to school bus
    pub school_bus: Vec<Disruption>,
    /// Subset of disruption information applicable to telebus services
    pub telebus: Vec<Disruption>,
    /// Subset of disruption information applicable to night bus
    pub night_bus: Vec<Disruption>,
    /// Subset of disruption information applicable to ferry
    pub ferry: Vec<Disruption>,
    /// Subset of disruption information applicable to interstate train
    pub interstate_train: Vec<Disruption>,
    /// Subset of disruption information applicable to skybus
    pub skybus: Vec<Disruption>,
    /// Subset of disruption information applicable to taxi
    pub taxi: Vec<Disruption>,
}

#[derive(Deserialize, Debug)]
pub struct Disruption {
    /// Disruption information identifier
    pub disruption_id: i32,
    /// Headline title summarising disruption information
    pub title: String,
    /// URL of relevant article on PTV website
    pub url: String,
    /// Description of the disruption
    pub description: String,
    /// Status of the disruption (e.g. "Planned", "Current")
    pub disruption_status: DisruptionStatus, // TODO: This might want to be a String
    /// Type of disruption
    pub disruption_type: String,
    /// Date and time disruption information is published on PTV website
    #[serde(deserialize_with = "de_rfc3339")]
    pub published_on: NaiveDateTime,
    /// Date and time disruption information was last updated by PTV
    #[serde(deserialize_with = "de_rfc3339")]
    pub last_updated: NaiveDateTime,
    /// Date and time at which disruption begins
    #[serde(deserialize_with = "de_rfc3339")]
    pub from_date: NaiveDateTime,
    /// Date and time at which disruption ends (returns None if unknown)
    #[serde(deserialize_with = "opt_de_rfc3339")]
    pub to_date: Option<NaiveDateTime>,
    /// Route relevant to a disruption (if applicable)
    pub routes: Vec<DisruptionRoute>,
    /// Stop relevant to a disruption (if applicable)
    pub stops: Vec<DisruptionStop>,
    pub colour: String,
    pub display_on_board: bool,
    pub display_status: bool,
}

#[derive(Deserialize, Debug)]
pub struct DisruptionStop {
    #[serde(rename = "stop_id")]
    pub id: i32,
    #[serde(rename = "stop_name")]
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct DisruptionRoute {
    /// Transport mode identifier
    pub route_type: RouteType,
    /// Route identifier
    pub route_id: i32,
    /// Name of route
    pub route_name: String,
    /// Route number presented to public (i.e not route_id)
    pub route_number: String,
    /// Route GTFS identifier
    pub route_gtfs_id: String,
    /// Direction of travel relevant to disruption
    pub direction: Option<DisruptionDirection>,
}

#[derive(Deserialize, Debug)]
pub struct DisruptionDirection {
    /// Route and direction of travel combination identifier
    #[serde(rename = "route_direction_id")]
    pub combination_id: i32,
    /// Direction of travel identifier
    #[serde(rename = "direction_id")]
    pub id: i32,
    /// Name of direction of travel
    #[serde(rename = "direction_name")]
    pub name: String,
    /// Time of service to which disruption applies. Returns None if disruption applies to multiple, or no services
    #[serde(deserialize_with = "de_service_time")]
    pub service_time: Option<NaiveDateTime>,
}

//

#[derive(Serialize, Default)]
pub struct FareEstimateOpts {
    /// Journey touch on
    #[serde(serialize_with = "ser_touch_utc")]
    #[serde(rename = "journey_touch_on_utc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touch_on: Option<NaiveDateTime>,
    /// Journey touch off
    #[serde(serialize_with = "ser_touch_utc")]
    #[serde(rename = "journey_touch_off_utc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub touch_off: Option<NaiveDateTime>,
    #[serde(rename = "is_journey_in_free_tram_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_tram_zone: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traveled_route_types: Option<Vec<RouteType>>,
}

#[derive(Deserialize, Debug)]
pub struct FareEstimateResponse {
    // TODO: This is undefined on the API documentation
}

//

#[derive(Serialize, Default)]
pub struct OutletsOpts {
    /// Maximum number of results returned
    /// (default = 30)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

#[derive(Serialize, Default)]
pub struct OutletsLatLongOpts {
    /// Maximum number of results returned
    /// (default = 30)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,

    /// Maximum distance (in metres) from the specified location
    /// (default = 300)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct OutletsResponse {
    /// Myki ticket outlets
    pub outlets: Vec<Outlet>,
    /// API Status / Metadata
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct Outlet {
    /// The SLID / SPID
    #[serde(rename = "outlet_slid_spid")]
    pub id: i32,
    /// The location name of the outlet
    #[serde(rename = "outlet_name")]
    pub name: String,
    /// The buisness name of the outlet
    #[serde(rename = "outlet_business")]
    pub business: String,
    /// Geographic coordinate of the latitude at outlet
    #[serde(rename = "outlet_latitude")]
    pub latitude: f64,
    /// Geographic coordinate of the longitude at outlet
    #[serde(rename = "outlet_longitude")]
    pub longitude: f64,
    /// The city/municipality of the outlet
    #[serde(rename = "outlet_suburb")]
    pub suburb: String,
    /// The postcode of the outlet
    #[serde(rename = "outlet_postcode")]
    pub postcode: usize,
    /// The business hours on Monday
    #[serde(rename = "outlet_business_hour_mon")]
    pub hours_monday: String,
    /// The business hours on Tuesday
    #[serde(rename = "outlet_business_hour_tue")]
    pub hours_tuesday: String,
    /// The business hours on Wednesday
    #[serde(rename = "outlet_business_hour_wed")]
    pub hours_wednesday: String,
    /// The business hours on Thursday
    #[serde(rename = "outlet_business_hour_thu")]
    pub hours_thursday: String,
    /// The business hours on Friday
    #[serde(rename = "outlet_business_hour_fri")]
    pub hours_friday: String,
    /// The business hours on Saturday
    #[serde(rename = "outlet_business_hour_sat")]
    pub hours_saturday: String,
    /// The business hours on Sunday
    #[serde(rename = "outlet_business_hour_sun")]
    pub hours_sunday: String,
    /// Any additional notes for the outlet such as
    /// 'Buy pre-loaded myki cards only'
    #[serde(rename = "outlet_note")]
    pub note: String,
}

//

#[derive(Serialize, Default)]
pub struct PatternsRunRouteOpts {
    /// List of objects to be returned in full
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<ExpandOptions>>,
    /// Filter by stop_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_id: Option<i32>,
    /// Filter by the date and time of the request
    #[serde(serialize_with = "ser_iso_8601")]
    #[serde(rename = "date_utc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDateTime>,
    /// Include any skipped stops in a stopping pattern
    /// (default = false)
    #[serde(rename = "include_skipped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_skipped: Option<bool>,
    /// Incidates if the route geopath should be returned
    /// (default = false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_geopath: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct PatternResponse {
    /// Disruption information applicable to relevant routes or stops
    pub disruptions: Vec<Disruption>,
    /// Timetabled and real-time service departures
    pub departures: Vec<Departure>,
    /// A train station, tram stop, bus stop, regional coach stop or Night Bus stop
    pub stops: Value, // TODO: T
    /// Train lines, tram routes, bus routes, regional coach routes, Night Bus routes
    pub routes: Value, // TODO: T
    /// Individual trips/services of a route
    pub runs: Value, // TODO: T
    /// Directions of travel of route
    pub directions: Value, // TODO: T
    /// API Status / Metadata
    pub status: Status,
}

#[derive(Serialize, Default)]
pub struct RouteOpts {
    /// Filterered by route_types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_types: Option<Vec<RouteType>>,
    /// Filter by name of route.
    /// Accepts partial route name matches
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_name: Option<String>,
}

#[derive(Serialize, Default)]
pub struct RouteIdOpts {
    /// Indicates kif geopath will be returned (default = false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_geopath: Option<bool>,
    // Filter geopath by date (default = current date)
    #[serde(serialize_with = "ser_iso_8601")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDateTime>,
}

/// This is just documented wrong?
#[derive(Deserialize, Debug)]
pub struct RoutesResponse {
    /// Train lines, tram routes, bus routes, regional coach routes, Night Bus routes
    pub routes: Vec<RouteWithStatus>,
    /// Documented as route: RouteWithStatus?
    /// API Status / Metadata
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct RoutesIdResponse {
    /// Train lines, tram routes, bus routes, regional coach routes, Night Bus routes
    pub route: Option<RouteWithStatus>,
    /// API Status / Metadata
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct RouteWithStatus {
    /// Service status for the route (indicates disruptions)
    #[serde(rename = "route_service_status")]
    pub service_status: RouteServiceStatus,
    /// Transport mode identifier
    pub route_type: RouteType,
    /// Route identifier
    pub route_id: i32,
    /// Name of route
    pub route_name: String,
    /// Route number presented to public (i.e not route_id)
    pub route_number: String,
    /// Route GTFS identifier
    pub route_gtfs_id: String,
    /// Geopath of the route
    pub geopath: Value, // TODO: T
}

#[derive(Deserialize, Debug)]
pub struct RouteServiceStatus {
    pub description: String,
    pub timestamp: String, // TODO: Add a deser. No information in docs.
}

#[derive(Serialize, Default)]
pub struct RunsIdOpts {
    /// List of objects to be returned in full
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<ExpandOptions>>,
    /// Filter by the date and time of the request
    #[serde(serialize_with = "ser_iso_8601")]
    #[serde(rename = "date_utc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDateTime>,
}

#[derive(Serialize, Default)]
pub struct RunsRefOpts {
    /// List of objects to be returned in full
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<ExpandOptions>>,
    /// Filter by the date and time of the request
    #[serde(serialize_with = "ser_iso_8601")]
    #[serde(rename = "date_utc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDateTime>,
    /// Indicates if the route geopath should be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_geopath: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct RunsResponse {
    /// Individual trips/services of a route
    pub runs: Vec<Run>,
    /// API Status / Metadata
    pub status: Status,
}

#[derive(Deserialize, Debug)]
pub struct Run {
    /// Numeric trip/service run identifier.
    /// Defaults to -1 when run identifier is Alphanumeric
    pub run_id: i32,
    /// Alphanumeric trip/service run identifier
    pub run_ref: String,
    /// Route identifier
    pub route_id: i32,
    /// Transport mode identifier
    pub route_type: RouteType,
    /// stop_id of final stop of run
    pub final_stop_id: i32,
    /// Name of destination of run
    pub destination_name: String,
    /// Status of metropolitan train run; returns "scheduled" for other modes
    pub status: String,
    /// Direction of travel identifier
    pub direction_id: i32,
    /// Chronological sequence of the trip/service run on the route in direction
    /// Order ascendingly by this field to get chronological order (earliest first) of runs with the same route_id and direction_id
    ///
    /// What a mouthful
    pub run_sequence: i32,
    // The number of remaining skipped/express stations for the run/service from a stop
    pub express_stop_count: i32,
    // Position of the trip/service run. Available for some Bus, Nightrider and Train runs.
    pub vehicle_position: Option<VehiclePosition>,
    // Descriptor of the trip/service run. Only available for some runs.
    pub vehicle_descriptor: Option<VehicleDescriptor>,
    /// Geopath of the route
    pub geopath: Value, // TODO: T
}

#[derive(Deserialize, Debug)]
pub struct VehiclePosition {
    /// Geographic coordinate of latitude of the vehicle when known.
    pub latitude: Option<f64>,
    /// Geographic coordinate of longitude of the vehicle when known.
    pub longitude: Option<f64>,
    /// CIS - Metro Train Vehicle Location Easting coordinate
    pub easting: Option<f64>,
    /// CIS - Metro Train Vehicle Location Northing coordinate
    pub northing: Option<f64>,
    /// CIS - Metro Train Vehicle Location Direction
    pub direction: Option<String>,
    /// Compass bearing of the vehicle when known, clockwise from True North.
    /// ie. 0 is North and 90 is East
    pub bearing: Option<f32>,
    /// Supplier of the vehicle position data
    pub supplier: String,
    /// Date and time that the vehicle position data was supplied
    #[serde(deserialize_with = "de_iso_8601")]
    #[serde(rename = "datetime_utc")]
    pub datetime: NaiveDateTime,
    /// CIS - Metro Train Vehicle Location data expiry time
    pub expiry_time: Option<String>, // TODO: Add a deser. No information in docs.
}

#[derive(Deserialize, Debug)]
pub struct VehicleDescriptor {
    /// Operator name of the vehicle such as "Metro Trains Melbourne", "Yarra Trams", "Ventura Bus Line", etc.
    /// Only available for some runs.
    pub operator: Option<String>,
    /// Operator identifier of the vehicle. Only available for some runs.
    pub id: Option<String>,
    /// Indicator if the vehicle has a low floor. Only available for some tram runs.
    pub low_floor: Option<bool>,
    /// Indicator if the vehicle is air conditioned. Only available for some tram runs.
    pub air_conditioned: Option<bool>,
    /// Vehicle description such as "6 Car Comeng". Only available for some train runs.
    pub description: Option<String>,
    /// Supplier of the vehicle descriptor data
    pub supplier: String,
    /// The length of the vehicle. Applies to CIS - Metro Trains
    /// Meters? Feet? Who knows.
    pub length: Option<String>,
}
