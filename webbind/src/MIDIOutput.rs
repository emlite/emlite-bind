use super::*;

/// The MIDIOutput class.
/// [`MIDIOutput`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIOutput {
    inner: MIDIPort,
}
impl FromVal for MIDIOutput {
    fn from_val(v: &Any) -> Self {
        MIDIOutput {
            inner: MIDIPort::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MIDIOutput {
    type Target = MIDIPort;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MIDIOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MIDIOutput {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MIDIOutput {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MIDIOutput> for Any {
    fn from(s: MIDIOutput) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MIDIOutput> for Any {
    fn from(s: &MIDIOutput) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MIDIOutput);

impl MIDIOutput {
    /// The send method.
    /// [`MIDIOutput.send`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/send)
    pub fn send0(&self, data: TypedArray<u8>) -> Undefined {
        self.inner.call("send", &[data.into()]).as_::<Undefined>()
    }
    /// The send method.
    /// [`MIDIOutput.send`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/send)
    pub fn send1(&self, data: TypedArray<u8>, timestamp: &Any) -> Undefined {
        self.inner
            .call("send", &[data.into(), timestamp.into()])
            .as_::<Undefined>()
    }
}
impl MIDIOutput {
    /// The clear method.
    /// [`MIDIOutput.clear`](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/clear)
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
