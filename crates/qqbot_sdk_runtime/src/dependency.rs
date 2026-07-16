use std::any::{Any, TypeId};
use std::sync::Arc;

/// 面向对象安全的应用依赖访问接口。
pub trait DependencyProvider: Send + Sync {
    fn get_dependency(&self, type_id: TypeId) -> Option<Arc<dyn Any + Send + Sync>>;

    fn as_any(&self) -> &dyn Any;
}

/// 按具体类型解析并向下转型一个依赖。
pub fn resolve_dependency<T>(provider: &dyn DependencyProvider) -> Arc<T>
where
    T: Any + Send + Sync,
{
    let value = provider
        .get_dependency(TypeId::of::<T>())
        .unwrap_or_else(|| {
            panic!(
                "dependency not found for type {:?}",
                std::any::type_name::<T>()
            )
        });

    Arc::downcast::<T>(value).unwrap_or_else(|_| {
        panic!(
            "dependency type mismatch when downcasting {}",
            std::any::type_name::<T>()
        )
    })
}
