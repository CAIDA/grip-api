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
use std::collections::HashMap;

use elastic::prelude::*;
use serde_json::json;
use serde_json::Value;

use crate::backend::errors::MyError;
use crate::backend::utils::*;

pub struct ElasticSearchBackend {
    es_client: SyncClient,
}

pub struct SearchResult {
    pub results: Vec<Value>,
    pub total: u64,
}

pub struct CountResult {
    pub count: u64,
}

impl ElasticSearchBackend {
    // constructor static method
    pub fn new(base_url: &str) -> Result<ElasticSearchBackend, Box<dyn Error>> {
        let client = SyncClientBuilder::new().static_node(base_url).build()?;
        Ok(ElasticSearchBackend { es_client: client })
    }

    pub fn get_event_by_id(&self, id: &str) -> Result<Value, Box<dyn Error>> {
        let fields = id.split("-").collect::<Vec<&str>>();
        let event_type: &str = fields[0];
        let view_ts: u64 = fields[1].parse::<u64>().unwrap();

        let d = UNIX_EPOCH + Duration::from_secs(view_ts);
        let datetime = DateTime::<Utc>::from(d);

        let query = format!(
            "http://clayface.caida.org:9200/observatory-v3-events-{}-{}-{:02}/_doc/{}",
            event_type,
            datetime.year(),
            datetime.month(),
            id
        );

        let doc: Value = reqwest::get(query.as_str()).unwrap().json().unwrap();
        if doc["found"] == true {
            let mut document = doc["_source"].clone();
            document["url"] = Value::String(query);
            return Ok(document);
        } else {
            let query = format!(
                "http://clayface.caida.org:9200/observatory-v3-events-{}-{}-{:02}/event_result/{}",
                event_type,
                datetime.year(),
                datetime.month(),
                id
            );
            let doc: Value = reqwest::get(query.as_str()).unwrap().json().unwrap();
            if doc["found"] == true {
                let mut document = doc["_source"].clone();
                document["url"] = Value::String(query);
                return Ok(document);
            } else {
                Err(Box::new(MyError("Oops".into())))
            }
        }
    }

