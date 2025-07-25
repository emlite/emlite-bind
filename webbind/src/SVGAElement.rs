use super::*;

/// The SVGAElement class.
/// [`SVGAElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGAElement {
    fn from_val(v: &Any) -> Self {
        SVGAElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGAElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGAElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGAElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGAElement> for Any {
    fn from(s: SVGAElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGAElement> for Any {
    fn from(s: &SVGAElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAElement);

impl SVGAElement {
    /// Getter of the `target` attribute.
    /// [`SVGAElement.target`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/target)
    pub fn target(&self) -> SVGAnimatedString {
        self.inner.get("target").as_::<SVGAnimatedString>()
    }
}
impl SVGAElement {
    /// Getter of the `download` attribute.
    /// [`SVGAElement.download`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)
    pub fn download(&self) -> DOMString {
        self.inner.get("download").as_::<DOMString>()
    }

    /// Setter of the `download` attribute.
    /// [`SVGAElement.download`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)
    pub fn set_download(&mut self, value: &DOMString) {
        self.inner.set("download", value);
    }
}
impl SVGAElement {
    /// Getter of the `ping` attribute.
    /// [`SVGAElement.ping`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)
    pub fn ping(&self) -> USVString {
        self.inner.get("ping").as_::<USVString>()
    }

    /// Setter of the `ping` attribute.
    /// [`SVGAElement.ping`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)
    pub fn set_ping(&mut self, value: &USVString) {
        self.inner.set("ping", value);
    }
}
impl SVGAElement {
    /// Getter of the `rel` attribute.
    /// [`SVGAElement.rel`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)
    pub fn rel(&self) -> DOMString {
        self.inner.get("rel").as_::<DOMString>()
    }

    /// Setter of the `rel` attribute.
    /// [`SVGAElement.rel`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)
    pub fn set_rel(&mut self, value: &DOMString) {
        self.inner.set("rel", value);
    }
}
impl SVGAElement {
    /// Getter of the `relList` attribute.
    /// [`SVGAElement.relList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/relList)
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl SVGAElement {
    /// Getter of the `hreflang` attribute.
    /// [`SVGAElement.hreflang`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)
    pub fn hreflang(&self) -> DOMString {
        self.inner.get("hreflang").as_::<DOMString>()
    }

    /// Setter of the `hreflang` attribute.
    /// [`SVGAElement.hreflang`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)
    pub fn set_hreflang(&mut self, value: &DOMString) {
        self.inner.set("hreflang", value);
    }
}
impl SVGAElement {
    /// Getter of the `type` attribute.
    /// [`SVGAElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    /// Setter of the `type` attribute.
    /// [`SVGAElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)
    pub fn set_type_(&mut self, value: &DOMString) {
        self.inner.set("type", value);
    }
}
impl SVGAElement {
    /// Getter of the `text` attribute.
    /// [`SVGAElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)
    pub fn text(&self) -> DOMString {
        self.inner.get("text").as_::<DOMString>()
    }

    /// Setter of the `text` attribute.
    /// [`SVGAElement.text`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)
    pub fn set_text(&mut self, value: &DOMString) {
        self.inner.set("text", value);
    }
}
impl SVGAElement {
    /// Getter of the `referrerPolicy` attribute.
    /// [`SVGAElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)
    pub fn referrer_policy(&self) -> DOMString {
        self.inner.get("referrerPolicy").as_::<DOMString>()
    }

    /// Setter of the `referrerPolicy` attribute.
    /// [`SVGAElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)
    pub fn set_referrer_policy(&mut self, value: &DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl SVGAElement {
    /// Getter of the `origin` attribute.
    /// [`SVGAElement.origin`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/origin)
    pub fn origin(&self) -> USVString {
        self.inner.get("origin").as_::<USVString>()
    }
}
impl SVGAElement {
    /// Getter of the `protocol` attribute.
    /// [`SVGAElement.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/protocol)
    pub fn protocol(&self) -> USVString {
        self.inner.get("protocol").as_::<USVString>()
    }

    /// Setter of the `protocol` attribute.
    /// [`SVGAElement.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/protocol)
    pub fn set_protocol(&mut self, value: &USVString) {
        self.inner.set("protocol", value);
    }
}
impl SVGAElement {
    /// Getter of the `username` attribute.
    /// [`SVGAElement.username`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/username)
    pub fn username(&self) -> USVString {
        self.inner.get("username").as_::<USVString>()
    }

    /// Setter of the `username` attribute.
    /// [`SVGAElement.username`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/username)
    pub fn set_username(&mut self, value: &USVString) {
        self.inner.set("username", value);
    }
}
impl SVGAElement {
    /// Getter of the `password` attribute.
    /// [`SVGAElement.password`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/password)
    pub fn password(&self) -> USVString {
        self.inner.get("password").as_::<USVString>()
    }

    /// Setter of the `password` attribute.
    /// [`SVGAElement.password`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/password)
    pub fn set_password(&mut self, value: &USVString) {
        self.inner.set("password", value);
    }
}
impl SVGAElement {
    /// Getter of the `host` attribute.
    /// [`SVGAElement.host`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/host)
    pub fn host(&self) -> USVString {
        self.inner.get("host").as_::<USVString>()
    }

    /// Setter of the `host` attribute.
    /// [`SVGAElement.host`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/host)
    pub fn set_host(&mut self, value: &USVString) {
        self.inner.set("host", value);
    }
}
impl SVGAElement {
    /// Getter of the `hostname` attribute.
    /// [`SVGAElement.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hostname)
    pub fn hostname(&self) -> USVString {
        self.inner.get("hostname").as_::<USVString>()
    }

    /// Setter of the `hostname` attribute.
    /// [`SVGAElement.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hostname)
    pub fn set_hostname(&mut self, value: &USVString) {
        self.inner.set("hostname", value);
    }
}
impl SVGAElement {
    /// Getter of the `port` attribute.
    /// [`SVGAElement.port`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/port)
    pub fn port(&self) -> USVString {
        self.inner.get("port").as_::<USVString>()
    }

    /// Setter of the `port` attribute.
    /// [`SVGAElement.port`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/port)
    pub fn set_port(&mut self, value: &USVString) {
        self.inner.set("port", value);
    }
}
impl SVGAElement {
    /// Getter of the `pathname` attribute.
    /// [`SVGAElement.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/pathname)
    pub fn pathname(&self) -> USVString {
        self.inner.get("pathname").as_::<USVString>()
    }

    /// Setter of the `pathname` attribute.
    /// [`SVGAElement.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/pathname)
    pub fn set_pathname(&mut self, value: &USVString) {
        self.inner.set("pathname", value);
    }
}
impl SVGAElement {
    /// Getter of the `search` attribute.
    /// [`SVGAElement.search`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/search)
    pub fn search(&self) -> USVString {
        self.inner.get("search").as_::<USVString>()
    }

    /// Setter of the `search` attribute.
    /// [`SVGAElement.search`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/search)
    pub fn set_search(&mut self, value: &USVString) {
        self.inner.set("search", value);
    }
}
impl SVGAElement {
    /// Getter of the `hash` attribute.
    /// [`SVGAElement.hash`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hash)
    pub fn hash(&self) -> USVString {
        self.inner.get("hash").as_::<USVString>()
    }

    /// Setter of the `hash` attribute.
    /// [`SVGAElement.hash`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hash)
    pub fn set_hash(&mut self, value: &USVString) {
        self.inner.set("hash", value);
    }
}
impl SVGAElement {
    /// Getter of the `href` attribute.
    /// [`SVGAElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/href)
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
