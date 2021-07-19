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

use crate::backend::api_auth::Feedback;
use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub(crate) struct Mailer {
    mailer: SmtpTransport,
    user_email: String,
}

#[derive(Debug)]
pub(crate) struct MailError {
    kind: Kind,
}

#[derive(Debug)]
pub(crate) enum Kind {
    MissingCredential(String),
    MailRelayFail,
    MailSendError,
}

macro_rules! get_env {
    ($e:expr) => {
        match std::env::var($e) {
            Ok(res) => res,
            Err(_) => {
                return Err(MailError {
                    kind: Kind::MissingCredential($e.to_string()),
                })
            }
        }
    };
}

impl Mailer {
    pub fn new() -> Result<Mailer, MailError> {
        let smtp_username = get_env!("MAIL_SMTP_USERNAME");
        let smtp_password = get_env!("MAIL_SMTP_PASSWORD");
        let smtp_endpoint = get_env!("MAIL_SMTP_ENDPOINT");
        let user_email = get_env!("MAIL_USER_EMAIL");

        let creds = Credentials::new(smtp_username, smtp_password);

        // Open a remote connection to gmail
        let mailer = match SmtpTransport::relay(smtp_endpoint.as_str()) {
            Ok(builder) => builder.credentials(creds).build(),
            Err(_) => {
                return Err(MailError {
                    kind: Kind::MailRelayFail,
                })
            }
        };

        Ok(Mailer { mailer, user_email })
    }

    pub fn send(self: &Self, feedback: &Feedback) -> Result<(), MailError> {
        let email = Message::builder()
            .to(format!("{}, {}", feedback.from_email, self.user_email)
                .parse()
                .unwrap())
            .from(self.user_email.parse().unwrap())
            .subject("GRIP Event Feedback Confirmation")
            .multipart(self.build_email_body(feedback))
            .unwrap();
        match self.mailer.send(&email) {
            Ok(_) => {
                println!("Email sent successfully!");
                Ok(())
            }
            Err(e) => Err(MailError {
                kind: Kind::MailSendError,
            }),
        }
    }

    fn build_email_body(self: &Self, feedback: &Feedback) -> MultiPart {
        let text_body = format!(
            r#"Dear {name},

Thank you for submitting your feedback. Here is a recap of what you submitted:
Event ID: {event_id}
Feedback Type: {feedback_type}
Feedback Details:
{feedback_details}

We will process the feedback soon.

Best regards,
GRIP Dev Team
"#,
            name = feedback.from_name,
            feedback_type = feedback.feedback_type,
            feedback_details = feedback.feedback_details,
            event_id = feedback.event_id,
        );

        let html_body = format!(
            r#"Dear {name},
<br/>
<br/>
Thank you for submitting your feedback. Here is a recap of what you submitted:
<ul>
    <li> Feedback Type: 
        <pre> {feedback_type} </pre>
    </li>
    <li>
        Event ID: 
        <pre> {event_id} </pre>
    </li>
    <li> Feedback Details:
        <pre>{feedback_details}</pre>
    </li>
</ul>
<br/>
<br/>

We will process the feedback soon.
<br/>
<br/>

Best regards, <br/>
GRIP Dev Team
"#,
            name = feedback.from_name,
            feedback_type = feedback.feedback_type,
            feedback_details = feedback.feedback_details,
            event_id = feedback.event_id,
        );
        MultiPart::alternative_plain_html(text_body, html_body)
    }
}
