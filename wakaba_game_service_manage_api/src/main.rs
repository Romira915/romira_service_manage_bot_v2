use listenfd::ListenFd;
use wakaba_game_service_manage_api::config::CONFIG;
use wakaba_game_service_manage_api::{app, AppState};

#[tokio::main]
async fn main() {
    easy_init_newrelic_opentelemetry::NewRelicSubscriberInitializer::default()
        .newrelic_license_key(&CONFIG.newrelic_license_key)
        .newrelic_service_name(&CONFIG.newrelic_service_name)
        .host_name("localhost")
        .init()
        .expect("Failed to initialize New Relic OpenTelemetry");

    let app = app(AppState::default());

    // run it
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            tokio::net::TcpListener::from_std(listener).unwrap()
        }
        None => tokio::net::TcpListener::bind(&CONFIG.listen_address)
            .await
            .unwrap(),
    };
    tracing::debug!("listening on {}", CONFIG.listen_address);
    axum::serve(listener, app).await.unwrap();
}
