use grip_api::backend::data::process_raw_event;
use grip_api::backend::elastic::ElasticSearchBackend;
use serde_json::{json, Value};

use clap::Clap;

#[derive(Debug, Clap)]
#[clap()]
struct Opts {
    /// Event type to search for
    #[clap(long)]
    event_type: Option<String>,
    /// Starting index for pagination
    #[clap(long)]
    start: Option<usize>,
    /// Maximum number of events to find
    #[clap(long)]
    max: Option<usize>,
    /// Event asns, comma-separated string
    #[clap(long)]
    asns: Option<String>,
    /// Event prefixes, comma-separated string
    #[clap(long)]
    pfxs: Option<String>,
    /// Minimum start timestamp of for an event
    #[clap(long)]
    ts_start: Option<String>,
    /// Maximum  start timestamp of for an event
    #[clap(long)]
    ts_end: Option<String>,
    /// Event tags, comma-separated string
    #[clap(long)]
    tags: Option<String>,
    /// Event codes, comma-separated string
    #[clap(long)]
    codes: Option<String>,
    /// Minimum suspicion level of an event
    #[clap(long)]
    min_susp: Option<isize>,
    /// Maximum suspicion level of an event
    #[clap(long)]
    max_susp: Option<isize>,
    /// Minimum duration of an event
    #[clap(long)]
    min_duration: Option<usize>,
    /// Maximum duration of an event
    #[clap(long)]
    max_duration: Option<usize>,
    /// Pretty print the resulting JSON object
    #[clap(short, long)]
    pretty_print: bool,
    /// Slimmed-down version of the JSON object
    #[clap(short, long)]
    slim: bool,
    /// Return full events including traceroutes and AS paths
    #[clap(short, long)]
    full: bool,
    /// Whether to include events that overlaps with the time range in query
    #[clap(short, long)]
    overlap: bool,
    /// Count matches only
    #[clap(short, long)]
    count: bool,
}

/// Convert a raw object to a slim object
fn slim_result(value: &Value) -> Value {
    let mut event = json!({});
    // filter easy fields
    for field in vec!["id", "event_type", "view_ts", "finished_ts", "summary"] {
        event[field] = value[field].to_owned();
    }

    event["url"] = json!(format!(
        "https://dev.hicube.caida.org/feeds/hijacks/events/{}/{}",
        value["event_type"].as_str().unwrap(), value["id"].as_str().unwrap()
    ));

    if let Some(asrank) = value["external"].get("asrank") {
        event["asinfo"] = asrank.to_owned();
    }

    event
}

fn search(opts: &Opts) -> Value {

    let backend = ElasticSearchBackend::new("http://clayface.caida.org:9200").unwrap();
    let query_result = backend
        .list_events(
            &opts.event_type,
            &opts.start,
            &opts.max,
            &opts.asns,
            &opts.pfxs,
            &opts.ts_start,
            &opts.ts_end,
            &opts.tags,
            &opts.codes,
            &opts.min_susp,
            &opts.max_susp,
            &opts.min_duration,
            &opts.max_duration,
            opts.overlap,
        )
        .unwrap();
    let res_iter = query_result.results.iter();
    let res_data: Vec<Value> = match opts.slim {
        true => res_iter.map(|v| slim_result(v)).collect::<Vec<Value>>(),
        false => res_iter
            .map(|v|
                 match opts.full {
                     true => {v.to_owned()}
                     false => {process_raw_event(v, opts.full, opts.full)}
                 }
            )
            .collect::<Vec<Value>>(),
    };
    json!(
        {
            "cnt_total": query_result.total,
            "cnt_returned": res_data.len(),
            "data": res_data,
        }
    )
}

fn count(opts: &Opts) -> Value {

    let backend = ElasticSearchBackend::new("http://clayface.caida.org:9200").unwrap();
    let query_result = backend
        .count_events(
            &opts.event_type,
            &opts.asns,
            &opts.pfxs,
            &opts.ts_start,
            &opts.ts_end,
            &opts.tags,
            &opts.codes,
            &opts.min_susp,
            &opts.max_susp,
            &opts.min_duration,
            &opts.max_duration,
            opts.overlap,
        )
        .unwrap();
    json!({"count":query_result.count})
}


fn main() {
    let opts: Opts = Opts::parse();

    let object = match &opts.count {
        true => {
            count(&opts)
        },
        false => {
            search(&opts)
        }
    };

    if opts.pretty_print {
        println!("{}", serde_json::to_string_pretty(&object).unwrap());
    } else {
        println!("{}", serde_json::to_string(&object).unwrap());
    }
}
