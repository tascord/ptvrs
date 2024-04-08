<img width="200" height="200" align="left" style="float: left; margin: 0 10px 0 0;" alt="Icon" src="https://github.com/tascord/ptvrs/blob/main/icon.png?raw=true"> 

# PTV (rs)
## Public transport Victoria's API in rust

[![GitHub top language](https://img.shields.io/github/languages/top/tascord/ptvrs?color=0072CE&style=for-the-badge)](#)
[![Crates.io Version](https://img.shields.io/crates/v/ptv?style=for-the-badge)](https://crates.io/crates/ptv)
[![docs.rs](https://img.shields.io/docsrs/ptv?style=for-the-badge)](https://docs.rs/ptv)

## Status
🟩 ; Complete, 🟦 ; To be tested ([you can help!](https://github.com/tascord/ptvrs/issues/new)), 🟨 ; Needs work, 🟥 ; Avoid use in current state ; ❌ Not implemented, yet.
| Feature           | Endpoint                                                                                                                     | Status | Notes                             |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------- |
| **Departures**    | [/departures/route_type/stop/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.departures_stop)                   | 🟦      |                                   |
|                   | [/departures/route_type/{}/stop/{}/route/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.departures_stop_route) | 🟦      |                                   |
| **Directions**    | [/directions/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.directions_id)                                     | 🟦      |                                   |
|                   | [/directions/route/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.directions_route)                            | 🟦      |                                   |
|                   | [/directions/{}/route_type/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.directions_id_route)                 | 🟦      |                                   |
| **Disruptions**   | [/disruptions](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.disruptions)                                         | 🟦      |                                   |
|                   | [/disruptions/route/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.disruptions_route)                          | 🟦      |                                   |
|                   | [/disruptions/route/{}/stop/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.disruptions_route_stop)             | 🟦      |                                   |
|                   | [/disruptions/stop/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.disruptions_stop)                            | 🟦      |                                   |
| **Disruptions**   | [/disruptions/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.disruptions_id)                                   | 🟦      |                                   |
| **Fare Estimate** | [/fare_estimate/min_zone/{}/max_zone/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.fare_estimate)             | 🟥      | Not enough docs.                  |
| **Outlets**       | [/outlets](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.outlets)                                                 | 🟦      |                                   |
|                   | [/outlets/location/{}/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.outlets_lat_long)                         | 🟦      |                                   |
| **Patterns**      | [/pattern/run/{}/route_type/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.patterns_run_route)                 | 🟦      |                                   |
| **Routes**        | [/routes](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.routes)                                                   | 🟨      | Types not yet concrete. See docs. |
|                   | [/routes/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.routes_id)                                             | 🟨      | "                                 |
| **Runs**          | [/runs/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.runs_ref)                                                | 🟨      | "                                 |
|                   | [/runs/route/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.runs_id)                                           | 🟨      | "                                 |
|                   | [/runs/{}/route_type/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.runs_ref_type)                             | 🟨      | "                                 |
|                   | [/runs/route/{}/route_type/{}](https://docs.rs/ptv/latest/ptv/struct.Client.html#method.runs_id_type)                        | 🟨      | "                                 |
| **Search**        | /search/{}                                                                                                                   | ❌      | Not implemented                   |
| **Stops**         | /stops/{}/route_type/{}                                                                                                      | ❌      | "                                 |
|                   | /stops/route/{}/route_type/{}                                                                                                | ❌      | "                                 |
|                   | /stops/location/{}/{}                                                                                                        | ❌      | "                                 |
