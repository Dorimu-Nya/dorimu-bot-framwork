use crate::app::config::AppConfig;
use qqbot_sdk_commands::CommandHandler;

impl AppConfig {
    /// 手动注册命令处理函数。
    ///
    /// 与 `#[command]` 不同，手动注册建议使用可拥有所有权的参数类型
    /// （如 `Option<String>`、`Option<Vec<String>>`）或 `Context<T>`。
    pub fn with_command<H, Args, Kind>(mut self, prefix: &'static str, handler: H) -> Self
    where
        H: CommandHandler<Args, Kind>,
    {
        self.commands.push((prefix, handler.into_dyn()));
        self
    }
}
