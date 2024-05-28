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
- [X] redis request caching
- [ ] otel open telemetry tracing
- [X] http server
- [ ] websocket messages for:
  - [X] car_data
  - [X] intervals
  - [X] team_radio
  - [X] laps
  - [X] pit
  - [X] position
  - [X] stints
  - [ ] session

Note: more endponts might be required to make metadata available for filtering
the above endpoints.


### How to know when redis cache has been updated?
- somehow pubsub redis keys? https://redis.io/docs/latest/develop/use/keyspace-notifications/
- use channel between websocket server, and event_sync that sends a message from event_sync to server that a key has been updated.


### Notes

started getting this error with position message passing:
```[2024-05-28T12:59:08Z ERROR formula_one_telemetry::algebras::channel_queue] failed to send Events message: channel closed```

^ could drop changes and see what's wrong
