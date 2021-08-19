use crate::db::id::LoginID;
use crate::db::models::Login;
use crate::Context;
use chrono::{DateTime, Utc};
use juniper::{graphql_object, GraphQLObject};
use juniper_relay::RelayConnectionNode;

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
}
