use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioOutputOptions {
    inner: emlite::Val,
}
impl FromVal for AudioOutputOptions {
    fn from_val(v: &emlite::Val) -> Self {
        AudioOutputOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioOutputOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioOutputOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioOutputOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioOutputOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioOutputOptions> for emlite::Val {
    fn from(s: AudioOutputOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AudioOutputOptions> for emlite::Val {
    fn from(s: &AudioOutputOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl AudioOutputOptions {
    pub fn device_id(&self) -> DOMString {
        self.inner.get("deviceId").as_::<DOMString>()
    }

    pub fn set_device_id(&mut self, value: DOMString) {
        self.inner.set("deviceId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaptureHandleConfig {
    inner: emlite::Val,
}
impl FromVal for CaptureHandleConfig {
    fn from_val(v: &emlite::Val) -> Self {
        CaptureHandleConfig { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CaptureHandleConfig {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CaptureHandleConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CaptureHandleConfig {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CaptureHandleConfig {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CaptureHandleConfig> for emlite::Val {
    fn from(s: CaptureHandleConfig) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CaptureHandleConfig> for emlite::Val {
    fn from(s: &CaptureHandleConfig) -> emlite::Val {
        s.inner.clone()
    }
}

impl CaptureHandleConfig {
    pub fn expose_origin(&self) -> bool {
        self.inner.get("exposeOrigin").as_::<bool>()
    }

    pub fn set_expose_origin(&mut self, value: bool) {
        self.inner.set("exposeOrigin", value);
    }
}
impl CaptureHandleConfig {
    pub fn handle(&self) -> DOMString {
        self.inner.get("handle").as_::<DOMString>()
    }

    pub fn set_handle(&mut self, value: DOMString) {
        self.inner.set("handle", value);
    }
}
impl CaptureHandleConfig {
    pub fn permitted_origins(&self) -> Sequence<DOMString> {
        self.inner
            .get("permittedOrigins")
            .as_::<Sequence<DOMString>>()
    }

    pub fn set_permitted_origins(&mut self, value: Sequence<DOMString>) {
        self.inner.set("permittedOrigins", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaTrackSupportedConstraints {
    inner: emlite::Val,
}
impl FromVal for MediaTrackSupportedConstraints {
    fn from_val(v: &emlite::Val) -> Self {
        MediaTrackSupportedConstraints { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaTrackSupportedConstraints {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaTrackSupportedConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaTrackSupportedConstraints {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaTrackSupportedConstraints {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaTrackSupportedConstraints> for emlite::Val {
    fn from(s: MediaTrackSupportedConstraints) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaTrackSupportedConstraints> for emlite::Val {
    fn from(s: &MediaTrackSupportedConstraints) -> emlite::Val {
        s.inner.clone()
    }
}

impl MediaTrackSupportedConstraints {
    pub fn display_surface(&self) -> bool {
        self.inner.get("displaySurface").as_::<bool>()
    }

    pub fn set_display_surface(&mut self, value: bool) {
        self.inner.set("displaySurface", value);
    }
}
impl MediaTrackSupportedConstraints {
    pub fn logical_surface(&self) -> bool {
        self.inner.get("logicalSurface").as_::<bool>()
    }

    pub fn set_logical_surface(&mut self, value: bool) {
        self.inner.set("logicalSurface", value);
    }
}
impl MediaTrackSupportedConstraints {
    pub fn cursor(&self) -> bool {
        self.inner.get("cursor").as_::<bool>()
    }

    pub fn set_cursor(&mut self, value: bool) {
        self.inner.set("cursor", value);
    }
}
impl MediaTrackSupportedConstraints {
    pub fn restrict_own_audio(&self) -> bool {
        self.inner.get("restrictOwnAudio").as_::<bool>()
    }

    pub fn set_restrict_own_audio(&mut self, value: bool) {
        self.inner.set("restrictOwnAudio", value);
    }
}
impl MediaTrackSupportedConstraints {
    pub fn suppress_local_audio_playback(&self) -> bool {
        self.inner.get("suppressLocalAudioPlayback").as_::<bool>()
    }

    pub fn set_suppress_local_audio_playback(&mut self, value: bool) {
        self.inner.set("suppressLocalAudioPlayback", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamConstraints {
    inner: emlite::Val,
}
impl FromVal for MediaStreamConstraints {
    fn from_val(v: &emlite::Val) -> Self {
        MediaStreamConstraints { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaStreamConstraints {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaStreamConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaStreamConstraints {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaStreamConstraints {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaStreamConstraints> for emlite::Val {
    fn from(s: MediaStreamConstraints) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaStreamConstraints> for emlite::Val {
    fn from(s: &MediaStreamConstraints) -> emlite::Val {
        s.inner.clone()
    }
}

impl MediaStreamConstraints {
    pub fn peer_identity(&self) -> DOMString {
        self.inner.get("peerIdentity").as_::<DOMString>()
    }

    pub fn set_peer_identity(&mut self, value: DOMString) {
        self.inner.set("peerIdentity", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DisplayMediaStreamOptions {
    inner: emlite::Val,
}
impl FromVal for DisplayMediaStreamOptions {
    fn from_val(v: &emlite::Val) -> Self {
        DisplayMediaStreamOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DisplayMediaStreamOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DisplayMediaStreamOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DisplayMediaStreamOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DisplayMediaStreamOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DisplayMediaStreamOptions> for emlite::Val {
    fn from(s: DisplayMediaStreamOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&DisplayMediaStreamOptions> for emlite::Val {
    fn from(s: &DisplayMediaStreamOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl DisplayMediaStreamOptions {
    pub fn video(&self) -> Any {
        self.inner.get("video").as_::<Any>()
    }

    pub fn set_video(&mut self, value: Any) {
        self.inner.set("video", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn audio(&self) -> Any {
        self.inner.get("audio").as_::<Any>()
    }

    pub fn set_audio(&mut self, value: Any) {
        self.inner.set("audio", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn controller(&self) -> CaptureController {
        self.inner.get("controller").as_::<CaptureController>()
    }

    pub fn set_controller(&mut self, value: CaptureController) {
        self.inner.set("controller", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn self_browser_surface(&self) -> SelfCapturePreferenceEnum {
        self.inner
            .get("selfBrowserSurface")
            .as_::<SelfCapturePreferenceEnum>()
    }

    pub fn set_self_browser_surface(&mut self, value: SelfCapturePreferenceEnum) {
        self.inner.set("selfBrowserSurface", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn system_audio(&self) -> SystemAudioPreferenceEnum {
        self.inner
            .get("systemAudio")
            .as_::<SystemAudioPreferenceEnum>()
    }

    pub fn set_system_audio(&mut self, value: SystemAudioPreferenceEnum) {
        self.inner.set("systemAudio", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn window_audio(&self) -> WindowAudioPreferenceEnum {
        self.inner
            .get("windowAudio")
            .as_::<WindowAudioPreferenceEnum>()
    }

    pub fn set_window_audio(&mut self, value: WindowAudioPreferenceEnum) {
        self.inner.set("windowAudio", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn surface_switching(&self) -> SurfaceSwitchingPreferenceEnum {
        self.inner
            .get("surfaceSwitching")
            .as_::<SurfaceSwitchingPreferenceEnum>()
    }

    pub fn set_surface_switching(&mut self, value: SurfaceSwitchingPreferenceEnum) {
        self.inner.set("surfaceSwitching", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn monitor_type_surfaces(&self) -> MonitorTypeSurfacesEnum {
        self.inner
            .get("monitorTypeSurfaces")
            .as_::<MonitorTypeSurfacesEnum>()
    }

    pub fn set_monitor_type_surfaces(&mut self, value: MonitorTypeSurfacesEnum) {
        self.inner.set("monitorTypeSurfaces", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaDevices {
    inner: EventTarget,
}
impl FromVal for MediaDevices {
    fn from_val(v: &emlite::Val) -> Self {
        MediaDevices {
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
impl core::ops::Deref for MediaDevices {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaDevices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaDevices {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaDevices {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaDevices> for emlite::Val {
    fn from(s: MediaDevices) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaDevices> for emlite::Val {
    fn from(s: &MediaDevices) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaDevices);

impl MediaDevices {
    pub fn ondevicechange(&self) -> Any {
        self.inner.get("ondevicechange").as_::<Any>()
    }

    pub fn set_ondevicechange(&mut self, value: Any) {
        self.inner.set("ondevicechange", value);
    }
}
impl MediaDevices {
    pub fn enumerate_devices(&self) -> Promise {
        self.inner.call("enumerateDevices", &[]).as_::<Promise>()
    }
}
impl MediaDevices {
    pub fn select_audio_output0(&self) -> Promise {
        self.inner.call("selectAudioOutput", &[]).as_::<Promise>()
    }

    pub fn select_audio_output1(&self, options: AudioOutputOptions) -> Promise {
        self.inner
            .call("selectAudioOutput", &[options.into()])
            .as_::<Promise>()
    }
}
impl MediaDevices {
    pub fn set_capture_handle_config0(&self) -> Undefined {
        self.inner
            .call("setCaptureHandleConfig", &[])
            .as_::<Undefined>()
    }

    pub fn set_capture_handle_config1(&self, config: CaptureHandleConfig) -> Undefined {
        self.inner
            .call("setCaptureHandleConfig", &[config.into()])
            .as_::<Undefined>()
    }
}
impl MediaDevices {
    pub fn set_supported_capture_actions(&self, actions: Sequence<DOMString>) -> Undefined {
        self.inner
            .call("setSupportedCaptureActions", &[actions.into()])
            .as_::<Undefined>()
    }
}
impl MediaDevices {
    pub fn oncaptureaction(&self) -> Any {
        self.inner.get("oncaptureaction").as_::<Any>()
    }

    pub fn set_oncaptureaction(&mut self, value: Any) {
        self.inner.set("oncaptureaction", value);
    }
}
impl MediaDevices {
    pub fn get_supported_constraints(&self) -> MediaTrackSupportedConstraints {
        self.inner
            .call("getSupportedConstraints", &[])
            .as_::<MediaTrackSupportedConstraints>()
    }
}
impl MediaDevices {
    pub fn get_user_media0(&self) -> Promise {
        self.inner.call("getUserMedia", &[]).as_::<Promise>()
    }

    pub fn get_user_media1(&self, constraints: MediaStreamConstraints) -> Promise {
        self.inner
            .call("getUserMedia", &[constraints.into()])
            .as_::<Promise>()
    }
}
impl MediaDevices {
    pub fn get_viewport_media0(&self) -> Promise {
        self.inner.call("getViewportMedia", &[]).as_::<Promise>()
    }

    pub fn get_viewport_media1(&self, options: DisplayMediaStreamOptions) -> Promise {
        self.inner
            .call("getViewportMedia", &[options.into()])
            .as_::<Promise>()
    }
}
impl MediaDevices {
    pub fn get_display_media0(&self) -> Promise {
        self.inner.call("getDisplayMedia", &[]).as_::<Promise>()
    }

    pub fn get_display_media1(&self, options: DisplayMediaStreamOptions) -> Promise {
        self.inner
            .call("getDisplayMedia", &[options.into()])
            .as_::<Promise>()
    }
}
