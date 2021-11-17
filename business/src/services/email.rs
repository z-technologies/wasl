use crate::result::Result;
use lettre::message::Mailbox;

use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;

pub struct EmailService {
    mailer: SmtpTransport,
}

impl EmailService {
    pub fn new(settings: &EmailSettings) -> Result<EmailService> {
        let builder = SmtpTransport::relay(&settings.smtp_host)?
            .port(settings.smtp_port)
            .credentials(
                (
                    settings.smtp_username.clone(),
                    settings.smtp_password.clone(),
                )
                    .into(),
            );

        Ok(EmailService {
            mailer: if settings.enable_tls {
                builder
                    .tls(Tls::Required(
                        TlsParameters::builder(settings.smtp_host.clone())
                            .build_native()?,
                    ))
                    .build()
            } else {
                builder.build()
            },
        })
    }

    pub fn send_email<'a>(
        &self,
        subject: &'a str,
        body: &'a str,
        from: Mailbox,
        to: Mailbox,
    ) -> Result<Response> {
        let email = Message::builder()
            .from(from)
            .to(to)
            .subject(subject)
            .date_now()
            .body(body.to_owned())?;

        Ok(self.mailer.send(&email)?)
    }
}

pub fn make_mail_box<'a>(name: &'a str, email: &'a str) -> Result<Mailbox> {
    Ok(format!("{} <{}>", name, email).parse()?)
}

#[derive(Debug, Deserialize)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,

    pub noreply_address: String,
    pub enable_tls: bool,
}
