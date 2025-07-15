use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MIDIOutput {
    inner: MIDIPort,
}
impl FromVal for MIDIOutput {
    fn from_val(v: &emlite::Val) -> Self {
        MIDIOutput {
            inner: MIDIPort::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for MIDIOutput {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MIDIOutput {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MIDIOutput> for emlite::Val {
    fn from(s: MIDIOutput) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MIDIOutput);

impl MIDIOutput {
    pub fn send0(&self, data: Sequence<u8>) -> Undefined {
        self.inner.call("send", &[data.into()]).as_::<Undefined>()
    }

    pub fn send1(&self, data: Sequence<u8>, timestamp: Any) -> Undefined {
        self.inner
            .call("send", &[data.into(), timestamp.into()])
            .as_::<Undefined>()
    }
}
impl MIDIOutput {
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
