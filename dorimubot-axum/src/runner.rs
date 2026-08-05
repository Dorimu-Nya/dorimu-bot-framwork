use axum::routing::any;
use axum::{Json, Router};
use dorimubot_framework_core::QQBot;
use dorimubot_framework_core::qqbot_rust_sdk::events::payload::payload::WebhookPayload;
use std::sync::Arc;
use tracing::info;

/// 启动基于Axum的 QQ Bot 程序
///
/// * `app` - 已完成事件和命令注册的应用
/// * `base_router` axum的router，当为Some时，将会以其为基础构造axum的路由
/// example:
/// ```no_run
/// use dorimubot_framework_core::{QQBotConfig, CredentialConfig, QQBot};
/// use dorimubot_axum::run_axum;
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     let config = QQBotConfig {
///         credential: CredentialConfig {
///             app_id: "YOUR APP ID".to_string(),
///             secret: "YOUR SECRET".to_string(),
///         },
///         ..Default::default()
///     };
///     run_axum(QQBot::new(config)).await
/// }
/// ```
pub async fn run_axum_with_router(
    app: QQBot,
    base_router: Option<Router>,
) -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let webhook_path = app.listening_config().webhook_path.clone();
    let bind_addr = app.listening_config().bind_addr.clone();
    let app = Arc::new(app);

    let base_router = base_router.unwrap_or(Router::new());
    let router = base_router.route(
        &webhook_path,
        any({
            let app = Arc::clone(&app);
            async move |Json(payload): Json<WebhookPayload>| {
                Json(app.webhook_handler(payload).await)
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Listening on {}", bind_addr);
    axum::serve(listener, router).await
}

/// 启动基于Axum的 QQ Bot 程序
///
/// 将会用默认方式构造axum的router
pub async fn run_axum(app: QQBot) -> std::io::Result<()> {
    run_axum_with_router(app, None).await
}
