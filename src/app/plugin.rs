use super::App;

/// 应用插件的注册契约。
///
/// 插件加载时会获得应用实例，并可通过 [`App::register`] 注册自己需要的
/// 事件处理器。该 trait 保持对象安全，因此原生插件和未来的 WASM 插件适配器
/// 都可以使用同一个加载入口。
pub trait Plugin: Send + Sync + 'static {
    /// 将插件的事件处理器和其他应用资源注册到应用中。
    fn register(&self, app: &App);
}
