use serde::Deserialize;

/// 监听配置
#[derive(Clone, Deserialize)]
#[serde(default)]
pub struct ListeningConfig {
    /// 监听地址, 如0.0.0.0:3000
    pub bind_addr: String,
    /// Webhook 路径，如 `/webhook`。
    pub webhook_path: String,
}

impl Default for ListeningConfig {
    fn default() -> Self {
        Self {
            bind_addr: String::from("0.0.0.0:3000"),
            webhook_path: String::from("/webhook"),
        }
    }
}

/// QQ 机器人官网下发的票据。
#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct CredentialConfig {
    pub app_id: String,
    pub secret: String,
}

/// 应用配置
#[derive(Default, Deserialize)]
#[serde(default)]
pub struct QQBotConfig {
    /// 监听配置
    pub listening: ListeningConfig,
    /// QQ 机器人票据配置。
    pub credential: CredentialConfig,
}

impl QQBotConfig {
    /// 创建使用默认配置的应用构建器。
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_addr(mut self, bind: &str) -> Self {
        self.listening.bind_addr = bind.to_string();
        self
    }

    pub fn webhook_path(mut self, path: &str) -> Self {
        self.listening.webhook_path = path.to_string();
        self
    }

    pub fn credential(mut self, credential: CredentialConfig) -> Self {
        self.credential = credential;
        self
    }
}
