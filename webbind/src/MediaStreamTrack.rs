use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for MediaTrackCapabilities {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaTrackCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaTrackCapabilities {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaTrackCapabilities {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaTrackCapabilities> for emlite::Val {
    fn from(s: MediaTrackCapabilities) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaTrackCapabilities> for emlite::Val {
    fn from(s: &MediaTrackCapabilities) -> emlite::Val {
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
impl core::ops::Deref for MediaTrackConstraints {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaTrackConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaTrackConstraints {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaTrackConstraints {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaTrackConstraints> for emlite::Val {
    fn from(s: MediaTrackConstraints) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaTrackConstraints> for emlite::Val {
    fn from(s: &MediaTrackConstraints) -> emlite::Val {
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
impl core::ops::Deref for MediaTrackSettings {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaTrackSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaTrackSettings {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaTrackSettings {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaTrackSettings> for emlite::Val {
    fn from(s: MediaTrackSettings) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaTrackSettings> for emlite::Val {
    fn from(s: &MediaTrackSettings) -> emlite::Val {
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
impl core::ops::Deref for CaptureHandle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CaptureHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CaptureHandle {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CaptureHandle {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CaptureHandle> for emlite::Val {
    fn from(s: CaptureHandle) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CaptureHandle> for emlite::Val {
    fn from(s: &CaptureHandle) -> emlite::Val {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for MediaStreamTrack {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaStreamTrack {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaStreamTrack> for emlite::Val {
    fn from(s: MediaStreamTrack) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaStreamTrack> for emlite::Val {
    fn from(s: &MediaStreamTrack) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaStreamTrack);

impl MediaStreamTrack {
    pub fn kind(&self) -> String {
        self.inner.get("kind").as_::<String>()
    }
}
impl MediaStreamTrack {
    pub fn id(&self) -> String {
        self.inner.get("id").as_::<String>()
    }
}
impl MediaStreamTrack {
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
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
    pub fn onmute(&self) -> Any {
        self.inner.get("onmute").as_::<Any>()
    }

    pub fn set_onmute(&mut self, value: &Any) {
        self.inner.set("onmute", value);
    }
}
impl MediaStreamTrack {
    pub fn onunmute(&self) -> Any {
        self.inner.get("onunmute").as_::<Any>()
    }

    pub fn set_onunmute(&mut self, value: &Any) {
        self.inner.set("onunmute", value);
    }
}
impl MediaStreamTrack {
    pub fn ready_state(&self) -> MediaStreamTrackState {
        self.inner.get("readyState").as_::<MediaStreamTrackState>()
    }
}
impl MediaStreamTrack {
    pub fn onended(&self) -> Any {
        self.inner.get("onended").as_::<Any>()
    }

    pub fn set_onended(&mut self, value: &Any) {
        self.inner.set("onended", value);
    }
}
impl MediaStreamTrack {
    pub fn clone_(&self) -> MediaStreamTrack {
        self.inner.call("clone", &[]).as_::<MediaStreamTrack>()
    }
}
impl MediaStreamTrack {
    pub fn stop(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
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
    pub fn apply_constraints0(&self) -> Promise {
        self.inner.call("applyConstraints", &[]).as_::<Promise>()
    }

    pub fn apply_constraints1(&self, constraints: &MediaTrackConstraints) -> Promise {
        self.inner
            .call("applyConstraints", &[constraints.into()])
            .as_::<Promise>()
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
    pub fn oncapturehandlechange(&self) -> Any {
        self.inner.get("oncapturehandlechange").as_::<Any>()
    }

    pub fn set_oncapturehandlechange(&mut self, value: &Any) {
        self.inner.set("oncapturehandlechange", value);
    }
}
impl MediaStreamTrack {
    pub fn get_supported_capture_actions(&self) -> Sequence<String> {
        self.inner
            .call("getSupportedCaptureActions", &[])
            .as_::<Sequence<String>>()
    }
}
impl MediaStreamTrack {
    pub fn send_capture_action(&self, action: &CaptureAction) -> Promise {
        self.inner
            .call("sendCaptureAction", &[action.into()])
            .as_::<Promise>()
    }
}
impl MediaStreamTrack {
    pub fn content_hint(&self) -> String {
        self.inner.get("contentHint").as_::<String>()
    }

    pub fn set_content_hint(&mut self, value: &str) {
        self.inner.set("contentHint", value);
    }
}
impl MediaStreamTrack {
    pub fn isolated(&self) -> bool {
        self.inner.get("isolated").as_::<bool>()
    }
}
impl MediaStreamTrack {
    pub fn onisolationchange(&self) -> Any {
        self.inner.get("onisolationchange").as_::<Any>()
    }

    pub fn set_onisolationchange(&mut self, value: &Any) {
        self.inner.set("onisolationchange", value);
    }
}
