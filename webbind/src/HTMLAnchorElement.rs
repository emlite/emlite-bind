use super::*;

/// The HTMLAnchorElement class.
/// [`HTMLAnchorElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLAnchorElement {
    inner: HTMLElement,
}
impl FromVal for HTMLAnchorElement {
    fn from_val(v: &Any) -> Self {
        HTMLAnchorElement {
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
impl core::ops::Deref for HTMLAnchorElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLAnchorElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLAnchorElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLAnchorElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLAnchorElement> for Any {
    fn from(s: HTMLAnchorElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLAnchorElement> for Any {
    fn from(s: &HTMLAnchorElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLAnchorElement);

impl HTMLAnchorElement {
    /// The `new HTMLAnchorElement(..)` constructor, creating a new HTMLAnchorElement instance
    pub fn new() -> HTMLAnchorElement {
        Self {
            inner: Any::global("HTMLAnchorElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLAnchorElement {
    /// Getter of the `target` attribute.
    /// [`HTMLAnchorElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)
    pub fn target(&self) -> DOMString {
        self.inner.get("target").as_::<DOMString>()
    }

    /// Setter of the `target` attribute.
    /// [`HTMLAnchorElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)
    pub fn set_target(&mut self, value: &DOMString) {
        self.inner.set("target", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `download` attribute.
    /// [`HTMLAnchorElement.download`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)
    pub fn download(&self) -> DOMString {
        self.inner.get("download").as_::<DOMString>()
    }

    /// Setter of the `download` attribute.
    /// [`HTMLAnchorElement.download`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)
    pub fn set_download(&mut self, value: &DOMString) {
        self.inner.set("download", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `ping` attribute.
    /// [`HTMLAnchorElement.ping`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)
    pub fn ping(&self) -> USVString {
        self.inner.get("ping").as_::<USVString>()
    }

    /// Setter of the `ping` attribute.
    /// [`HTMLAnchorElement.ping`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)
    pub fn set_ping(&mut self, value: &USVString) {
        self.inner.set("ping", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `rel` attribute.
    /// [`HTMLAnchorElement.rel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)
    pub fn rel(&self) -> DOMString {
        self.inner.get("rel").as_::<DOMString>()
    }

    /// Setter of the `rel` attribute.
    /// [`HTMLAnchorElement.rel`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)
    pub fn set_rel(&mut self, value: &DOMString) {
        self.inner.set("rel", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `relList` attribute.
    /// [`HTMLAnchorElement.relList`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/relList)
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl HTMLAnchorElement {
    /// Getter of the `hreflang` attribute.
    /// [`HTMLAnchorElement.hreflang`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)
    pub fn hreflang(&self) -> DOMString {
        self.inner.get("hreflang").as_::<DOMString>()
    }

    /// Setter of the `hreflang` attribute.
    /// [`HTMLAnchorElement.hreflang`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)
    pub fn set_hreflang(&mut self, value: &DOMString) {
        self.inner.set("hreflang", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `type` attribute.
    /// [`HTMLAnchorElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLAnchorElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)
    pub fn set_type_(&mut self, value: &DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `text` attribute.
    /// [`HTMLAnchorElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)
    pub fn text(&self) -> DOMString {
        self.inner.get("text").as_::<DOMString>()
    }

    /// Setter of the `text` attribute.
    /// [`HTMLAnchorElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)
    pub fn set_text(&mut self, value: &DOMString) {
        self.inner.set("text", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `referrerPolicy` attribute.
    /// [`HTMLAnchorElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)
    pub fn referrer_policy(&self) -> DOMString {
        self.inner.get("referrerPolicy").as_::<DOMString>()
    }

    /// Setter of the `referrerPolicy` attribute.
    /// [`HTMLAnchorElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)
    pub fn set_referrer_policy(&mut self, value: &DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `coords` attribute.
    /// [`HTMLAnchorElement.coords`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)
    pub fn coords(&self) -> DOMString {
        self.inner.get("coords").as_::<DOMString>()
    }

    /// Setter of the `coords` attribute.
    /// [`HTMLAnchorElement.coords`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)
    pub fn set_coords(&mut self, value: &DOMString) {
        self.inner.set("coords", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `charset` attribute.
    /// [`HTMLAnchorElement.charset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)
    pub fn charset(&self) -> DOMString {
        self.inner.get("charset").as_::<DOMString>()
    }

    /// Setter of the `charset` attribute.
    /// [`HTMLAnchorElement.charset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)
    pub fn set_charset(&mut self, value: &DOMString) {
        self.inner.set("charset", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `name` attribute.
    /// [`HTMLAnchorElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLAnchorElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)
    pub fn set_name(&mut self, value: &DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `rev` attribute.
    /// [`HTMLAnchorElement.rev`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)
    pub fn rev(&self) -> DOMString {
        self.inner.get("rev").as_::<DOMString>()
    }

    /// Setter of the `rev` attribute.
    /// [`HTMLAnchorElement.rev`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)
    pub fn set_rev(&mut self, value: &DOMString) {
        self.inner.set("rev", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `shape` attribute.
    /// [`HTMLAnchorElement.shape`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)
    pub fn shape(&self) -> DOMString {
        self.inner.get("shape").as_::<DOMString>()
    }

    /// Setter of the `shape` attribute.
    /// [`HTMLAnchorElement.shape`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)
    pub fn set_shape(&mut self, value: &DOMString) {
        self.inner.set("shape", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `attributionSourceId` attribute.
    /// [`HTMLAnchorElement.attributionSourceId`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/attributionSourceId)
    pub fn attribution_source_id(&self) -> u32 {
        self.inner.get("attributionSourceId").as_::<u32>()
    }

    /// Setter of the `attributionSourceId` attribute.
    /// [`HTMLAnchorElement.attributionSourceId`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/attributionSourceId)
    pub fn set_attribution_source_id(&mut self, value: u32) {
        self.inner.set("attributionSourceId", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `attributionSrc` attribute.
    /// [`HTMLAnchorElement.attributionSrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/attributionSrc)
    pub fn attribution_src(&self) -> USVString {
        self.inner.get("attributionSrc").as_::<USVString>()
    }

    /// Setter of the `attributionSrc` attribute.
    /// [`HTMLAnchorElement.attributionSrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/attributionSrc)
    pub fn set_attribution_src(&mut self, value: &USVString) {
        self.inner.set("attributionSrc", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `href` attribute.
    /// [`HTMLAnchorElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }

    /// Setter of the `href` attribute.
    /// [`HTMLAnchorElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)
    pub fn set_href(&mut self, value: &USVString) {
        self.inner.set("href", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `origin` attribute.
    /// [`HTMLAnchorElement.origin`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/origin)
    pub fn origin(&self) -> USVString {
        self.inner.get("origin").as_::<USVString>()
    }
}
impl HTMLAnchorElement {
    /// Getter of the `protocol` attribute.
    /// [`HTMLAnchorElement.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)
    pub fn protocol(&self) -> USVString {
        self.inner.get("protocol").as_::<USVString>()
    }

    /// Setter of the `protocol` attribute.
    /// [`HTMLAnchorElement.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)
    pub fn set_protocol(&mut self, value: &USVString) {
        self.inner.set("protocol", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `username` attribute.
    /// [`HTMLAnchorElement.username`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)
    pub fn username(&self) -> USVString {
        self.inner.get("username").as_::<USVString>()
    }

    /// Setter of the `username` attribute.
    /// [`HTMLAnchorElement.username`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)
    pub fn set_username(&mut self, value: &USVString) {
        self.inner.set("username", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `password` attribute.
    /// [`HTMLAnchorElement.password`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)
    pub fn password(&self) -> USVString {
        self.inner.get("password").as_::<USVString>()
    }

    /// Setter of the `password` attribute.
    /// [`HTMLAnchorElement.password`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)
    pub fn set_password(&mut self, value: &USVString) {
        self.inner.set("password", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `host` attribute.
    /// [`HTMLAnchorElement.host`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)
    pub fn host(&self) -> USVString {
        self.inner.get("host").as_::<USVString>()
    }

    /// Setter of the `host` attribute.
    /// [`HTMLAnchorElement.host`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)
    pub fn set_host(&mut self, value: &USVString) {
        self.inner.set("host", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `hostname` attribute.
    /// [`HTMLAnchorElement.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)
    pub fn hostname(&self) -> USVString {
        self.inner.get("hostname").as_::<USVString>()
    }

    /// Setter of the `hostname` attribute.
    /// [`HTMLAnchorElement.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)
    pub fn set_hostname(&mut self, value: &USVString) {
        self.inner.set("hostname", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `port` attribute.
    /// [`HTMLAnchorElement.port`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)
    pub fn port(&self) -> USVString {
        self.inner.get("port").as_::<USVString>()
    }

    /// Setter of the `port` attribute.
    /// [`HTMLAnchorElement.port`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)
    pub fn set_port(&mut self, value: &USVString) {
        self.inner.set("port", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `pathname` attribute.
    /// [`HTMLAnchorElement.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)
    pub fn pathname(&self) -> USVString {
        self.inner.get("pathname").as_::<USVString>()
    }

    /// Setter of the `pathname` attribute.
    /// [`HTMLAnchorElement.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)
    pub fn set_pathname(&mut self, value: &USVString) {
        self.inner.set("pathname", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `search` attribute.
    /// [`HTMLAnchorElement.search`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)
    pub fn search(&self) -> USVString {
        self.inner.get("search").as_::<USVString>()
    }

    /// Setter of the `search` attribute.
    /// [`HTMLAnchorElement.search`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)
    pub fn set_search(&mut self, value: &USVString) {
        self.inner.set("search", value);
    }
}
impl HTMLAnchorElement {
    /// Getter of the `hash` attribute.
    /// [`HTMLAnchorElement.hash`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)
    pub fn hash(&self) -> USVString {
        self.inner.get("hash").as_::<USVString>()
    }

    /// Setter of the `hash` attribute.
    /// [`HTMLAnchorElement.hash`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)
    pub fn set_hash(&mut self, value: &USVString) {
        self.inner.set("hash", value);
    }
}
