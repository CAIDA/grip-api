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

use chrono::prelude::DateTime;
use chrono::Utc;
use std::error::Error;
use std::time::{Duration, UNIX_EPOCH};

use elastic::prelude::*;
use lazy_static::lazy_static;
use serde_json::json;
use serde_json::Value;

use crate::backend::errors::MyError;
use regex::Regex;

lazy_static! {
    static ref OLD_FORMAT: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$").unwrap();
}

pub struct ElasticSearchBackend {
    es_client: SyncClient,
}

pub struct SearchResult {
    pub results: Vec<Value>,
    pub total: u64,
}

fn convert_time_str(ts_str: &String) -> String {
    match OLD_FORMAT.is_match(ts_str) {
        true => {
            let ts_vec = ts_str.split("T").collect::<Vec<&str>>();
            format!("{} {}:00", ts_vec[0], ts_vec[1])
        }
        false => ts_str.to_owned(),
    }
}

impl ElasticSearchBackend {
    // constructor static method
    pub fn new(base_url: &str) -> Result<ElasticSearchBackend, Box<dyn Error>> {
        let client = SyncClientBuilder::new().static_node(base_url).build()?;
        Ok(ElasticSearchBackend { es_client: client })
    }

    pub fn get_event_by_id(&self, id: &str) -> Result<SearchResult, Box<dyn Error>> {
        let fields = id.split("-").collect::<Vec<&str>>();
        let event_type: &str = fields[0];
        let view_ts: u64 = fields[1].parse::<u64>().unwrap();

        let d = UNIX_EPOCH + Duration::from_secs(view_ts);
        let datetime = DateTime::<Utc>::from(d);

        let query = format!(
            "http://clayface.caida.org:9200/observatory-events-{}-{}-{:02}/event_result/{}",
            event_type,
            datetime.year(),
            datetime.month(),
            id
        );

        let doc: Value = reqwest::get(query.as_str()).unwrap().json().unwrap();
        if doc["found"] == true {
            let mut document = doc["_source"].clone();
            document["url"] = Value::String(query);
            return Ok(SearchResult {
                results: vec![document],
                total: 1,
            });
        } else {
            Err(Box::new(MyError("Oops".into())))
        }
    }

