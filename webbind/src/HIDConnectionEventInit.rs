use super::*;

/// The HIDConnectionEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDConnectionEventInit {
    inner: Any,
}

impl FromVal for HIDConnectionEventInit {
    fn from_val(v: &Any) -> Self {
        HIDConnectionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HIDConnectionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HIDConnectionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HIDConnectionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HIDConnectionEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HIDConnectionEventInit> for Any {
    fn from(s: HIDConnectionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HIDConnectionEventInit> for Any {
    fn from(s: &HIDConnectionEventInit) -> Any {
        s.inner.clone()
    }
}

impl HIDConnectionEventInit {
    /// Getter of the `device` attribute.
    pub fn device(&self) -> HIDDevice {
        self.inner.get("device").as_::<HIDDevice>()
    }

    /// Setter of the `device` attribute.
    pub fn set_device(&mut self, value: &HIDDevice) {
        self.inner.set("device", value);
    }
}
