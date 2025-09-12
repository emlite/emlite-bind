use super::*;

/// The MediaStreamTrack class.
/// [`MediaStreamTrack`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamTrack {
    inner: EventTarget,
}

impl FromVal for MediaStreamTrack {
    fn from_val(v: &Any) -> Self {
        MediaStreamTrack {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaStreamTrack {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaStreamTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaStreamTrack {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaStreamTrack {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaStreamTrack> for Any {
    fn from(s: MediaStreamTrack) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaStreamTrack> for Any {
    fn from(s: &MediaStreamTrack) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaStreamTrack);

impl MediaStreamTrack {
    /// Getter of the `kind` attribute.
    /// [`MediaStreamTrack.kind`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/kind)
    pub fn kind(&self) -> JsString {
        self.inner.get("kind").as_::<JsString>()
    }
}
impl MediaStreamTrack {
    /// Getter of the `id` attribute.
    /// [`MediaStreamTrack.id`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}
impl MediaStreamTrack {
    /// Getter of the `label` attribute.
    /// [`MediaStreamTrack.label`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }
}
impl MediaStreamTrack {
    /// Getter of the `enabled` attribute.
    /// [`MediaStreamTrack.enabled`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/enabled)
    pub fn enabled(&self) -> bool {
        self.inner.get("enabled").as_::<bool>()
    }

    /// Setter of the `enabled` attribute.
    /// [`MediaStreamTrack.enabled`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/enabled)
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.set("enabled", value);
    }
}
impl MediaStreamTrack {
    /// Getter of the `muted` attribute.
    /// [`MediaStreamTrack.muted`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/muted)
    pub fn muted(&self) -> bool {
        self.inner.get("muted").as_::<bool>()
    }
}
impl MediaStreamTrack {
    /// Getter of the `onmute` attribute.
    /// [`MediaStreamTrack.onmute`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onmute)
    pub fn onmute(&self) -> Any {
        self.inner.get("onmute").as_::<Any>()
    }

    /// Setter of the `onmute` attribute.
    /// [`MediaStreamTrack.onmute`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onmute)
    pub fn set_onmute(&mut self, value: &Any) {
        self.inner.set("onmute", value);
    }
}
impl MediaStreamTrack {
    /// Getter of the `onunmute` attribute.
    /// [`MediaStreamTrack.onunmute`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onunmute)
    pub fn onunmute(&self) -> Any {
        self.inner.get("onunmute").as_::<Any>()
    }

    /// Setter of the `onunmute` attribute.
    /// [`MediaStreamTrack.onunmute`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onunmute)
    pub fn set_onunmute(&mut self, value: &Any) {
        self.inner.set("onunmute", value);
    }
}
impl MediaStreamTrack {
    /// Getter of the `readyState` attribute.
    /// [`MediaStreamTrack.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/readyState)
    pub fn ready_state(&self) -> MediaStreamTrackState {
        self.inner.get("readyState").as_::<MediaStreamTrackState>()
    }
}
impl MediaStreamTrack {
    /// Getter of the `onended` attribute.
    /// [`MediaStreamTrack.onended`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onended)
    pub fn onended(&self) -> Any {
        self.inner.get("onended").as_::<Any>()
    }

    /// Setter of the `onended` attribute.
    /// [`MediaStreamTrack.onended`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onended)
    pub fn set_onended(&mut self, value: &Any) {
        self.inner.set("onended", value);
    }
}
impl MediaStreamTrack {
    /// Getter of the `oncapturehandlechange` attribute.
    /// [`MediaStreamTrack.oncapturehandlechange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/oncapturehandlechange)
    pub fn oncapturehandlechange(&self) -> Any {
        self.inner.get("oncapturehandlechange").as_::<Any>()
    }

    /// Setter of the `oncapturehandlechange` attribute.
    /// [`MediaStreamTrack.oncapturehandlechange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/oncapturehandlechange)
    pub fn set_oncapturehandlechange(&mut self, value: &Any) {
        self.inner.set("oncapturehandlechange", value);
    }
}
impl MediaStreamTrack {
    /// Getter of the `contentHint` attribute.
    /// [`MediaStreamTrack.contentHint`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/contentHint)
    pub fn content_hint(&self) -> JsString {
        self.inner.get("contentHint").as_::<JsString>()
    }

    /// Setter of the `contentHint` attribute.
    /// [`MediaStreamTrack.contentHint`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/contentHint)
    pub fn set_content_hint(&mut self, value: &JsString) {
        self.inner.set("contentHint", value);
    }
}
impl MediaStreamTrack {
    /// Getter of the `isolated` attribute.
    /// [`MediaStreamTrack.isolated`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/isolated)
    pub fn isolated(&self) -> bool {
        self.inner.get("isolated").as_::<bool>()
    }
}
impl MediaStreamTrack {
    /// Getter of the `onisolationchange` attribute.
    /// [`MediaStreamTrack.onisolationchange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onisolationchange)
    pub fn onisolationchange(&self) -> Any {
        self.inner.get("onisolationchange").as_::<Any>()
    }

    /// Setter of the `onisolationchange` attribute.
    /// [`MediaStreamTrack.onisolationchange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/onisolationchange)
    pub fn set_onisolationchange(&mut self, value: &Any) {
        self.inner.set("onisolationchange", value);
    }
}
impl MediaStreamTrack {
    /// The clone method.
    /// [`MediaStreamTrack.clone`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/clone)
    pub fn clone_(&self) -> MediaStreamTrack {
        self.inner.call("clone", &[]).as_::<MediaStreamTrack>()
    }
}
impl MediaStreamTrack {
    /// The stop method.
    /// [`MediaStreamTrack.stop`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/stop)
    pub fn stop(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
impl MediaStreamTrack {
    /// The getCapabilities method.
    /// [`MediaStreamTrack.getCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getCapabilities)
    pub fn get_capabilities(&self) -> MediaTrackCapabilities {
        self.inner
            .call("getCapabilities", &[])
            .as_::<MediaTrackCapabilities>()
    }
}
impl MediaStreamTrack {
    /// The getConstraints method.
    /// [`MediaStreamTrack.getConstraints`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getConstraints)
    pub fn get_constraints(&self) -> MediaTrackConstraints {
        self.inner
            .call("getConstraints", &[])
            .as_::<MediaTrackConstraints>()
    }
}
impl MediaStreamTrack {
    /// The getSettings method.
    /// [`MediaStreamTrack.getSettings`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getSettings)
    pub fn get_settings(&self) -> MediaTrackSettings {
        self.inner
            .call("getSettings", &[])
            .as_::<MediaTrackSettings>()
    }
}
impl MediaStreamTrack {
    /// The applyConstraints method.
    /// [`MediaStreamTrack.applyConstraints`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/applyConstraints)
    pub fn apply_constraints0(&self) -> Promise<Undefined> {
        self.inner
            .call("applyConstraints", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The applyConstraints method.
    /// [`MediaStreamTrack.applyConstraints`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/applyConstraints)
    pub fn apply_constraints1(&self, constraints: &MediaTrackConstraints) -> Promise<Undefined> {
        self.inner
            .call("applyConstraints", &[constraints.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl MediaStreamTrack {
    /// The getCaptureHandle method.
    /// [`MediaStreamTrack.getCaptureHandle`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getCaptureHandle)
    pub fn get_capture_handle(&self) -> CaptureHandle {
        self.inner
            .call("getCaptureHandle", &[])
            .as_::<CaptureHandle>()
    }
}
impl MediaStreamTrack {
    /// The getSupportedCaptureActions method.
    /// [`MediaStreamTrack.getSupportedCaptureActions`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getSupportedCaptureActions)
    pub fn get_supported_capture_actions(&self) -> TypedArray<JsString> {
        self.inner
            .call("getSupportedCaptureActions", &[])
            .as_::<TypedArray<JsString>>()
    }
}
impl MediaStreamTrack {
    /// The sendCaptureAction method.
    /// [`MediaStreamTrack.sendCaptureAction`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/sendCaptureAction)
    pub fn send_capture_action(&self, action: &CaptureAction) -> Promise<Undefined> {
        self.inner
            .call("sendCaptureAction", &[action.into()])
            .as_::<Promise<Undefined>>()
    }
}
