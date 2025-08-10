use super::*;

/// The ProfilerStack dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProfilerStack {
    inner: Any,
}

impl FromVal for ProfilerStack {
    fn from_val(v: &Any) -> Self {
        ProfilerStack { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ProfilerStack {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ProfilerStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ProfilerStack {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ProfilerStack {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ProfilerStack> for Any {
    fn from(s: ProfilerStack) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ProfilerStack> for Any {
    fn from(s: &ProfilerStack) -> Any {
        s.inner.clone()
    }
}

impl ProfilerStack {
    /// Getter of the `parentId` attribute.
    pub fn parent_id(&self) -> u64 {
        self.inner.get("parentId").as_::<u64>()
    }

    /// Setter of the `parentId` attribute.
    pub fn set_parent_id(&mut self, value: u64) {
        self.inner.set("parentId", value);
    }
}
impl ProfilerStack {
    /// Getter of the `frameId` attribute.
    pub fn frame_id(&self) -> u64 {
        self.inner.get("frameId").as_::<u64>()
    }

    /// Setter of the `frameId` attribute.
    pub fn set_frame_id(&mut self, value: u64) {
        self.inner.set("frameId", value);
    }
}
