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
 */

//! Authentication module.
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// JSON web token claims.
///
/// The included fields are:
/// - `aud`: audience
/// - `exp`: expiration timestamp
/// - `iat`: issued at timestamp
/// - `iss`: issuer
/// - `sub`: subject (request maker, client)
///
/// Example claims:
/// ```
/// Claims {
///         aud: "https://api.grip.caida.org",
///         exp: 1625673098,
///         iat: 1625586698,
///         iss: "https://mingwei.us.auth0.com/",
///         sub: "yOpJdegTpVchaXCMayQJdE9GYJn8oeUW@clients",
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: Value,  // Optional. Audience
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Optional. Issued at (as UTC timestamp)
    pub iss: String, // Optional. Issuer
    pub sub: String, // Optional. Subject (whom token refers to)
}

/// Validate an access token, and return claims if the token is valid.
///
/// Input:
/// - `token`: a access token to be validated
/// - `rsa_n`: modulus of the RSA public key
/// - `rsa_e`: exponent of the RSA public key
///
/// Procedure:
/// 1. decode token into header, which includes the `kid` field (i.e. key ID)
/// 2. use the `kid` field to find corresponding key from the given keys (`jwks`)
/// 3. validate the token using [`jsonwebtoken`] library
/// 4. return [`Claims`] if the validation succeeded, otherwise return reutrn `Err(String)`
pub fn validate_token(token: &str, rsa_n: &str, rsa_e: &str) -> Result<Value, String> {
    match decode::<Value>(
        &token,
        &DecodingKey::from_rsa_components(rsa_n, rsa_e),
        &Validation::new(Algorithm::RS256),
    ) {
        Ok(decoded) => Ok(decoded.claims),
        Err(_) => Err("validation failed".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonwebtoken() {
        let rsa_n = "nmaWAMzKICx_Ja7FvyaQcfzsFIc3zXJS0qOYsQW3INDTCfCDnvgmVGqIrsyJot83svjm5WL8n3cgkaWWGVQ4FOykgmwdIcgfov9ieSmTUnoVcmpBs1HA9QehAC65E3fe1F4V4cpj01vDjiP-hM092IOpR48KSRu6vOw23_0QE_VGlxzcq5su9ujU8QYP63apeqDPUMw1GmMGd_QgAEIGbwXSJt5IB6RKFO2-gFPVylBu1W9tVmZb-yoP5LoqX-LaZ5JZ3O5304E5-nxNhhXq-f2Z65VSpApACFkIS7_jNksXHsjBMqt50OdN0qPSYMeYC6jzPxHMTGrE1ojPqLoX6w";
        let rsa_e = "AQAB";
        let token: String = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IllaUmRXR25XYjJXUjVZQUc3Mms5SSJ9.eyJpc3MiOiJodHRwczovL21pbmd3ZWkudXMuYXV0aDAuY29tLyIsInN1YiI6InlPcEpkZWdUcFZjaGFYQ01heVFKZEU5R1lKbjhvZVVXQGNsaWVudHMiLCJhdWQiOiJodHRwczovL2FwaS5ncmlwLmNhaWRhLm9yZyIsImlhdCI6MTYyNTU4NjY5OCwiZXhwIjoxNjI1NjczMDk4LCJhenAiOiJ5T3BKZGVnVHBWY2hhWENNYXlRSmRFOUdZSm44b2VVVyIsImd0eSI6ImNsaWVudC1jcmVkZW50aWFscyJ9.PnOmZri5hw3_xQZc3EujTCqBKai3zBzCfMCUHJptln5ubrsKf51cgjQl-cosye-U-MuXntcu885cR6WJEZYecKnwJaiDw0g7RLHsB-XER_tzBiU0rKgnKTSDS7xU7q2F57pXQyc7v-BxYdcjUj-HRsTzEREh1x3sE456wGAkUTKm31cccYOq4bzxTLkxKrIf7a-Wdtp_LyDVbTSRh4_C3A2qWsGa-RmAtUz9Gi62GJ5XPWEi7wCuLwLcxIcTjLe9SCrW11jVQIzpil9bkY4FrsMXuExkPot_vT337gd9fSS4okLQcxKRNDcJahONLhI3YijgEiMVubO4mylefNPnyQ".to_string();
        let res = validate_token(&token, rsa_n, rsa_e);
        dbg!(res);
    }
}
