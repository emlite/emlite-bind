use super::*;

#[derive(Clone, Debug)]
pub struct ContentDescription {
    inner: emlite::Val,
}
impl FromVal for ContentDescription {
    fn from_val(v: &emlite::Val) -> Self {
        ContentDescription { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ContentDescription {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ContentDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ContentDescription> for emlite::Val {
    fn from(s: ContentDescription) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ContentDescription {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }

    pub fn set_id(&mut self, value: jsbind::DOMString) {
        self.inner.set("id", value);
    }
}
impl ContentDescription {
    pub fn title(&self) -> jsbind::DOMString {
        self.inner.get("title").as_::<jsbind::DOMString>()
    }

    pub fn set_title(&mut self, value: jsbind::DOMString) {
        self.inner.set("title", value);
    }
}
impl ContentDescription {
    pub fn description(&self) -> jsbind::DOMString {
        self.inner.get("description").as_::<jsbind::DOMString>()
    }

    pub fn set_description(&mut self, value: jsbind::DOMString) {
        self.inner.set("description", value);
    }
}
impl ContentDescription {
    pub fn category(&self) -> ContentCategory {
        self.inner.get("category").as_::<ContentCategory>()
    }

    pub fn set_category(&mut self, value: ContentCategory) {
        self.inner.set("category", value);
    }
}
impl ContentDescription {
    pub fn icons(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("icons")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_icons(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("icons", value);
    }
}
impl ContentDescription {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }

    pub fn set_url(&mut self, value: jsbind::USVString) {
        self.inner.set("url", value);
    }
}
#[derive(Clone, Debug)]
pub struct ContentIndex {
    inner: emlite::Val,
}
impl FromVal for ContentIndex {
    fn from_val(v: &emlite::Val) -> Self {
        ContentIndex {
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
impl std::ops::Deref for ContentIndex {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ContentIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ContentIndex> for emlite::Val {
    fn from(s: ContentIndex) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ContentIndex {
    pub fn add(&self, description: ContentDescription) -> jsbind::Promise {
        self.inner
            .call("add", &[description.into()])
            .as_::<jsbind::Promise>()
    }
}
impl ContentIndex {
    pub fn delete(&self, id: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("delete", &[id.into()])
            .as_::<jsbind::Promise>()
    }
}
impl ContentIndex {
    pub fn get_all(&self) -> jsbind::Promise {
        self.inner.call("getAll", &[]).as_::<jsbind::Promise>()
    }
}
