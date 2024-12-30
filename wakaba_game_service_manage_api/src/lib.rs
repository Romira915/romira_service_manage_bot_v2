use axum::extract::FromRef;
use axum::middleware;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
pub use systemd::SystemdControl;

pub mod config;
mod custom_middleware;
pub(crate) mod domain;
mod handler;
mod systemd;

pub fn app(app_state: AppState) -> Router {
    let api = Router::new()
        .route("/sdtd", post(handler::sdtd_handler))
        .route("/wol", post(handler::wol_handler))
        .layer(middleware::from_fn(custom_middleware::require_auth))
        .with_state(app_state);

    Router::new().nest("/api", api)
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub sdtd_systemd: Arc<dyn SystemdControl>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sdtd_systemd: Arc::new(systemd::Systemd::new("sdtdserver.service")),
        }
    }
}

impl AppState {
    pub fn new(sdtd_control: Arc<dyn SystemdControl>) -> Self {
        Self {
            sdtd_systemd: sdtd_control,
        }
    }
}
