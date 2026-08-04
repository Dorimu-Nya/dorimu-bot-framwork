use dorimubot_framework_core::QQBot;

/// 启动 dorimubot
///
/// 将根据配置的Feature自动选择启动器。
pub async fn run_dorimubot(bot: QQBot) -> std::io::Result<()> {
    dorimubot_axum::run_axum(bot).await
}
