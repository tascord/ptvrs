use anyhow::Result;
use hmac::{Hmac, Mac};
use serde::de::DeserializeOwned;
use sha1::Sha1;

pub const API_URL: &str = "https://timetableapi.ptv.vic.gov.au";

type PtvHmac = Hmac<Sha1>;

pub mod helpers;
pub use helpers::*;
pub mod ty;
pub use ty::*;

pub struct Client {
    devid: String,
    key: String,
}

impl Client {
    pub fn new(devid: String, key: String) -> Client {
        Client { devid, key }
    }

    async fn rq<T: DeserializeOwned>(&self, path: String) -> Result<T> {
        let path = format!(
            "/{path}{}devid={}",
            {
                if path.ends_with('?') {
                    ""
                } else {
                    "&"
                }
            },
            self.devid
        );

        let mut hasher: PtvHmac = Hmac::new_from_slice(self.key.as_bytes()).unwrap();
        hasher.update(path.as_bytes());

        let hash = hex::encode(hasher.finalize().into_bytes());
        let url = format!("{API_URL}{}&signature={}", path, hash);

        println!("{}", path);
        println!("{}, {}", self.devid, self.key);
        println!("{url}");
        let res = reqwest::get(&url).await?;
        if !res.status().is_success() {
            return Err(anyhow::anyhow!("Request failed: {}", res.status()));
        }

        Ok(res.json().await?)
    }

    /* > Departures */

    /// View departures for all routes from a specific stop
    pub async fn departures_stop(
        &self,
        route_type: RouteType,
        stop_id: i32,
        options: DeparturesStopOps,
    ) -> Result<DeparturesResponse> {
        self.rq(format!(
            "v3/departures/route_type/{}/stop/{}?{}",
            route_type.as_number(),
            stop_id,
            to_query(options)
        ))
        .await
    }

    /// View departures for a specic route from a stop
    pub async fn departures_stop_route(
        &self,
        route_type: RouteType,
        route_id: i32,
        stop_id: i32,
        options: DeparturesStopOps,
    ) -> Result<DeparturesResponse> {
        self.rq(format!(
            "v3/departures/route_type/{}/route/{}/stop/{}?{}",
            route_type.as_number(),
            route_id,
            stop_id,
            to_query(options)
        ))
        .await
    }

    /* > Directions */

    /// View directions that a route travels in
    pub async fn directions_route(&self, route_id: i32) -> Result<DirectionsResponse> {
        self.rq(format!("v3/directions/route/{}", route_id)).await
    }

    /// View all routes for a direction of travel
    pub async fn directions_id(&self, direction_id: i32) -> Result<DirectionsResponse> {
        self.rq(format!("v3/directions/{}", direction_id)).await
    }

    /// View all routes of a particular type for a direction of travel
    pub async fn directions_id_route(
        &self,
        direction_id: i32,
        route_type: RouteType,
    ) -> Result<DirectionsResponse> {
        self.rq(format!(
            "v3/directions/{}/route_type/{}",
            direction_id,
            route_type.as_number()
        ))
        .await
    }

    /* > Disruptions */

    /// View all disruptions for all route types
    pub async fn disruptions(&self, options: DisruptionsOpts) -> Result<DisruptionsResponse> {
        self.rq(format!("v3/disruptions?{}", to_query(options)))
            .await
    }

    /// View all disruptions for a particular route
    pub async fn disruptions_route(
        &self,
        route_id: i32,
        options: DisruptionStatusOpts,
    ) -> Result<DisruptionsResponse> {
        self.rq(format!(
            "v3/disruptions/route/{}?{}",
            route_id,
            to_query(options)
        ))
        .await
    }

    /// View all disruptions for a particular route and stop
    pub async fn disruptions_route_stop(
        &self,
        route_id: i32,
        stop_id: i32,
        options: DisruptionStatusOpts,
    ) -> Result<DisruptionsResponse> {
        self.rq(format!(
            "v3/disruptions/route/{}/stop/{}?{}",
            route_id,
            stop_id,
            to_query(options)
        ))
        .await
    }

