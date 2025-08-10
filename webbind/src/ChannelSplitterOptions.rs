use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ChannelSplitterOptions {
    inner: Any,
}
impl FromVal for ChannelSplitterOptions {
    fn from_val(v: &Any) -> Self {
        ChannelSplitterOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ChannelSplitterOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ChannelSplitterOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ChannelSplitterOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ChannelSplitterOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ChannelSplitterOptions> for Any {
    fn from(s: ChannelSplitterOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ChannelSplitterOptions> for Any {
    fn from(s: &ChannelSplitterOptions) -> Any {
        s.inner.clone()
    }
}

impl ChannelSplitterOptions {
    pub fn number_of_outputs(&self) -> u32 {
        self.inner.get("numberOfOutputs").as_::<u32>()
    }

    pub fn set_number_of_outputs(&mut self, value: u32) {
        self.inner.set("numberOfOutputs", value);
    }
}
