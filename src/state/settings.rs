use std::time::{Duration, Instant};

use tokio::sync::oneshot::Receiver;
use tui_input::Input;


#[derive(Debug)]
pub struct Settings {
    pub host: Input,
    pub model_name: Input,
    pub api_key: Input,
    pub status: SettingStatus,
}

#[derive(Debug)]
pub enum SettingStatus {
    Unhealthy(&'static str),
    Checking(Instant, Receiver<Duration>),
    Healthy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            host: Input::new("".to_string()),
            model_name: Input::new("".to_string()),
            api_key: Input::new("".to_string()),
            status: SettingStatus::Unhealthy("Host is empty"),
        }
    }
}

impl Settings {
    pub fn health_check(&mut self) {
        if self.host.value().is_empty() {
            self.status = SettingStatus::Unhealthy("Host is empty");
            return;
        }
        if self.model_name.value().is_empty() {
            self.status = SettingStatus::Unhealthy("Model name is empty");
            return;
        }
        if self.api_key.value().is_empty() {
            self.status = SettingStatus::Unhealthy("API key is empty");
            return;
        }

        let (tx, rx) = tokio::sync::oneshot::channel::<Duration>();
        let start_time = std::time::Instant::now();
        self.status = SettingStatus::Checking(start_time, rx);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(3)).await;
            let duration = start_time.elapsed();
            let _ = tx.send(duration);
        });
    }

    pub fn tick(&mut self) {
        if let SettingStatus::Checking(start_time, rx) = &mut self.status {
            if let Ok(_) = rx.try_recv() {
                self.status = SettingStatus::Healthy;
            }
        }
    }
}

