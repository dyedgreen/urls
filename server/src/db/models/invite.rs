use crate::db::id::{InviteID, UserID};
use crate::db::models::User;
use crate::schema::invites;
use crate::Context;
use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use nanoid::nanoid;

const MAX_INVITES_PER_USER: i64 = 3;
const TOKEN_ALPHABET: &[char] = &[
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset)]
pub struct Invite {
    id: InviteID,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,

    token: String,
    created_by: UserID,
    claimed_by: Option<UserID>,
}

impl Invite {
    pub fn id(&self) -> InviteID {
        self.id
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.created_at, Utc)
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.updated_at, Utc)
    }
}

impl Invite {
    /// Create a new invite issued by the given user.
    pub async fn create(ctx: &Context, created_by: &User) -> Result<Self> {
        let total_invites_issued: i64 = invites::table
            .filter(invites::dsl::created_by.eq(created_by.id()))
            .select(diesel::dsl::count_star())
            .get_result(&*ctx.conn().await?)?;

        let allow_invite = total_invites_issued < MAX_INVITES_PER_USER
            || created_by
                .permissions(ctx)
                .await?
                .into_iter()
                .any(|perm| perm.unlimited_invites());
        if allow_invite {
            let invite = Invite {
                id: InviteID::new(),
                created_at: ctx.now().naive_utc(),
                updated_at: ctx.now().naive_utc(),

                token: nanoid!(32, TOKEN_ALPHABET),
                created_by: created_by.id(),
                claimed_by: None,
            };
            diesel::insert_into(invites::table)
                .values(&invite)
                .execute(&*ctx.conn().await?)?;

            Ok(invite)
        } else {
            Err(anyhow!(
                "This account is not allowed to issue more than {} invitations",
                MAX_INVITES_PER_USER
            ))
        }
    }

    /// Claim this invite for the given
    /// user.
    pub async fn claim(&mut self, ctx: &Context, claimed_by: &User) -> Result<()> {
        if self.claimed_by.is_some() {
            Err(anyhow!("This invitation is already claimed"))
        } else {
            self.claimed_by = Some(claimed_by.id());
            self.updated_at = ctx.now().naive_utc();
            *self = self.save_changes(&*ctx.conn().await?)?;
            Ok(())
        }
    }
}