use anyhow::Result;
use dotenv::var;
use nanoid::nanoid;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};

static DEFAULT_WWW: &str = "www/static";
static DEFAULT_SMTP_PORT: u16 = 587;

static ENV: Lazy<Config> = Lazy::new(|| match load_from_env() {
    Ok(conf) => conf,
    Err(msg) => {
        log::error!("Failed to load configuration: {}", msg);
        panic!("Failed to load configuration: {}", msg);
    }
});

#[derive(Debug, Clone)]
pub struct Config {
    database_url: String,
    www_dir: PathBuf,
    session_key: Vec<u8>,
    hostname: String,
    smtp: Option<SmtpConfig>,
}

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    host: String,
    port: Option<u16>,
    user: String,
    password: String,
}

impl Config {
    /// Configuration loaded from the
    /// environment.
    pub fn env() -> &'static Self {
        &ENV
    }

    /// Configuration suitable for unit
    /// or integration tests. Database
    /// connections are in-memory, and no
    /// smtp config is provided.
    pub fn test() -> Self {
        Self {
            database_url: format!("file:{}?mode=memory&cache=shared", nanoid!(16)),
            www_dir: DEFAULT_WWW.into(),
            session_key: random_session_key(),
            hostname: "localhost".into(),
            smtp: None,
        }
    }

    /// SQLite database URI.
    pub fn database(&self) -> &str {
        self.database_url.as_str()
    }

    /// Extract the database file from the
    /// SQLite URI.
    pub fn database_file(&self) -> &Path {
        let uri = self.database();
        let path = match uri.get(0..5) {
            Some("file:") => uri[5..].split_once('?').map(|p| p.0).unwrap_or(&uri[5..]),
            Some(_) | None => uri,
        };
        Path::new(path)
    }

    /// Directory to serve static files
    /// from.
    pub fn www(&self) -> &Path {
        self.www_dir.as_path()
    }

    /// SMTP server host and credentials.
    pub fn smtp(&self) -> Option<&SmtpConfig> {
        self.smtp.as_ref()
    }

    /// Key to sign session strings.
    pub fn session_key(&self) -> &[u8] {
        &self.session_key[..]
    }

    /// Host name to use in communications and things
    /// like API responses. E.g. `http://localhost:8080`.
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
}

impl SmtpConfig {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(DEFAULT_SMTP_PORT)
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

fn random_session_key() -> Vec<u8> {
    nanoid!(64).into()
}

fn load_from_env() -> Result<Config> {
    let database_url = var("DATABASE_URL")?;

    let www_dir = var("WWW_DIR")
        .unwrap_or_else(|_| {
            log::info!(
                "WWW_DIR configuration not set, using default '{}'",
                DEFAULT_WWW
            );
            DEFAULT_WWW.to_string()
        })
        .into();

    let smtp = match (var("SMTP_HOST"), var("SMTP_USER"), var("SMTP_PASS")) {
        (Ok(host), Ok(user), Ok(password)) => Some(SmtpConfig {
            host,
            port: var("SMTP_PORT").ok().and_then(|port| {
                port.parse()
                    .map_err(|_| {
                        log::warn!("Invalid SMTP_PORT set, using default {}", DEFAULT_SMTP_PORT);
                    })
                    .ok()
            }),
            user,
            password,
        }),
        _ => {
            log::info!("SMTP_HOST, SMTP_PORT, SMTP_USER, or SMTP_PASS not set");
            None
        }
    };

    let session_key = var("SESSION_KEY").map(Into::into).unwrap_or_else(|_| {
        log::warn!("SESSION_KEY configuration not set, using random key");
        random_session_key()
    });

    let hostname = var("HOSTNAME")?;

    Ok(Config {
        database_url,
        www_dir,
        smtp,
        session_key,
        hostname,
    })
}
