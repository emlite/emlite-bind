use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBCursor {
    inner: emlite::Val,
}
impl FromVal for IDBCursor {
    fn from_val(v: &emlite::Val) -> Self {
        IDBCursor { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IDBCursor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBCursor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IDBCursor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBCursor {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<IDBCursor> for emlite::Val {
    fn from(s: IDBCursor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(IDBCursor);


impl IDBCursor {
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }

}
impl IDBCursor {
    pub fn direction(&self) -> IDBCursorDirection {
        self.inner.get("direction").as_::<IDBCursorDirection>()
    }

}
impl IDBCursor {
    pub fn key(&self) -> Any {
        self.inner.get("key").as_::<Any>()
    }

}
impl IDBCursor {
    pub fn primary_key(&self) -> Any {
        self.inner.get("primaryKey").as_::<Any>()
    }

}
impl IDBCursor {
    pub fn request(&self) -> IDBRequest {
        self.inner.get("request").as_::<IDBRequest>()
    }

}
impl IDBCursor {
    pub fn advance(&self, count: u32) -> Undefined {
        self.inner.call("advance", &[count.into(), ]).as_::<Undefined>()
    }

}
impl IDBCursor {
    pub fn continue_0(&self, ) -> Undefined {
        self.inner.call("continue", &[]).as_::<Undefined>()
    }

    pub fn continue_1(&self, key: Any) -> Undefined {
        self.inner.call("continue", &[key.into(), ]).as_::<Undefined>()
    }

}
impl IDBCursor {
    pub fn continue_primary_key(&self, key: Any, primary_key: Any) -> Undefined {
        self.inner.call("continuePrimaryKey", &[key.into(), primary_key.into(), ]).as_::<Undefined>()
    }

}
impl IDBCursor {
    pub fn update(&self, value: Any) -> IDBRequest {
        self.inner.call("update", &[value.into(), ]).as_::<IDBRequest>()
    }

}
impl IDBCursor {
    pub fn delete(&self, ) -> IDBRequest {
        self.inner.call("delete", &[]).as_::<IDBRequest>()
    }

}
