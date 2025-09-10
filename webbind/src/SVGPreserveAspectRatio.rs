use super::*;

/// The SVGPreserveAspectRatio class.
/// [`SVGPreserveAspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPreserveAspectRatio {
    inner: Any,
}

impl FromVal for SVGPreserveAspectRatio {
    fn from_val(v: &Any) -> Self {
        SVGPreserveAspectRatio {
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

impl core::ops::Deref for SVGPreserveAspectRatio {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGPreserveAspectRatio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGPreserveAspectRatio {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGPreserveAspectRatio {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGPreserveAspectRatio> for Any {
    fn from(s: SVGPreserveAspectRatio) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGPreserveAspectRatio> for Any {
    fn from(s: &SVGPreserveAspectRatio) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGPreserveAspectRatio);

impl SVGPreserveAspectRatio {
    /// Getter of the `align` attribute.
    /// [`SVGPreserveAspectRatio.align`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/align)
    pub fn align(&self) -> u16 {
        self.inner.get("align").as_::<u16>()
    }

    /// Setter of the `align` attribute.
    /// [`SVGPreserveAspectRatio.align`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/align)
    pub fn set_align(&mut self, value: u16) {
        self.inner.set("align", value);
    }
}
impl SVGPreserveAspectRatio {
    /// Getter of the `meetOrSlice` attribute.
    /// [`SVGPreserveAspectRatio.meetOrSlice`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/meetOrSlice)
    pub fn meet_or_slice(&self) -> u16 {
        self.inner.get("meetOrSlice").as_::<u16>()
    }

    /// Setter of the `meetOrSlice` attribute.
    /// [`SVGPreserveAspectRatio.meetOrSlice`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/meetOrSlice)
    pub fn set_meet_or_slice(&mut self, value: u16) {
        self.inner.set("meetOrSlice", value);
    }
}
