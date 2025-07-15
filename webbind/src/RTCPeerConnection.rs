use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCOfferOptions {
    inner: emlite::Val,
}
impl FromVal for RTCOfferOptions {
    fn from_val(v: &emlite::Val) -> Self {
        RTCOfferOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCOfferOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCOfferOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCOfferOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCOfferOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCOfferOptions> for emlite::Val {
    fn from(s: RTCOfferOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCOfferOptions> for emlite::Val {
    fn from(s: &RTCOfferOptions) -> emlite::Val {
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
    inner: emlite::Val,
}
impl FromVal for RTCLocalSessionDescriptionInit {
    fn from_val(v: &emlite::Val) -> Self {
        RTCLocalSessionDescriptionInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCLocalSessionDescriptionInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCLocalSessionDescriptionInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCLocalSessionDescriptionInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCLocalSessionDescriptionInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCLocalSessionDescriptionInit> for emlite::Val {
    fn from(s: RTCLocalSessionDescriptionInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCLocalSessionDescriptionInit> for emlite::Val {
    fn from(s: &RTCLocalSessionDescriptionInit) -> emlite::Val {
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
    pub fn sdp(&self) -> String {
        self.inner.get("sdp").as_::<String>()
    }

    pub fn set_sdp(&mut self, value: &str) {
        self.inner.set("sdp", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCSessionDescriptionInit {
    inner: emlite::Val,
}
impl FromVal for RTCSessionDescriptionInit {
    fn from_val(v: &emlite::Val) -> Self {
        RTCSessionDescriptionInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCSessionDescriptionInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCSessionDescriptionInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCSessionDescriptionInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCSessionDescriptionInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCSessionDescriptionInit> for emlite::Val {
    fn from(s: RTCSessionDescriptionInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCSessionDescriptionInit> for emlite::Val {
    fn from(s: &RTCSessionDescriptionInit) -> emlite::Val {
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
    pub fn sdp(&self) -> String {
        self.inner.get("sdp").as_::<String>()
    }

    pub fn set_sdp(&mut self, value: &str) {
        self.inner.set("sdp", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCConfiguration {
    inner: emlite::Val,
}
impl FromVal for RTCConfiguration {
    fn from_val(v: &emlite::Val) -> Self {
        RTCConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCConfiguration {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCConfiguration {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCConfiguration {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCConfiguration> for emlite::Val {
    fn from(s: RTCConfiguration) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCConfiguration> for emlite::Val {
    fn from(s: &RTCConfiguration) -> emlite::Val {
        s.inner.clone()
    }
}

impl RTCConfiguration {
    pub fn ice_servers(&self) -> Sequence<Any> {
        self.inner.get("iceServers").as_::<Sequence<Any>>()
    }

    pub fn set_ice_servers(&mut self, value: &Sequence<Any>) {
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
    pub fn certificates(&self) -> Sequence<RTCCertificate> {
        self.inner
            .get("certificates")
            .as_::<Sequence<RTCCertificate>>()
    }

    pub fn set_certificates(&mut self, value: &Sequence<RTCCertificate>) {
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
    inner: emlite::Val,
}
impl FromVal for RTCIdentityProviderOptions {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIdentityProviderOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIdentityProviderOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIdentityProviderOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCIdentityProviderOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCIdentityProviderOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCIdentityProviderOptions> for emlite::Val {
    fn from(s: RTCIdentityProviderOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCIdentityProviderOptions> for emlite::Val {
    fn from(s: &RTCIdentityProviderOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl RTCIdentityProviderOptions {
    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }

    pub fn set_protocol(&mut self, value: &str) {
        self.inner.set("protocol", value);
    }
}
impl RTCIdentityProviderOptions {
    pub fn username_hint(&self) -> String {
        self.inner.get("usernameHint").as_::<String>()
    }

    pub fn set_username_hint(&mut self, value: &str) {
        self.inner.set("usernameHint", value);
    }
}
impl RTCIdentityProviderOptions {
    pub fn peer_identity(&self) -> String {
        self.inner.get("peerIdentity").as_::<String>()
    }

    pub fn set_peer_identity(&mut self, value: &str) {
        self.inner.set("peerIdentity", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpTransceiverInit {
    inner: emlite::Val,
}
impl FromVal for RTCRtpTransceiverInit {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpTransceiverInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpTransceiverInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpTransceiverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCRtpTransceiverInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCRtpTransceiverInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCRtpTransceiverInit> for emlite::Val {
    fn from(s: RTCRtpTransceiverInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCRtpTransceiverInit> for emlite::Val {
    fn from(s: &RTCRtpTransceiverInit) -> emlite::Val {
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
    pub fn streams(&self) -> Sequence<MediaStream> {
        self.inner.get("streams").as_::<Sequence<MediaStream>>()
    }

    pub fn set_streams(&mut self, value: &Sequence<MediaStream>) {
        self.inner.set("streams", value);
    }
}
impl RTCRtpTransceiverInit {
    pub fn send_encodings(&self) -> Sequence<Any> {
        self.inner.get("sendEncodings").as_::<Sequence<Any>>()
    }

    pub fn set_send_encodings(&mut self, value: &Sequence<Any>) {
        self.inner.set("sendEncodings", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDataChannelInit {
    inner: emlite::Val,
}
impl FromVal for RTCDataChannelInit {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDataChannelInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCDataChannelInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDataChannelInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCDataChannelInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCDataChannelInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCDataChannelInit> for emlite::Val {
    fn from(s: RTCDataChannelInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCDataChannelInit> for emlite::Val {
    fn from(s: &RTCDataChannelInit) -> emlite::Val {
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
    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }

    pub fn set_protocol(&mut self, value: &str) {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCPeerConnection {
    inner: EventTarget,
}
impl FromVal for RTCPeerConnection {
    fn from_val(v: &emlite::Val) -> Self {
        RTCPeerConnection {
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
impl AsRef<emlite::Val> for RTCPeerConnection {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCPeerConnection {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCPeerConnection> for emlite::Val {
    fn from(s: RTCPeerConnection) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCPeerConnection> for emlite::Val {
    fn from(s: &RTCPeerConnection) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCPeerConnection);

impl RTCPeerConnection {
    pub fn new0() -> RTCPeerConnection {
        Self {
            inner: emlite::Val::global("RTCPeerConnection")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(configuration: &RTCConfiguration) -> RTCPeerConnection {
        Self {
            inner: emlite::Val::global("RTCPeerConnection")
                .new(&[configuration.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl RTCPeerConnection {
    pub fn create_offer0(
        &self,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise {
        self.inner
            .call(
                "createOffer",
                &[success_callback.into(), failure_callback.into()],
            )
            .as_::<Promise>()
    }

    pub fn create_offer1(
        &self,
        success_callback: &Function,
        failure_callback: &Function,
        options: &RTCOfferOptions,
    ) -> Promise {
        self.inner
            .call(
                "createOffer",
                &[
                    success_callback.into(),
                    failure_callback.into(),
                    options.into(),
                ],
            )
            .as_::<Promise>()
    }
}
impl RTCPeerConnection {
    pub fn create_answer(
        &self,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise {
        self.inner
            .call(
                "createAnswer",
                &[success_callback.into(), failure_callback.into()],
            )
            .as_::<Promise>()
    }
}
impl RTCPeerConnection {
    pub fn set_local_description(
        &self,
        description: &RTCLocalSessionDescriptionInit,
        success_callback: &Any,
        failure_callback: &Function,
    ) -> Promise {
        self.inner
            .call(
                "setLocalDescription",
                &[
                    description.into(),
                    success_callback.into(),
                    failure_callback.into(),
                ],
            )
            .as_::<Promise>()
    }
}
impl RTCPeerConnection {
    pub fn local_description(&self) -> RTCSessionDescription {
        self.inner
            .get("localDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    pub fn current_local_description(&self) -> RTCSessionDescription {
        self.inner
            .get("currentLocalDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    pub fn pending_local_description(&self) -> RTCSessionDescription {
        self.inner
            .get("pendingLocalDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    pub fn set_remote_description(
        &self,
        description: &RTCSessionDescriptionInit,
        success_callback: &Any,
        failure_callback: &Function,
    ) -> Promise {
        self.inner
            .call(
                "setRemoteDescription",
                &[
                    description.into(),
                    success_callback.into(),
                    failure_callback.into(),
                ],
            )
            .as_::<Promise>()
    }
}
impl RTCPeerConnection {
    pub fn remote_description(&self) -> RTCSessionDescription {
        self.inner
            .get("remoteDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    pub fn current_remote_description(&self) -> RTCSessionDescription {
        self.inner
            .get("currentRemoteDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    pub fn pending_remote_description(&self) -> RTCSessionDescription {
        self.inner
            .get("pendingRemoteDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    pub fn add_ice_candidate(
        &self,
        candidate: &RTCIceCandidateInit,
        success_callback: &Any,
        failure_callback: &Function,
    ) -> Promise {
        self.inner
            .call(
                "addIceCandidate",
                &[
                    candidate.into(),
                    success_callback.into(),
                    failure_callback.into(),
                ],
            )
            .as_::<Promise>()
    }
}
impl RTCPeerConnection {
    pub fn signaling_state(&self) -> RTCSignalingState {
        self.inner.get("signalingState").as_::<RTCSignalingState>()
    }
}
impl RTCPeerConnection {
    pub fn ice_gathering_state(&self) -> RTCIceGatheringState {
        self.inner
            .get("iceGatheringState")
            .as_::<RTCIceGatheringState>()
    }
}
impl RTCPeerConnection {
    pub fn ice_connection_state(&self) -> RTCIceConnectionState {
        self.inner
            .get("iceConnectionState")
            .as_::<RTCIceConnectionState>()
    }
}
impl RTCPeerConnection {
    pub fn connection_state(&self) -> RTCPeerConnectionState {
        self.inner
            .get("connectionState")
            .as_::<RTCPeerConnectionState>()
    }
}
impl RTCPeerConnection {
    pub fn can_trickle_ice_candidates(&self) -> bool {
        self.inner.get("canTrickleIceCandidates").as_::<bool>()
    }
}
impl RTCPeerConnection {
    pub fn restart_ice(&self) -> Undefined {
        self.inner.call("restartIce", &[]).as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    pub fn get_configuration(&self) -> RTCConfiguration {
        self.inner
            .call("getConfiguration", &[])
            .as_::<RTCConfiguration>()
    }
}
impl RTCPeerConnection {
    pub fn set_configuration0(&self) -> Undefined {
        self.inner.call("setConfiguration", &[]).as_::<Undefined>()
    }

    pub fn set_configuration1(&self, configuration: &RTCConfiguration) -> Undefined {
        self.inner
            .call("setConfiguration", &[configuration.into()])
            .as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    pub fn onnegotiationneeded(&self) -> Any {
        self.inner.get("onnegotiationneeded").as_::<Any>()
    }

    pub fn set_onnegotiationneeded(&mut self, value: &Any) {
        self.inner.set("onnegotiationneeded", value);
    }
}
impl RTCPeerConnection {
    pub fn onicecandidate(&self) -> Any {
        self.inner.get("onicecandidate").as_::<Any>()
    }

    pub fn set_onicecandidate(&mut self, value: &Any) {
        self.inner.set("onicecandidate", value);
    }
}
impl RTCPeerConnection {
    pub fn onicecandidateerror(&self) -> Any {
        self.inner.get("onicecandidateerror").as_::<Any>()
    }

    pub fn set_onicecandidateerror(&mut self, value: &Any) {
        self.inner.set("onicecandidateerror", value);
    }
}
impl RTCPeerConnection {
    pub fn onsignalingstatechange(&self) -> Any {
        self.inner.get("onsignalingstatechange").as_::<Any>()
    }

    pub fn set_onsignalingstatechange(&mut self, value: &Any) {
        self.inner.set("onsignalingstatechange", value);
    }
}
impl RTCPeerConnection {
    pub fn oniceconnectionstatechange(&self) -> Any {
        self.inner.get("oniceconnectionstatechange").as_::<Any>()
    }

    pub fn set_oniceconnectionstatechange(&mut self, value: &Any) {
        self.inner.set("oniceconnectionstatechange", value);
    }
}
impl RTCPeerConnection {
    pub fn onicegatheringstatechange(&self) -> Any {
        self.inner.get("onicegatheringstatechange").as_::<Any>()
    }

    pub fn set_onicegatheringstatechange(&mut self, value: &Any) {
        self.inner.set("onicegatheringstatechange", value);
    }
}
impl RTCPeerConnection {
    pub fn onconnectionstatechange(&self) -> Any {
        self.inner.get("onconnectionstatechange").as_::<Any>()
    }

    pub fn set_onconnectionstatechange(&mut self, value: &Any) {
        self.inner.set("onconnectionstatechange", value);
    }
}
impl RTCPeerConnection {
    pub fn set_identity_provider0(&self, provider: &str) -> Undefined {
        self.inner
            .call("setIdentityProvider", &[provider.into()])
            .as_::<Undefined>()
    }

    pub fn set_identity_provider1(
        &self,
        provider: &str,
        options: &RTCIdentityProviderOptions,
    ) -> Undefined {
        self.inner
            .call("setIdentityProvider", &[provider.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    pub fn get_identity_assertion(&self) -> Promise {
        self.inner
            .call("getIdentityAssertion", &[])
            .as_::<Promise>()
    }
}
impl RTCPeerConnection {
    pub fn peer_identity(&self) -> Promise {
        self.inner.get("peerIdentity").as_::<Promise>()
    }
}
impl RTCPeerConnection {
    pub fn idp_login_url(&self) -> String {
        self.inner.get("idpLoginUrl").as_::<String>()
    }
}
impl RTCPeerConnection {
    pub fn idp_error_info(&self) -> String {
        self.inner.get("idpErrorInfo").as_::<String>()
    }
}
impl RTCPeerConnection {
    pub fn generate_certificate(keygen_algorithm: &Any) -> Promise {
        emlite::Val::global("RTCPeerConnection")
            .call("generateCertificate", &[keygen_algorithm.into()])
            .as_::<Promise>()
    }
}
impl RTCPeerConnection {
    pub fn get_senders(&self) -> Sequence<RTCRtpSender> {
        self.inner
            .call("getSenders", &[])
            .as_::<Sequence<RTCRtpSender>>()
    }
}
impl RTCPeerConnection {
    pub fn get_receivers(&self) -> Sequence<RTCRtpReceiver> {
        self.inner
            .call("getReceivers", &[])
            .as_::<Sequence<RTCRtpReceiver>>()
    }
}
impl RTCPeerConnection {
    pub fn get_transceivers(&self) -> Sequence<RTCRtpTransceiver> {
        self.inner
            .call("getTransceivers", &[])
            .as_::<Sequence<RTCRtpTransceiver>>()
    }
}
impl RTCPeerConnection {
    pub fn add_track(&self, track: &MediaStreamTrack, streams: &MediaStream) -> RTCRtpSender {
        self.inner
            .call("addTrack", &[track.into(), streams.into()])
            .as_::<RTCRtpSender>()
    }
}
impl RTCPeerConnection {
    pub fn remove_track(&self, sender: &RTCRtpSender) -> Undefined {
        self.inner
            .call("removeTrack", &[sender.into()])
            .as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    pub fn add_transceiver0(&self, track_or_kind: &Any) -> RTCRtpTransceiver {
        self.inner
            .call("addTransceiver", &[track_or_kind.into()])
            .as_::<RTCRtpTransceiver>()
    }

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
    pub fn ontrack(&self) -> Any {
        self.inner.get("ontrack").as_::<Any>()
    }

    pub fn set_ontrack(&mut self, value: &Any) {
        self.inner.set("ontrack", value);
    }
}
impl RTCPeerConnection {
    pub fn sctp(&self) -> RTCSctpTransport {
        self.inner.get("sctp").as_::<RTCSctpTransport>()
    }
}
impl RTCPeerConnection {
    pub fn create_data_channel0(&self, label: &str) -> RTCDataChannel {
        self.inner
            .call("createDataChannel", &[label.into()])
            .as_::<RTCDataChannel>()
    }

    pub fn create_data_channel1(
        &self,
        label: &str,
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
    pub fn ondatachannel(&self) -> Any {
        self.inner.get("ondatachannel").as_::<Any>()
    }

    pub fn set_ondatachannel(&mut self, value: &Any) {
        self.inner.set("ondatachannel", value);
    }
}
impl RTCPeerConnection {
    pub fn get_stats0(&self) -> Promise {
        self.inner.call("getStats", &[]).as_::<Promise>()
    }

    pub fn get_stats1(&self, selector: &MediaStreamTrack) -> Promise {
        self.inner
            .call("getStats", &[selector.into()])
            .as_::<Promise>()
    }
}
