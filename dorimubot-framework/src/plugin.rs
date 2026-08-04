use crate::QQBotApp;

/// 原生应用插件的注册契约。
///
pub trait Plugin: Send + Sync + 'static {
    /// 将插件的事件处理器注册到应用中。
    fn register(&self, app: &QQBotApp);
}
