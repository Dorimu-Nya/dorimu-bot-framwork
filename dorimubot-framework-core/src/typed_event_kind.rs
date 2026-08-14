use std::marker::PhantomData;

/// 在类型层面关联事件注册键与其原始 data 类型。
///
/// 这是 SDK 提供类型化事件键前的临时实现。
pub struct TypedEventKind<K, Data> {
    kind: K,
    marker: PhantomData<fn() -> Data>,
}

impl<K, Data> TypedEventKind<K, Data> {
    pub const fn new(kind: K) -> Self {
        Self {
            kind,
            marker: PhantomData,
        }
    }

    pub fn into_kind(self) -> K {
        self.kind
    }
}

impl<K: Clone, Data> Clone for TypedEventKind<K, Data> {
    fn clone(&self) -> Self {
        Self::new(self.kind.clone())
    }
}

impl<K: Copy, Data> Copy for TypedEventKind<K, Data> {}
