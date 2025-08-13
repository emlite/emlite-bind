use super::*;




/// The ChannelMergerOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ChannelMergerOptions {
    inner: Any,
}

impl FromVal for ChannelMergerOptions {
    fn from_val(v: &Any) -> Self {
        ChannelMergerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ChannelMergerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ChannelMergerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ChannelMergerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ChannelMergerOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ChannelMergerOptions> for Any {
    fn from(s: ChannelMergerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ChannelMergerOptions> for Any {
    fn from(s: &ChannelMergerOptions) -> Any {
        s.inner.clone()
    }
}

impl ChannelMergerOptions {
    /// Getter of the `numberOfInputs` attribute.
    pub fn number_of_inputs(&self) -> u32 {
        self.inner.get("numberOfInputs").as_::<u32>()
    }

    /// Setter of the `numberOfInputs` attribute.
    pub fn set_number_of_inputs(&mut self, value: u32) {
        self.inner.set("numberOfInputs", value);
    }
}
