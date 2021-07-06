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
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
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
    pub aud: String, // Optional. Audience
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Optional. Issued at (as UTC timestamp)
    pub iss: String, // Optional. Issuer
    pub sub: String, // Optional. Subject (whom token refers to)
}

/// Validate an access token, and return claims if the token is valid.
///
/// Input:
/// - `token`: a access token to be validated
/// - `jwks`: a set of json web keys obtained from authentication provider
///
/// Procedure:
/// 1. decode token into header, which includes the `kid` field (i.e. key ID)
/// 2. use the `kid` field to find corresponding key from the given keys (`jwks`)
/// 3. validate the token using [`jsonwebtoken`] library
/// 4. return [`Claims`] if the validation succeeded, otherwise return reutrn `Err(String)`
pub fn validate_token(token: &String, jwks: &Value) -> Result<Claims, String> {
    // TODO: properly handle errors (i.e. replacing "unwrap"s)
    // TODO: check `exp` and `iat` timestamps
    let header = decode_header(&token).unwrap();
    let key_id = header.kid.unwrap();
    let mut jwk: Option<&Map<String, Value>> = None;
    for key in jwks
        .as_object()
        .unwrap()
        .get("keys")
        .unwrap()
        .as_array()
        .unwrap()
    {
        if key
            .as_object()
            .unwrap()
            .get("kid")
            .unwrap()
            .as_str()
            .unwrap()
            == key_id
        {
            jwk = Some(key.as_object().unwrap());
        }
    }
    if jwk.is_none() {
        return Err("no corresponding key found for the token".to_string());
    }

    let n = jwk.unwrap().get("n").unwrap().as_str().unwrap();
    let e = jwk.unwrap().get("e").unwrap().as_str().unwrap();

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_rsa_components(n, e),
        &Validation::new(Algorithm::RS256),
    )
    .unwrap();
    Ok(decoded.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonwebtoken() {
        let jwks: Value = serde_json::from_str(r#"{
  "keys": [
    {
      "alg": "RS256",
      "kty": "RSA",
      "use": "sig",
      "n": "nmaWAMzKICx_Ja7FvyaQcfzsFIc3zXJS0qOYsQW3INDTCfCDnvgmVGqIrsyJot83svjm5WL8n3cgkaWWGVQ4FOykgmwdIcgfov9ieSmTUnoVcmpBs1HA9QehAC65E3fe1F4V4cpj01vDjiP-hM092IOpR48KSRu6vOw23_0QE_VGlxzcq5su9ujU8QYP63apeqDPUMw1GmMGd_QgAEIGbwXSJt5IB6RKFO2-gFPVylBu1W9tVmZb-yoP5LoqX-LaZ5JZ3O5304E5-nxNhhXq-f2Z65VSpApACFkIS7_jNksXHsjBMqt50OdN0qPSYMeYC6jzPxHMTGrE1ojPqLoX6w",
      "e": "AQAB",
      "kid": "YZRdWGnWb2WR5YAG72k9I",
      "x5t": "OVbioKJsw059mJD2O6vlsdICAgg",
      "x5c": [
        "MIIDAzCCAeugAwIBAgIJEkpmfIyqiK/vMA0GCSqGSIb3DQEBCwUAMB8xHTAbBgNVBAMTFG1pbmd3ZWkudXMuYXV0aDAuY29tMB4XDTIxMDcwMTE2NTE0NloXDTM1MDMxMDE2NTE0NlowHzEdMBsGA1UEAxMUbWluZ3dlaS51cy5hdXRoMC5jb20wggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQCeZpYAzMogLH8lrsW/JpBx/OwUhzfNclLSo5ixBbcg0NMJ8IOe+CZUaoiuzImi3zey+OblYvyfdyCRpZYZVDgU7KSCbB0hyB+i/2J5KZNSehVyakGzUcD1B6EALrkTd97UXhXhymPTW8OOI/6EzT3Yg6lHjwpJG7q87Dbf/RAT9UaXHNyrmy726NTxBg/rdql6oM9QzDUaYwZ39CAAQgZvBdIm3kgHpEoU7b6AU9XKUG7Vb21WZlv7Kg/kuipf4tpnklnc7nfTgTn6fE2GFer5/ZnrlVKkCkAIWQhLv+M2SxceyMEyq3nQ503So9Jgx5gLqPM/EcxMasTWiM+ouhfrAgMBAAGjQjBAMA8GA1UdEwEB/wQFMAMBAf8wHQYDVR0OBBYEFHKQ0wUfhbvqJfz9qMmPlk1OPuNpMA4GA1UdDwEB/wQEAwIChDANBgkqhkiG9w0BAQsFAAOCAQEAVAcvodd62poqTuN1XRpkrlmZhDItdKtu+9XmLtjhllPmp4XKIImb2k5Djx8kSOFm8fScQd5Nujo5RtO0Ufwpju7dnrjmPF0lgtDNYLyPCoUVscdKCl8kEqgBmqHujvtSLpt9sA2RU2uhopfoiRgsAhHwmT7DaMVkBQ19sq5tBz4cyKL+PijvvOKkkTooeZp9x7XVwLFohDFgY7Uy9GnklIxviT33QvNJ4PUw9YN5SCrRbaPP1FQIjc2ffPKGC13fJDHuRQ++7gW64xk3S+INztmivxtvV4Vr58qhl4aBnGK5IYZUD1Dme6Yc1N2GGBVwBD4BSOsEZkZU2yM8jhuWDQ=="
      ]
    },
    {
      "alg": "RS256",
      "kty": "RSA",
      "use": "sig",
      "n": "9SmalXl9D7fMz-z6T0q-xPtw5OD7D3K2EUz03lni4eWL7YKBsYiQe9qz4hJxunEntzNZwUmBfebCoxGnEr4CBam8KqFUzRDlywolIHUn9koJMqMXCWu8GWQAlxJEHgK09YK-6dtLFZxyEQ8tdAtv_t2FmXuHSCZzHKYDOSfJw8UKwl7102fPU0J5003YgNV-2xgIJQu-Sgn4Mdg0r4e6V7JfSUTlI4XfR2wS1eCasGiwvrX6j_RhiDVkm4hBGyw1BzYv6N_JQx72lyr8WLsqpMnYjQ2IAovyExiH1g8LW1aYOQhI8SexItNw6Oxx9oRqC0xgbd9e_9QDOnrLChK5oQ",
      "e": "AQAB",
      "kid": "onptyAX7qq3JNTYSs3x9r",
      "x5t": "ZD0SIUYsI-WsX4krCUVXQTOO8P0",
      "x5c": [
        "MIIDAzCCAeugAwIBAgIJKGXU3yKsf+bvMA0GCSqGSIb3DQEBCwUAMB8xHTAbBgNVBAMTFG1pbmd3ZWkudXMuYXV0aDAuY29tMB4XDTIxMDcwMTE2NTE0NloXDTM1MDMxMDE2NTE0NlowHzEdMBsGA1UEAxMUbWluZ3dlaS51cy5hdXRoMC5jb20wggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQD1KZqVeX0Pt8zP7PpPSr7E+3Dk4PsPcrYRTPTeWeLh5YvtgoGxiJB72rPiEnG6cSe3M1nBSYF95sKjEacSvgIFqbwqoVTNEOXLCiUgdSf2SgkyoxcJa7wZZACXEkQeArT1gr7p20sVnHIRDy10C2/+3YWZe4dIJnMcpgM5J8nDxQrCXvXTZ89TQnnTTdiA1X7bGAglC75KCfgx2DSvh7pXsl9JROUjhd9HbBLV4JqwaLC+tfqP9GGINWSbiEEbLDUHNi/o38lDHvaXKvxYuyqkydiNDYgCi/ITGIfWDwtbVpg5CEjxJ7Ei03Do7HH2hGoLTGBt317/1AM6essKErmhAgMBAAGjQjBAMA8GA1UdEwEB/wQFMAMBAf8wHQYDVR0OBBYEFHWYlUrxNjDUtJ75cDdzTPX+2IMfMA4GA1UdDwEB/wQEAwIChDANBgkqhkiG9w0BAQsFAAOCAQEAH7U4GH1dPDR1njW2ZgAopGUsMoBJM5M4x0TEsJToOcT+Mzgvypy+y54nKAAC8V6FD0WOr+XBC3bZtl2NRMEsuo2a46Xg4ZhGZuQDp8HOR1r87ZFTHKjo7G9TemvFuh/jkAC2lX2e1f6874A/Mu5UpcDU6mb6mzpSRcfQYmwoWIxConJ0S6OzySdNt9uirR/pz13ybVWIZH0F+jeBxJ36TX1+buoWvpb1tkkKrtcd24Lqr6cwKjLLuZpFW8jczfXpAcnJPzRdg5LwIwhF5lexhrxRRQ3zlOigTWPIzh6bgR74S2VAuGPFkTmiWnQGI2LVQ5EOoBFXHTQmtFSyQ7bXAA=="
      ]
    }
  ]
}"#).unwrap();
        let token: String = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IllaUmRXR25XYjJXUjVZQUc3Mms5SSJ9.eyJpc3MiOiJodHRwczovL21pbmd3ZWkudXMuYXV0aDAuY29tLyIsInN1YiI6InlPcEpkZWdUcFZjaGFYQ01heVFKZEU5R1lKbjhvZVVXQGNsaWVudHMiLCJhdWQiOiJodHRwczovL2FwaS5ncmlwLmNhaWRhLm9yZyIsImlhdCI6MTYyNTU4NjY5OCwiZXhwIjoxNjI1NjczMDk4LCJhenAiOiJ5T3BKZGVnVHBWY2hhWENNYXlRSmRFOUdZSm44b2VVVyIsImd0eSI6ImNsaWVudC1jcmVkZW50aWFscyJ9.PnOmZri5hw3_xQZc3EujTCqBKai3zBzCfMCUHJptln5ubrsKf51cgjQl-cosye-U-MuXntcu885cR6WJEZYecKnwJaiDw0g7RLHsB-XER_tzBiU0rKgnKTSDS7xU7q2F57pXQyc7v-BxYdcjUj-HRsTzEREh1x3sE456wGAkUTKm31cccYOq4bzxTLkxKrIf7a-Wdtp_LyDVbTSRh4_C3A2qWsGa-RmAtUz9Gi62GJ5XPWEi7wCuLwLcxIcTjLe9SCrW11jVQIzpil9bkY4FrsMXuExkPot_vT337gd9fSS4okLQcxKRNDcJahONLhI3YijgEiMVubO4mylefNPnyQ".to_string();
        let res = validate_token(&token, &jwks);
        dbg!(res);
    }
}
