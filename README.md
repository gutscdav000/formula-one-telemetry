# formula-one-telemetry

Currently a work in progress using the [openf1.org](https://openf1.org/) to aggregate real time race data. When complete this is intended to be a highly concurrent data streaming application. The impetus for the project is to learn rust, and specifically understand it's concurrency model.

## api requests supported: (13/13)
- car data
- drivers 
- intervals
- laps
- location
- meetings
- pit
- position
- race controls
- sessions
- stints
- team radio
- weather


## Desired Feature List
- redis request caching
- otel open telemetry tracing
- http server
- http endpoints for:
  - car_data
  - intervals
  - team_radio
  - laps
  - pit
  - position
  - stints

Note: more endponts might be required to make metadata available for filtering
the above endpoints. 
