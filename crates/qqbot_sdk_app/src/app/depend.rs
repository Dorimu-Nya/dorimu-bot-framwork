use qqbot_sdk_commands::FromCommandArg;
use qqbot_sdk_core::events::common::CommonMessage;
use qqbot_sdk_core::{resolve_dependency, DependencyProvider, DispatchPayload, FromEventArg};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

/// 注入处理器的类型化依赖。
pub struct Depend<T: ?Sized>(Arc<T>);

impl<T: ?Sized> Clone for Depend<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Depend<T>
where
    T: Any + Send + Sync,
{
    pub fn new(value: T) -> Self {
        Self(Arc::new(value))
    }

    #[doc(hidden)]
    pub fn from_provider(provider: &dyn DependencyProvider) -> Self {
        Self(resolve_dependency::<T>(provider))
    }
}

impl<T: ?Sized> Depend<T> {
    pub fn from_arc(value: Arc<T>) -> Self {
        Self(value)
    }

    pub fn as_arc(&self) -> Arc<T> {
        Arc::clone(&self.0)
    }

    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for Depend<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

/// 可克隆、按类型索引的应用依赖存储。
#[derive(Clone, Default)]
pub struct DependStore {
    dependencies: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl DependStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T>(&self, value: T) -> Option<Arc<dyn Any + Send + Sync>>
    where
        T: Any + Send + Sync,
    {
        self.dependencies
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), Arc::new(value))
    }

    pub fn insert_arc<T>(&self, value: Arc<T>) -> Option<&'static str>
    where
        T: Any + Send + Sync,
    {
        self.dependencies
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), value)
            .map(|_| std::any::type_name::<T>())
    }

    pub fn get<T>(&self) -> Arc<T>
    where
        T: Any + Send + Sync,
    {
        resolve_dependency::<T>(self)
    }

    pub fn get_depend<T>(&self) -> Depend<T>
    where
        T: Any + Send + Sync,
    {
        Depend::from_arc(self.get::<T>())
    }
}

impl DependencyProvider for DependStore {
    fn get_dependency(&self, type_id: TypeId) -> Option<Arc<dyn Any + Send + Sync>> {
        self.dependencies.read().unwrap().get(&type_id).cloned()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 标记从依赖存储中解析的事件或命令参数。
pub struct DependArg;

impl<T> FromEventArg<DependArg> for Depend<T>
where
    T: Any + Send + Sync,
{
    fn from_event_arg(
        _payload: &DispatchPayload,
        dependencies: &dyn DependencyProvider,
    ) -> Option<Self> {
        Some(Self::from_provider(dependencies))
    }
}

impl FromEventArg<DependArg> for DependStore {
    fn from_event_arg(
        _payload: &DispatchPayload,
        dependencies: &dyn DependencyProvider,
    ) -> Option<Self> {
        Some(
            dependencies
                .as_any()
                .downcast_ref::<DependStore>()
                .expect("event dependency provider must be DependStore")
                .clone(),
        )
    }
}

impl<T> FromCommandArg<DependArg> for Depend<T>
where
    T: Any + Send + Sync,
{
    fn from_command_arg(
        _message: &dyn CommonMessage,
        dependencies: &dyn DependencyProvider,
    ) -> Self {
        Self::from_provider(dependencies)
    }
}

impl FromCommandArg<DependArg> for DependStore {
    fn from_command_arg(
        _message: &dyn CommonMessage,
        dependencies: &dyn DependencyProvider,
    ) -> Self {
        dependencies
            .as_any()
            .downcast_ref::<DependStore>()
            .expect("command dependency provider must be DependStore")
            .clone()
    }
}
