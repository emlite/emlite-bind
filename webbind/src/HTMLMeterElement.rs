use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMeterElement {
    inner: HTMLElement,
}
impl FromVal for HTMLMeterElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLMeterElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLMeterElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLMeterElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLMeterElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLMeterElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLMeterElement> for emlite::Val {
    fn from(s: HTMLMeterElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLMeterElement);

impl HTMLMeterElement {
    pub fn new() -> HTMLMeterElement {
        Self {
            inner: emlite::Val::global("HTMLMeterElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLMeterElement {
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }

    pub fn set_value(&mut self, value: f64) {
        self.inner.set("value", value);
    }
}
impl HTMLMeterElement {
    pub fn min(&self) -> f64 {
        self.inner.get("min").as_::<f64>()
    }

    pub fn set_min(&mut self, value: f64) {
        self.inner.set("min", value);
    }
}
impl HTMLMeterElement {
    pub fn max(&self) -> f64 {
        self.inner.get("max").as_::<f64>()
    }

    pub fn set_max(&mut self, value: f64) {
        self.inner.set("max", value);
    }
}
impl HTMLMeterElement {
    pub fn low(&self) -> f64 {
        self.inner.get("low").as_::<f64>()
    }

    pub fn set_low(&mut self, value: f64) {
        self.inner.set("low", value);
    }
}
impl HTMLMeterElement {
    pub fn high(&self) -> f64 {
        self.inner.get("high").as_::<f64>()
    }

    pub fn set_high(&mut self, value: f64) {
        self.inner.set("high", value);
    }
}
impl HTMLMeterElement {
    pub fn optimum(&self) -> f64 {
        self.inner.get("optimum").as_::<f64>()
    }

    pub fn set_optimum(&mut self, value: f64) {
        self.inner.set("optimum", value);
    }
}
impl HTMLMeterElement {
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
