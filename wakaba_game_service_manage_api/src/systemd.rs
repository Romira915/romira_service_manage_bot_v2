use axum::async_trait;

#[async_trait]
pub trait SystemdControl: Send + Sync {
    async fn start(&self) -> anyhow::Result<()>;
    async fn is_active(&self) -> anyhow::Result<bool>;
    async fn stop(&self) -> anyhow::Result<()>;
    async fn restart(&self) -> anyhow::Result<()>;
}

pub(crate) struct Systemd {
    service_name: String,
}

impl Systemd {
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
        }
    }
}

#[async_trait]

impl SystemdControl for Systemd {
    async fn start(&self) -> anyhow::Result<()> {
        let output = tokio::process::Command::new("systemctl")
            .arg("start")
            .arg(&self.service_name)
            .output()
            .await?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to start service {}: {}",
                self.service_name,
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }

    async fn is_active(&self) -> anyhow::Result<bool> {
        let status = tokio::process::Command::new("systemctl")
            .arg("is-active")
            .arg(&self.service_name)
            .spawn()?
            .wait()
            .await?;

        Ok(status.success())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let output = tokio::process::Command::new("systemctl")
            .arg("stop")
            .arg(&self.service_name)
            .output()
            .await?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to stop service {}: {}",
                self.service_name,
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }

    async fn restart(&self) -> anyhow::Result<()> {
        let output = tokio::process::Command::new("systemctl")
            .arg("restart")
            .arg(&self.service_name)
            .output()
            .await?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to restart service {}: {}",
                self.service_name,
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }
}

//noinspection NonAsciiCharacters
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_systemd_is_active_がtrueを返す() {
        let systemd = Systemd::new("systemd-resolved");

        assert!(systemd.is_active().await.unwrap());
    }

    #[tokio::test]
    async fn test_systemd_start_がsudo権限がないのでfailする() {
        let systemd = Systemd::new("systemd-resolved");

        let result = systemd.start().await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_systemd_stop_がsudo権限がないのでfailする() {
        let systemd = Systemd::new("systemd-resolved");

        let result = systemd.stop().await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_systemd_restart_がsudo権限がないのでfailする() {
        let systemd = Systemd::new("systemd-resolved");

        let result = systemd.restart().await;

        assert!(result.is_err());
    }
}
