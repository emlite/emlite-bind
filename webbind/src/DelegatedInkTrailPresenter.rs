use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InkTrailStyle {
    inner: emlite::Val,
}
impl FromVal for InkTrailStyle {
    fn from_val(v: &emlite::Val) -> Self {
        InkTrailStyle { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for InkTrailStyle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InkTrailStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for InkTrailStyle {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for InkTrailStyle {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<InkTrailStyle> for emlite::Val {
    fn from(s: InkTrailStyle) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&InkTrailStyle> for emlite::Val {
    fn from(s: &InkTrailStyle) -> emlite::Val {
        s.inner.clone()
    }
}

impl InkTrailStyle {
    pub fn color(&self) -> String {
        self.inner.get("color").as_::<String>()
    }

    pub fn set_color(&mut self, value: &str) {
        self.inner.set("color", value);
    }
}
impl InkTrailStyle {
    pub fn diameter(&self) -> f64 {
        self.inner.get("diameter").as_::<f64>()
    }

    pub fn set_diameter(&mut self, value: f64) {
        self.inner.set("diameter", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DelegatedInkTrailPresenter {
    inner: emlite::Val,
}
impl FromVal for DelegatedInkTrailPresenter {
    fn from_val(v: &emlite::Val) -> Self {
        DelegatedInkTrailPresenter {
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
impl core::ops::Deref for DelegatedInkTrailPresenter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DelegatedInkTrailPresenter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DelegatedInkTrailPresenter {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DelegatedInkTrailPresenter {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DelegatedInkTrailPresenter> for emlite::Val {
    fn from(s: DelegatedInkTrailPresenter) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&DelegatedInkTrailPresenter> for emlite::Val {
    fn from(s: &DelegatedInkTrailPresenter) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DelegatedInkTrailPresenter);

impl DelegatedInkTrailPresenter {
    pub fn presentation_area(&self) -> Element {
        self.inner.get("presentationArea").as_::<Element>()
    }
}
impl DelegatedInkTrailPresenter {
    pub fn update_ink_trail_start_point(
        &self,
        event: &PointerEvent,
        style: &InkTrailStyle,
    ) -> Undefined {
        self.inner
            .call("updateInkTrailStartPoint", &[event.into(), style.into()])
            .as_::<Undefined>()
    }
}
