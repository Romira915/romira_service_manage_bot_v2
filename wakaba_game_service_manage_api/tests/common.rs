use axum::async_trait;
use std::sync::Arc;
use wakaba_game_service_manage_api::{AppState, SystemdControl};

pub trait AppStateTest {
    fn default_for_test() -> Self;
    fn sdtd_systemd(self, sdtd_systemd: Arc<dyn SystemdControl>) -> Self;
}

impl AppStateTest for AppState {
    fn default_for_test() -> Self {
        Self::new(Arc::new(MockSystemd {
            is_active_return_value: true,
        }))
    }

    fn sdtd_systemd(self, sdtd_systemd: Arc<dyn SystemdControl>) -> Self {
        Self {
            sdtd_systemd,
            ..self
        }
    }
}

pub struct MockSystemd {
    pub(crate) is_active_return_value: bool,
}

#[async_trait]
impl SystemdControl for MockSystemd {
    async fn start(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn is_active(&self) -> anyhow::Result<bool> {
        Ok(self.is_active_return_value)
    }

    async fn stop(&self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn restart(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
