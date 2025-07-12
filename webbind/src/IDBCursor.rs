use super::*;

#[derive(Clone, Debug)]
pub struct IDBCursor {
    inner: emlite::Val,
}
impl FromVal for IDBCursor {
    fn from_val(v: &emlite::Val) -> Self {
        IDBCursor {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IDBCursor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IDBCursor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBCursor> for emlite::Val {
    fn from(s: IDBCursor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBCursor {
    pub fn source(&self) -> jsbind::Any {
        self.inner.get("source").as_::<jsbind::Any>()
    }
}
impl IDBCursor {
    pub fn direction(&self) -> IDBCursorDirection {
        self.inner.get("direction").as_::<IDBCursorDirection>()
    }
}
impl IDBCursor {
    pub fn key(&self) -> jsbind::Any {
        self.inner.get("key").as_::<jsbind::Any>()
    }
}
impl IDBCursor {
    pub fn primary_key(&self) -> jsbind::Any {
        self.inner.get("primaryKey").as_::<jsbind::Any>()
    }
}
impl IDBCursor {
    pub fn request(&self) -> IDBRequest {
        self.inner.get("request").as_::<IDBRequest>()
    }
}
impl IDBCursor {
    pub fn advance(&self, count: u32) -> jsbind::Undefined {
        self.inner
            .call("advance", &[count.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl IDBCursor {
    pub fn continue_0(&self) -> jsbind::Undefined {
        self.inner.call("continue", &[]).as_::<jsbind::Undefined>()
    }

    pub fn continue_1(&self, key: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("continue", &[key.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl IDBCursor {
    pub fn continue_primary_key(
        &self,
        key: jsbind::Any,
        primary_key: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("continuePrimaryKey", &[key.into(), primary_key.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl IDBCursor {
    pub fn update(&self, value: jsbind::Any) -> IDBRequest {
        self.inner
            .call("update", &[value.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBCursor {
    pub fn delete(&self) -> IDBRequest {
        self.inner.call("delete", &[]).as_::<IDBRequest>()
    }
}
