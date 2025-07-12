use super::*;

#[derive(Clone, Debug)]
pub struct MediaTrackCapabilities {
    inner: emlite::Val,
}
impl FromVal for MediaTrackCapabilities {
    fn from_val(v: &emlite::Val) -> Self {
        MediaTrackCapabilities { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaTrackCapabilities {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaTrackCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaTrackCapabilities> for emlite::Val {
    fn from(s: MediaTrackCapabilities) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaTrackCapabilities {
    pub fn display_surface(&self) -> jsbind::DOMString {
        self.inner.get("displaySurface").as_::<jsbind::DOMString>()
    }

    pub fn set_display_surface(&mut self, value: jsbind::DOMString) {
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
    pub fn cursor(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("cursor")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_cursor(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
        self.inner.set("cursor", value);
    }
}
#[derive(Clone, Debug)]
pub struct MediaTrackConstraints {
    inner: emlite::Val,
}
impl FromVal for MediaTrackConstraints {
    fn from_val(v: &emlite::Val) -> Self {
        MediaTrackConstraints { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaTrackConstraints {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaTrackConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaTrackConstraints> for emlite::Val {
    fn from(s: MediaTrackConstraints) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaTrackConstraints {
    pub fn advanced(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("advanced")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_advanced(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("advanced", value);
    }
}
#[derive(Clone, Debug)]
pub struct MediaTrackSettings {
    inner: emlite::Val,
}
impl FromVal for MediaTrackSettings {
    fn from_val(v: &emlite::Val) -> Self {
        MediaTrackSettings { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaTrackSettings {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaTrackSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaTrackSettings> for emlite::Val {
    fn from(s: MediaTrackSettings) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaTrackSettings {
    pub fn display_surface(&self) -> jsbind::DOMString {
        self.inner.get("displaySurface").as_::<jsbind::DOMString>()
    }

    pub fn set_display_surface(&mut self, value: jsbind::DOMString) {
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
    pub fn cursor(&self) -> jsbind::DOMString {
        self.inner.get("cursor").as_::<jsbind::DOMString>()
    }

    pub fn set_cursor(&mut self, value: jsbind::DOMString) {
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
#[derive(Clone, Debug)]
pub struct CaptureHandle {
    inner: emlite::Val,
}
impl FromVal for CaptureHandle {
    fn from_val(v: &emlite::Val) -> Self {
        CaptureHandle { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CaptureHandle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CaptureHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CaptureHandle> for emlite::Val {
    fn from(s: CaptureHandle) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CaptureHandle {
    pub fn origin(&self) -> jsbind::DOMString {
        self.inner.get("origin").as_::<jsbind::DOMString>()
    }

    pub fn set_origin(&mut self, value: jsbind::DOMString) {
        self.inner.set("origin", value);
    }
}
impl CaptureHandle {
    pub fn handle(&self) -> jsbind::DOMString {
        self.inner.get("handle").as_::<jsbind::DOMString>()
    }

    pub fn set_handle(&mut self, value: jsbind::DOMString) {
        self.inner.set("handle", value);
    }
}
#[derive(Clone, Debug)]
pub struct MediaStreamTrack {
    inner: EventTarget,
}
impl FromVal for MediaStreamTrack {
    fn from_val(v: &emlite::Val) -> Self {
        MediaStreamTrack {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaStreamTrack {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaStreamTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaStreamTrack> for emlite::Val {
    fn from(s: MediaStreamTrack) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaStreamTrack {
    pub fn kind(&self) -> jsbind::DOMString {
        self.inner.get("kind").as_::<jsbind::DOMString>()
    }
}
impl MediaStreamTrack {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }
}
impl MediaStreamTrack {
    pub fn label(&self) -> jsbind::DOMString {
        self.inner.get("label").as_::<jsbind::DOMString>()
    }
}
impl MediaStreamTrack {
    pub fn enabled(&self) -> bool {
        self.inner.get("enabled").as_::<bool>()
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.inner.set("enabled", value);
    }
}
impl MediaStreamTrack {
    pub fn muted(&self) -> bool {
        self.inner.get("muted").as_::<bool>()
    }
}
impl MediaStreamTrack {
    pub fn onmute(&self) -> jsbind::Any {
        self.inner.get("onmute").as_::<jsbind::Any>()
    }

    pub fn set_onmute(&mut self, value: jsbind::Any) {
        self.inner.set("onmute", value);
    }
}
impl MediaStreamTrack {
    pub fn onunmute(&self) -> jsbind::Any {
        self.inner.get("onunmute").as_::<jsbind::Any>()
    }

    pub fn set_onunmute(&mut self, value: jsbind::Any) {
        self.inner.set("onunmute", value);
    }
}
impl MediaStreamTrack {
    pub fn ready_state(&self) -> MediaStreamTrackState {
        self.inner.get("readyState").as_::<MediaStreamTrackState>()
    }
}
impl MediaStreamTrack {
    pub fn onended(&self) -> jsbind::Any {
        self.inner.get("onended").as_::<jsbind::Any>()
    }

    pub fn set_onended(&mut self, value: jsbind::Any) {
        self.inner.set("onended", value);
    }
}
impl MediaStreamTrack {
    pub fn clone_(&self) -> MediaStreamTrack {
        self.inner.call("clone", &[]).as_::<MediaStreamTrack>()
    }
}
impl MediaStreamTrack {
    pub fn stop(&self) -> jsbind::Undefined {
        self.inner.call("stop", &[]).as_::<jsbind::Undefined>()
    }
}
impl MediaStreamTrack {
    pub fn get_capabilities(&self) -> MediaTrackCapabilities {
        self.inner
            .call("getCapabilities", &[])
            .as_::<MediaTrackCapabilities>()
    }
}
impl MediaStreamTrack {
    pub fn get_constraints(&self) -> MediaTrackConstraints {
        self.inner
            .call("getConstraints", &[])
            .as_::<MediaTrackConstraints>()
    }
}
impl MediaStreamTrack {
    pub fn get_settings(&self) -> MediaTrackSettings {
        self.inner
            .call("getSettings", &[])
            .as_::<MediaTrackSettings>()
    }
}
impl MediaStreamTrack {
    pub fn apply_constraints0(&self) -> jsbind::Promise {
        self.inner
            .call("applyConstraints", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn apply_constraints1(&self, constraints: MediaTrackConstraints) -> jsbind::Promise {
        self.inner
            .call("applyConstraints", &[constraints.into()])
            .as_::<jsbind::Promise>()
    }
}
impl MediaStreamTrack {
    pub fn get_capture_handle(&self) -> CaptureHandle {
        self.inner
            .call("getCaptureHandle", &[])
            .as_::<CaptureHandle>()
    }
}
impl MediaStreamTrack {
    pub fn oncapturehandlechange(&self) -> jsbind::Any {
        self.inner.get("oncapturehandlechange").as_::<jsbind::Any>()
    }

    pub fn set_oncapturehandlechange(&mut self, value: jsbind::Any) {
        self.inner.set("oncapturehandlechange", value);
    }
}
impl MediaStreamTrack {
    pub fn get_supported_capture_actions(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .call("getSupportedCaptureActions", &[])
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }
}
impl MediaStreamTrack {
    pub fn send_capture_action(&self, action: CaptureAction) -> jsbind::Promise {
        self.inner
            .call("sendCaptureAction", &[action.into()])
            .as_::<jsbind::Promise>()
    }
}
impl MediaStreamTrack {
    pub fn content_hint(&self) -> jsbind::DOMString {
        self.inner.get("contentHint").as_::<jsbind::DOMString>()
    }

    pub fn set_content_hint(&mut self, value: jsbind::DOMString) {
        self.inner.set("contentHint", value);
    }
}
impl MediaStreamTrack {
    pub fn isolated(&self) -> bool {
        self.inner.get("isolated").as_::<bool>()
    }
}
impl MediaStreamTrack {
    pub fn onisolationchange(&self) -> jsbind::Any {
        self.inner.get("onisolationchange").as_::<jsbind::Any>()
    }

    pub fn set_onisolationchange(&mut self, value: jsbind::Any) {
        self.inner.set("onisolationchange", value);
    }
}
