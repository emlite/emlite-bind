use super::*;

#[derive(Clone, Debug)]
pub struct HTMLProgressElement {
    inner: HTMLElement,
}
impl FromVal for HTMLProgressElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLProgressElement {
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
impl std::ops::Deref for HTMLProgressElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLProgressElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLProgressElement> for emlite::Val {
    fn from(s: HTMLProgressElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLProgressElement {
    pub fn new() -> HTMLProgressElement {
        Self {
            inner: emlite::Val::global("HTMLProgressElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLProgressElement {
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }

    pub fn set_value(&mut self, value: f64) {
        self.inner.set("value", value);
    }
}
impl HTMLProgressElement {
    pub fn max(&self) -> f64 {
        self.inner.get("max").as_::<f64>()
    }

    pub fn set_max(&mut self, value: f64) {
        self.inner.set("max", value);
    }
}
impl HTMLProgressElement {
    pub fn position(&self) -> f64 {
        self.inner.get("position").as_::<f64>()
    }
}
impl HTMLProgressElement {
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
