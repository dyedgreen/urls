pub use server::*;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("LOG", "info")).init();

    let pool = db::connect(Config::env())
        .await
        .map_err(|err| log::error!("Failed to connect to database: {}", err))
        .unwrap();
    let mailer = email::connect(Config::env())
        .await
        .map_err(|err| log::error!("Failed to connect to mailer: {}", err))
        .unwrap();

    setup::run(&pool, &mailer)
        .await
        .map_err(|err| log::error!("Failed to run setup: {}", err))
        .unwrap();

    let server = global_routes(Config::env(), pool, mailer);
    warp::serve(server).run(([0, 0, 0, 0], 8080)).await;
}
