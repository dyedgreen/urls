use crate::db::id::UrlID;
use crate::db::models::Url;
use crate::schema::urls;
use crate::Context;
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult, ID};
use juniper_relay::RelayConnection;
use std::collections::HashMap;

pub struct Search(String);

impl Search {
    pub fn new(query: String) -> Self {
        Self(query)
    }
}

#[graphql_object(context = Context)]
impl Search {
    pub fn id(&self) -> ID {
        self.0.clone().into()
    }

    /// The list of results returned by this search.
    pub async fn results(
        &self,
        ctx: &Context,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> FieldResult<RelayConnection<Url>> {
        // TODO(dyedgreen): Use the offset/ limit as cursors ...
        let results = ctx.search().find(&self.0)?;
        let conn = ctx.conn().await?;
        let urls = RelayConnection::new(first, after, last, before, |after, before, _| {
            let mut urls: HashMap<UrlID, Url> = urls::table
                .filter(urls::id.eq_any(&results))
                .load::<Url>(&*conn)?
                .into_iter()
                .map(|url| (url.id(), url))
                .collect();
            drop(conn);

            let results = results
                .into_iter()
                .skip_while(|&id| after.map(|a| a != id).unwrap_or(false))
                .skip(after.map(|_| 1).unwrap_or(0))
                .take_while(|&id| before.map(|b| b != id).unwrap_or(true))
                .filter_map(|id| urls.remove(&id))
                .collect();
            Ok(results)
        })?;
        Ok(urls)
    }
}
