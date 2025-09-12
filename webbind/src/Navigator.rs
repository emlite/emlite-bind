use super::*;

/// The Navigator class.
/// [`Navigator`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Navigator {
    inner: Any,
}

impl FromVal for Navigator {
    fn from_val(v: &Any) -> Self {
        Navigator {
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

impl core::ops::Deref for Navigator {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Navigator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Navigator {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Navigator {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Navigator> for Any {
    fn from(s: Navigator) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Navigator> for Any {
    fn from(s: &Navigator) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Navigator);

impl Navigator {
    /// Getter of the `audioSession` attribute.
    /// [`Navigator.audioSession`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/audioSession)
    pub fn audio_session(&self) -> AudioSession {
        self.inner.get("audioSession").as_::<AudioSession>()
    }
}
impl Navigator {
    /// Getter of the `clipboard` attribute.
    /// [`Navigator.clipboard`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/clipboard)
    pub fn clipboard(&self) -> Clipboard {
        self.inner.get("clipboard").as_::<Clipboard>()
    }
}
impl Navigator {
    /// Getter of the `contacts` attribute.
    /// [`Navigator.contacts`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/contacts)
    pub fn contacts(&self) -> ContactsManager {
        self.inner.get("contacts").as_::<ContactsManager>()
    }
}
impl Navigator {
    /// Getter of the `credentials` attribute.
    /// [`Navigator.credentials`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/credentials)
    pub fn credentials(&self) -> CredentialsContainer {
        self.inner.get("credentials").as_::<CredentialsContainer>()
    }
}
impl Navigator {
    /// Getter of the `devicePosture` attribute.
    /// [`Navigator.devicePosture`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/devicePosture)
    pub fn device_posture(&self) -> DevicePosture {
        self.inner.get("devicePosture").as_::<DevicePosture>()
    }
}
impl Navigator {
    /// Getter of the `geolocation` attribute.
    /// [`Navigator.geolocation`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/geolocation)
    pub fn geolocation(&self) -> Geolocation {
        self.inner.get("geolocation").as_::<Geolocation>()
    }
}
impl Navigator {
    /// Getter of the `userActivation` attribute.
    /// [`Navigator.userActivation`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userActivation)
    pub fn user_activation(&self) -> UserActivation {
        self.inner.get("userActivation").as_::<UserActivation>()
    }
}
impl Navigator {
    /// Getter of the `ink` attribute.
    /// [`Navigator.ink`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/ink)
    pub fn ink(&self) -> Ink {
        self.inner.get("ink").as_::<Ink>()
    }
}
impl Navigator {
    /// Getter of the `scheduling` attribute.
    /// [`Navigator.scheduling`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/scheduling)
    pub fn scheduling(&self) -> Scheduling {
        self.inner.get("scheduling").as_::<Scheduling>()
    }
}
impl Navigator {
    /// Getter of the `keyboard` attribute.
    /// [`Navigator.keyboard`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/keyboard)
    pub fn keyboard(&self) -> Keyboard {
        self.inner.get("keyboard").as_::<Keyboard>()
    }
}
impl Navigator {
    /// Getter of the `login` attribute.
    /// [`Navigator.login`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/login)
    pub fn login(&self) -> NavigatorLogin {
        self.inner.get("login").as_::<NavigatorLogin>()
    }
}
impl Navigator {
    /// Getter of the `managed` attribute.
    /// [`Navigator.managed`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/managed)
    pub fn managed(&self) -> NavigatorManagedData {
        self.inner.get("managed").as_::<NavigatorManagedData>()
    }
}
impl Navigator {
    /// Getter of the `mediaCapabilities` attribute.
    /// [`Navigator.mediaCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaCapabilities)
    pub fn media_capabilities(&self) -> MediaCapabilities {
        self.inner
            .get("mediaCapabilities")
            .as_::<MediaCapabilities>()
    }
}
impl Navigator {
    /// Getter of the `mediaDevices` attribute.
    /// [`Navigator.mediaDevices`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaDevices)
    pub fn media_devices(&self) -> MediaDevices {
        self.inner.get("mediaDevices").as_::<MediaDevices>()
    }
}
impl Navigator {
    /// Getter of the `preferences` attribute.
    /// [`Navigator.preferences`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/preferences)
    pub fn preferences(&self) -> PreferenceManager {
        self.inner.get("preferences").as_::<PreferenceManager>()
    }
}
impl Navigator {
    /// Getter of the `mediaSession` attribute.
    /// [`Navigator.mediaSession`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaSession)
    pub fn media_session(&self) -> MediaSession {
        self.inner.get("mediaSession").as_::<MediaSession>()
    }
}
impl Navigator {
    /// Getter of the `permissions` attribute.
    /// [`Navigator.permissions`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/permissions)
    pub fn permissions(&self) -> Permissions {
        self.inner.get("permissions").as_::<Permissions>()
    }
}
impl Navigator {
    /// Getter of the `maxTouchPoints` attribute.
    /// [`Navigator.maxTouchPoints`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/maxTouchPoints)
    pub fn max_touch_points(&self) -> i32 {
        self.inner.get("maxTouchPoints").as_::<i32>()
    }
}
impl Navigator {
    /// Getter of the `presentation` attribute.
    /// [`Navigator.presentation`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/presentation)
    pub fn presentation(&self) -> Presentation {
        self.inner.get("presentation").as_::<Presentation>()
    }
}
impl Navigator {
    /// Getter of the `attribution` attribute.
    /// [`Navigator.attribution`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/attribution)
    pub fn attribution(&self) -> Attribution {
        self.inner.get("attribution").as_::<Attribution>()
    }
}
impl Navigator {
    /// Getter of the `wakeLock` attribute.
    /// [`Navigator.wakeLock`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/wakeLock)
    pub fn wake_lock(&self) -> WakeLock {
        self.inner.get("wakeLock").as_::<WakeLock>()
    }
}
impl Navigator {
    /// Getter of the `serial` attribute.
    /// [`Navigator.serial`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serial)
    pub fn serial(&self) -> Serial {
        self.inner.get("serial").as_::<Serial>()
    }
}
impl Navigator {
    /// Getter of the `serviceWorker` attribute.
    /// [`Navigator.serviceWorker`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serviceWorker)
    pub fn service_worker(&self) -> ServiceWorkerContainer {
        self.inner
            .get("serviceWorker")
            .as_::<ServiceWorkerContainer>()
    }
}
impl Navigator {
    /// Getter of the `deprecatedRunAdAuctionEnforcesKAnonymity` attribute.
    /// [`Navigator.deprecatedRunAdAuctionEnforcesKAnonymity`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deprecatedRunAdAuctionEnforcesKAnonymity)
    pub fn deprecated_run_ad_auction_enforces_k_anonymity(&self) -> bool {
        self.inner
            .get("deprecatedRunAdAuctionEnforcesKAnonymity")
            .as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `protectedAudience` attribute.
    /// [`Navigator.protectedAudience`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/protectedAudience)
    pub fn protected_audience(&self) -> ProtectedAudience {
        self.inner
            .get("protectedAudience")
            .as_::<ProtectedAudience>()
    }
}
impl Navigator {
    /// Getter of the `virtualKeyboard` attribute.
    /// [`Navigator.virtualKeyboard`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/virtualKeyboard)
    pub fn virtual_keyboard(&self) -> VirtualKeyboard {
        self.inner.get("virtualKeyboard").as_::<VirtualKeyboard>()
    }
}
impl Navigator {
    /// Getter of the `bluetooth` attribute.
    /// [`Navigator.bluetooth`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/bluetooth)
    pub fn bluetooth(&self) -> Bluetooth {
        self.inner.get("bluetooth").as_::<Bluetooth>()
    }
}
impl Navigator {
    /// Getter of the `hid` attribute.
    /// [`Navigator.hid`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hid)
    pub fn hid(&self) -> HID {
        self.inner.get("hid").as_::<HID>()
    }
}
impl Navigator {
    /// Getter of the `usb` attribute.
    /// [`Navigator.usb`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/usb)
    pub fn usb(&self) -> USB {
        self.inner.get("usb").as_::<USB>()
    }
}
impl Navigator {
    /// Getter of the `xr` attribute.
    /// [`Navigator.xr`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/xr)
    pub fn xr(&self) -> XRSystem {
        self.inner.get("xr").as_::<XRSystem>()
    }
}
impl Navigator {
    /// Getter of the `windowControlsOverlay` attribute.
    /// [`Navigator.windowControlsOverlay`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/windowControlsOverlay)
    pub fn window_controls_overlay(&self) -> WindowControlsOverlay {
        self.inner
            .get("windowControlsOverlay")
            .as_::<WindowControlsOverlay>()
    }
}
impl Navigator {
    /// Getter of the `deviceMemory` attribute.
    /// [`Navigator.deviceMemory`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deviceMemory)
    pub fn device_memory(&self) -> f64 {
        self.inner.get("deviceMemory").as_::<f64>()
    }
}
impl Navigator {
    /// Getter of the `globalPrivacyControl` attribute.
    /// [`Navigator.globalPrivacyControl`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/globalPrivacyControl)
    pub fn global_privacy_control(&self) -> bool {
        self.inner.get("globalPrivacyControl").as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `oscpu` attribute.
    /// [`Navigator.oscpu`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/oscpu)
    pub fn oscpu(&self) -> JsString {
        self.inner.get("oscpu").as_::<JsString>()
    }
}
impl Navigator {
    /// Getter of the `language` attribute.
    /// [`Navigator.language`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language)
    pub fn language(&self) -> JsString {
        self.inner.get("language").as_::<JsString>()
    }
}
impl Navigator {
    /// Getter of the `languages` attribute.
    /// [`Navigator.languages`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages)
    pub fn languages(&self) -> TypedArray<JsString> {
        self.inner.get("languages").as_::<TypedArray<JsString>>()
    }
}
impl Navigator {
    /// Getter of the `onLine` attribute.
    /// [`Navigator.onLine`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/onLine)
    pub fn on_line(&self) -> bool {
        self.inner.get("onLine").as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `cookieEnabled` attribute.
    /// [`Navigator.cookieEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/cookieEnabled)
    pub fn cookie_enabled(&self) -> bool {
        self.inner.get("cookieEnabled").as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `plugins` attribute.
    /// [`Navigator.plugins`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/plugins)
    pub fn plugins(&self) -> PluginArray {
        self.inner.get("plugins").as_::<PluginArray>()
    }
}
impl Navigator {
    /// Getter of the `mimeTypes` attribute.
    /// [`Navigator.mimeTypes`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mimeTypes)
    pub fn mime_types(&self) -> MimeTypeArray {
        self.inner.get("mimeTypes").as_::<MimeTypeArray>()
    }
}
impl Navigator {
    /// Getter of the `pdfViewerEnabled` attribute.
    /// [`Navigator.pdfViewerEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/pdfViewerEnabled)
    pub fn pdf_viewer_enabled(&self) -> bool {
        self.inner.get("pdfViewerEnabled").as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `hardwareConcurrency` attribute.
    /// [`Navigator.hardwareConcurrency`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency)
    pub fn hardware_concurrency(&self) -> u64 {
        self.inner.get("hardwareConcurrency").as_::<u64>()
    }
}
impl Navigator {
    /// Getter of the `connection` attribute.
    /// [`Navigator.connection`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/connection)
    pub fn connection(&self) -> NetworkInformation {
        self.inner.get("connection").as_::<NetworkInformation>()
    }
}
impl Navigator {
    /// Getter of the `storageBuckets` attribute.
    /// [`Navigator.storageBuckets`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/storageBuckets)
    pub fn storage_buckets(&self) -> StorageBucketManager {
        self.inner
            .get("storageBuckets")
            .as_::<StorageBucketManager>()
    }
}
impl Navigator {
    /// Getter of the `storage` attribute.
    /// [`Navigator.storage`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/storage)
    pub fn storage(&self) -> StorageManager {
        self.inner.get("storage").as_::<StorageManager>()
    }
}
impl Navigator {
    /// Getter of the `userAgentData` attribute.
    /// [`Navigator.userAgentData`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgentData)
    pub fn user_agent_data(&self) -> NavigatorUAData {
        self.inner.get("userAgentData").as_::<NavigatorUAData>()
    }
}
impl Navigator {
    /// Getter of the `locks` attribute.
    /// [`Navigator.locks`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/locks)
    pub fn locks(&self) -> LockManager {
        self.inner.get("locks").as_::<LockManager>()
    }
}
impl Navigator {
    /// Getter of the `webdriver` attribute.
    /// [`Navigator.webdriver`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/webdriver)
    pub fn webdriver(&self) -> bool {
        self.inner.get("webdriver").as_::<bool>()
    }
}
impl Navigator {
    /// Getter of the `gpu` attribute.
    /// [`Navigator.gpu`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/gpu)
    pub fn gpu(&self) -> GPU {
        self.inner.get("gpu").as_::<GPU>()
    }
}
impl Navigator {
    /// Getter of the `ml` attribute.
    /// [`Navigator.ml`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/ml)
    pub fn ml(&self) -> ML {
        self.inner.get("ml").as_::<ML>()
    }
}
impl Navigator {
    /// The getAutoplayPolicy method.
    /// [`Navigator.getAutoplayPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getAutoplayPolicy)
    pub fn get_autoplay_policy(&self, type_: &AutoplayPolicyMediaType) -> AutoplayPolicy {
        self.inner
            .call("getAutoplayPolicy", &[type_.into()])
            .as_::<AutoplayPolicy>()
    }
}
impl Navigator {
    /// The getAutoplayPolicy method.
    /// [`Navigator.getAutoplayPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getAutoplayPolicy)
    pub fn get_autoplay_policy1(&self, element: &HTMLMediaElement) -> AutoplayPolicy {
        self.inner
            .call("getAutoplayPolicy", &[element.into()])
            .as_::<AutoplayPolicy>()
    }
}
impl Navigator {
    /// The getAutoplayPolicy method.
    /// [`Navigator.getAutoplayPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getAutoplayPolicy)
    pub fn get_autoplay_policy2(&self, context: &AudioContext) -> AutoplayPolicy {
        self.inner
            .call("getAutoplayPolicy", &[context.into()])
            .as_::<AutoplayPolicy>()
    }
}
impl Navigator {
    /// The getBattery method.
    /// [`Navigator.getBattery`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getBattery)
    pub fn get_battery(&self) -> Promise<BatteryManager> {
        self.inner
            .call("getBattery", &[])
            .as_::<Promise<BatteryManager>>()
    }
}
impl Navigator {
    /// The sendBeacon method.
    /// [`Navigator.sendBeacon`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    pub fn send_beacon0(&self, url: &JsString) -> bool {
        self.inner.call("sendBeacon", &[url.into()]).as_::<bool>()
    }
    /// The sendBeacon method.
    /// [`Navigator.sendBeacon`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)
    pub fn send_beacon1(&self, url: &JsString, data: &Any) -> bool {
        self.inner
            .call("sendBeacon", &[url.into(), data.into()])
            .as_::<bool>()
    }
}
impl Navigator {
    /// The requestMediaKeySystemAccess method.
    /// [`Navigator.requestMediaKeySystemAccess`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMediaKeySystemAccess)
    pub fn request_media_key_system_access(
        &self,
        key_system: &JsString,
        supported_configurations: &TypedArray<MediaKeySystemConfiguration>,
    ) -> Promise<MediaKeySystemAccess> {
        self.inner
            .call(
                "requestMediaKeySystemAccess",
                &[key_system.into(), supported_configurations.into()],
            )
            .as_::<Promise<MediaKeySystemAccess>>()
    }
}
impl Navigator {
    /// The deprecatedReplaceInURN method.
    /// [`Navigator.deprecatedReplaceInURN`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deprecatedReplaceInURN)
    pub fn deprecated_replace_in_urn(
        &self,
        urn_or_config: &Any,
        replacements: &Record<JsString, JsString>,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "deprecatedReplaceInURN",
                &[urn_or_config.into(), replacements.into()],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl Navigator {
    /// The deprecatedURNtoURL method.
    /// [`Navigator.deprecatedURNtoURL`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deprecatedURNtoURL)
    pub fn deprecated_ur_nto_url0(&self, urn_or_config: &Any) -> Promise<JsString> {
        self.inner
            .call("deprecatedURNtoURL", &[urn_or_config.into()])
            .as_::<Promise<JsString>>()
    }
    /// The deprecatedURNtoURL method.
    /// [`Navigator.deprecatedURNtoURL`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deprecatedURNtoURL)
    pub fn deprecated_ur_nto_url1(
        &self,
        urn_or_config: &Any,
        send_reports: bool,
    ) -> Promise<JsString> {
        self.inner
            .call(
                "deprecatedURNtoURL",
                &[urn_or_config.into(), send_reports.into()],
            )
            .as_::<Promise<JsString>>()
    }
}
impl Navigator {
    /// The adAuctionComponents method.
    /// [`Navigator.adAuctionComponents`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/adAuctionComponents)
    pub fn ad_auction_components(&self, num_ad_components: u16) -> TypedArray<JsString> {
        self.inner
            .call("adAuctionComponents", &[num_ad_components.into()])
            .as_::<TypedArray<JsString>>()
    }
}
impl Navigator {
    /// The getGamepads method.
    /// [`Navigator.getGamepads`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getGamepads)
    pub fn get_gamepads(&self) -> TypedArray<Gamepad> {
        self.inner
            .call("getGamepads", &[])
            .as_::<TypedArray<Gamepad>>()
    }
}
impl Navigator {
    /// The getInstalledRelatedApps method.
    /// [`Navigator.getInstalledRelatedApps`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getInstalledRelatedApps)
    pub fn get_installed_related_apps(&self) -> Promise<TypedArray<RelatedApplication>> {
        self.inner
            .call("getInstalledRelatedApps", &[])
            .as_::<Promise<TypedArray<RelatedApplication>>>()
    }
}
impl Navigator {
    /// The queryHandwritingRecognizer method.
    /// [`Navigator.queryHandwritingRecognizer`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/queryHandwritingRecognizer)
    pub fn query_handwriting_recognizer(
        &self,
        constraint: &HandwritingModelConstraint,
    ) -> Promise<HandwritingRecognizerQueryResult> {
        self.inner
            .call("queryHandwritingRecognizer", &[constraint.into()])
            .as_::<Promise<HandwritingRecognizerQueryResult>>()
    }
}
impl Navigator {
    /// The createHandwritingRecognizer method.
    /// [`Navigator.createHandwritingRecognizer`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/createHandwritingRecognizer)
    pub fn create_handwriting_recognizer(
        &self,
        constraint: &HandwritingModelConstraint,
    ) -> Promise<HandwritingRecognizer> {
        self.inner
            .call("createHandwritingRecognizer", &[constraint.into()])
            .as_::<Promise<HandwritingRecognizer>>()
    }
}
impl Navigator {
    /// The joinAdInterestGroup method.
    /// [`Navigator.joinAdInterestGroup`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/joinAdInterestGroup)
    pub fn join_ad_interest_group(&self, group: &AuctionAdInterestGroup) -> Promise<Undefined> {
        self.inner
            .call("joinAdInterestGroup", &[group.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl Navigator {
    /// The leaveAdInterestGroup method.
    /// [`Navigator.leaveAdInterestGroup`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/leaveAdInterestGroup)
    pub fn leave_ad_interest_group0(&self) -> Promise<Undefined> {
        self.inner
            .call("leaveAdInterestGroup", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The leaveAdInterestGroup method.
    /// [`Navigator.leaveAdInterestGroup`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/leaveAdInterestGroup)
    pub fn leave_ad_interest_group1(
        &self,
        group: &AuctionAdInterestGroupKey,
    ) -> Promise<Undefined> {
        self.inner
            .call("leaveAdInterestGroup", &[group.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl Navigator {
    /// The clearOriginJoinedAdInterestGroups method.
    /// [`Navigator.clearOriginJoinedAdInterestGroups`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/clearOriginJoinedAdInterestGroups)
    pub fn clear_origin_joined_ad_interest_groups0(&self, owner: &JsString) -> Promise<Undefined> {
        self.inner
            .call("clearOriginJoinedAdInterestGroups", &[owner.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The clearOriginJoinedAdInterestGroups method.
    /// [`Navigator.clearOriginJoinedAdInterestGroups`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/clearOriginJoinedAdInterestGroups)
    pub fn clear_origin_joined_ad_interest_groups1(
        &self,
        owner: &JsString,
        interest_groups_to_keep: &TypedArray<JsString>,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "clearOriginJoinedAdInterestGroups",
                &[owner.into(), interest_groups_to_keep.into()],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl Navigator {
    /// The runAdAuction method.
    /// [`Navigator.runAdAuction`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/runAdAuction)
    pub fn run_ad_auction(&self, config: &AuctionAdConfig) -> Promise<Any> {
        self.inner
            .call("runAdAuction", &[config.into()])
            .as_::<Promise<Any>>()
    }
}
impl Navigator {
    /// The canLoadAdAuctionFencedFrame method.
    /// [`Navigator.canLoadAdAuctionFencedFrame`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/canLoadAdAuctionFencedFrame)
    pub fn can_load_ad_auction_fenced_frame(&self) -> bool {
        self.inner
            .call("canLoadAdAuctionFencedFrame", &[])
            .as_::<bool>()
    }
}
impl Navigator {
    /// The getInterestGroupAdAuctionData method.
    /// [`Navigator.getInterestGroupAdAuctionData`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getInterestGroupAdAuctionData)
    pub fn get_interest_group_ad_auction_data0(&self) -> Promise<AdAuctionData> {
        self.inner
            .call("getInterestGroupAdAuctionData", &[])
            .as_::<Promise<AdAuctionData>>()
    }
    /// The getInterestGroupAdAuctionData method.
    /// [`Navigator.getInterestGroupAdAuctionData`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getInterestGroupAdAuctionData)
    pub fn get_interest_group_ad_auction_data1(
        &self,
        config: &AdAuctionDataConfig,
    ) -> Promise<AdAuctionData> {
        self.inner
            .call("getInterestGroupAdAuctionData", &[config.into()])
            .as_::<Promise<AdAuctionData>>()
    }
}
impl Navigator {
    /// The createAuctionNonce method.
    /// [`Navigator.createAuctionNonce`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/createAuctionNonce)
    pub fn create_auction_nonce(&self) -> Promise<JsString> {
        self.inner
            .call("createAuctionNonce", &[])
            .as_::<Promise<JsString>>()
    }
}
impl Navigator {
    /// The updateAdInterestGroups method.
    /// [`Navigator.updateAdInterestGroups`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/updateAdInterestGroups)
    pub fn update_ad_interest_groups(&self) -> Undefined {
        self.inner
            .call("updateAdInterestGroups", &[])
            .as_::<Undefined>()
    }
}
impl Navigator {
    /// The vibrate method.
    /// [`Navigator.vibrate`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)
    pub fn vibrate(&self, pattern: &Any) -> bool {
        self.inner.call("vibrate", &[pattern.into()]).as_::<bool>()
    }
}
impl Navigator {
    /// The share method.
    /// [`Navigator.share`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share)
    pub fn share0(&self) -> Promise<Undefined> {
        self.inner.call("share", &[]).as_::<Promise<Undefined>>()
    }
    /// The share method.
    /// [`Navigator.share`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share)
    pub fn share1(&self, data: &ShareData) -> Promise<Undefined> {
        self.inner
            .call("share", &[data.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl Navigator {
    /// The canShare method.
    /// [`Navigator.canShare`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/canShare)
    pub fn can_share0(&self) -> bool {
        self.inner.call("canShare", &[]).as_::<bool>()
    }
    /// The canShare method.
    /// [`Navigator.canShare`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/canShare)
    pub fn can_share1(&self, data: &ShareData) -> bool {
        self.inner.call("canShare", &[data.into()]).as_::<bool>()
    }
}
impl Navigator {
    /// The requestMIDIAccess method.
    /// [`Navigator.requestMIDIAccess`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)
    pub fn request_midi_access0(&self) -> Promise<MIDIAccess> {
        self.inner
            .call("requestMIDIAccess", &[])
            .as_::<Promise<MIDIAccess>>()
    }
    /// The requestMIDIAccess method.
    /// [`Navigator.requestMIDIAccess`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)
    pub fn request_midi_access1(&self, options: &MIDIOptions) -> Promise<MIDIAccess> {
        self.inner
            .call("requestMIDIAccess", &[options.into()])
            .as_::<Promise<MIDIAccess>>()
    }
}
impl Navigator {
    /// The setAppBadge method.
    /// [`Navigator.setAppBadge`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/setAppBadge)
    pub fn set_app_badge0(&self) -> Promise<Undefined> {
        self.inner
            .call("setAppBadge", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The setAppBadge method.
    /// [`Navigator.setAppBadge`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/setAppBadge)
    pub fn set_app_badge1(&self, contents: u64) -> Promise<Undefined> {
        self.inner
            .call("setAppBadge", &[contents.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl Navigator {
    /// The clearAppBadge method.
    /// [`Navigator.clearAppBadge`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/clearAppBadge)
    pub fn clear_app_badge(&self) -> Promise<Undefined> {
        self.inner
            .call("clearAppBadge", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl Navigator {
    /// The taintEnabled method.
    /// [`Navigator.taintEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/taintEnabled)
    pub fn taint_enabled(&self) -> bool {
        self.inner.call("taintEnabled", &[]).as_::<bool>()
    }
}
impl Navigator {
    /// The registerProtocolHandler method.
    /// [`Navigator.registerProtocolHandler`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerProtocolHandler)
    pub fn register_protocol_handler(&self, scheme: &JsString, url: &JsString) -> Undefined {
        self.inner
            .call("registerProtocolHandler", &[scheme.into(), url.into()])
            .as_::<Undefined>()
    }
}
impl Navigator {
    /// The unregisterProtocolHandler method.
    /// [`Navigator.unregisterProtocolHandler`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/unregisterProtocolHandler)
    pub fn unregister_protocol_handler(&self, scheme: &JsString, url: &JsString) -> Undefined {
        self.inner
            .call("unregisterProtocolHandler", &[scheme.into(), url.into()])
            .as_::<Undefined>()
    }
}
impl Navigator {
    /// The javaEnabled method.
    /// [`Navigator.javaEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/javaEnabled)
    pub fn java_enabled(&self) -> bool {
        self.inner.call("javaEnabled", &[]).as_::<bool>()
    }
}
