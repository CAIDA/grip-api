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

use serde::Serialize;
use serde_json::Value;

use clap::Clap;

#[derive(Debug, Clap, Serialize)]
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
    length: Option<usize>,
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

macro_rules! push_param {
    ($opts: expr, $dst: expr, $params: expr) => {
        let data = serde_json::to_value($opts).unwrap();
        for param in $params.iter() {
            if let Some(d) = data.get(param) {
                match d {
                    Value::String(x) => {
                        $dst.push(format!("{}={}", param, x));
                    }
                    Value::Number(x) => {
                        $dst.push(format!("{}={}", param, x));
                    }
                    Value::Bool(x) => {
                        if *x {
                            $dst.push(format!("{}", param));
                        }
                    }
                    _ => {}
                }
            }
        }
    };
}

fn construct_parameters(opts: &Opts) -> String {
    let mut params: Vec<String> = vec![];
    push_param!(
        opts,
        params,
        [
            "event_type",
            "start",
            "length",
            "asns",
            "pfxs",
            "ts_start",
            "ts_end",
            "tags",
            "codes",
            "min_susp",
            "max_susp",
            "min_duration",
            "max_duration",
            "overlap",
            "brief",
            "full",
        ]
    );
    params.join("&")
}

fn search(base_url: &str, opts: &Opts) -> Value {
    let params = construct_parameters(&opts);
    let res = reqwest::get(format!("{}?{}", base_url, params).as_str());
    res.unwrap().json().unwrap()
}

fn main() {
    let opts: Opts = Opts::parse();
    let base_url =
        std::env::var("API_BASE_URL").unwrap_or("https://api.grip.caida.org/dev".to_string());

    let search_url = format!("{}/json/events", base_url);
    let object = search(search_url.as_str(), &opts);
    if opts.pretty_print {
        println!("{}", serde_json::to_string_pretty(&object).unwrap());
    } else {
        println!("{}", serde_json::to_string(&object).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params() {
        let opts = Opts {
            event_type: Some("moas".to_string()),
            start: None,
            length: Some(1),
            asns: None,
            pfxs: None,
            ts_start: None,
            ts_end: None,
            tags: None,
            codes: None,
            min_susp: None,
            max_susp: None,
            min_duration: None,
            max_duration: None,
            pretty_print: false,
            full: false,
            overlap: false,
            count: false,
            brief: true,
            debug: false,
        };

        let res = construct_parameters(&opts);

        assert_eq!(res, "event_type=moas&length=1&brief");
        let v = search("https://api.grip.caida.org/dev/json/events", &opts);
        println!("{}", serde_json::to_string_pretty(&v).unwrap());
    }
}
