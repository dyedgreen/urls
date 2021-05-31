use crate::Config;
use anyhow::Result;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncFileTransport, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use std::sync::Arc;
use tokio::sync::Mutex;

const DEBUG_MAIL_PATH: &'static str = "./emails";

/// A mailer can be used to send email messages.
#[derive(Debug, Clone)]
pub enum Mailer {
    Smtp(AsyncSmtpTransport<Tokio1Executor>),
    File {
        path: &'static str,
        last_message: Arc<Mutex<Option<String>>>,
    },
}

impl Mailer {
    /// Send an email message.
    pub async fn send(&self, message: Message) -> Result<()> {
        match self {
            Mailer::Smtp(mailer) => {
                log::info!(
                    "Sending email to: {}",
                    message
                        .envelope()
                        .to()
                        .into_iter()
                        .map(|add| add.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                mailer.send(message).await?;
                Ok(())
            }
            Mailer::File { path, last_message } => {
                let mailer: AsyncFileTransport<Tokio1Executor> = AsyncFileTransport::new(path);
                let id = mailer.send(message).await?;
                *last_message.lock().await = Some(format!("{}/{}.eml", DEBUG_MAIL_PATH, id));
                Ok(())
            }
        }
    }
}

/// Connect to the mail transport. This returns a mailer
/// which can be used to send messages. The mailer can
/// be cloned and shared across multiple threads.
pub async fn connect(config: &Config) -> Result<Mailer> {
    if let Some(conf) = config.smtp() {
        let creds = Credentials::new(conf.user().to_string(), conf.password().to_string());
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(conf.host())?
            .port(conf.port())
            .credentials(creds)
            .build();
        log::info!("Emails will be sent via smtp");
        Ok(Mailer::Smtp(mailer))
    } else {
        if let Err(err) = std::fs::create_dir(DEBUG_MAIL_PATH) {
            if err.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(err.into());
            }
        }
        log::warn!(
            "Emails will be saved to {}, only use this in development",
            DEBUG_MAIL_PATH
        );
        Ok(Mailer::File {
            path: DEBUG_MAIL_PATH.into(),
            last_message: Arc::new(Mutex::new(None)),
        })
    }
}
