use anyhow::Result;
use dotenv::var;
use nanoid::nanoid;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};

static DEFAULT_WWW: &str = "www/static";
static DEFAULT_TEMPLATES: &str = "www/templates/**/*.html";

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
    templates_glob: String,
    session_key: Vec<u8>,
    hostname: String,
    smtp: Option<SmtpConfig>,
}

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    host: String,
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
            templates_glob: DEFAULT_TEMPLATES.into(),
            session_key: random_session_key(),
            hostname: "localhost".into(),
            smtp: None,
        }
    }

    /// SQLite database file.
    pub fn database(&self) -> &str {
        self.database_url.as_str()
    }

    /// Directory to serve static files
    /// from.
    pub fn www(&self) -> &Path {
        self.www_dir.as_path()
    }

    /// Directory glob to load template files
    /// from.
    pub fn templates(&self) -> &str {
        self.templates_glob.as_str()
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

    let templates_glob = var("TEMPLATES_DIR").unwrap_or_else(|_| {
        log::info!(
            "TEMPLATES_DIR configuration not set, using default '{}'",
            DEFAULT_TEMPLATES
        );
        DEFAULT_TEMPLATES.to_string()
    });

    let smtp = match (var("SMTP_HOST"), var("SMTP_USER"), var("SMTP_PASS")) {
        (Ok(host), Ok(user), Ok(password)) => Some(SmtpConfig {
            host,
            user,
            password,
        }),
        _ => {
            log::info!("SMTP_HOST, SMTP_USER, or SMTP_PASS not set");
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
        templates_glob,
        smtp,
        session_key,
        hostname,
    })
}
