// send confirm email
use crate::errors::ServiceError;
use base64;
use lettre::file::FileTransport;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::SmtpClient;
use lettre::{SendableEmail, Transport};
use lettre_email::Email;
use std::path::Path;

#[derive(Debug)]
pub struct MailConfig {
    pub smtp_login: String,
    pub smtp_password: String,
    pub smtp_server: String,
}

pub fn init_config() -> Option<MailConfig> {
    match (
        dotenv::var("MAIL_SMTP_LOGIN"),
        dotenv::var("MAIL_SMTP_PASSWORD"),
        dotenv::var("MAIL_SMTP_SERVER"),
    ) {
        (Ok(login), Ok(password), Ok(server)) => Some(MailConfig {
            smtp_login: login,
            smtp_password: password,
            smtp_server: server,
        }),
        _ => None,
    }
}

fn build_email(
    recipient: &str,
    subject: &str,
    body: &str,
    mail_config: &Option<MailConfig>,
) -> Result<SendableEmail, ServiceError> {
    let sender = mail_config
        .as_ref()
        .map(|s| s.smtp_login.as_str())
        .unwrap_or("test@ruthub");

    let email = Email::builder()
        .to(recipient)
        .from(sender)
        .subject(subject)
        .body(body)
        .build()
        .map_err(|_| ServiceError::BadRequest("Error in Building email".into()))?;

    Ok(email.into())
}

fn send_email(
    recipient: &str,
    subject: &str,
    body: &str,
) -> Result<(), ServiceError> {
    let mail_config = init_config();
    let email = build_email(recipient, subject, body, &mail_config)?;

    match mail_config {
        Some(mail_config) => {
            let mut transport = SmtpClient::new_simple(&mail_config.smtp_server)
                .map_err(|_| {
                    ServiceError::BadRequest("Error in Building email".into())
                })?
                .credentials(Credentials::new(
                    mail_config.smtp_login,
                    mail_config.smtp_password,
                ))
                .smtp_utf8(true)
                .authentication_mechanism(Mechanism::Plain)
                .transport();

            let result = transport.send(email);
            result.map_err(|_| {
                ServiceError::BadRequest("Error in sending email".into())
            })?;
        }
        None => {
            let mut sender = FileTransport::new(Path::new("/tmp"));
            let result = sender.send(email);
            result.map_err(|_| {
                ServiceError::BadRequest("Email file could not be generated".into())
            })?;
        }
    }

    Ok(())
}

pub fn try_send_confirm_email(
    email: &str,
    user_name: &str,
    token: &str,
) -> Result<(), ServiceError> {
    let subject = "Please confirm your email address";
    let body = format!(
        "Hello {}: \n\n Welcome to RutHub. Please click the link below to verify your email address. Thank you! \n\n https://ruthub.com/confirm/{} \n\n This link will expire in 48 hours. \n\n\n The RutHub Team",
        user_name, base64::encode(token)
    );

    send_email(email, subject, &body)
}

pub fn try_send_reset_email(
    email: &str,
    user_name: &str,
    token: &str,
) -> Result<(), ServiceError> {
    let subject = "Please Reset Your password";
    let body = format!(
        "Hello {}: \n\n The Token to reset your password as below:\n\n {} \n\n This Token will expire in 2 hours. \n\n\n The RutHub Team",
        user_name, base64::encode(token)
    );
    //println!("reset: {:?}", token);

    send_email(email, subject, &body)
}

//use crate::bot::jobs::Environment;
use swirl::errors::PerformError;

// #[swirl::background_job]
// pub fn send_email_back_job(
//     env: &Environment,
//     ty: String,
//     email: String,
//     user_name: String,
//     token: String
// ) -> Result<(), PerformError> {
//     if ty.trim() == "reset" {
//         try_send_reset_email(&email, &user_name, &token)
//         .map_err(|e|{Box::new(dyn e.into())})
//     } else {
//         try_send_confirm_email(&email, &user_name, &token)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sending_to_invalid_email_fails() {
        let result = send_email(
            "String.Format(\"{0}.{1}@ruthub.com\", FirstName, LastName)",
            "test",
            "test",
        );
        assert!(result.is_err());
    }

    #[test]
    fn sending_to_valid_email_succeeds() {
        let result = send_email("****@gmail.com", "test", "test");
        assert!(result.is_ok());
    }
}
