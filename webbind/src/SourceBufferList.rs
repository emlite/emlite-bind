use super::*;

/// The SourceBufferList class.
/// [`SourceBufferList`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SourceBufferList {
    inner: EventTarget,
}

impl FromVal for SourceBufferList {
    fn from_val(v: &Any) -> Self {
        SourceBufferList {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SourceBufferList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SourceBufferList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SourceBufferList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SourceBufferList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SourceBufferList> for Any {
    fn from(s: SourceBufferList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SourceBufferList> for Any {
    fn from(s: &SourceBufferList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SourceBufferList);

impl SourceBufferList {
    /// Getter of the `length` attribute.
    /// [`SourceBufferList.length`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SourceBufferList {
    /// Getter of the `onaddsourcebuffer` attribute.
    /// [`SourceBufferList.onaddsourcebuffer`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)
    pub fn onaddsourcebuffer(&self) -> Any {
        self.inner.get("onaddsourcebuffer").as_::<Any>()
    }

    /// Setter of the `onaddsourcebuffer` attribute.
    /// [`SourceBufferList.onaddsourcebuffer`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)
    pub fn set_onaddsourcebuffer(&mut self, value: &Any) {
        self.inner.set("onaddsourcebuffer", value);
    }
}
impl SourceBufferList {
    /// Getter of the `onremovesourcebuffer` attribute.
    /// [`SourceBufferList.onremovesourcebuffer`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)
    pub fn onremovesourcebuffer(&self) -> Any {
        self.inner.get("onremovesourcebuffer").as_::<Any>()
    }

    /// Setter of the `onremovesourcebuffer` attribute.
    /// [`SourceBufferList.onremovesourcebuffer`](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)
    pub fn set_onremovesourcebuffer(&mut self, value: &Any) {
        self.inner.set("onremovesourcebuffer", value);
    }
}
