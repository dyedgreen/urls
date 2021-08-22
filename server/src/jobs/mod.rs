use crate::{db, email, Context};
use clokwerk::{Interval, ScheduleHandle, Scheduler};
use std::time::Duration;
use tokio::runtime::Handle;

mod check_old_urls;
mod index_urls;

fn schedule<J, F>(
    scheduler: &mut Scheduler,
    interval: Interval,
    pool: &db::Pool,
    mailer: &email::Mailer,
    runtime: &Handle,
    job: J,
) where
    J: Fn(Context) -> F + Send + 'static,
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    let pool = pool.clone();
    let mailer = mailer.clone();
    let runtime = runtime.clone();
    scheduler.every(interval).run(move || {
        let ctx = Context::for_server(&pool, &mailer);
        runtime.spawn((job)(ctx));
    });
}

/// Run scheduled background jobs.
pub fn watch_thread(
    async_runtime: Handle,
    pool: db::Pool,
    mailer: email::Mailer,
) -> ScheduleHandle {
    let mut scheduler = Scheduler::new();

    schedule(
        &mut scheduler,
        Interval::Days(1),
        &pool,
        &mailer,
        &async_runtime,
        check_old_urls::job,
    );

    schedule(
        &mut scheduler,
        Interval::Minutes(1),
        &pool,
        &mailer,
        &async_runtime,
        index_urls::job,
    );

    scheduler.watch_thread(Duration::from_millis(1000))
}
