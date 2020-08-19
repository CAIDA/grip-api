use hijacks_dashboard::backend::data::process_raw_event;
use hijacks_dashboard::backend::elastic::ElasticSearchBackend;
use serde_json::{json, Value};

use clap::Clap;

#[derive(Debug, Clap)]
#[clap()]
struct Opts {
    #[clap(long)]
    event_type: Option<String>,
    #[clap(long)]
    start: Option<usize>,
    #[clap(long)]
    max: Option<usize>,
    #[clap(long)]
    asns: Option<String>,
    #[clap(long)]
    pfxs: Option<String>,
    #[clap(long)]
    ts_start: Option<String>,
    #[clap(long)]
    ts_end: Option<String>,
    #[clap(long)]
    tags: Option<String>,
    #[clap(long)]
    codes: Option<String>,
    #[clap(long)]
    min_susp: Option<usize>,
    #[clap(long)]
    max_susp: Option<usize>,
    #[clap(long)]
    min_duration: Option<usize>,
    #[clap(long)]
    max_duration: Option<usize>,
    #[clap(short, long)]
    pretty_print: bool,
    #[clap(short, long)]
    slim: bool,
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

fn main() {
    let opts: Opts = Opts::parse();
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
        )
        .unwrap();

    let res_iter = query_result.results.iter();
    let res_data: Vec<Value> = match opts.slim {
        true => res_iter.map(|v| slim_result(v)).collect::<Vec<Value>>(),
        false => res_iter
            .map(|v| process_raw_event(v, false, false))
            .collect::<Vec<Value>>(),
    };

    let object = json!(
        {
            "cnt_total": query_result.total,
            "cnt_returned": res_data.len(),
            "data": res_data,
        }
    );

    if opts.pretty_print {
        println!("{}", serde_json::to_string_pretty(&object).unwrap());
    } else {
        println!("{}", serde_json::to_string(&object).unwrap());
    }
}