    pub fn list_events(
        &self,
        event_type: &Option<String>,
        start: &Option<usize>,
        max: &Option<usize>,
        asns: &Option<String>,
        pfxs: &Option<String>,
        ts_start: &Option<String>,
        ts_end: &Option<String>,
        tags: &Option<String>,
        codes: &Option<String>,
        min_susp: &Option<usize>,
        max_susp: &Option<usize>,
        min_duration: &Option<usize>,
        max_duration: &Option<usize>,
        misconf: &Option<bool>,
        misconf_type: &Option<String>,
    ) -> Result<SearchResult, Box<dyn Error>> {
        // event type default to "*"
        let mut etype = "*".to_owned();
        if let Some(et) = event_type {
            etype = match et.as_str() {
                "all" => "*".to_owned(),
                _ => et.to_owned(),
            }
        }

        let mut query_from = 0;
        if let Some(s) = start {
            query_from = s.to_owned() as i32;
        }

        let mut range_filter = json!({"view_ts":{}});
        if let Some(start_str) = ts_start {
            range_filter["view_ts"]["gte"] = json!(convert_time_str(start_str));
        }
        if let Some(end_str) = ts_end {
            range_filter["view_ts"]["lte"] = json!(convert_time_str(end_str));
        }

        let max_entries = match max {
            Some(n) => n.to_owned() as i32,
            None => 100 as i32,
        };

        // match must terms
        let mut must_terms = vec![];
        let mut must_not_terms = vec![];
        must_not_terms.push(json!({ "match": { "position": "FINISHED" } }));

        let mut suspicion_filter = json!({"inference.suspicion.suspicion_level": {}});
        if let Some(max) = max_susp {
            suspicion_filter["inference.suspicion.suspicion_level"]["lte"] =
                json!(max.to_owned() as i32);
        }
        if let Some(min) = min_susp {
            suspicion_filter["inference.suspicion.suspicion_level"]["gte"] =
                json!(min.to_owned() as i32);
        }
        must_terms.push(json!({ "range": suspicion_filter }));

        let mut duration_filter = json!({"duration": {}});
        if let Some(max) = max_duration {
            duration_filter["duration"]["lte"] = json!(max.to_owned() as i32);
        }
        if let Some(min) = min_duration {
            duration_filter["duration"]["gte"] = json!(min.to_owned() as i32);
        }
        must_terms.push(json!({ "range": duration_filter }));

        if let Some(mis) = misconf {
            must_terms.push(json!({"term": {"inference.misconfiguration": mis}}));
            must_not_terms.push(json!({"match":{"tags":"newcomer-is-sibling"}}));
            must_not_terms.push(json!({"match":{"tags":"newcomer-is-friend"}}));
            let mut mistype = "all";
            if let Some(t) = misconf_type {
                mistype = t.as_str();
            }
            match mistype {
                "all" => {}
                "asn_prepend" => {
                    must_terms.push(json!({"term":{"tags":"newcomer-small-asn"}}));
                    must_terms.push(json!({"term":{"tags":"all-newcomers-next-to-an-oldcomer"}}));
                }
                "fatfinger_prefix" => {
                    must_terms.push(json!({"term":{"tags":"prefix-small-edit-distance"}}));
                }
                "fatfinger_asn" => {
                    must_terms.push(json!({"term":{"tags":"origin-small-edit-distance"}}));
                }
                "reserved_space" => {
                    must_terms.push(json!({"term":{"tags":"reserved-space"}}));
                }
                _ => {}
            }
        }

        match pfxs {
            Some(prefixes_string) => {
                let pfx_lst: Vec<&str> = prefixes_string.split(",").collect();
                for pfx in pfx_lst {
                    if pfx.starts_with("!") {
                        // negative match
                        let new_p = pfx.trim_start_matches('!');
                        must_not_terms.push(json!({"term":{"summary.prefixes":new_p}}))
                    } else {
                        must_terms.push(json!({"term":{"summary.prefixes":pfx}}))
                    }
                }
            }
            _ => {}
        }
        match asns {
            Some(asn_string) => {
                let asn_lst: Vec<&str> = asn_string.split(",").collect();
                for asn in asn_lst {
                    if asn.starts_with("!") {
                        // negative match
                        let new_a = asn.trim_start_matches('!');
                        must_not_terms.push(json!({"term":{"summary.ases":new_a}}))
                    } else {
                        must_terms.push(json!({"term":{"summary.ases":asn}}))
                    }
                }
            }
            _ => {}
        }
        match tags {
            Some(tags_string) => {
                let tags_lst: Vec<&str> = tags_string.split(",").collect::<Vec<&str>>();
                for t in tags_lst {
                    if t.starts_with("!") {
                        // negative match
                        let new_t = t.trim_start_matches('!');
                        must_not_terms.push(json!({"term":{"tags":new_t}}))
                    } else {
                        must_terms.push(json!({"term":{"tags":t}}))
                    }
                }
            }
            _ => {}
        }
        match codes {
            Some(codes_string) => {
                let codes_lst: Vec<&str> = codes_string.split(",").collect::<Vec<&str>>();
                for t in codes_lst {
                    if t.starts_with("!") {
                        // negative match
                        let new_t = t.trim_start_matches('!');
                        must_not_terms.push(json!({"term":{"inference.event_codes":new_t}}))
                    } else {
                        must_terms.push(json!({"term":{"inference.event_codes":t}}))
                    }
                }
            }
            _ => {}
        }

        let query: serde_json::Value = json!({
            "from":query_from, "size":max_entries,
            "query": {
                "bool": {
                    "must": must_terms,
                    "must_not": must_not_terms,
                    "filter": {
                        "range": range_filter
                    }
                }
            },
            "sort": { "view_ts": { "order": "desc" }}
        });

        // DEBUG line below

        let res = self
            .es_client
            .search::<Value>()
            .index(format!("observatory-events-{}-*", etype))
            .body(query)
            .send()?;

        // Iterate through the hits in the response and build a vector.
        let mut res_vec: Vec<Value> = Vec::new();
        for hit in res.hits() {
            let mut doc = json!(hit.document().unwrap());
            doc["_esid"] = json!(hit.index().to_owned().to_string());
            res_vec.push(doc);
        }

        Ok(SearchResult {
            results: res_vec,
            total: res.total(),
        })
    }
}
