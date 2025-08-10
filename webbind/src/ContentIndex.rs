use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContentDescription {
    inner: Any,
}
impl FromVal for ContentDescription {
    fn from_val(v: &Any) -> Self {
        ContentDescription { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ContentDescription {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContentDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ContentDescription {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ContentDescription {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ContentDescription> for Any {
    fn from(s: ContentDescription) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ContentDescription> for Any {
    fn from(s: &ContentDescription) -> Any {
        s.inner.clone()
    }
}

impl ContentDescription {
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl ContentDescription {
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }

    pub fn set_title(&mut self, value: &JsString) {
        self.inner.set("title", value);
    }
}
impl ContentDescription {
    pub fn description(&self) -> JsString {
        self.inner.get("description").as_::<JsString>()
    }

    pub fn set_description(&mut self, value: &JsString) {
        self.inner.set("description", value);
    }
}
impl ContentDescription {
    pub fn category(&self) -> ContentCategory {
        self.inner.get("category").as_::<ContentCategory>()
    }

    pub fn set_category(&mut self, value: &ContentCategory) {
        self.inner.set("category", value);
    }
}
impl ContentDescription {
    pub fn icons(&self) -> TypedArray<ImageResource> {
        self.inner.get("icons").as_::<TypedArray<ImageResource>>()
    }

    pub fn set_icons(&mut self, value: &TypedArray<ImageResource>) {
        self.inner.set("icons", value);
    }
}
impl ContentDescription {
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
/// The ContentIndex class.
/// [`ContentIndex`](https://developer.mozilla.org/en-US/docs/Web/API/ContentIndex)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContentIndex {
    inner: Any,
}
impl FromVal for ContentIndex {
    fn from_val(v: &Any) -> Self {
        ContentIndex {
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
impl core::ops::Deref for ContentIndex {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContentIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ContentIndex {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ContentIndex {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ContentIndex> for Any {
    fn from(s: ContentIndex) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ContentIndex> for Any {
    fn from(s: &ContentIndex) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ContentIndex);

impl ContentIndex {
    /// The add method.
    /// [`ContentIndex.add`](https://developer.mozilla.org/en-US/docs/Web/API/ContentIndex/add)
    pub fn add(&self, description: &ContentDescription) -> Promise<Undefined> {
        self.inner
            .call("add", &[description.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl ContentIndex {
    /// The delete method.
    /// [`ContentIndex.delete`](https://developer.mozilla.org/en-US/docs/Web/API/ContentIndex/delete)
    pub fn delete(&self, id: &JsString) -> Promise<Undefined> {
        self.inner
            .call("delete", &[id.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl ContentIndex {
    /// The getAll method.
    /// [`ContentIndex.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/ContentIndex/getAll)
    pub fn get_all(&self) -> Promise<TypedArray<ContentDescription>> {
        self.inner
            .call("getAll", &[])
            .as_::<Promise<TypedArray<ContentDescription>>>()
    }
}
