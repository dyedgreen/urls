use juniper::{FieldResult, GraphQLObject};
use std::convert::TryInto;

mod traits;

/// To return objects inside a connection, they must
/// implement this trait.
pub trait RelayConnectionNode {
    type Cursor: std::string::ToString + std::str::FromStr + Clone;

    /// Returns the cursor associated with this node.
    fn cursor(&self) -> Self::Cursor;

    /// Returns the type name connections
    /// over these nodes should have in the
    /// API.
    fn connection_type_name() -> &'static str;

    /// Returns the type name edges containing
    /// these nodes should have in the API.
    fn edge_type_name() -> &'static str;
}

#[derive(Debug)]
pub struct RelayConnectionEdge<N> {
    node: N,
    cursor: String,
}

#[derive(Debug, GraphQLObject)]
#[graphql(name = "PageInfo")]
pub struct RelayConnectionPageInfo {
    has_previous_page: bool,
    has_next_page: bool,
    start_cursor: Option<String>,
    end_cursor: Option<String>,
}

/// Implements the relay connection [specification][spec], and allows to
/// easily paginate over any given list of GraphQL objects.
///
/// [spec]: https://relay.dev/assets/files/connections-61fc54c286f0afc0b4f230f7c4b150bf.htm
#[derive(Debug)]
pub struct RelayConnection<N> {
    edges: Vec<RelayConnectionEdge<N>>,
    page_info: RelayConnectionPageInfo,
}

fn leq_zero(val: i64) -> Result<i64, &'static str> {
    if val < 0 {
        Err("Pagination argument must be positive")
    } else {
        Ok(val)
    }
}

impl<N> RelayConnection<N> {
    /// Build a relay-style paginated list. You must supply a
    /// closure which is used to load the data from some backing
    /// store. It takes arguments: `after: Option<C>`,
    /// `before: Option<C>`, and `limit: Option<i64>`.
    ///
    /// The `limit` argument is purely an optimization and may
    /// be ignored, without breaking the connection specification.
    pub fn new<F>(
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
        load: F,
    ) -> FieldResult<RelayConnection<N>>
    where
        N: RelayConnectionNode,
        F: FnOnce(Option<N::Cursor>, Option<N::Cursor>, Option<i64>) -> FieldResult<Vec<N>>,
        <N::Cursor as std::str::FromStr>::Err: std::fmt::Display,
    {
        let after: Option<N::Cursor> = after.map(|s| s.parse()).transpose()?;
        let before: Option<N::Cursor> = before.map(|s| s.parse()).transpose()?;

        let first: Option<i64> = first.map(Into::into).map(leq_zero).transpose()?;
        let last: Option<i64> = last.map(Into::into).map(leq_zero).transpose()?;

        // to ensure `hasNextPage` can be set correctly
        let limit = first.map(|l| l + 1);
        let edges = load(after, before, limit)?;
        let edges_len: i64 = edges.len().try_into()?;

        let has_previous_page = if let Some(last) = last {
            edges_len > last
        } else {
            false
        };
        let has_next_page = if let Some(first) = first {
            edges_len > first
        } else {
            false
        };

        let first = first.unwrap_or(edges_len);
        let last = last.unwrap_or(edges_len);

        let len_after_take = i64::min(edges_len, first);
        let skip = i64::max(0, len_after_take - last);

        let edges: Vec<RelayConnectionEdge<N>> = edges
            .into_iter()
            .take(first.try_into()?)
            .skip(skip.try_into()?)
            .map(|node| RelayConnectionEdge {
                cursor: node.cursor().to_string(),
                node,
            })
            .collect();

        Ok(RelayConnection {
            page_info: RelayConnectionPageInfo {
                has_previous_page,
                has_next_page,
                start_cursor: edges.first().map(|edge| edge.cursor.clone()),
                end_cursor: edges.last().map(|edge| edge.cursor.clone()),
            },
            edges,
        })
    }

    /// Returns a relay connection with no elements.
    pub fn empty() -> Self {
        Self {
            edges: vec![],
            page_info: RelayConnectionPageInfo {
                has_previous_page: false,
                has_next_page: false,
                start_cursor: None,
                end_cursor: None,
            },
        }
    }
}
