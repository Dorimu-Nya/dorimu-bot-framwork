use super::event_registry::EventHandlerRegistry;
use qqbot_rust_sdk::openapi::{
    models::User, HttpTokenProvider, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths,
    TokenManager,
};
use std::sync::Arc;
use std::time::Duration;

use crate::config::{CredentialConfig, ListeningConfig, QQBotConfig};

pub type QQApiCLient = OpenApi<HttpTokenProvider>;

#[derive(Clone)]
pub struct QQBot {
    /// 票据配置
    pub(crate) credential: CredentialConfig,
    /// Webhook 监听配置。
    listening: ListeningConfig,
    /// 生产环境的 api 客户端
    prod_api_client: Arc<QQApiCLient>,
    /// 当前机器人用户信息。
    bot_info: Option<User>,
    /// 当前应用实例注册的事件处理器。
    pub(crate) event_handlers: EventHandlerRegistry,
}

impl QQBot {
    /// 根据应用配置初始化 API、机器人信息和事件处理器。
    pub async fn new(config: QQBotConfig) -> Self {
        // api 客户端初始化
        let token_provider = HttpTokenProvider::from_env_or_official(
            &config.credential.app_id,
            &config.credential.secret,
        );
        let token_manager = TokenManager::new(token_provider, Duration::from_secs(120));
        let mut openapi_config = OpenApiConfig::official();
        if let Some(url) = &config.api_override {
            openapi_config.base_url = url.clone();
        }
        let client = OpenApiClient::new(token_manager, openapi_config);
        let api = Arc::new(OpenApi::new(client, OpenApiPaths::official_defaults()));
        // api 客户端初始化 end

        let bot_info = match api.users().me().await {
            Ok((status, bot_info)) if status.is_success() => Some(bot_info),
            Ok((status, _)) => {
                tracing::warn!(%status, "获取当前机器人用户信息失败");
                None
            }
            Err(error) => {
                tracing::warn!(%error, "获取当前机器人用户信息失败");
                None
            }
        };

        Self {
            credential: config.credential.clone(),
            listening: config.listening.clone(),
            prod_api_client: api,
            bot_info,
            event_handlers: EventHandlerRegistry::new(),
        }
    }

    /// 获取 Webhook 监听配置。
    pub fn listening_config(&self) -> &ListeningConfig {
        &self.listening
    }

    /// 获取 api 客户端。
    pub fn get_api_client(&self) -> Arc<QQApiCLient> {
        Arc::clone(&self.prod_api_client)
    }

    /// 获取当前机器人用户信息。
    pub fn bot_info(&self) -> Option<&User> {
        self.bot_info.as_ref()
    }
}
