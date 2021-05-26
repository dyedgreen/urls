pub use server::*;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("LOG", "info")).init();
    let pool = db::connect(&config::ENV)
        .await
        .map_err(|err| log::error!("Failed to connect to database: {}", err))
        .unwrap();
    let mailer = email::connect(&config::ENV)
        .await
        .map_err(|err| log::error!("Failed to connect to mailer: {}", err))
        .unwrap();
    let server = global_routes(&config::ENV, pool, mailer);
    warp::serve(server).run(([0, 0, 0, 0], 8080)).await;
}
