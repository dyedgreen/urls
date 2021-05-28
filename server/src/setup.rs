use crate::db::models::{NewUserInput, Permission, Role, User};
use crate::db::Pool;
use crate::email::Mailer;
use crate::schema::roles;
use crate::Context;
use anyhow::Result;
use diesel::prelude::*;
use std::io::{stdin, stdout, Write};

/// Check if any administrator is registered and if not,
/// start an interactive registration flow in the terminal
/// on startup.
pub async fn run(pool: &Pool, mailer: &Mailer) -> Result<()> {
    let ctx = Context::new(pool, mailer, "".into(), None);

    let admin_count: i64 = roles::table
        .filter(roles::dsl::permission.eq(Permission::Administrator))
        .select(diesel::dsl::count_star())
        .get_result(&*ctx.conn().await?)?;

    if admin_count == 0 {
        println!("No administrators where found, please register an administrator.");

        print!("Name: ");
        stdout().flush()?;
        let mut name = String::new();
        stdin().read_line(&mut name)?;

        print!("Email address: ");
        stdout().flush()?;
        let mut email = String::new();
        stdin().read_line(&mut email)?;

        let admin = User::create(&ctx, NewUserInput { name, email }).await?;
        Role::create(&ctx, admin.id(), Permission::Administrator).await?;

        println!(
            "Administrator {} ({}) was registered!",
            admin.name(),
            admin.email()?
        );
        Ok(())
    } else {
        Ok(())
    }
}
