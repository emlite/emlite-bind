use super::*;




/// The PromptResponseObject dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PromptResponseObject {
    inner: Any,
}

impl FromVal for PromptResponseObject {
    fn from_val(v: &Any) -> Self {
        PromptResponseObject { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PromptResponseObject {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PromptResponseObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PromptResponseObject {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PromptResponseObject {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PromptResponseObject> for Any {
    fn from(s: PromptResponseObject) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PromptResponseObject> for Any {
    fn from(s: &PromptResponseObject) -> Any {
        s.inner.clone()
    }
}

impl PromptResponseObject {
    /// Getter of the `userChoice` attribute.
    pub fn user_choice(&self) -> AppBannerPromptOutcome {
        self.inner.get("userChoice").as_::<AppBannerPromptOutcome>()
    }

    /// Setter of the `userChoice` attribute.
    pub fn set_user_choice(&mut self, value: &AppBannerPromptOutcome) {
        self.inner.set("userChoice", value);
    }
}
