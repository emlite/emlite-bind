use super::*;

/// The HTMLAreaElement class.
/// [`HTMLAreaElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLAreaElement {
    inner: HTMLElement,
}
impl FromVal for HTMLAreaElement {
    fn from_val(v: &Any) -> Self {
        HTMLAreaElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLAreaElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLAreaElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLAreaElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLAreaElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLAreaElement> for Any {
    fn from(s: HTMLAreaElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLAreaElement> for Any {
    fn from(s: &HTMLAreaElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLAreaElement);

impl HTMLAreaElement {
    /// The `new HTMLAreaElement(..)` constructor, creating a new HTMLAreaElement instance
    pub fn new() -> HTMLAreaElement {
        Self {
            inner: Any::global("HTMLAreaElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLAreaElement {
    /// Getter of the `alt` attribute.
    /// [`HTMLAreaElement.alt`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/alt)
    pub fn alt(&self) -> JsString {
        self.inner.get("alt").as_::<JsString>()
    }

    /// Setter of the `alt` attribute.
    /// [`HTMLAreaElement.alt`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/alt)
    pub fn set_alt(&mut self, value: &JsString) {
        self.inner.set("alt", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `coords` attribute.
    /// [`HTMLAreaElement.coords`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/coords)
    pub fn coords(&self) -> JsString {
        self.inner.get("coords").as_::<JsString>()
    }

    /// Setter of the `coords` attribute.
    /// [`HTMLAreaElement.coords`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/coords)
    pub fn set_coords(&mut self, value: &JsString) {
        self.inner.set("coords", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `shape` attribute.
    /// [`HTMLAreaElement.shape`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/shape)
    pub fn shape(&self) -> JsString {
        self.inner.get("shape").as_::<JsString>()
    }

    /// Setter of the `shape` attribute.
    /// [`HTMLAreaElement.shape`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/shape)
    pub fn set_shape(&mut self, value: &JsString) {
        self.inner.set("shape", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `target` attribute.
    /// [`HTMLAreaElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/target)
    pub fn target(&self) -> JsString {
        self.inner.get("target").as_::<JsString>()
    }

    /// Setter of the `target` attribute.
    /// [`HTMLAreaElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/target)
    pub fn set_target(&mut self, value: &JsString) {
        self.inner.set("target", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `download` attribute.
    /// [`HTMLAreaElement.download`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/download)
    pub fn download(&self) -> JsString {
        self.inner.get("download").as_::<JsString>()
    }

    /// Setter of the `download` attribute.
    /// [`HTMLAreaElement.download`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/download)
    pub fn set_download(&mut self, value: &JsString) {
        self.inner.set("download", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `ping` attribute.
    /// [`HTMLAreaElement.ping`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/ping)
    pub fn ping(&self) -> JsString {
        self.inner.get("ping").as_::<JsString>()
    }

    /// Setter of the `ping` attribute.
    /// [`HTMLAreaElement.ping`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/ping)
    pub fn set_ping(&mut self, value: &JsString) {
        self.inner.set("ping", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `rel` attribute.
    /// [`HTMLAreaElement.rel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/rel)
    pub fn rel(&self) -> JsString {
        self.inner.get("rel").as_::<JsString>()
    }

    /// Setter of the `rel` attribute.
    /// [`HTMLAreaElement.rel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/rel)
    pub fn set_rel(&mut self, value: &JsString) {
        self.inner.set("rel", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `relList` attribute.
    /// [`HTMLAreaElement.relList`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/relList)
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl HTMLAreaElement {
    /// Getter of the `referrerPolicy` attribute.
    /// [`HTMLAreaElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/referrerPolicy)
    pub fn referrer_policy(&self) -> JsString {
        self.inner.get("referrerPolicy").as_::<JsString>()
    }

    /// Setter of the `referrerPolicy` attribute.
    /// [`HTMLAreaElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/referrerPolicy)
    pub fn set_referrer_policy(&mut self, value: &JsString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `noHref` attribute.
    /// [`HTMLAreaElement.noHref`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/noHref)
    pub fn no_href(&self) -> bool {
        self.inner.get("noHref").as_::<bool>()
    }

    /// Setter of the `noHref` attribute.
    /// [`HTMLAreaElement.noHref`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/noHref)
    pub fn set_no_href(&mut self, value: bool) {
        self.inner.set("noHref", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `attributionSrc` attribute.
    /// [`HTMLAreaElement.attributionSrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/attributionSrc)
    pub fn attribution_src(&self) -> JsString {
        self.inner.get("attributionSrc").as_::<JsString>()
    }

    /// Setter of the `attributionSrc` attribute.
    /// [`HTMLAreaElement.attributionSrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/attributionSrc)
    pub fn set_attribution_src(&mut self, value: &JsString) {
        self.inner.set("attributionSrc", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `href` attribute.
    /// [`HTMLAreaElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/href)
    pub fn href(&self) -> JsString {
        self.inner.get("href").as_::<JsString>()
    }

    /// Setter of the `href` attribute.
    /// [`HTMLAreaElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/href)
    pub fn set_href(&mut self, value: &JsString) {
        self.inner.set("href", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `origin` attribute.
    /// [`HTMLAreaElement.origin`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/origin)
    pub fn origin(&self) -> JsString {
        self.inner.get("origin").as_::<JsString>()
    }
}
impl HTMLAreaElement {
    /// Getter of the `protocol` attribute.
    /// [`HTMLAreaElement.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/protocol)
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    /// Setter of the `protocol` attribute.
    /// [`HTMLAreaElement.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/protocol)
    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `username` attribute.
    /// [`HTMLAreaElement.username`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/username)
    pub fn username(&self) -> JsString {
        self.inner.get("username").as_::<JsString>()
    }

    /// Setter of the `username` attribute.
    /// [`HTMLAreaElement.username`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/username)
    pub fn set_username(&mut self, value: &JsString) {
        self.inner.set("username", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `password` attribute.
    /// [`HTMLAreaElement.password`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/password)
    pub fn password(&self) -> JsString {
        self.inner.get("password").as_::<JsString>()
    }

    /// Setter of the `password` attribute.
    /// [`HTMLAreaElement.password`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/password)
    pub fn set_password(&mut self, value: &JsString) {
        self.inner.set("password", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `host` attribute.
    /// [`HTMLAreaElement.host`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/host)
    pub fn host(&self) -> JsString {
        self.inner.get("host").as_::<JsString>()
    }

    /// Setter of the `host` attribute.
    /// [`HTMLAreaElement.host`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/host)
    pub fn set_host(&mut self, value: &JsString) {
        self.inner.set("host", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `hostname` attribute.
    /// [`HTMLAreaElement.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hostname)
    pub fn hostname(&self) -> JsString {
        self.inner.get("hostname").as_::<JsString>()
    }

    /// Setter of the `hostname` attribute.
    /// [`HTMLAreaElement.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hostname)
    pub fn set_hostname(&mut self, value: &JsString) {
        self.inner.set("hostname", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `port` attribute.
    /// [`HTMLAreaElement.port`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/port)
    pub fn port(&self) -> JsString {
        self.inner.get("port").as_::<JsString>()
    }

    /// Setter of the `port` attribute.
    /// [`HTMLAreaElement.port`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/port)
    pub fn set_port(&mut self, value: &JsString) {
        self.inner.set("port", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `pathname` attribute.
    /// [`HTMLAreaElement.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/pathname)
    pub fn pathname(&self) -> JsString {
        self.inner.get("pathname").as_::<JsString>()
    }

    /// Setter of the `pathname` attribute.
    /// [`HTMLAreaElement.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/pathname)
    pub fn set_pathname(&mut self, value: &JsString) {
        self.inner.set("pathname", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `search` attribute.
    /// [`HTMLAreaElement.search`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/search)
    pub fn search(&self) -> JsString {
        self.inner.get("search").as_::<JsString>()
    }

    /// Setter of the `search` attribute.
    /// [`HTMLAreaElement.search`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/search)
    pub fn set_search(&mut self, value: &JsString) {
        self.inner.set("search", value);
    }
}
impl HTMLAreaElement {
    /// Getter of the `hash` attribute.
    /// [`HTMLAreaElement.hash`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hash)
    pub fn hash(&self) -> JsString {
        self.inner.get("hash").as_::<JsString>()
    }

    /// Setter of the `hash` attribute.
    /// [`HTMLAreaElement.hash`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement/hash)
    pub fn set_hash(&mut self, value: &JsString) {
        self.inner.set("hash", value);
    }
}
