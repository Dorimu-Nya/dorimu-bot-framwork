use serde::Deserialize;

/// 监听配置
#[derive(Clone, Deserialize)]
#[serde(default)]
pub struct ListeningConfig {
    /// 监听地址, 如0.0.0.0:3000
    pub bind_addr: String,
    /// webhook路径, 如/webhook
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

/// qqbot官网下发的票据
#[derive(Clone, Deserialize)]
#[serde(default)]
pub struct CredentialConfig {
    pub app_id: String,
    pub secret: String,
}

impl Default for CredentialConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            secret: String::new(),
        }
    }
}

/// 应用配置
#[derive(Deserialize)]
#[serde(default)]
pub struct QQBotConfig {
    /// 监听配置
    pub listening: ListeningConfig,
    /// qqbot票据配置
    pub credential: CredentialConfig,
    /// api地址覆写
    pub api_override: Option<String>,
}

impl Default for QQBotConfig {
    fn default() -> Self {
        Self {
            listening: Default::default(),
            credential: Default::default(),
            api_override: Default::default(),
        }
    }
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

    pub fn api_override(mut self, api: &str) -> Self {
        self.api_override = Some(api.to_string());
        self
    }
}
