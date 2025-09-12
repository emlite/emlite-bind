use super::*;

/// The IDBCursor class.
/// [`IDBCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBCursor {
    inner: Any,
}

impl FromVal for IDBCursor {
    fn from_val(v: &Any) -> Self {
        IDBCursor {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBCursor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBCursor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBCursor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBCursor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IDBCursor> for Any {
    fn from(s: IDBCursor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBCursor> for Any {
    fn from(s: &IDBCursor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IDBCursor);

impl IDBCursor {
    /// Getter of the `source` attribute.
    /// [`IDBCursor.source`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/source)
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }
}
impl IDBCursor {
    /// Getter of the `direction` attribute.
    /// [`IDBCursor.direction`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/direction)
    pub fn direction(&self) -> IDBCursorDirection {
        self.inner.get("direction").as_::<IDBCursorDirection>()
    }
}
impl IDBCursor {
    /// Getter of the `key` attribute.
    /// [`IDBCursor.key`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/key)
    pub fn key(&self) -> Any {
        self.inner.get("key").as_::<Any>()
    }
}
impl IDBCursor {
    /// Getter of the `primaryKey` attribute.
    /// [`IDBCursor.primaryKey`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/primaryKey)
    pub fn primary_key(&self) -> Any {
        self.inner.get("primaryKey").as_::<Any>()
    }
}
impl IDBCursor {
    /// Getter of the `request` attribute.
    /// [`IDBCursor.request`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/request)
    pub fn request(&self) -> IDBRequest {
        self.inner.get("request").as_::<IDBRequest>()
    }
}
impl IDBCursor {
    /// The advance method.
    /// [`IDBCursor.advance`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/advance)
    pub fn advance(&self, count: u32) -> Undefined {
        self.inner
            .call("advance", &[count.into()])
            .as_::<Undefined>()
    }
}
impl IDBCursor {
    /// The continue method.
    /// [`IDBCursor.continue`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)
    pub fn continue_(&self) -> Undefined {
        self.inner.call("continue", &[]).as_::<Undefined>()
    }
}
impl IDBCursor {
    /// The continue method.
    /// [`IDBCursor.continue`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)
    pub fn continue_with_key(&self, key: &Any) -> Undefined {
        self.inner
            .call("continue", &[key.into()])
            .as_::<Undefined>()
    }
}
impl IDBCursor {
    /// The continuePrimaryKey method.
    /// [`IDBCursor.continuePrimaryKey`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continuePrimaryKey)
    pub fn continue_primary_key(&self, key: &Any, primary_key: &Any) -> Undefined {
        self.inner
            .call("continuePrimaryKey", &[key.into(), primary_key.into()])
            .as_::<Undefined>()
    }
}
impl IDBCursor {
    /// The update method.
    /// [`IDBCursor.update`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/update)
    pub fn update(&self, value: &Any) -> IDBRequest {
        self.inner
            .call("update", &[value.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBCursor {
    /// The delete method.
    /// [`IDBCursor.delete`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/delete)
    pub fn delete(&self) -> IDBRequest {
        self.inner.call("delete", &[]).as_::<IDBRequest>()
    }
}
