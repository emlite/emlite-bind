use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<MIDIOutput> for emlite::Val {
    fn from(s: MIDIOutput) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MIDIOutput {
    pub fn send0(&self, data: jsbind::Sequence<u8>) -> jsbind::Undefined {
        self.inner
            .call("send", &[data.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn send1(&self, data: jsbind::Sequence<u8>, timestamp: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("send", &[data.into(), timestamp.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl MIDIOutput {
    pub fn clear(&self) -> jsbind::Undefined {
        self.inner.call("clear", &[]).as_::<jsbind::Undefined>()
    }
}
