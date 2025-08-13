use super::*;




/// The ChildBreakToken class.
/// [`ChildBreakToken`](https://developer.mozilla.org/en-US/docs/Web/API/ChildBreakToken)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ChildBreakToken {
    inner: Any,
}

impl FromVal for ChildBreakToken {
    fn from_val(v: &Any) -> Self {
        ChildBreakToken { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ChildBreakToken {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ChildBreakToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ChildBreakToken {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ChildBreakToken {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ChildBreakToken> for Any {
    fn from(s: ChildBreakToken) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ChildBreakToken> for Any {
    fn from(s: &ChildBreakToken) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ChildBreakToken);


impl ChildBreakToken {
    /// Getter of the `breakType` attribute.
    /// [`ChildBreakToken.breakType`](https://developer.mozilla.org/en-US/docs/Web/API/ChildBreakToken/breakType)
    pub fn break_type(&self) -> BreakType {
        self.inner.get("breakType").as_::<BreakType>()
    }

}
impl ChildBreakToken {
    /// Getter of the `child` attribute.
    /// [`ChildBreakToken.child`](https://developer.mozilla.org/en-US/docs/Web/API/ChildBreakToken/child)
    pub fn child(&self) -> LayoutChild {
        self.inner.get("child").as_::<LayoutChild>()
    }

}
