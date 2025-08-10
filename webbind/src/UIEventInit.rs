use super::*;

/// The UIEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UIEventInit {
    inner: Any,
}

impl FromVal for UIEventInit {
    fn from_val(v: &Any) -> Self {
        UIEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for UIEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for UIEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for UIEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for UIEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<UIEventInit> for Any {
    fn from(s: UIEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&UIEventInit> for Any {
    fn from(s: &UIEventInit) -> Any {
        s.inner.clone()
    }
}

impl UIEventInit {
    /// Getter of the `which` attribute.
    pub fn which(&self) -> u32 {
        self.inner.get("which").as_::<u32>()
    }

    /// Setter of the `which` attribute.
    pub fn set_which(&mut self, value: u32) {
        self.inner.set("which", value);
    }
}
