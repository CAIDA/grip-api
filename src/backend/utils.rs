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
