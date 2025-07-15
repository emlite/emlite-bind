use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for SanitizerConfig {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SanitizerConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SanitizerConfig {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SanitizerConfig {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SanitizerConfig> for emlite::Val {
    fn from(s: SanitizerConfig) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SanitizerConfig> for emlite::Val {
    fn from(s: &SanitizerConfig) -> emlite::Val {
        s.inner.clone()
    }
}

impl SanitizerConfig {
    pub fn elements(&self) -> Sequence<Any> {
        self.inner.get("elements").as_::<Sequence<Any>>()
    }

    pub fn set_elements(&mut self, value: &Sequence<Any>) {
        self.inner.set("elements", value);
    }
}
impl SanitizerConfig {
    pub fn remove_elements(&self) -> Sequence<Any> {
        self.inner.get("removeElements").as_::<Sequence<Any>>()
    }

    pub fn set_remove_elements(&mut self, value: &Sequence<Any>) {
        self.inner.set("removeElements", value);
    }
}
impl SanitizerConfig {
    pub fn replace_with_children_elements(&self) -> Sequence<Any> {
        self.inner
            .get("replaceWithChildrenElements")
            .as_::<Sequence<Any>>()
    }

    pub fn set_replace_with_children_elements(&mut self, value: &Sequence<Any>) {
        self.inner.set("replaceWithChildrenElements", value);
    }
}
impl SanitizerConfig {
    pub fn attributes(&self) -> Sequence<Any> {
        self.inner.get("attributes").as_::<Sequence<Any>>()
    }

    pub fn set_attributes(&mut self, value: &Sequence<Any>) {
        self.inner.set("attributes", value);
    }
}
impl SanitizerConfig {
    pub fn remove_attributes(&self) -> Sequence<Any> {
        self.inner.get("removeAttributes").as_::<Sequence<Any>>()
    }

    pub fn set_remove_attributes(&mut self, value: &Sequence<Any>) {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for Sanitizer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Sanitizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Sanitizer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Sanitizer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Sanitizer> for emlite::Val {
    fn from(s: Sanitizer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Sanitizer> for emlite::Val {
    fn from(s: &Sanitizer) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Sanitizer);

impl Sanitizer {
    pub fn new0() -> Sanitizer {
        Self {
            inner: emlite::Val::global("Sanitizer")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(configuration: &Any) -> Sanitizer {
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
    pub fn allow_element(&self, element: &Any) -> Undefined {
        self.inner
            .call("allowElement", &[element.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    pub fn remove_element(&self, element: &Any) -> Undefined {
        self.inner
            .call("removeElement", &[element.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    pub fn replace_element_with_children(&self, element: &Any) -> Undefined {
        self.inner
            .call("replaceElementWithChildren", &[element.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    pub fn allow_attribute(&self, attribute: &Any) -> Undefined {
        self.inner
            .call("allowAttribute", &[attribute.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    pub fn remove_attribute(&self, attribute: &Any) -> Undefined {
        self.inner
            .call("removeAttribute", &[attribute.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    pub fn set_comments(&self, allow: bool) -> Undefined {
        self.inner
            .call("setComments", &[allow.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    pub fn set_data_attributes(&self, allow: bool) -> Undefined {
        self.inner
            .call("setDataAttributes", &[allow.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    pub fn remove_unsafe(&self) -> Undefined {
        self.inner.call("removeUnsafe", &[]).as_::<Undefined>()
    }
}
