use super::*;

/// The MIDIInputMap class.
/// [`MIDIInputMap`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIInputMap {
    inner: Any,
}
impl FromVal for MIDIInputMap {
    fn from_val(v: &Any) -> Self {
        MIDIInputMap {
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
impl core::ops::Deref for MIDIInputMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIInputMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MIDIInputMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MIDIInputMap {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MIDIInputMap> for Any {
    fn from(s: MIDIInputMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MIDIInputMap> for Any {
    fn from(s: &MIDIInputMap) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MIDIInputMap);
