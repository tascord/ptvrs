<img width="200" height="200" align="left" style="float: left; margin: 0 10px 0 0;" alt="Icon" src="https://github.com/tascord/ptvrs/blob/main/icon.png?raw=true"> 

# PTV (rs)
## Public transport Victoria's API in rust

[![GitHub top language](https://img.shields.io/github/languages/top/tascord/ptvrs?color=0072CE&style=for-the-badge)](#)
[![Crates.io Version](https://img.shields.io/crates/v/ptv?style=for-the-badge)](https://crates.io/crates/ptv)
[![docs.rs](https://img.shields.io/docsrs/ptv?style=for-the-badge)](https://docs.rs/ptv)

## Status
🟩 ; Complete, 🟦 ; To be tested ([you can help!](https://github.com/tascord/ptvrs/issues/new)), 🟨 ; Needs work, 🟥 ; Avoid use in current state
| Feature           | Endpoint<br>                           | Status | Notes                                 |
|-------------------|----------------------------------------|--------|---------------------------------------|
| **Departures**    | /departures/stop/{}<br>                | 🟨     | Types not yet concrete. See docs.<br> |
|                   | /departures/route_type/{}/stop/{}      | 🟨     | "                                     |
| **Directions**    | /directions/{}                         | 🟦     |                                       |
|                   | /directions/route/{}                   | 🟦     |                                       |
|                   | /directions/{}/route_type/{}           | 🟦     |                                       |
| **Disruptions**   | /disruptions/route/{}                  | 🟦     |                                       |
|                   | /disruptions/route/{}/stop/{}          | 🟦     |                                       |
|                   | /disruptions/stop/{}                   | 🟦     |                                       |
|                   | /disruptions/{}                        | 🟦     |                                       |
| **Fare Estimate** | /fare_estimate/min_zone/{}/max_zone/{} | 🟥     | Not enough docs.<br>                  |
| **Outlets**       | /outlets                               | 🟦     |                                       |
|                   | /outlets/location/{}/{}                | 🟦     |                                       |
| **Patterns**      | /pattern/run/{}/route_type/{}          | 🟦     |                                       |
| **Routes**        | /routes                                | 🟨     | Types not yet concrete. See docs.     |
|                   | /routes/{}                             | 🟨     | "                                     |
| **Runs**          | /runs/route/{}/route_type/{}           | 🟨     | "                                     |
|                   | /runs/{}                               | 🟨     | "                                     |
|                   | /runs/{}/route_type/{}                 | 🟨     | "                                     |
|                   | /runs/route/{}                         | 🟨     | "                                     |
