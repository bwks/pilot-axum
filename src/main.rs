use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use pilot::config::PilotConfig;
use pilot::startup::{app, run};

#[tokio::main]
async fn main() {
    let pilot_config = PilotConfig::load().expect("failed to read configuration");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("pilot_axum={}", pilot_config.logging.level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = TcpListener::bind(format!(
        "{}:{}",
        pilot_config.ipv4_listener.address, pilot_config.ipv4_listener.port
    ))
    .await
    .expect("error binding TCP listener");

    let db_connection_str = pilot_config.database.connection_string();

    // set up connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("can't run migrations");

    // build our application with a route
    let app = app();

    // Run forever-ish...
    if let Err(err) = run(listener, app).await {
        // run it
        eprintln!("server error: {}", err);
    };
}
