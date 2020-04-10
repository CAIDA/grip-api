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

/*
    Utilities
*/

use chrono::{DateTime, NaiveDateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

/// Find one specific prefix event from all prefix events in a event
pub fn filter_pfx_events_by_fingerprint<'a>(
    fingerprint: &str,
    event: &'a Value,
) -> Option<&'a Value> {
    let event_type = match event["event_type"].as_str() {
        Some(t) => t,
        None => return None,
    };

    let re = Regex::new(r"-").unwrap();
    let result = re.replace_all(fingerprint, "/");

    let prefixes: Vec<&str> = result.split("_").collect();
    if prefixes.len() == 0 {
        return None;
    }

    let pfx_events: &Vec<Value> = match event["pfx_events"].as_array() {
        Some(events) => events,
        None => return None,
    };

    match event_type {
        "moas" | "edges" => {
            if prefixes.len() != 1 {
                // must only have one prefix in the fingerprint for moas and edges cases
                return None;
            }

            for pfx_event in pfx_events {
                match pfx_event["details"]["prefix"].as_str() {
                    Some(pfx) => {
                        if pfx == prefixes[0] {
                            return Some(&pfx_event);
                        }
                    }
                    None => continue,
                }
            }
            return None;
        }
        "submoas" | "defcon" => {
            if prefixes.len() != 2 {
                // must only have one prefix in the fingerprint for defcon and submoas cases
                return None;
            }

            for pfx_event in pfx_events {
                let sub_pfx = match pfx_event["details"]["sub_pfx"].as_str() {
                    Some(pfx) => pfx,
                    None => continue,
                };
                let super_pfx = match pfx_event["details"]["super_pfx"].as_str() {
                    Some(pfx) => pfx,
                    None => continue,
                };

                if sub_pfx == prefixes[0] && super_pfx == prefixes[1] {
                    // if we found the one
                    return Some(&pfx_event);
                }
            }
            return None;
        }
        _ => return None,
    }
}

lazy_static! {
    static ref OLD_FORMAT: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$").unwrap();
}

/// convert time string into a serde::Value.
///
/// the time string can be either a rfc3339 formatted string or a string representing the epoch
/// time in milliseconds.
///
/// expect:
/// -  2020-04-09T23:52
/// -  2020-04-09 23:52:00
/// -  1586476363000
///
/// output:
/// -  2020-04-09 23:52:00
pub fn convert_time_str(ts_str: &String) -> String {
    match OLD_FORMAT.is_match(ts_str) {
        true => {
            let ts_vec = ts_str.split("T").collect::<Vec<&str>>();
            format!("{} {}:00", ts_vec[0], ts_vec[1])
        }
        false => match ts_str.parse::<i64>() {
            Ok(ts) => {
                let dt =
                    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts / 1000, 0), Utc);
                dt.format("%Y-%m-%d %H:%M:00").to_string()
            }
            Err(_) => ts_str.to_owned(),
        },
    }
}
#[cfg(test)]
mod tests {
    use crate::backend::utils::convert_time_str;

    #[test]
    fn it_works() {
        // GMT: Thursday, April 9, 2020 23:52:43
        // Your time zone: Thursday, April 9, 2020 16:52:43 GMT-07:00 DST
        let value1 = convert_time_str(&"1586476363000".to_owned());
        let value2 = convert_time_str(&"2020-04-09T23:52".to_owned());
        assert_eq!(value1, value2)
    }
}
