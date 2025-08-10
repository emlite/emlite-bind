use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCOfferOptions {
    inner: Any,
}
impl FromVal for RTCOfferOptions {
    fn from_val(v: &Any) -> Self {
        RTCOfferOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCOfferOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCOfferOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCOfferOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCOfferOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCOfferOptions> for Any {
    fn from(s: RTCOfferOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCOfferOptions> for Any {
    fn from(s: &RTCOfferOptions) -> Any {
        s.inner.clone()
    }
}

impl RTCOfferOptions {
    pub fn offer_to_receive_audio(&self) -> bool {
        self.inner.get("offerToReceiveAudio").as_::<bool>()
    }

    pub fn set_offer_to_receive_audio(&mut self, value: bool) {
        self.inner.set("offerToReceiveAudio", value);
    }
}
impl RTCOfferOptions {
    pub fn offer_to_receive_video(&self) -> bool {
        self.inner.get("offerToReceiveVideo").as_::<bool>()
    }

    pub fn set_offer_to_receive_video(&mut self, value: bool) {
        self.inner.set("offerToReceiveVideo", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCLocalSessionDescriptionInit {
    inner: Any,
}
impl FromVal for RTCLocalSessionDescriptionInit {
    fn from_val(v: &Any) -> Self {
        RTCLocalSessionDescriptionInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCLocalSessionDescriptionInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCLocalSessionDescriptionInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCLocalSessionDescriptionInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCLocalSessionDescriptionInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCLocalSessionDescriptionInit> for Any {
    fn from(s: RTCLocalSessionDescriptionInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCLocalSessionDescriptionInit> for Any {
    fn from(s: &RTCLocalSessionDescriptionInit) -> Any {
        s.inner.clone()
    }
}

impl RTCLocalSessionDescriptionInit {
    pub fn type_(&self) -> RTCSdpType {
        self.inner.get("type").as_::<RTCSdpType>()
    }

    pub fn set_type_(&mut self, value: &RTCSdpType) {
        self.inner.set("type", value);
    }
}
impl RTCLocalSessionDescriptionInit {
    pub fn sdp(&self) -> JsString {
        self.inner.get("sdp").as_::<JsString>()
    }

    pub fn set_sdp(&mut self, value: &JsString) {
        self.inner.set("sdp", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCSessionDescriptionInit {
    inner: Any,
}
impl FromVal for RTCSessionDescriptionInit {
    fn from_val(v: &Any) -> Self {
        RTCSessionDescriptionInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCSessionDescriptionInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCSessionDescriptionInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCSessionDescriptionInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCSessionDescriptionInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCSessionDescriptionInit> for Any {
    fn from(s: RTCSessionDescriptionInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCSessionDescriptionInit> for Any {
    fn from(s: &RTCSessionDescriptionInit) -> Any {
        s.inner.clone()
    }
}

impl RTCSessionDescriptionInit {
    pub fn type_(&self) -> RTCSdpType {
        self.inner.get("type").as_::<RTCSdpType>()
    }

    pub fn set_type_(&mut self, value: &RTCSdpType) {
        self.inner.set("type", value);
    }
}
impl RTCSessionDescriptionInit {
    pub fn sdp(&self) -> JsString {
        self.inner.get("sdp").as_::<JsString>()
    }

    pub fn set_sdp(&mut self, value: &JsString) {
        self.inner.set("sdp", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCConfiguration {
    inner: Any,
}
impl FromVal for RTCConfiguration {
    fn from_val(v: &Any) -> Self {
        RTCConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCConfiguration> for Any {
    fn from(s: RTCConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCConfiguration> for Any {
    fn from(s: &RTCConfiguration) -> Any {
        s.inner.clone()
    }
}

impl RTCConfiguration {
    pub fn ice_servers(&self) -> TypedArray<RTCIceServer> {
        self.inner
            .get("iceServers")
            .as_::<TypedArray<RTCIceServer>>()
    }

    pub fn set_ice_servers(&mut self, value: &TypedArray<RTCIceServer>) {
        self.inner.set("iceServers", value);
    }
}
impl RTCConfiguration {
    pub fn ice_transport_policy(&self) -> RTCIceTransportPolicy {
        self.inner
            .get("iceTransportPolicy")
            .as_::<RTCIceTransportPolicy>()
    }

    pub fn set_ice_transport_policy(&mut self, value: &RTCIceTransportPolicy) {
        self.inner.set("iceTransportPolicy", value);
    }
}
impl RTCConfiguration {
    pub fn bundle_policy(&self) -> RTCBundlePolicy {
        self.inner.get("bundlePolicy").as_::<RTCBundlePolicy>()
    }

    pub fn set_bundle_policy(&mut self, value: &RTCBundlePolicy) {
        self.inner.set("bundlePolicy", value);
    }
}
impl RTCConfiguration {
    pub fn rtcp_mux_policy(&self) -> RTCRtcpMuxPolicy {
        self.inner.get("rtcpMuxPolicy").as_::<RTCRtcpMuxPolicy>()
    }

    pub fn set_rtcp_mux_policy(&mut self, value: &RTCRtcpMuxPolicy) {
        self.inner.set("rtcpMuxPolicy", value);
    }
}
impl RTCConfiguration {
    pub fn certificates(&self) -> TypedArray<RTCCertificate> {
        self.inner
            .get("certificates")
            .as_::<TypedArray<RTCCertificate>>()
    }

    pub fn set_certificates(&mut self, value: &TypedArray<RTCCertificate>) {
        self.inner.set("certificates", value);
    }
}
impl RTCConfiguration {
    pub fn ice_candidate_pool_size(&self) -> u8 {
        self.inner.get("iceCandidatePoolSize").as_::<u8>()
    }

    pub fn set_ice_candidate_pool_size(&mut self, value: u8) {
        self.inner.set("iceCandidatePoolSize", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityProviderOptions {
    inner: Any,
}
impl FromVal for RTCIdentityProviderOptions {
    fn from_val(v: &Any) -> Self {
        RTCIdentityProviderOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIdentityProviderOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIdentityProviderOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCIdentityProviderOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCIdentityProviderOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCIdentityProviderOptions> for Any {
    fn from(s: RTCIdentityProviderOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCIdentityProviderOptions> for Any {
    fn from(s: &RTCIdentityProviderOptions) -> Any {
        s.inner.clone()
    }
}

impl RTCIdentityProviderOptions {
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl RTCIdentityProviderOptions {
    pub fn username_hint(&self) -> JsString {
        self.inner.get("usernameHint").as_::<JsString>()
    }

    pub fn set_username_hint(&mut self, value: &JsString) {
        self.inner.set("usernameHint", value);
    }
}
impl RTCIdentityProviderOptions {
    pub fn peer_identity(&self) -> JsString {
        self.inner.get("peerIdentity").as_::<JsString>()
    }

    pub fn set_peer_identity(&mut self, value: &JsString) {
        self.inner.set("peerIdentity", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpTransceiverInit {
    inner: Any,
}
impl FromVal for RTCRtpTransceiverInit {
    fn from_val(v: &Any) -> Self {
        RTCRtpTransceiverInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpTransceiverInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpTransceiverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtpTransceiverInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtpTransceiverInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtpTransceiverInit> for Any {
    fn from(s: RTCRtpTransceiverInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtpTransceiverInit> for Any {
    fn from(s: &RTCRtpTransceiverInit) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpTransceiverInit {
    pub fn direction(&self) -> RTCRtpTransceiverDirection {
        self.inner
            .get("direction")
            .as_::<RTCRtpTransceiverDirection>()
    }

    pub fn set_direction(&mut self, value: &RTCRtpTransceiverDirection) {
        self.inner.set("direction", value);
    }
}
impl RTCRtpTransceiverInit {
    pub fn streams(&self) -> TypedArray<MediaStream> {
        self.inner.get("streams").as_::<TypedArray<MediaStream>>()
    }

    pub fn set_streams(&mut self, value: &TypedArray<MediaStream>) {
        self.inner.set("streams", value);
    }
}
impl RTCRtpTransceiverInit {
    pub fn send_encodings(&self) -> TypedArray<RTCRtpEncodingParameters> {
        self.inner
            .get("sendEncodings")
            .as_::<TypedArray<RTCRtpEncodingParameters>>()
    }

    pub fn set_send_encodings(&mut self, value: &TypedArray<RTCRtpEncodingParameters>) {
        self.inner.set("sendEncodings", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDataChannelInit {
    inner: Any,
}
impl FromVal for RTCDataChannelInit {
    fn from_val(v: &Any) -> Self {
        RTCDataChannelInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCDataChannelInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDataChannelInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCDataChannelInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCDataChannelInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCDataChannelInit> for Any {
    fn from(s: RTCDataChannelInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCDataChannelInit> for Any {
    fn from(s: &RTCDataChannelInit) -> Any {
        s.inner.clone()
    }
}

impl RTCDataChannelInit {
    pub fn ordered(&self) -> bool {
        self.inner.get("ordered").as_::<bool>()
    }

    pub fn set_ordered(&mut self, value: bool) {
        self.inner.set("ordered", value);
    }
}
impl RTCDataChannelInit {
    pub fn max_packet_life_time(&self) -> u16 {
        self.inner.get("maxPacketLifeTime").as_::<u16>()
    }

    pub fn set_max_packet_life_time(&mut self, value: u16) {
        self.inner.set("maxPacketLifeTime", value);
    }
}
impl RTCDataChannelInit {
    pub fn max_retransmits(&self) -> u16 {
        self.inner.get("maxRetransmits").as_::<u16>()
    }

    pub fn set_max_retransmits(&mut self, value: u16) {
        self.inner.set("maxRetransmits", value);
    }
}
impl RTCDataChannelInit {
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl RTCDataChannelInit {
    pub fn negotiated(&self) -> bool {
        self.inner.get("negotiated").as_::<bool>()
    }

    pub fn set_negotiated(&mut self, value: bool) {
        self.inner.set("negotiated", value);
    }
}
impl RTCDataChannelInit {
    pub fn id(&self) -> u16 {
        self.inner.get("id").as_::<u16>()
    }

    pub fn set_id(&mut self, value: u16) {
        self.inner.set("id", value);
    }
}
/// The RTCPeerConnection class.
/// [`RTCPeerConnection`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCPeerConnection {
    inner: EventTarget,
}
impl FromVal for RTCPeerConnection {
    fn from_val(v: &Any) -> Self {
        RTCPeerConnection {
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
impl core::ops::Deref for RTCPeerConnection {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCPeerConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCPeerConnection {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCPeerConnection {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCPeerConnection> for Any {
    fn from(s: RTCPeerConnection) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCPeerConnection> for Any {
    fn from(s: &RTCPeerConnection) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCPeerConnection);

impl RTCPeerConnection {
    /// The `new RTCPeerConnection(..)` constructor, creating a new RTCPeerConnection instance
    pub fn new0() -> RTCPeerConnection {
        Self {
            inner: Any::global("RTCPeerConnection")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }

    /// The `new RTCPeerConnection(..)` constructor, creating a new RTCPeerConnection instance
    pub fn new1(configuration: &RTCConfiguration) -> RTCPeerConnection {
        Self {
            inner: Any::global("RTCPeerConnection")
                .new(&[configuration.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl RTCPeerConnection {
    /// The createOffer method.
    /// [`RTCPeerConnection.createOffer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)
    pub fn create_offer0(
        &self,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "createOffer",
                &[success_callback.into(), failure_callback.into()],
            )
            .as_::<Promise<Undefined>>()
    }
    /// The createOffer method.
    /// [`RTCPeerConnection.createOffer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)
    pub fn create_offer1(
        &self,
        success_callback: &Function,
        failure_callback: &Function,
        options: &RTCOfferOptions,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "createOffer",
                &[
                    success_callback.into(),
                    failure_callback.into(),
                    options.into(),
                ],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The createAnswer method.
    /// [`RTCPeerConnection.createAnswer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)
    pub fn create_answer(
        &self,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "createAnswer",
                &[success_callback.into(), failure_callback.into()],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The setLocalDescription method.
    /// [`RTCPeerConnection.setLocalDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)
    pub fn set_local_description(
        &self,
        description: &RTCLocalSessionDescriptionInit,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "setLocalDescription",
                &[
                    description.into(),
                    success_callback.into(),
                    failure_callback.into(),
                ],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `localDescription` attribute.
    /// [`RTCPeerConnection.localDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/localDescription)
    pub fn local_description(&self) -> RTCSessionDescription {
        self.inner
            .get("localDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `currentLocalDescription` attribute.
    /// [`RTCPeerConnection.currentLocalDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentLocalDescription)
    pub fn current_local_description(&self) -> RTCSessionDescription {
        self.inner
            .get("currentLocalDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `pendingLocalDescription` attribute.
    /// [`RTCPeerConnection.pendingLocalDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingLocalDescription)
    pub fn pending_local_description(&self) -> RTCSessionDescription {
        self.inner
            .get("pendingLocalDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// The setRemoteDescription method.
    /// [`RTCPeerConnection.setRemoteDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)
    pub fn set_remote_description(
        &self,
        description: &RTCSessionDescriptionInit,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "setRemoteDescription",
                &[
                    description.into(),
                    success_callback.into(),
                    failure_callback.into(),
                ],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `remoteDescription` attribute.
    /// [`RTCPeerConnection.remoteDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/remoteDescription)
    pub fn remote_description(&self) -> RTCSessionDescription {
        self.inner
            .get("remoteDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `currentRemoteDescription` attribute.
    /// [`RTCPeerConnection.currentRemoteDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentRemoteDescription)
    pub fn current_remote_description(&self) -> RTCSessionDescription {
        self.inner
            .get("currentRemoteDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `pendingRemoteDescription` attribute.
    /// [`RTCPeerConnection.pendingRemoteDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingRemoteDescription)
    pub fn pending_remote_description(&self) -> RTCSessionDescription {
        self.inner
            .get("pendingRemoteDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// The addIceCandidate method.
    /// [`RTCPeerConnection.addIceCandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)
    pub fn add_ice_candidate(
        &self,
        candidate: &RTCIceCandidateInit,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "addIceCandidate",
                &[
                    candidate.into(),
                    success_callback.into(),
                    failure_callback.into(),
                ],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `signalingState` attribute.
    /// [`RTCPeerConnection.signalingState`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/signalingState)
    pub fn signaling_state(&self) -> RTCSignalingState {
        self.inner.get("signalingState").as_::<RTCSignalingState>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `iceGatheringState` attribute.
    /// [`RTCPeerConnection.iceGatheringState`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceGatheringState)
    pub fn ice_gathering_state(&self) -> RTCIceGatheringState {
        self.inner
            .get("iceGatheringState")
            .as_::<RTCIceGatheringState>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `iceConnectionState` attribute.
    /// [`RTCPeerConnection.iceConnectionState`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceConnectionState)
    pub fn ice_connection_state(&self) -> RTCIceConnectionState {
        self.inner
            .get("iceConnectionState")
            .as_::<RTCIceConnectionState>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `connectionState` attribute.
    /// [`RTCPeerConnection.connectionState`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/connectionState)
    pub fn connection_state(&self) -> RTCPeerConnectionState {
        self.inner
            .get("connectionState")
            .as_::<RTCPeerConnectionState>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `canTrickleIceCandidates` attribute.
    /// [`RTCPeerConnection.canTrickleIceCandidates`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/canTrickleIceCandidates)
    pub fn can_trickle_ice_candidates(&self) -> bool {
        self.inner.get("canTrickleIceCandidates").as_::<bool>()
    }
}
impl RTCPeerConnection {
    /// The restartIce method.
    /// [`RTCPeerConnection.restartIce`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/restartIce)
    pub fn restart_ice(&self) -> Undefined {
        self.inner.call("restartIce", &[]).as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The getConfiguration method.
    /// [`RTCPeerConnection.getConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getConfiguration)
    pub fn get_configuration(&self) -> RTCConfiguration {
        self.inner
            .call("getConfiguration", &[])
            .as_::<RTCConfiguration>()
    }
}
impl RTCPeerConnection {
    /// The setConfiguration method.
    /// [`RTCPeerConnection.setConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setConfiguration)
    pub fn set_configuration0(&self) -> Undefined {
        self.inner.call("setConfiguration", &[]).as_::<Undefined>()
    }
    /// The setConfiguration method.
    /// [`RTCPeerConnection.setConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setConfiguration)
    pub fn set_configuration1(&self, configuration: &RTCConfiguration) -> Undefined {
        self.inner
            .call("setConfiguration", &[configuration.into()])
            .as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The close method.
    /// [`RTCPeerConnection.close`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `onnegotiationneeded` attribute.
    /// [`RTCPeerConnection.onnegotiationneeded`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)
    pub fn onnegotiationneeded(&self) -> Any {
        self.inner.get("onnegotiationneeded").as_::<Any>()
    }

    /// Setter of the `onnegotiationneeded` attribute.
    /// [`RTCPeerConnection.onnegotiationneeded`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)
    pub fn set_onnegotiationneeded(&mut self, value: &Any) {
        self.inner.set("onnegotiationneeded", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `onicecandidate` attribute.
    /// [`RTCPeerConnection.onicecandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)
    pub fn onicecandidate(&self) -> Any {
        self.inner.get("onicecandidate").as_::<Any>()
    }

    /// Setter of the `onicecandidate` attribute.
    /// [`RTCPeerConnection.onicecandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)
    pub fn set_onicecandidate(&mut self, value: &Any) {
        self.inner.set("onicecandidate", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `onicecandidateerror` attribute.
    /// [`RTCPeerConnection.onicecandidateerror`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidateerror)
    pub fn onicecandidateerror(&self) -> Any {
        self.inner.get("onicecandidateerror").as_::<Any>()
    }

    /// Setter of the `onicecandidateerror` attribute.
    /// [`RTCPeerConnection.onicecandidateerror`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidateerror)
    pub fn set_onicecandidateerror(&mut self, value: &Any) {
        self.inner.set("onicecandidateerror", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `onsignalingstatechange` attribute.
    /// [`RTCPeerConnection.onsignalingstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)
    pub fn onsignalingstatechange(&self) -> Any {
        self.inner.get("onsignalingstatechange").as_::<Any>()
    }

    /// Setter of the `onsignalingstatechange` attribute.
    /// [`RTCPeerConnection.onsignalingstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)
    pub fn set_onsignalingstatechange(&mut self, value: &Any) {
        self.inner.set("onsignalingstatechange", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `oniceconnectionstatechange` attribute.
    /// [`RTCPeerConnection.oniceconnectionstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)
    pub fn oniceconnectionstatechange(&self) -> Any {
        self.inner.get("oniceconnectionstatechange").as_::<Any>()
    }

    /// Setter of the `oniceconnectionstatechange` attribute.
    /// [`RTCPeerConnection.oniceconnectionstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)
    pub fn set_oniceconnectionstatechange(&mut self, value: &Any) {
        self.inner.set("oniceconnectionstatechange", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `onicegatheringstatechange` attribute.
    /// [`RTCPeerConnection.onicegatheringstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)
    pub fn onicegatheringstatechange(&self) -> Any {
        self.inner.get("onicegatheringstatechange").as_::<Any>()
    }

    /// Setter of the `onicegatheringstatechange` attribute.
    /// [`RTCPeerConnection.onicegatheringstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)
    pub fn set_onicegatheringstatechange(&mut self, value: &Any) {
        self.inner.set("onicegatheringstatechange", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `onconnectionstatechange` attribute.
    /// [`RTCPeerConnection.onconnectionstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onconnectionstatechange)
    pub fn onconnectionstatechange(&self) -> Any {
        self.inner.get("onconnectionstatechange").as_::<Any>()
    }

    /// Setter of the `onconnectionstatechange` attribute.
    /// [`RTCPeerConnection.onconnectionstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onconnectionstatechange)
    pub fn set_onconnectionstatechange(&mut self, value: &Any) {
        self.inner.set("onconnectionstatechange", value);
    }
}
impl RTCPeerConnection {
    /// The setIdentityProvider method.
    /// [`RTCPeerConnection.setIdentityProvider`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)
    pub fn set_identity_provider0(&self, provider: &JsString) -> Undefined {
        self.inner
            .call("setIdentityProvider", &[provider.into()])
            .as_::<Undefined>()
    }
    /// The setIdentityProvider method.
    /// [`RTCPeerConnection.setIdentityProvider`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)
    pub fn set_identity_provider1(
        &self,
        provider: &JsString,
        options: &RTCIdentityProviderOptions,
    ) -> Undefined {
        self.inner
            .call("setIdentityProvider", &[provider.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The getIdentityAssertion method.
    /// [`RTCPeerConnection.getIdentityAssertion`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getIdentityAssertion)
    pub fn get_identity_assertion(&self) -> Promise<JsString> {
        self.inner
            .call("getIdentityAssertion", &[])
            .as_::<Promise<JsString>>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `peerIdentity` attribute.
    /// [`RTCPeerConnection.peerIdentity`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/peerIdentity)
    pub fn peer_identity(&self) -> Promise<RTCIdentityAssertion> {
        self.inner
            .get("peerIdentity")
            .as_::<Promise<RTCIdentityAssertion>>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `idpLoginUrl` attribute.
    /// [`RTCPeerConnection.idpLoginUrl`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/idpLoginUrl)
    pub fn idp_login_url(&self) -> JsString {
        self.inner.get("idpLoginUrl").as_::<JsString>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `idpErrorInfo` attribute.
    /// [`RTCPeerConnection.idpErrorInfo`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/idpErrorInfo)
    pub fn idp_error_info(&self) -> JsString {
        self.inner.get("idpErrorInfo").as_::<JsString>()
    }
}
impl RTCPeerConnection {
    /// The generateCertificate method.
    /// [`RTCPeerConnection.generateCertificate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/generateCertificate)
    pub fn generate_certificate(keygen_algorithm: &Any) -> Promise<RTCCertificate> {
        Any::global("RTCPeerConnection")
            .call("generateCertificate", &[keygen_algorithm.into()])
            .as_::<Promise<RTCCertificate>>()
    }
}
impl RTCPeerConnection {
    /// The getSenders method.
    /// [`RTCPeerConnection.getSenders`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getSenders)
    pub fn get_senders(&self) -> TypedArray<RTCRtpSender> {
        self.inner
            .call("getSenders", &[])
            .as_::<TypedArray<RTCRtpSender>>()
    }
}
impl RTCPeerConnection {
    /// The getReceivers method.
    /// [`RTCPeerConnection.getReceivers`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getReceivers)
    pub fn get_receivers(&self) -> TypedArray<RTCRtpReceiver> {
        self.inner
            .call("getReceivers", &[])
            .as_::<TypedArray<RTCRtpReceiver>>()
    }
}
impl RTCPeerConnection {
    /// The getTransceivers method.
    /// [`RTCPeerConnection.getTransceivers`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getTransceivers)
    pub fn get_transceivers(&self) -> TypedArray<RTCRtpTransceiver> {
        self.inner
            .call("getTransceivers", &[])
            .as_::<TypedArray<RTCRtpTransceiver>>()
    }
}
impl RTCPeerConnection {
    /// The addTrack method.
    /// [`RTCPeerConnection.addTrack`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)
    pub fn add_track(&self, track: &MediaStreamTrack, streams: &MediaStream) -> RTCRtpSender {
        self.inner
            .call("addTrack", &[track.into(), streams.into()])
            .as_::<RTCRtpSender>()
    }
}
impl RTCPeerConnection {
    /// The removeTrack method.
    /// [`RTCPeerConnection.removeTrack`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/removeTrack)
    pub fn remove_track(&self, sender: &RTCRtpSender) -> Undefined {
        self.inner
            .call("removeTrack", &[sender.into()])
            .as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The addTransceiver method.
    /// [`RTCPeerConnection.addTransceiver`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)
    pub fn add_transceiver0(&self, track_or_kind: &Any) -> RTCRtpTransceiver {
        self.inner
            .call("addTransceiver", &[track_or_kind.into()])
            .as_::<RTCRtpTransceiver>()
    }
    /// The addTransceiver method.
    /// [`RTCPeerConnection.addTransceiver`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)
    pub fn add_transceiver1(
        &self,
        track_or_kind: &Any,
        init: &RTCRtpTransceiverInit,
    ) -> RTCRtpTransceiver {
        self.inner
            .call("addTransceiver", &[track_or_kind.into(), init.into()])
            .as_::<RTCRtpTransceiver>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `ontrack` attribute.
    /// [`RTCPeerConnection.ontrack`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)
    pub fn ontrack(&self) -> Any {
        self.inner.get("ontrack").as_::<Any>()
    }

    /// Setter of the `ontrack` attribute.
    /// [`RTCPeerConnection.ontrack`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)
    pub fn set_ontrack(&mut self, value: &Any) {
        self.inner.set("ontrack", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `sctp` attribute.
    /// [`RTCPeerConnection.sctp`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/sctp)
    pub fn sctp(&self) -> RTCSctpTransport {
        self.inner.get("sctp").as_::<RTCSctpTransport>()
    }
}
impl RTCPeerConnection {
    /// The createDataChannel method.
    /// [`RTCPeerConnection.createDataChannel`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)
    pub fn create_data_channel0(&self, label: &JsString) -> RTCDataChannel {
        self.inner
            .call("createDataChannel", &[label.into()])
            .as_::<RTCDataChannel>()
    }
    /// The createDataChannel method.
    /// [`RTCPeerConnection.createDataChannel`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)
    pub fn create_data_channel1(
        &self,
        label: &JsString,
        data_channel_dict: &RTCDataChannelInit,
    ) -> RTCDataChannel {
        self.inner
            .call(
                "createDataChannel",
                &[label.into(), data_channel_dict.into()],
            )
            .as_::<RTCDataChannel>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `ondatachannel` attribute.
    /// [`RTCPeerConnection.ondatachannel`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)
    pub fn ondatachannel(&self) -> Any {
        self.inner.get("ondatachannel").as_::<Any>()
    }

    /// Setter of the `ondatachannel` attribute.
    /// [`RTCPeerConnection.ondatachannel`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)
    pub fn set_ondatachannel(&mut self, value: &Any) {
        self.inner.set("ondatachannel", value);
    }
}
impl RTCPeerConnection {
    /// The getStats method.
    /// [`RTCPeerConnection.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)
    pub fn get_stats0(&self) -> Promise<RTCStatsReport> {
        self.inner
            .call("getStats", &[])
            .as_::<Promise<RTCStatsReport>>()
    }
    /// The getStats method.
    /// [`RTCPeerConnection.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)
    pub fn get_stats1(&self, selector: &MediaStreamTrack) -> Promise<RTCStatsReport> {
        self.inner
            .call("getStats", &[selector.into()])
            .as_::<Promise<RTCStatsReport>>()
    }
}
