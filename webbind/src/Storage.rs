use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Storage {
    inner: emlite::Val,
}
impl FromVal for Storage {
    fn from_val(v: &emlite::Val) -> Self {
        Storage { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Storage {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Storage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Storage {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Storage {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<Storage> for emlite::Val {
    fn from(s: Storage) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Storage);


impl Storage {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl Storage {
    pub fn key(&self, index: u32) -> DOMString {
        self.inner.call("key", &[index.into(), ]).as_::<DOMString>()
    }

}
impl Storage {
    pub fn get_item(&self, key: DOMString) -> DOMString {
        self.inner.call("getItem", &[key.into(), ]).as_::<DOMString>()
    }

}
impl Storage {
    pub fn set_item(&self, key: DOMString, value: DOMString) -> Undefined {
        self.inner.call("setItem", &[key.into(), value.into(), ]).as_::<Undefined>()
    }

}
impl Storage {
    pub fn remove_item(&self, key: DOMString) -> Undefined {
        self.inner.call("removeItem", &[key.into(), ]).as_::<Undefined>()
    }

}
impl Storage {
    pub fn clear(&self, ) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }

}