    /// View all disruptions for a particular stop
    pub async fn disruptions_stop(
        &self,
        stop_id: i32,
        options: DisruptionStatusOpts,
    ) -> Result<DisruptionsResponse> {
        self.rq(format!(
            "v3/disruptions/stop/{}?{}",
            stop_id,
            to_query(options)
        ))
        .await
    }

    /// View a specific disruption
    pub async fn disruptions_id(&self, disruption_id: i32) -> Result<Disruption> {
        // TODO: Technically this has Status too but I dont want to
        // dupe the struct 17 times
        self.rq(format!("v3/disruptions/{}", disruption_id)).await
    }

    /* > Fare Estimate */

    /// Estimate a fare by zone
    pub async fn fare_estimate(
        &self,
        min_zone: u8,
        max_zone: u8,
        options: FareEstimateOpts,
    ) -> Result<FareEstimateResponse> {
        self.rq(format!(
            "v3/fare_estimate/min_zone/{}/max_zone/{}?{}",
            min_zone,
            max_zone,
            to_query(options)
        ))
        .await
    }

    /* > Outlets */

    /// Last all ticket outlets
    pub async fn outlets(&self, options: OutletsOpts) -> Result<OutletsResponse> {
        self.rq(format!("v3/outlets?{}", to_query(options))).await
    }

    /// List outlets near a specific location
    pub async fn outlets_lat_long(
        &self,
        latitude: f64,
        longitude: f64,
        options: OutletsLatLongOpts,
    ) -> Result<OutletsResponse> {
        self.rq(format!(
            "v3/outlets/location/{}/{}?{}",
            latitude,
            longitude,
            to_query(options)
        ))
        .await
    }

    /* > Patterns */

    /// View the stopping pattern for a specific tip / service run
    pub async fn patterns_run_route(
        &self,
        run_ref: String,
        route_type: RouteType,
        options: PatternsRunRouteOpts,
    ) -> Result<PatternResponse> {
        self.rq(format!(
            "v3/pattern/run/{}/route_type/{}?{}",
            run_ref,
            route_type.as_number(),
            to_query(options)
        ))
        .await
    }

    /* > Routes */

    /// View route names and numbers for all routes
    pub async fn routes(&self, options: RouteOpts) -> Result<RoutesResponse> {
        self.rq(format!("v3/routes?{}", to_query(options))).await
    }

    // View route name and number for a specific route ID
    pub async fn routes_id(&self, route_id: i32, options: RouteIdOpts) -> Result<RoutesIdResponse> {
        self.rq(format!("v3/routes/{}?{}", route_id, to_query(options)))
            .await
    }

    /* > Runs */

    /// View all trip/service runs for a specific route ID
    pub async fn runs_id(
        &self,
        run_id: i32,
        options: RunsIdOpts,
    ) -> Result<RunsResponse> {
        self.rq(format!(
            "v3/runs/route/{}?{}",
            run_id,
            to_query(options)
        ))
        .await
    }

    /// View all trip/service runs for a specific run ID and route type
    pub async fn runs_id_type(
        &self,
        run_id: i32,
        route_type: RouteType,
        options: RunsIdOpts,
    ) -> Result<RunsResponse> {
        self.rq(format!(
            "v3/runs/route/{}/route_type/{}?{}",
            run_id,
            route_type.as_number(),
            to_query(options)
        ))
        .await
    }

    /// View all trip/service runs for a specific run_ref
    pub async fn runs_ref(
        &self,
        run_ref: String,
        options: RunsRefOpts,
    ) -> Result<RunsResponse> {
        self.rq(format!(
            "v3/runs/{}?{}",
            run_ref,
            to_query(options)
        ))
        .await
    }

    /// View all trip/service runs for a specific run_ref and route type
    pub async fn runs_ref_type(
        &self,
        run_ref: String,
        route_type: RouteType,
        options: RunsRefOpts,
    ) -> Result<RunsResponse> {
        self.rq(format!(
            "v3/runs/{}/route_type/{}?{}",
            run_ref,
            route_type.as_number(),
            to_query(options)
        ))
        .await
    }
}