    fn build_query (
        &self,
        asns: &Option<String>,
        pfxs: &Option<String>,
        ts_start: &Option<String>,
        ts_end: &Option<String>,
        tags: &Option<String>,
        codes: &Option<String>,
        min_susp: &Option<isize>,
        max_susp: &Option<isize>,
        min_duration: &Option<usize>,
        max_duration: &Option<usize>,
        include_overlap: bool,
    ) -> Value {

        // time range filters
        let mut should_filters = vec![];
        let mut must_filters = vec![];
        if let Some(start_str) = ts_start {
            if include_overlap {
                // we want events after start_ts, there are two cases:
                // 1. event start time after start_ts
                should_filters.push(json!(
                    {
                        "range": {
                            "view_ts": {
                                "gte": json!(convert_time_str(start_str))
                            }
                        }
                    }
                ));
                // 2. event start time before start_ts and
                //   - either finished time after start_ts
                //   - or no finished time (i.e. ongoing event)
                should_filters.push(json!(
                    {
                        "bool": {
                            "must": [
                                {
                                    "range": {
                                        "view_ts": {
                                            "lt": json!(convert_time_str(start_str))
                                        }
                                    }
                                },
                                {
                                    "bool": {
                                        "should": [
                                            // case 1: finished_ts after start time
                                            {
                                                "range": {
                                                    "finished_ts": {
                                                        "gte": json!(convert_time_str(start_str))
                                                    }
                                                }
                                            },
                                            // case 2: finished_ts not exist (ongoing)
                                            {
                                                "bool" : {
                                                    "must_not": {
                                                        "exists": {
                                                            "field": "finished_ts"
                                                        }
                                                    }
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ));

                must_filters.push(json!(
                    {
                        "bool": {
                            "should": should_filters
                        }
                    }
                ))
            } else {
                // we only check event start time
                must_filters.push(json!(
                    {
                        "range": {
                            "view_ts": {
                                "gte": json!(convert_time_str(start_str))
                            }
                        }
                    }
                ));
            }
        }
        if let Some(end_str) = ts_end {
            // we want events before end_ts:
            // - event start time must before end_ts, finished time does not matter
            must_filters.push(json!(
                {
                    "range": {
                        "view_ts": {
                            "lte": json!(convert_time_str(end_str))
                        }

                    }
                }
            ));
        }
        let filter = json!({
                "bool": {
                    "must": must_filters
                }
            }
        );

        // match must terms
        let mut must_terms = vec![];
        let mut must_not_terms = vec![];
        must_not_terms.push(json!({ "match": { "position": "FINISHED" } }));

        // inference structure must exist first
        must_terms.push(json!({"exists":{"field": "summary.inference_result.primary_inference"}}));

        let mut suspicion_filter =
            json!({"summary.inference_result.primary_inference.suspicion_level": {}});
        if let Some(max) = max_susp {
            suspicion_filter["summary.inference_result.primary_inference.suspicion_level"]["lte"] =
                json!(max.to_owned() as i32);
        }
        if let Some(min) = min_susp {
            suspicion_filter["summary.inference_result.primary_inference.suspicion_level"]["gte"] =
                json!(min.to_owned() as i32);
        }
        must_terms.push(json!({ "range": suspicion_filter }));

        let mut duration_filter = json!({"duration": {}});
        let mut has_duration = false;
        if let Some(max) = max_duration {
            duration_filter["duration"]["lte"] = json!(max.to_owned() as i32);
            has_duration = true;
        }
        if let Some(min) = min_duration {
            duration_filter["duration"]["gte"] = json!(min.to_owned() as i32);
            has_duration = true;
        }
        if has_duration {
            // NOTE: only push duration filter if we specified duration, otherwise events without a
            // a duration field will not show up in the search results
            must_terms.push(json!({ "range": duration_filter }));
        }

        // prefixes filter
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

        // asns filter
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

        // tags filter
        match tags {
            Some(tags_string) => {
                let tags_lst: Vec<&str> = tags_string.split(",").collect::<Vec<&str>>();
                for t in tags_lst {
                    if t.starts_with("!") {
                        // negative match
                        let new_t = t.trim_start_matches('!');
                        must_not_terms.push(json!({"term":{"summary.tags.name":new_t}}))
                    } else {
                        must_terms.push(json!({"term":{"summary.tags.name":t}}))
                    }
                }
            }
            _ => {}
        }

        // codes filter
        match codes {
            Some(codes_string) => {
                let codes_lst: Vec<&str> = codes_string.split(",").collect::<Vec<&str>>();
                for t in codes_lst {
                    if t.starts_with("!") {
                        // negative match
                        let new_t = t.trim_start_matches('!');
                        must_not_terms.push(json!({"term":{"summary.inference_result.inferences.inference_id":new_t}}))
                    } else {
                        must_terms.push(
                            json!({"term":{"summary.inference_result.inferences.inference_id":t}}),
                        )
                    }
                }
            }
            _ => {}
        }

        json!(
            {
                "bool": {
                    "must": must_terms,
                    "must_not": must_not_terms,
                    "filter": filter
                }
            }
        )
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
        min_susp: &Option<isize>,
        max_susp: &Option<isize>,
        min_duration: &Option<usize>,
        max_duration: &Option<usize>,
        include_overlap: bool,
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

        let max_entries = match max {
            Some(n) => n.to_owned() as i32,
            None => 100 as i32,
        };

        let query: serde_json::Value = json!(
            {
                "from":query_from,
                "size":max_entries,
                "track_total_hits": true,
                "query":self.build_query(asns, pfxs, ts_start, ts_end, tags, codes, min_susp, max_susp, min_duration, max_duration, include_overlap),
                "sort": { "view_ts": { "order": "desc" }}
            }
        );

        let res = self
            .es_client
            .search::<Value>()
            .index(format!("observatory-v3-events-{}-*", etype))
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

    pub fn count_events(
        &self,
        event_type: &Option<String>,
        asns: &Option<String>,
        pfxs: &Option<String>,
        ts_start: &Option<String>,
        ts_end: &Option<String>,
        tags: &Option<String>,
        codes: &Option<String>,
        min_susp: &Option<isize>,
        max_susp: &Option<isize>,
        min_duration: &Option<usize>,
        max_duration: &Option<usize>,
        include_overlap: bool,
    ) -> Result<CountResult, Box<dyn Error>> {
        // event type default to "*"
        let mut etype = "*".to_owned();
        if let Some(et) = event_type {
            etype = match et.as_str() {
                "all" => "*".to_owned(),
                _ => et.to_owned(),
            }
        }

        let mut query = HashMap::new();
        query.insert("query", self.build_query(asns, pfxs, ts_start, ts_end, tags, codes, min_susp, max_susp, min_duration, max_duration, include_overlap));

        let client = reqwest::Client::new();
        let res: Value = client.post(format!("http://clayface.caida.org:9200/observatory-v3-events-{}-*/_count", etype).as_str())
                        .json(&query)
                        .send().unwrap().json().unwrap();
        Ok(CountResult {
            count: res["count"].as_u64().unwrap(),
        })
    }
}
