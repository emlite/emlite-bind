use super::*;

/// The ProcessingInstruction class.
/// [`ProcessingInstruction`](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProcessingInstruction {
    inner: CharacterData,
}
impl FromVal for ProcessingInstruction {
    fn from_val(v: &Any) -> Self {
        ProcessingInstruction {
            inner: CharacterData::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ProcessingInstruction {
    type Target = CharacterData;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ProcessingInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ProcessingInstruction {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ProcessingInstruction {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ProcessingInstruction> for Any {
    fn from(s: ProcessingInstruction) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ProcessingInstruction> for Any {
    fn from(s: &ProcessingInstruction) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ProcessingInstruction);

impl ProcessingInstruction {
    /// Getter of the `target` attribute.
    /// [`ProcessingInstruction.target`](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/target)
    pub fn target(&self) -> JsString {
        self.inner.get("target").as_::<JsString>()
    }
}
impl ProcessingInstruction {
    /// Getter of the `sheet` attribute.
    /// [`ProcessingInstruction.sheet`](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/sheet)
    pub fn sheet(&self) -> CSSStyleSheet {
        self.inner.get("sheet").as_::<CSSStyleSheet>()
    }
}
