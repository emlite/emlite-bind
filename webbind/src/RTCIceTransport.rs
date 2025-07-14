use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceParameters {
    inner: emlite::Val,
}
impl FromVal for RTCIceParameters {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceParameters { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIceParameters {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCIceParameters {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCIceParameters {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCIceParameters> for emlite::Val {
    fn from(s: RTCIceParameters) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceParameters {
    pub fn username_fragment(&self) -> jsbind::DOMString {
        self.inner
            .get("usernameFragment")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_username_fragment(&mut self, value: jsbind::DOMString) {
        self.inner.set("usernameFragment", value);
    }
}
impl RTCIceParameters {
    pub fn password(&self) -> jsbind::DOMString {
        self.inner.get("password").as_::<jsbind::DOMString>()
    }

    pub fn set_password(&mut self, value: jsbind::DOMString) {
        self.inner.set("password", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceGatherOptions {
    inner: emlite::Val,
}
impl FromVal for RTCIceGatherOptions {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceGatherOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIceGatherOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceGatherOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCIceGatherOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCIceGatherOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCIceGatherOptions> for emlite::Val {
    fn from(s: RTCIceGatherOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceGatherOptions {
    pub fn gather_policy(&self) -> RTCIceTransportPolicy {
        self.inner
            .get("gatherPolicy")
            .as_::<RTCIceTransportPolicy>()
    }

    pub fn set_gather_policy(&mut self, value: RTCIceTransportPolicy) {
        self.inner.set("gatherPolicy", value);
    }
}
impl RTCIceGatherOptions {
    pub fn ice_servers(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("iceServers")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_ice_servers(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("iceServers", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceCandidateInit {
    inner: emlite::Val,
}
impl FromVal for RTCIceCandidateInit {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceCandidateInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIceCandidateInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceCandidateInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCIceCandidateInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCIceCandidateInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCIceCandidateInit> for emlite::Val {
    fn from(s: RTCIceCandidateInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceCandidateInit {
    pub fn candidate(&self) -> jsbind::DOMString {
        self.inner.get("candidate").as_::<jsbind::DOMString>()
    }

    pub fn set_candidate(&mut self, value: jsbind::DOMString) {
        self.inner.set("candidate", value);
    }
}
impl RTCIceCandidateInit {
    pub fn sdp_mid(&self) -> jsbind::DOMString {
        self.inner.get("sdpMid").as_::<jsbind::DOMString>()
    }

    pub fn set_sdp_mid(&mut self, value: jsbind::DOMString) {
        self.inner.set("sdpMid", value);
    }
}
impl RTCIceCandidateInit {
    pub fn sdp_m_line_index(&self) -> u16 {
        self.inner.get("sdpMLineIndex").as_::<u16>()
    }

    pub fn set_sdp_m_line_index(&mut self, value: u16) {
        self.inner.set("sdpMLineIndex", value);
    }
}
impl RTCIceCandidateInit {
    pub fn username_fragment(&self) -> jsbind::DOMString {
        self.inner
            .get("usernameFragment")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_username_fragment(&mut self, value: jsbind::DOMString) {
        self.inner.set("usernameFragment", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceTransport {
    inner: EventTarget,
}
impl FromVal for RTCIceTransport {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceTransport {
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
impl core::ops::Deref for RTCIceTransport {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceTransport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCIceTransport {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCIceTransport {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCIceTransport> for emlite::Val {
    fn from(s: RTCIceTransport) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(RTCIceTransport);

impl RTCIceTransport {
    pub fn role(&self) -> RTCIceRole {
        self.inner.get("role").as_::<RTCIceRole>()
    }
}
impl RTCIceTransport {
    pub fn component(&self) -> RTCIceComponent {
        self.inner.get("component").as_::<RTCIceComponent>()
    }
}
impl RTCIceTransport {
    pub fn state(&self) -> RTCIceTransportState {
        self.inner.get("state").as_::<RTCIceTransportState>()
    }
}
impl RTCIceTransport {
    pub fn gathering_state(&self) -> RTCIceGathererState {
        self.inner
            .get("gatheringState")
            .as_::<RTCIceGathererState>()
    }
}
impl RTCIceTransport {
    pub fn get_local_candidates(&self) -> jsbind::Sequence<RTCIceCandidate> {
        self.inner
            .call("getLocalCandidates", &[])
            .as_::<jsbind::Sequence<RTCIceCandidate>>()
    }
}
impl RTCIceTransport {
    pub fn get_remote_candidates(&self) -> jsbind::Sequence<RTCIceCandidate> {
        self.inner
            .call("getRemoteCandidates", &[])
            .as_::<jsbind::Sequence<RTCIceCandidate>>()
    }
}
impl RTCIceTransport {
    pub fn get_selected_candidate_pair(&self) -> RTCIceCandidatePair {
        self.inner
            .call("getSelectedCandidatePair", &[])
            .as_::<RTCIceCandidatePair>()
    }
}
impl RTCIceTransport {
    pub fn get_local_parameters(&self) -> RTCIceParameters {
        self.inner
            .call("getLocalParameters", &[])
            .as_::<RTCIceParameters>()
    }
}
impl RTCIceTransport {
    pub fn get_remote_parameters(&self) -> RTCIceParameters {
        self.inner
            .call("getRemoteParameters", &[])
            .as_::<RTCIceParameters>()
    }
}
impl RTCIceTransport {
    pub fn onstatechange(&self) -> jsbind::Any {
        self.inner.get("onstatechange").as_::<jsbind::Any>()
    }

    pub fn set_onstatechange(&mut self, value: jsbind::Any) {
        self.inner.set("onstatechange", value);
    }
}
impl RTCIceTransport {
    pub fn ongatheringstatechange(&self) -> jsbind::Any {
        self.inner
            .get("ongatheringstatechange")
            .as_::<jsbind::Any>()
    }

    pub fn set_ongatheringstatechange(&mut self, value: jsbind::Any) {
        self.inner.set("ongatheringstatechange", value);
    }
}
impl RTCIceTransport {
    pub fn onselectedcandidatepairchange(&self) -> jsbind::Any {
        self.inner
            .get("onselectedcandidatepairchange")
            .as_::<jsbind::Any>()
    }

    pub fn set_onselectedcandidatepairchange(&mut self, value: jsbind::Any) {
        self.inner.set("onselectedcandidatepairchange", value);
    }
}

impl RTCIceTransport {
    pub fn new() -> RTCIceTransport {
        Self {
            inner: emlite::Val::global("RTCIceTransport")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }
}
impl RTCIceTransport {
    pub fn gather0(&self) -> jsbind::Undefined {
        self.inner.call("gather", &[]).as_::<jsbind::Undefined>()
    }

    pub fn gather1(&self, options: RTCIceGatherOptions) -> jsbind::Undefined {
        self.inner
            .call("gather", &[options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl RTCIceTransport {
    pub fn start0(&self) -> jsbind::Undefined {
        self.inner.call("start", &[]).as_::<jsbind::Undefined>()
    }

    pub fn start1(&self, remote_parameters: RTCIceParameters) -> jsbind::Undefined {
        self.inner
            .call("start", &[remote_parameters.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn start2(
        &self,
        remote_parameters: RTCIceParameters,
        role: RTCIceRole,
    ) -> jsbind::Undefined {
        self.inner
            .call("start", &[remote_parameters.into(), role.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl RTCIceTransport {
    pub fn stop(&self) -> jsbind::Undefined {
        self.inner.call("stop", &[]).as_::<jsbind::Undefined>()
    }
}
impl RTCIceTransport {
    pub fn add_remote_candidate0(&self) -> jsbind::Undefined {
        self.inner
            .call("addRemoteCandidate", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn add_remote_candidate1(
        &self,
        remote_candidate: RTCIceCandidateInit,
    ) -> jsbind::Undefined {
        self.inner
            .call("addRemoteCandidate", &[remote_candidate.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl RTCIceTransport {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl RTCIceTransport {
    pub fn onicecandidate(&self) -> jsbind::Any {
        self.inner.get("onicecandidate").as_::<jsbind::Any>()
    }

    pub fn set_onicecandidate(&mut self, value: jsbind::Any) {
        self.inner.set("onicecandidate", value);
    }
}
