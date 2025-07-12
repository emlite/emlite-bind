use super::*;

#[derive(Clone, Debug)]
pub struct SanitizerConfig {
    inner: emlite::Val,
}
impl FromVal for SanitizerConfig {
    fn from_val(v: &emlite::Val) -> Self {
        SanitizerConfig { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SanitizerConfig {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SanitizerConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SanitizerConfig> for emlite::Val {
    fn from(s: SanitizerConfig) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SanitizerConfig {
    pub fn elements(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("elements")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_elements(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("elements", value);
    }
}
impl SanitizerConfig {
    pub fn remove_elements(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("removeElements")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_remove_elements(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("removeElements", value);
    }
}
impl SanitizerConfig {
    pub fn replace_with_children_elements(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("replaceWithChildrenElements")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_replace_with_children_elements(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("replaceWithChildrenElements", value);
    }
}
impl SanitizerConfig {
    pub fn attributes(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("attributes")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_attributes(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("attributes", value);
    }
}
impl SanitizerConfig {
    pub fn remove_attributes(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("removeAttributes")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_remove_attributes(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("removeAttributes", value);
    }
}
impl SanitizerConfig {
    pub fn comments(&self) -> bool {
        self.inner.get("comments").as_::<bool>()
    }

    pub fn set_comments(&mut self, value: bool) {
        self.inner.set("comments", value);
    }
}
impl SanitizerConfig {
    pub fn data_attributes(&self) -> bool {
        self.inner.get("dataAttributes").as_::<bool>()
    }

    pub fn set_data_attributes(&mut self, value: bool) {
        self.inner.set("dataAttributes", value);
    }
}
#[derive(Clone, Debug)]
pub struct Sanitizer {
    inner: emlite::Val,
}
impl FromVal for Sanitizer {
    fn from_val(v: &emlite::Val) -> Self {
        Sanitizer {
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
impl std::ops::Deref for Sanitizer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Sanitizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Sanitizer> for emlite::Val {
    fn from(s: Sanitizer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Sanitizer {
    pub fn new0() -> Sanitizer {
        Self {
            inner: emlite::Val::global("Sanitizer")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(configuration: jsbind::Any) -> Sanitizer {
        Self {
            inner: emlite::Val::global("Sanitizer")
                .new(&[configuration.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Sanitizer {
    pub fn get(&self) -> SanitizerConfig {
        self.inner.call("get", &[]).as_::<SanitizerConfig>()
    }
}
impl Sanitizer {
    pub fn allow_element(&self, element: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("allowElement", &[element.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Sanitizer {
    pub fn remove_element(&self, element: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("removeElement", &[element.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Sanitizer {
    pub fn replace_element_with_children(&self, element: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("replaceElementWithChildren", &[element.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Sanitizer {
    pub fn allow_attribute(&self, attribute: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("allowAttribute", &[attribute.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Sanitizer {
    pub fn remove_attribute(&self, attribute: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("removeAttribute", &[attribute.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Sanitizer {
    pub fn set_comments(&self, allow: bool) -> jsbind::Undefined {
        self.inner
            .call("setComments", &[allow.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Sanitizer {
    pub fn set_data_attributes(&self, allow: bool) -> jsbind::Undefined {
        self.inner
            .call("setDataAttributes", &[allow.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Sanitizer {
    pub fn remove_unsafe(&self) -> jsbind::Undefined {
        self.inner
            .call("removeUnsafe", &[])
            .as_::<jsbind::Undefined>()
    }
}
