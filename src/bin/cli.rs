use hijacks_dashboard::backend::data::process_raw_event;
use serde_json::{Value,json};
use hijacks_dashboard::backend::elastic::ElasticSearchBackend;

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
        ).unwrap();

    let res_data: Vec<Value> = query_result
        .results
        .iter()
        .map(|v| process_raw_event(v, false, false))
        .collect();

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