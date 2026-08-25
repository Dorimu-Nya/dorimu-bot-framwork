use super::event_registry::EventHandlerRegistry;
use qqbot_rust_sdk::openapi::api::QQApiClient as SdkQQApiClient;
use qqbot_rust_sdk::openapi::create_api_client::create_qq_api_client;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::config::{CredentialConfig, ListeningConfig, QQBotConfig};

pub type QQApiClient = SdkQQApiClient;

#[derive(Clone)]
pub struct QQBot {
    /// 票据配置
    pub(crate) credential: CredentialConfig,
    /// Webhook 监听配置。
    listening: ListeningConfig,
    /// 按需初始化的 API 客户端。
    api_client: Arc<OnceCell<Arc<QQApiClient>>>,
    /// 当前应用实例注册的事件处理器。
    pub(crate) event_handlers: EventHandlerRegistry,
}

impl QQBot {
    /// 根据应用配置创建机器人；API 客户端将在首次使用时初始化。
    pub fn new(config: QQBotConfig) -> Self {
        Self {
            credential: config.credential.clone(),
            listening: config.listening.clone(),
            api_client: Arc::new(OnceCell::new()),
            event_handlers: EventHandlerRegistry::new(),
        }
    }

    /// 获取 Webhook 监听配置。
    pub fn listening_config(&self) -> &ListeningConfig {
        &self.listening
    }

    /// 获取 API 客户端，首次调用时获取 token 并完成初始化。
    pub async fn get_api_client(&self) -> Result<Arc<QQApiClient>, Box<dyn Error + Send + Sync>> {
        let app_id = self.credential.app_id.clone();
        let secret = self.credential.secret.clone();
        let client = self
            .api_client
            .get_or_try_init(|| async move {
                create_qq_api_client(app_id, secret)
                    .await
                    .map(Arc::new)
                    .map_err(|error| -> Box<dyn Error + Send + Sync> { Box::new(error) })
            })
            .await?;
        Ok(Arc::clone(client))
    }
}
