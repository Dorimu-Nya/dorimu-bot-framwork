use crate::EventHandlerRegistry;
use qqbot_rust_sdk::openapi::{
    HttpTokenProvider, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths, TokenManager,
};
use std::sync::Arc;
use std::time::Duration;

use crate::config::{AppConfig, CredentialConfig};

pub type QQApiCLient = OpenApi<HttpTokenProvider>;

#[derive(Clone)]
pub struct QQBotApp {
    /// 票据配置
    pub(crate) credential: CredentialConfig,
    /// 生产环境的 api 客户端
    prod_api_client: Arc<QQApiCLient>,
    /// 当前应用实例注册的事件处理器。
    pub(crate) event_handlers: EventHandlerRegistry,
}

impl QQBotApp {
    /// 根据应用配置初始化 API、插件和事件处理器。
    pub fn new(config: AppConfig) -> Self {
        // api 客户端初始化
        let token_provider = HttpTokenProvider::from_env_or_official(
            &config.credential.app_id,
            &config.credential.secret,
        );
        let token_manager = TokenManager::new(token_provider, Duration::from_secs(120));
        let mut openapi_config = OpenApiConfig::official();
        if let Some(url) = &config.api_overrides.prod_url_override {
            openapi_config.base_url = url.clone();
        }
        let client = OpenApiClient::new(token_manager, openapi_config);
        let api = Arc::new(OpenApi::new(client, OpenApiPaths::official_defaults()));
        // api 客户端初始化 end

        let app = Self {
            credential: config.credential.clone(),
            prod_api_client: api,
            event_handlers: EventHandlerRegistry::new(),
        };

        for plugin in &config.plugins {
            app.registe_plugin(plugin.as_ref());
        }

        app
    }

    /// 获取 api 客户端。
    pub fn get_api_client(&self) -> Arc<QQApiCLient> {
        //TODO: 沙盒分配策略
        return self.get_prod_client();
    }

    /// 获取生产环境的 api 客户端。
    pub fn get_prod_client(&self) -> Arc<QQApiCLient> {
        Arc::clone(&self.prod_api_client)
    }
}
