use axum::async_trait;
use std::sync::Arc;
use wakaba_game_service_manage_api::{AppState, SystemdControl};

struct MockSystemd;

#[async_trait]
impl SystemdControl for MockSystemd {
    async fn start(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn restart(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn is_active(&self) -> anyhow::Result<bool> {
        Ok(true)
    }
}

pub fn app_state_for_test() -> AppState {
    AppState::new(Arc::new(MockSystemd))
}
