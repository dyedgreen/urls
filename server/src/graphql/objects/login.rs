use crate::db::id::LoginID;
use crate::db::models::Login;
use crate::Context;
use chrono::{DateTime, Utc};
use juniper::{graphql_object, FieldResult, GraphQLObject};
use juniper_relay_connection::RelayConnectionNode;
use serde::Deserialize;

impl RelayConnectionNode for Login {
    type Cursor = LoginID;

    fn cursor(&self) -> Self::Cursor {
        self.id()
    }

    fn connection_type_name() -> &'static str {
        "LoginConnection"
    }

    fn edge_type_name() -> &'static str {
        "LoginConnectionEdge"
    }
}

#[derive(GraphQLObject)]
struct UserAgent<'a> {
    /// The raw user agent string.
    raw: &'a str,
    /// The name of the user agent (e.g. the browser used).
    name: &'a str,
    /// The name of the user agents operating system.
    operating_system: &'a str,
}

#[derive(GraphQLObject, Deserialize)]
struct LoginLocation {
    ip_address: String,
    country_code: Option<String>,
    country_name: Option<String>,
    city_name: Option<String>,
}

#[graphql_object(context = Context)]
impl Login {
    /// A globally unique identifier for this
    /// login session.
    fn id(&self) -> LoginID {
        self.id()
    }

    /// Last time this login session was used.
    fn last_used(&self) -> DateTime<Utc> {
        self.last_used()
    }

    /// The user agent that last used this
    /// session.
    fn last_user_agent(&self) -> Option<UserAgent<'_>> {
        self.last_user_agent().and_then(|raw| {
            use woothee::parser::Parser;
            let parser = Parser::new();
            parser.parse(raw).map(|res| UserAgent {
                raw,
                name: res.name,
                operating_system: res.os,
            })
        })
    }

    /// The last known location from where this
    /// login session was used. This information
    /// is approximate and based on the remote IP
    /// address.
    async fn last_location(&self, ctx: &Context) -> FieldResult<Option<LoginLocation>> {
        if let Some(ip_addr) = self.last_remote_ip() {
            let location: LoginLocation = ctx
                .http_client()
                .get(format!("https://api.geoip.rs/?ip={}", ip_addr))
                .send()
                .await?
                .json()
                .await?;
            Ok(Some(location))
        } else {
            Ok(None)
        }
    }
}
