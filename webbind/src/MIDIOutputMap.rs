use super::*;

/// The MIDIOutputMap class.
/// [`MIDIOutputMap`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIOutputMap {
    inner: Any,
}
impl FromVal for MIDIOutputMap {
    fn from_val(v: &Any) -> Self {
        MIDIOutputMap {
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
impl core::ops::Deref for MIDIOutputMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIOutputMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MIDIOutputMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MIDIOutputMap {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MIDIOutputMap> for Any {
    fn from(s: MIDIOutputMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MIDIOutputMap> for Any {
    fn from(s: &MIDIOutputMap) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MIDIOutputMap);
