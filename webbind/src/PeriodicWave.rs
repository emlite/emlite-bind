use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PeriodicWave {
    inner: emlite::Val,
}
impl FromVal for PeriodicWave {
    fn from_val(v: &emlite::Val) -> Self {
        PeriodicWave {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PeriodicWave {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PeriodicWave {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PeriodicWave {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PeriodicWave {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PeriodicWave> for emlite::Val {
    fn from(s: PeriodicWave) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PeriodicWave);

impl PeriodicWave {
    pub fn new0(context: BaseAudioContext) -> PeriodicWave {
        Self {
            inner: emlite::Val::global("PeriodicWave")
                .new(&[context.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: Any) -> PeriodicWave {
        Self {
            inner: emlite::Val::global("PeriodicWave")
                .new(&[context.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
