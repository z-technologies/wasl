use crate::result::Result;
use lettre::message::Mailbox;

use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};

pub struct EmailService {
    mailer: SmtpTransport,
}

impl EmailService {
    pub fn new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        tls: bool,
    ) -> Result<EmailService> {
        let mut builder = SmtpTransport::relay(host)?
            .port(port)
            .credentials((username.to_owned(), password.to_owned()).into());

        if tls {
            builder = builder.tls(Tls::Required(
                TlsParameters::builder(host.into()).build_native()?,
            ));
        }

        Ok(EmailService {
            mailer: builder.build(),
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
}

pub fn make_mail_box<'a>(name: &'a str, email: &'a str) -> Result<Mailbox> {
    Ok(format!("{} <{}>", name, email).parse()?)
}
