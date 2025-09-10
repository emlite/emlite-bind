use super::*;

/// The RealTimeReporting class.
/// [`RealTimeReporting`](https://developer.mozilla.org/en-US/docs/Web/API/RealTimeReporting)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RealTimeReporting {
    inner: Any,
}

impl FromVal for RealTimeReporting {
    fn from_val(v: &Any) -> Self {
        RealTimeReporting {
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

impl core::ops::Deref for RealTimeReporting {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RealTimeReporting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RealTimeReporting {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RealTimeReporting {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RealTimeReporting> for Any {
    fn from(s: RealTimeReporting) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RealTimeReporting> for Any {
    fn from(s: &RealTimeReporting) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RealTimeReporting);

impl RealTimeReporting {
    /// The contributeToHistogram method.
    /// [`RealTimeReporting.contributeToHistogram`](https://developer.mozilla.org/en-US/docs/Web/API/RealTimeReporting/contributeToHistogram)
    pub fn contribute_to_histogram(&self, contribution: &RealTimeContribution) -> Undefined {
        self.inner
            .call("contributeToHistogram", &[contribution.into()])
            .as_::<Undefined>()
    }
}
