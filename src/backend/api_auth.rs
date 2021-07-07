/*
 * This software is Copyright (c) 2019 The Regents of the University of
 * California. All Rights Reserved. Permission to copy, modify, and distribute this
 * software and its documentation for academic research and education purposes,
 * without fee, and without a written agreement is hereby granted, provided that
 * the above copyright notice, this paragraph and the following three paragraphs
 * appear in all copies. Permission to make use of this software for other than
 * academic research and education purposes may be obtained by contacting:
 *
 * Office of Innovation and Commercialization
 * 9500 Gilman Drive, Mail Code 0910
 * University of California
 * La Jolla, CA 92093-0910
 * (858) 534-5815
 * invent@ucsd.edu
 *
 * This software program and documentation are copyrighted by The Regents of the
 * University of California. The software program and documentation are supplied
 * "as is", without any accompanying services from The Regents. The Regents does
 * not warrant that the operation of the program will be uninterrupted or
 * error-free. The end-user understands that the program was developed for research
 * purposes and is advised not to rely exclusively on the program for any reason.
 *
 * IN NO EVENT SHALL THE UNIVERSITY OF CALIFORNIA BE LIABLE TO ANY PARTY FOR
 * DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES, INCLUDING LOST
 * PROFITS, ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF
 * THE UNIVERSITY OF CALIFORNIA HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE. THE UNIVERSITY OF CALIFORNIA SPECIFICALLY DISCLAIMS ANY WARRANTIES,
 * INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS FOR A PARTICULAR PURPOSE. THE SOFTWARE PROVIDED HEREUNDER IS ON AN "AS
 * IS" BASIS, AND THE UNIVERSITY OF CALIFORNIA HAS NO OBLIGATIONS TO PROVIDE
 * MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.
 *
 *
 */

use crate::backend::auth::validate_token;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use serde_json::json;
use serde_json::Value;

pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

const RSA_N: &str = "nmaWAMzKICx_Ja7FvyaQcfzsFIc3zXJS0qOYsQW3INDTCfCDnvgmVGqIrsyJot83svjm5WL8n3cgkaWWGVQ4FOykgmwdIcgfov9ieSmTUnoVcmpBs1HA9QehAC65E3fe1F4V4cpj01vDjiP-hM092IOpR48KSRu6vOw23_0QE_VGlxzcq5su9ujU8QYP63apeqDPUMw1GmMGd_QgAEIGbwXSJt5IB6RKFO2-gFPVylBu1W9tVmZb-yoP5LoqX-LaZ5JZ3O5304E5-nxNhhXq-f2Z65VSpApACFkIS7_jNksXHsjBMqt50OdN0qPSYMeYC6jzPxHMTGrE1ojPqLoX6w";
const RSA_E: &str = "AQAB";

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        /// A valid key string should look like "Bearer xxxxxxxx..."
        fn is_valid(key: &str) -> bool {
            let vec = key.split_whitespace().collect::<Vec<&str>>();
            if vec.len() != 2 {
                dbg!("invalid bearer");
                return false;
            }
            validate_token(vec[1], RSA_N, RSA_E).is_ok()
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey("SECRET CODE")),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

#[get("/sensitive")]
pub fn sensitive(key: ApiKey<'_>) -> Json<Value> {
    Json(json!("SENSITIVE CODE VALIDATED BY BACKEND API"))
}
