// This software is Copyright (c) 2018 The Regents of the University of
// California. All Rights Reserved. Permission to copy, modify, and distribute this
// software and its documentation for academic research and education purposes,
// without fee, and without a written agreement is hereby granted, provided that
// the above copyright notice, this paragraph and the following three paragraphs
// appear in all copies. Permission to make use of this software for other than
// academic research and education purposes may be obtained by contacting:
//
// Office of Innovation and Commercialization
// 9500 Gilman Drive, Mail Code 0910
// University of California
// La Jolla, CA 92093-0910
// (858) 534-5815
// invent@ucsd.edu
//
// This software program and documentation are copyrighted by The Regents of the
// University of California. The software program and documentation are supplied
// "as is", without any accompanying services from The Regents. The Regents does
// not warrant that the operation of the program will be uninterrupted or
// error-free. The end-user understands that the program was developed for research
// purposes and is advised not to rely exclusively on the program for any reason.
//
// IN NO EVENT SHALL THE UNIVERSITY OF CALIFORNIA BE LIABLE TO ANY PARTY FOR
// DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES, INCLUDING LOST
// PROFITS, ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF
// THE UNIVERSITY OF CALIFORNIA HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
// DAMAGE. THE UNIVERSITY OF CALIFORNIA SPECIFICALLY DISCLAIMS ANY WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS FOR A PARTICULAR PURPOSE. THE SOFTWARE PROVIDED HEREUNDER IS ON AN "AS
// IS" BASIS, AND THE UNIVERSITY OF CALIFORNIA HAS NO OBLIGATIONS TO PROVIDE
// MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.

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
    /// Return full events including traceroutes and AS paths
    #[clap(short, long)]
    full: bool,
    /// Whether to include events that overlaps with the time range in query
    #[clap(short, long)]
    overlap: bool,
    /// Count matches only
    #[clap(short, long)]
    count: bool,
    /// Retrieve brief information only
    #[clap(short, long)]
    brief: bool,
    /// Retrieve events from debug indcies
    #[clap(short, long)]
    debug: bool,
}

/// Convert a raw object to a brief object
fn brief_result(value: &Value) -> Value {
    let mut event = json!({});
    // filter easy fields
    for field in vec![
        "id",
        "event_type",
        "view_ts",
        "finished_ts",
        "insert_ts",
        "last_modified_ts",
        "summary",
    ] {
        event[field] = value[field].to_owned();
    }

    event["url"] = json!(format!(
        "https://dev.hicube.caida.org/feeds/hijacks/events/{}/{}",
        value["event_type"].as_str().unwrap(),
        value["id"].as_str().unwrap()
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
            opts.brief,
            opts.debug,
        )
        .unwrap();
    let res_iter = query_result.results.iter();
    let res_data: Vec<Value> = match opts.brief {
        true => res_iter.map(|v| brief_result(v)).collect::<Vec<Value>>(),
        false => res_iter
            .map(|v| match opts.full {
                true => v.to_owned(),
                false => process_raw_event(v, opts.full, opts.full, true),
            })
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
            opts.debug,
        )
        .unwrap();
    json!({"count":query_result.count})
}

fn main() {
    let opts: Opts = Opts::parse();

    let object = match &opts.count {
        true => count(&opts),
        false => search(&opts),
    };

    if opts.pretty_print {
        println!("{}", serde_json::to_string_pretty(&object).unwrap());
    } else {
        println!("{}", serde_json::to_string(&object).unwrap());
    }
}
