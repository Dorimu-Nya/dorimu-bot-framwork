use super::event_handler::DynEventHandler;

/// 用于定位特定事件处理器注册表的键。
///
/// `#[event_kind]` 为每个原始事件枚举生成实现。每个键变体拥有独立的
/// 静态处理器列表，因此容器本身不需要保存事件类型字段。
pub trait KindRegistryKey: Copy + Send + Sync + 'static {
    /// 克隆此键对应注册表中的处理器，供异步分发使用。
    fn get_readable_vec(&self) -> Vec<DynEventHandler>;

    /// 获取此键对应注册表的写锁，用于在应用构建阶段注册处理器。
    fn get_writable_vec(&self) -> std::sync::RwLockWriteGuard<'static, Vec<DynEventHandler>>;
}
