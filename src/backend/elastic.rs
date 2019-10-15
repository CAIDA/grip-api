use chrono::prelude::DateTime;
use chrono::Utc;
use std::error::Error;
use std::time::{Duration, UNIX_EPOCH};

use elastic::prelude::*;
use serde_json::json;
use serde_json::Value;

use crate::backend::errors::MyError;

pub struct ElasticSearchBackend {
    es_client: SyncClient,
}

pub struct SearchResult {
    pub results: Vec<Value>,
    pub total: u64,
}

impl ElasticSearchBackend {
    // constructor static method
    pub fn new(base_url: &str) -> Result<ElasticSearchBackend, Box<dyn Error>> {
        let client = SyncClientBuilder::new()
            // .base_url("http://hammer.caida.org:9200")
            .base_url(base_url)
            .build()?;
        Ok(ElasticSearchBackend { es_client: client })
    }

    pub fn get_event_by_id(&self, id: &str) -> Result<SearchResult, Box<dyn Error>> {
        let fields = id.split("-").collect::<Vec<&str>>();
        let event_type: &str = fields[0];
        let view_ts: u64 = fields[1].parse::<u64>().unwrap();

        let d = UNIX_EPOCH + Duration::from_secs(view_ts);
        let datetime = DateTime::<Utc>::from(d);

        let query = format!(
            "http://clayface.caida.org:9200/observatory-{}-{}-{:02}-{:02}/event_result/{}",
            event_type,
            datetime.year(),
            datetime.month(),
            datetime.day(),
            id
        );

        let doc: Value = reqwest::get(query.as_str()).unwrap().json().unwrap();
        if doc["found"] == true {
            return Ok(SearchResult {
                results: vec![doc["_source"].clone()],
                total: 1,
            });
        } else {
            Err(Box::new(MyError("Oops".into())))
        }
    }

    pub fn list_events(
        &self,
        event_type: &str,
        start: &Option<usize>,
        max: &Option<usize>,
        asn: &Option<usize>,
        prefix: &Option<String>,
        ts_start: &Option<String>,
        ts_end: &Option<String>,
        tags: &Option<String>,
        min_susp: &Option<usize>,
        max_susp: &Option<usize>,
        misconf: &Option<bool>,
        misconf_type: &Option<String>,
    ) -> Result<SearchResult, Box<dyn Error>> {
        let mut etype = event_type.to_owned();

        if etype == "all" {
            // if event type is all or misconf (misconfiguration), show all events that matches
            etype = "*".to_owned();
        }

        let mut query_from = 0;
        if let Some(s) = start {
            query_from = s.to_owned() as i32;
        }

        let mut range_filter = json!({"view_ts":{}});
        if let Some(start_str) = ts_start {
            range_filter["view_ts"]["gte"] = json!(start_str);
        }
        if let Some(end_str) = ts_end {
            range_filter["view_ts"]["lte"] = json!(end_str);
        }

        let max_entries = match max {
            Some(n) => n.to_owned() as i32,
            None => 100 as i32,
        };

        // match must terms
        let mut must_terms = vec![];
        let mut must_not_terms = vec![];
        must_not_terms.push(json!({ "match": { "position.keyword": "FINISHED" } }));

        let mut suspicion_filter = json!({"inference.suspicion.suspicion_level": {}});
        if let Some(max) = max_susp {
            suspicion_filter["inference.suspicion.suspicion_level"]["lte"] = json!(max.to_owned() as i32);
        }
        if let Some(min) = min_susp {
            suspicion_filter["inference.suspicion.suspicion_level"]["gte"] = json!(min.to_owned() as i32);
        }
        must_terms.push(json!({"range": suspicion_filter} ));

        if let Some(mis) = misconf {
            must_terms.push(json!({"term": {"inference.misconfiguration": mis}}));
            must_not_terms.push(json!({"match":{"tags":"newcomer-is-sibling"}}));
            must_not_terms.push(json!({"match":{"tags":"newcomer-is-friend"}}));
            let mut mistype = "all";
            if let Some(t) = misconf_type{
                mistype = t.as_str();
            }
            match mistype {
                "all" => {},
                "asn_prepend" => {
                    must_terms.push(json!({"term":{"tags":"newcomer-small-asn"}}));
                    must_terms.push(json!({"term":{"tags":"all-newcomers-next-to-an-oldcomer"}}));
                },
                "fatfinger_prefix" => {
                    must_terms.push(json!({"term":{"tags":"prefix-small-edit-distance"}}));
                },
                "fatfinger_asn" => {
                    must_terms.push(json!({"term":{"tags":"origin-small-edit-distance"}}));
                },
                "reserved_space" => {
                    must_terms.push(json!({"term":{"tags":"reserved-space"}}));
                },
                _ => {}
            }
        }

        match prefix {
            Some(p) => {
                // https://stackoverflow.com/questions/40573981/multiple-should-queries-with-must-query
                let mut pfx_must = vec![];
                pfx_must.push(json!({ "prefix": { "pfx_events.sub_pfx.keyword" : p }}));
                pfx_must.push(json!({ "prefix": { "pfx_events.super_pfx.keyword" : p }}));
                pfx_must.push(json!({ "prefix": { "pfx_events.prefix.keyword" : p }}));
                must_terms.push(json!({"bool": {"minimum_should_match": 1, "should": pfx_must}}));
            }
            _ => {}
        }
        match asn {
            Some(value) => {
                // https://stackoverflow.com/questions/40573981/multiple-should-queries-with-must-query
                let mut asn_must = vec![];
                asn_must.push(json!({ "match": { "pfx_events.origins" : value }}));
                asn_must.push(json!({ "match": { "pfx_events.super_origins" : value }}));
                asn_must.push(json!({ "match": { "pfx_events.sub_origins" : value }}));
                asn_must.push(json!({ "match": { "pfx_events.as1.keyword" : value }}));
                asn_must.push(json!({ "match": { "pfx_events.as2.keyword" : value }}));
                must_terms.push(json!({"bool": {"minimum_should_match": 1, "should": asn_must}}));
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
        println!("{}", serde_json::to_string_pretty(&query).unwrap());

        let res = self
            .es_client
            .search::<Value>()
            .index(format!("observatory-{}-*", etype))
            .body(query)
            .send()?;

        // Iterate through the hits in the response and build a vector.
        let mut res_vec: Vec<Value> = Vec::new();
        for hit in res.hits() {
            let mut doc = json!(hit.document().unwrap());
            doc["_esid"] = json!(hit.index().to_owned());
            res_vec.push(doc);
        }

        Ok(SearchResult {
            results: res_vec,
            total: res.total(),
        })
    }
}
