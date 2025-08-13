use super::*;




/// The ProfilerFrame dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProfilerFrame {
    inner: Any,
}

impl FromVal for ProfilerFrame {
    fn from_val(v: &Any) -> Self {
        ProfilerFrame { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ProfilerFrame {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ProfilerFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ProfilerFrame {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ProfilerFrame {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ProfilerFrame> for Any {
    fn from(s: ProfilerFrame) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ProfilerFrame> for Any {
    fn from(s: &ProfilerFrame) -> Any {
        s.inner.clone()
    }
}

impl ProfilerFrame {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl ProfilerFrame {
    /// Getter of the `resourceId` attribute.
    pub fn resource_id(&self) -> u64 {
        self.inner.get("resourceId").as_::<u64>()
    }

    /// Setter of the `resourceId` attribute.
    pub fn set_resource_id(&mut self, value: u64) {
        self.inner.set("resourceId", value);
    }
}
impl ProfilerFrame {
    /// Getter of the `line` attribute.
    pub fn line(&self) -> u64 {
        self.inner.get("line").as_::<u64>()
    }

    /// Setter of the `line` attribute.
    pub fn set_line(&mut self, value: u64) {
        self.inner.set("line", value);
    }
}
impl ProfilerFrame {
    /// Getter of the `column` attribute.
    pub fn column(&self) -> u64 {
        self.inner.get("column").as_::<u64>()
    }

    /// Setter of the `column` attribute.
    pub fn set_column(&mut self, value: u64) {
        self.inner.set("column", value);
    }
}
