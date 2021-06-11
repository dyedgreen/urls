use crate::{db, email, Context};
use clokwerk::{Interval, ScheduleHandle, Scheduler};
use std::time::Duration;
use tokio::runtime::Handle;

const BG_XSRF_TOKEN: &str = "background_job_xsrf";

mod check_old_urls;

/// Run scheduled background jobs.
pub fn watch_thread(
    async_runtime: Handle,
    pool: db::Pool,
    mailer: email::Mailer,
) -> ScheduleHandle {
    let mut scheduler = Scheduler::new();

    scheduler.every(Interval::Days(1)).run(move || {
        let ctx = Context::new(&pool, &mailer, BG_XSRF_TOKEN.to_string(), None);
        async_runtime.spawn(check_old_urls::job(ctx));
    });

    scheduler.watch_thread(Duration::from_millis(1000))
}
