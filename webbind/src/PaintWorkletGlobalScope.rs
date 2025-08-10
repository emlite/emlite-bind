use super::*;

/// The PaintWorkletGlobalScope class.
/// [`PaintWorkletGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaintWorkletGlobalScope {
    inner: WorkletGlobalScope,
}
impl FromVal for PaintWorkletGlobalScope {
    fn from_val(v: &Any) -> Self {
        PaintWorkletGlobalScope {
            inner: WorkletGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PaintWorkletGlobalScope {
    type Target = WorkletGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaintWorkletGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaintWorkletGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaintWorkletGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaintWorkletGlobalScope> for Any {
    fn from(s: PaintWorkletGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaintWorkletGlobalScope> for Any {
    fn from(s: &PaintWorkletGlobalScope) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PaintWorkletGlobalScope);

impl PaintWorkletGlobalScope {
    /// The registerPaint method.
    /// [`PaintWorkletGlobalScope.registerPaint`](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope/registerPaint)
    pub fn register_paint(&self, name: &JsString, paint_ctor: &Function) -> Undefined {
        self.inner
            .call("registerPaint", &[name.into(), paint_ctor.into()])
            .as_::<Undefined>()
    }
}
impl PaintWorkletGlobalScope {
    /// Getter of the `devicePixelRatio` attribute.
    /// [`PaintWorkletGlobalScope.devicePixelRatio`](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope/devicePixelRatio)
    pub fn device_pixel_ratio(&self) -> f64 {
        self.inner.get("devicePixelRatio").as_::<f64>()
    }
}
