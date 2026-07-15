#[cfg(feature = "builtin-message-handler")]
use super::commands::plugin::CommandPlugin;
use super::AppConfig;
use super::ContextStore;
use crate::openapi::{
    HttpTokenProvider, OpenApi, OpenApiClient, OpenApiConfig, OpenApiPaths, TokenManager,
};
use crate::CredentialConfig;
use std::sync::Arc;
use std::time::Duration;

pub type ApiClient = OpenApi<HttpTokenProvider>;

#[derive(Clone)]
pub struct App {
    /// 票据配置
    pub(crate) credential: CredentialConfig,
    /// 生产环境的 api 客户端
    prod_api_client: Arc<ApiClient>,
    /// 依赖容器
    pub dependency_container: ContextStore,
}

impl App {
    /// 根据应用配置初始化 API、依赖容器、命令插件和事件处理器。
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

        // 初始化ioc
        let container = ContextStore::new();
        if !config.ignore_checking {
            if let Some(context) = container.insert_arc(Arc::clone(&api)) {
                panic!("Context {:?} duplicated", context);
            }
        } else {
            container.insert_arc(Arc::clone(&api));
        }

        for register in &config.contexts {
            if !config.ignore_checking {
                if let Some(context) = register(&container) {
                    panic!("Context {:?} duplicated", context);
                }
            } else {
                register(&container);
            }
        }

        let app = Self {
            credential: config.credential.clone(),
            prod_api_client: api,
            dependency_container: container,
        };

        #[cfg(feature = "builtin-message-handler")]
        app.registe_plugin(&CommandPlugin::from_config(&config, &app));

        app
    }

    /// 获取 api 客户端。
    pub fn get_api_client(&self) -> Arc<ApiClient> {
        //TODO: 沙盒分配策略
        return self.get_prod_client();
    }

    /// 获取生产环境的 api 客户端。
    pub fn get_prod_client(&self) -> Arc<ApiClient> {
        Arc::clone(&self.prod_api_client)
    }
}
