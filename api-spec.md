# Basic Information

  - Base API URL: <https://api.grip.caida.org/dev/>
  - Current Prototype Web Interface:
    <https://grip-dev.caida.org>

# Events Data

## Search By Events

Query for a list of events by time, type, ASNs, prefixes, tags,
suspicion level, or event durations. The query returns a list of JSON
objects, each containing a matching detected event.

### Endpoint

<https://api.grip.caida.org/dev/json/events>

Example:

  - Default front-page query:
      - [example query](https://api.grip.caida.org/dev/json/events?length=10&start=0&ts_start=2020-08-09T19%3A03&ts_end=2020-08-10T19%3A03&min_susp=80&max_susp=100&event_type=moas)
      - Search for 10 suspicious MOAS events during one-day period
        (August 9 to August 10).

### Query parameters (none required)

| parameter      | default | type | range/format/example                    | definition                                               |
| -------------- | ------- | ---- | --------------------------------------- | -------------------------------------------------------- |
| `event_type`   | "all"   | str  | "moas","submoas","defcon","edges","all" | event type                                               |
| `ts_start`     | \-inf   | str  | "YYYY-MM-DDTHH:MM:SS"                   | UTC timestamp of the start of the event                  |
| `ts_end`       | \+inf   | str  | "YYYY-MM-DDTHH:MM:SS"                   | UTC timestamp of the end of the event                    |
| `start`        | 0       | int  | 0 – +inf                                | starting index (used for pagination)                     |
| `length`       | 100     | int  | 1 – 1000                                | the number of events should return                       |
| `asns`         | ""      | str  | e.g. `213,456`                          | list of AS numbers formatted as `,` separated string     |
| `tags`         | ""      | str  | e.g. `tag1,tag2`                        | list of event tags formatted as `,` separated string     |
| `pfxs`         | ""      | str  | e.g. `8.8.8.0/24,1.1.1.0/24`            | list of event prefixes formatted as `,` separated string |
| `min_susp`     | 0       | int  | 0 – 100                                 | minimum suspicion levels                                 |
| `max_susp`     | 100     | int  | 0 – 100                                 | maximum suspicion levels                                 |
| `min_duration` | 0       | int  | 0 – +inf                                | minimum event duration in seconds                        |
| `max_duration` | \+inf   | int  | 0 – +inf                                | maximum event duration in seconds                        |
| `full`         | false   | bool | true/false                              | whether to export full events including AS paths         |

## Event Details

Retrieve detail information for a specific event by ID.

### Endpoint:

<https://api.grip.caida.org/dev/json/event/id/>

Example:
<https://api.grip.caida.org/dev/json/event/id/moas-1605039900-207384_48282>

## Prefix Event Details

Retrieve detail information for a specific prefix event (part of a
event) by ID.

### Encode prefixes

We encode the prefixes by replacing `/` with `-`. For example,
`185.228.169.0/24` will be encoded as `185.228.169.0-24`.

### Endpoint

<https://api.grip.caida.org/dev/json/pfx_event/id/> where `pfxs` are
encoded prefixes in the event separated with `_`

Example:

<https://api.grip.caida.org/dev/json/pfx_event/id/moas-1605039900-207384_48282/178.208.78.0-24>

## JSON Responses

### Search results

  - `data`: data portion include a list of event objects. Each event
    object also contains a list of prefix event objects. **Details of
    the prefix events and traceroutes results are excluded in search
    results.**

  - `recordsFiltered` `recordsTotal`: usually the same, total number of
    records from the search. Currently, the system has a upper limit of
    10,000 records in searching.

### Event object

  - `id`: event ID
      - **this can be used in event details end-point to retrieve more
        detailed information**
  - `duration`: duration of the events in seconds, null if event is
    still ongoing
  - `event_type`: type of the event
  - `view_ts`: event time in unix time format
  - `finished_ts`: event finished time, null if still ongoing
  - `external`: data extracted from external sources (e.g. ASRank, and
    IIJ Hegemony Score)
  - `summary`: information summarized from the prefix events of this
    event
      - `ases`: ASes involved in the event
      - `prefixes`: prefixes involved in the event
      - `tr_worthy`: whether the event is traceroute worthy
      - `tags`: list of tags from all prefix events
      - `attackers` and `victims`: inferred potential attackers and
        victims of the event
      - `inference_result`: inference result for the event
          - `inferences` list of all inferences extracted from the
            prefix events
              - `inference_id`: name of the inference
              - `suspicion_level`: suspicion level of the prefix event
                from this inference
              - `confidence`: confidence level
              - `explanation`: explanation of this inference
              - `labels`: extra labels of the inference for grouping and
                searching
          - `primary_inference`: the main inference from the list of all
            inferences, highest confidence and highest
            suspicion<sub>level</sub>
  - `pfx_events`: list of prefix events objects (**as-paths excluded if
    `full` parameter is not true**)
  - `tr_metrics`: various counting metrics related to traceroute
    measurements
  - `event_metrics`: various counting metrics for the event itself
      - `per_tag_cnt`: count of number of prefixes that have each tag
      - `pfx_event_cnt`: number of prefix events
      - `pfx_event_with_tr_cnt`: number of prefix events that has
        traceroute results
      - `proc_time_driver`: time spent on active driver
      - `proc_time_inference`: time spent on inference
      - `proc_time_tagger`: time spent on tagging
      - `total_tags_cnt`: total number of tags generated for all prefix
        events, including duplicated tags among them

### Prefix event object

  - `prefix`, `sub_pfx`, `super_pfx`: prefixes involved in the events
  - `tags`: tags for the current prefix event
  - `tr_worthy`: whether the prefix event is traceroute-worhty
  - `tr_available`: whether the prefix event has corresponding
    traceroute results. note that not all traceroute worthy events have
    corresponding results, due to rate-limiting and other various
    reasons.
  - `traceroutes`: traceroute measurement results
  - `details`: more details about the prefix event
      - `new_origins`: the newcomer origin of the prefix
      - `old_origins`: the previous origin of the prefix
      - `origins`: all origins in the event
      - `as_path`: all as paths related to this event, collected from
        all collectors
  - `inferences` list of all inferences extracted from the prefix events
      - `inference_id`: name of the inference
      - `suspicion_level`: suspicion level of the prefix event from this
        inference
      - `confidence`: confidence level
      - `explanation`: explanation of this inference
      - `labels`: extra labels of the inference for grouping and
        searching

# Other Datasets

## `blocklist` and `asndrop`: ASes with Known Bad Reputation

### Endpoint

  - <https://api.grip.caida.org/dev/json/blocklist>
  - <https://api.grip.caida.org/dev/json/asndrop>

### Data format

  - `blocklist`: list of AS numbers
  - `asndrop`: list of AS numbers on Spamhaus
    [ASN-DROP](https://www.spamhaus.org/drop/asndrop.txt) list

## `tags`: List of All Event Tags

### **Endpoint**

<https://api.grip.caida.org/dev/json/tags>

### **Data format**

  - `definitions`:
    
      - `definition`: definition of the tag
      - `comments`: list of comments for the tag

  - `tr_worthy`: list of tag combinations and whether they are worthy of
    doing traceroutes
    
      - `apply_to`: types of events the code applies to, empty means it
        applies to all types
    
      - `explain`: reason for the worthiness
    
      - `tags`: list of tags
    
      - `worthy`: traceroute-worthiness of the tag combination
        
          - `yes`: the event is worthy of doing a traceroute
          - `no`: the event is not worthy of doing a traceroute
          - `na`: unknown nature
