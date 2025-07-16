use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaTrackCapabilities {
    inner: Any,
}
impl FromVal for MediaTrackCapabilities {
    fn from_val(v: &Any) -> Self {
        MediaTrackCapabilities { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaTrackCapabilities {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaTrackCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaTrackCapabilities {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaTrackCapabilities {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaTrackCapabilities> for Any {
    fn from(s: MediaTrackCapabilities) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaTrackCapabilities> for Any {
    fn from(s: &MediaTrackCapabilities) -> Any {
        s.inner.clone()
    }
}

impl MediaTrackCapabilities {
    pub fn display_surface(&self) -> String {
        self.inner.get("displaySurface").as_::<String>()
    }

    pub fn set_display_surface(&mut self, value: &str) {
        self.inner.set("displaySurface", value);
    }
}
impl MediaTrackCapabilities {
    pub fn logical_surface(&self) -> bool {
        self.inner.get("logicalSurface").as_::<bool>()
    }

    pub fn set_logical_surface(&mut self, value: bool) {
        self.inner.set("logicalSurface", value);
    }
}
impl MediaTrackCapabilities {
    pub fn cursor(&self) -> Sequence<String> {
        self.inner.get("cursor").as_::<Sequence<String>>()
    }

    pub fn set_cursor(&mut self, value: &Sequence<String>) {
        self.inner.set("cursor", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaTrackConstraints {
    inner: Any,
}
impl FromVal for MediaTrackConstraints {
    fn from_val(v: &Any) -> Self {
        MediaTrackConstraints { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaTrackConstraints {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaTrackConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaTrackConstraints {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaTrackConstraints {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaTrackConstraints> for Any {
    fn from(s: MediaTrackConstraints) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaTrackConstraints> for Any {
    fn from(s: &MediaTrackConstraints) -> Any {
        s.inner.clone()
    }
}

impl MediaTrackConstraints {
    pub fn advanced(&self) -> Sequence<Any> {
        self.inner.get("advanced").as_::<Sequence<Any>>()
    }

    pub fn set_advanced(&mut self, value: &Sequence<Any>) {
        self.inner.set("advanced", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaTrackSettings {
    inner: Any,
}
impl FromVal for MediaTrackSettings {
    fn from_val(v: &Any) -> Self {
        MediaTrackSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaTrackSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaTrackSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaTrackSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaTrackSettings {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaTrackSettings> for Any {
    fn from(s: MediaTrackSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaTrackSettings> for Any {
    fn from(s: &MediaTrackSettings) -> Any {
        s.inner.clone()
    }
}

impl MediaTrackSettings {
    pub fn display_surface(&self) -> String {
        self.inner.get("displaySurface").as_::<String>()
    }

    pub fn set_display_surface(&mut self, value: &str) {
        self.inner.set("displaySurface", value);
    }
}
impl MediaTrackSettings {
    pub fn logical_surface(&self) -> bool {
        self.inner.get("logicalSurface").as_::<bool>()
    }

    pub fn set_logical_surface(&mut self, value: bool) {
        self.inner.set("logicalSurface", value);
    }
}
impl MediaTrackSettings {
    pub fn cursor(&self) -> String {
        self.inner.get("cursor").as_::<String>()
    }

    pub fn set_cursor(&mut self, value: &str) {
        self.inner.set("cursor", value);
    }
}
impl MediaTrackSettings {
    pub fn restrict_own_audio(&self) -> bool {
        self.inner.get("restrictOwnAudio").as_::<bool>()
    }

    pub fn set_restrict_own_audio(&mut self, value: bool) {
        self.inner.set("restrictOwnAudio", value);
    }
}
impl MediaTrackSettings {
    pub fn suppress_local_audio_playback(&self) -> bool {
        self.inner.get("suppressLocalAudioPlayback").as_::<bool>()
    }

    pub fn set_suppress_local_audio_playback(&mut self, value: bool) {
        self.inner.set("suppressLocalAudioPlayback", value);
    }
}
impl MediaTrackSettings {
    pub fn screen_pixel_ratio(&self) -> f64 {
        self.inner.get("screenPixelRatio").as_::<f64>()
    }

    pub fn set_screen_pixel_ratio(&mut self, value: f64) {
        self.inner.set("screenPixelRatio", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaptureHandle {
    inner: Any,
}
impl FromVal for CaptureHandle {
    fn from_val(v: &Any) -> Self {
        CaptureHandle { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CaptureHandle {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CaptureHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CaptureHandle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CaptureHandle {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CaptureHandle> for Any {
    fn from(s: CaptureHandle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CaptureHandle> for Any {
    fn from(s: &CaptureHandle) -> Any {
        s.inner.clone()
    }
}

impl CaptureHandle {
    pub fn origin(&self) -> String {
        self.inner.get("origin").as_::<String>()
    }

    pub fn set_origin(&mut self, value: &str) {
        self.inner.set("origin", value);
    }
}
impl CaptureHandle {
    pub fn handle(&self) -> String {
        self.inner.get("handle").as_::<String>()
    }

    pub fn set_handle(&mut self, value: &str) {
        self.inner.set("handle", value);
    }
}
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
    pub fn kind(&self) -> String {
        self.inner.get("kind").as_::<String>()
    }
}
impl MediaStreamTrack {
    /// Getter of the `id` attribute.
    /// [`MediaStreamTrack.id`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/id)
    pub fn id(&self) -> String {
        self.inner.get("id").as_::<String>()
    }
}
impl MediaStreamTrack {
    /// Getter of the `label` attribute.
    /// [`MediaStreamTrack.label`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/label)
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
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
    pub fn apply_constraints0(&self) -> Promise {
        self.inner.call("applyConstraints", &[]).as_::<Promise>()
    }
    /// The applyConstraints method.
    /// [`MediaStreamTrack.applyConstraints`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/applyConstraints)
    pub fn apply_constraints1(&self, constraints: &MediaTrackConstraints) -> Promise {
        self.inner
            .call("applyConstraints", &[constraints.into()])
            .as_::<Promise>()
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
    /// The getSupportedCaptureActions method.
    /// [`MediaStreamTrack.getSupportedCaptureActions`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/getSupportedCaptureActions)
    pub fn get_supported_capture_actions(&self) -> Sequence<String> {
        self.inner
            .call("getSupportedCaptureActions", &[])
            .as_::<Sequence<String>>()
    }
}
impl MediaStreamTrack {
    /// The sendCaptureAction method.
    /// [`MediaStreamTrack.sendCaptureAction`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/sendCaptureAction)
    pub fn send_capture_action(&self, action: &CaptureAction) -> Promise {
        self.inner
            .call("sendCaptureAction", &[action.into()])
            .as_::<Promise>()
    }
}
impl MediaStreamTrack {
    /// Getter of the `contentHint` attribute.
    /// [`MediaStreamTrack.contentHint`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/contentHint)
    pub fn content_hint(&self) -> String {
        self.inner.get("contentHint").as_::<String>()
    }

    /// Setter of the `contentHint` attribute.
    /// [`MediaStreamTrack.contentHint`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack/contentHint)
    pub fn set_content_hint(&mut self, value: &str) {
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
