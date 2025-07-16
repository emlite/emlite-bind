use super::*;

/// The BreakToken class.
/// [`BreakToken`](https://developer.mozilla.org/en-US/docs/Web/API/BreakToken)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BreakToken {
    inner: Any,
}
impl FromVal for BreakToken {
    fn from_val(v: &Any) -> Self {
        BreakToken {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BreakToken {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BreakToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BreakToken {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BreakToken {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BreakToken> for Any {
    fn from(s: BreakToken) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BreakToken> for Any {
    fn from(s: &BreakToken) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BreakToken);

impl BreakToken {
    /// Getter of the `childBreakTokens` attribute.
    /// [`BreakToken.childBreakTokens`](https://developer.mozilla.org/en-US/docs/Web/API/BreakToken/childBreakTokens)
    pub fn child_break_tokens(&self) -> FrozenArray<ChildBreakToken> {
        self.inner
            .get("childBreakTokens")
            .as_::<FrozenArray<ChildBreakToken>>()
    }
}
impl BreakToken {
    /// Getter of the `data` attribute.
    /// [`BreakToken.data`](https://developer.mozilla.org/en-US/docs/Web/API/BreakToken/data)
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }
}
