use super::*;




/// The SyncEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SyncEventInit {
    inner: Any,
}

impl FromVal for SyncEventInit {
    fn from_val(v: &Any) -> Self {
        SyncEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SyncEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SyncEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SyncEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SyncEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SyncEventInit> for Any {
    fn from(s: SyncEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SyncEventInit> for Any {
    fn from(s: &SyncEventInit) -> Any {
        s.inner.clone()
    }
}

impl SyncEventInit {
    /// Getter of the `tag` attribute.
    pub fn tag(&self) -> JsString {
        self.inner.get("tag").as_::<JsString>()
    }

    /// Setter of the `tag` attribute.
    pub fn set_tag(&mut self, value: &JsString) {
        self.inner.set("tag", value);
    }
}
impl SyncEventInit {
    /// Getter of the `lastChance` attribute.
    pub fn last_chance(&self) -> bool {
        self.inner.get("lastChance").as_::<bool>()
    }

    /// Setter of the `lastChance` attribute.
    pub fn set_last_chance(&mut self, value: bool) {
        self.inner.set("lastChance", value);
    }
}
