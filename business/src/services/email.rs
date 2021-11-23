use crate::result::Result;

use lettre::message::Mailbox;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;

pub struct EmailService {
    mailer: SmtpTransport,
    noreply_mailbox: Mailbox,
}

impl EmailService {
    pub fn new(cfg: &EmailSettings) -> Result<EmailService> {
        let mailer = SmtpTransport::relay(&cfg.smtp_host)?
            .port(cfg.smtp_port)
            .credentials(
                (cfg.smtp_username.clone(), cfg.smtp_password.clone()).into(),
            )
            .tls(if cfg.require_tls {
                Tls::Required(
                    TlsParameters::builder(cfg.smtp_host.clone())
                        .build_native()?,
                )
            } else {
                Tls::None
            })
            .build();

        Ok(EmailService {
            mailer,
            noreply_mailbox: cfg.noreply_address.parse()?,
        })
    }

    pub fn send<'a>(
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

    pub fn send_noreply<'a>(
        &self,
        subject: &'a str,
        body: &'a str,
        to: Mailbox,
    ) -> Result<Response> {
        self.send(subject, body, self.noreply_mailbox.clone(), to)
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
    pub require_tls: bool,
}
