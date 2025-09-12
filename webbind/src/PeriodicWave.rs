use super::*;

/// The PeriodicWave class.
/// [`PeriodicWave`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicWave)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PeriodicWave {
    inner: Any,
}

impl FromVal for PeriodicWave {
    fn from_val(v: &Any) -> Self {
        PeriodicWave {
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

impl core::ops::Deref for PeriodicWave {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PeriodicWave {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PeriodicWave {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PeriodicWave {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PeriodicWave> for Any {
    fn from(s: PeriodicWave) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PeriodicWave> for Any {
    fn from(s: &PeriodicWave) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PeriodicWave);

impl PeriodicWave {
    /// The `new PeriodicWave(..)` constructor, creating a new PeriodicWave instance
    pub fn new(context: &BaseAudioContext) -> PeriodicWave {
        Self {
            inner: Any::global("PeriodicWave")
                .new(&[context.into()])
                .as_::<Any>(),
        }
    }
}

impl PeriodicWave {
    /// The `new PeriodicWave(..)` constructor, creating a new PeriodicWave instance
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &PeriodicWaveOptions,
    ) -> PeriodicWave {
        Self {
            inner: Any::global("PeriodicWave")
                .new(&[context.into(), options.into()])
                .as_::<Any>(),
        }
    }
}
