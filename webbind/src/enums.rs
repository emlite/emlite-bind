use super::*;

#[derive(Clone, Debug)]
pub struct SecurityPolicyViolationEventDisposition {
    inner: emlite::Val,
}
impl FromVal for SecurityPolicyViolationEventDisposition {
    fn from_val(v: &emlite::Val) -> Self {
        SecurityPolicyViolationEventDisposition { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SecurityPolicyViolationEventDisposition {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SecurityPolicyViolationEventDisposition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SecurityPolicyViolationEventDisposition> for emlite::Val {
    fn from(s: SecurityPolicyViolationEventDisposition) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SecurityPolicyViolationEventDisposition {
    pub const ENFORCE: &str = "enforce";
    pub const REPORT: &str = "report";
}

#[derive(Clone, Debug)]
pub struct EndingType {
    inner: emlite::Val,
}
impl FromVal for EndingType {
    fn from_val(v: &emlite::Val) -> Self {
        EndingType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for EndingType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EndingType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EndingType> for emlite::Val {
    fn from(s: EndingType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EndingType {
    pub const TRANSPARENT: &str = "transparent";
    pub const NATIVE: &str = "native";
}

#[derive(Clone, Debug)]
pub struct IDBRequestReadyState {
    inner: emlite::Val,
}
impl FromVal for IDBRequestReadyState {
    fn from_val(v: &emlite::Val) -> Self {
        IDBRequestReadyState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IDBRequestReadyState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IDBRequestReadyState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBRequestReadyState> for emlite::Val {
    fn from(s: IDBRequestReadyState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBRequestReadyState {
    pub const PENDING: &str = "pending";
    pub const DONE: &str = "done";
}

#[derive(Clone, Debug)]
pub struct IDBTransactionDurability {
    inner: emlite::Val,
}
impl FromVal for IDBTransactionDurability {
    fn from_val(v: &emlite::Val) -> Self {
        IDBTransactionDurability { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IDBTransactionDurability {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IDBTransactionDurability {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBTransactionDurability> for emlite::Val {
    fn from(s: IDBTransactionDurability) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBTransactionDurability {
    pub const DEFAULT: &str = "default";
    pub const STRICT: &str = "strict";
    pub const RELAXED: &str = "relaxed";
}

#[derive(Clone, Debug)]
pub struct IDBCursorDirection {
    inner: emlite::Val,
}
impl FromVal for IDBCursorDirection {
    fn from_val(v: &emlite::Val) -> Self {
        IDBCursorDirection { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IDBCursorDirection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IDBCursorDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBCursorDirection> for emlite::Val {
    fn from(s: IDBCursorDirection) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBCursorDirection {
    pub const NEXT: &str = "next";
    pub const NEXTUNIQUE: &str = "nextunique";
    pub const PREV: &str = "prev";
    pub const PREVUNIQUE: &str = "prevunique";
}

#[derive(Clone, Debug)]
pub struct IDBTransactionMode {
    inner: emlite::Val,
}
impl FromVal for IDBTransactionMode {
    fn from_val(v: &emlite::Val) -> Self {
        IDBTransactionMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IDBTransactionMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IDBTransactionMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBTransactionMode> for emlite::Val {
    fn from(s: IDBTransactionMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBTransactionMode {
    pub const READONLY: &str = "readonly";
    pub const READWRITE: &str = "readwrite";
    pub const VERSIONCHANGE: &str = "versionchange";
}

#[derive(Clone, Debug)]
pub struct AccelerometerLocalCoordinateSystem {
    inner: emlite::Val,
}
impl FromVal for AccelerometerLocalCoordinateSystem {
    fn from_val(v: &emlite::Val) -> Self {
        AccelerometerLocalCoordinateSystem { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AccelerometerLocalCoordinateSystem {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AccelerometerLocalCoordinateSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AccelerometerLocalCoordinateSystem> for emlite::Val {
    fn from(s: AccelerometerLocalCoordinateSystem) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AccelerometerLocalCoordinateSystem {
    pub const DEVICE: &str = "device";
    pub const SCREEN: &str = "screen";
}

#[derive(Clone, Debug)]
pub struct AudioSessionType {
    inner: emlite::Val,
}
impl FromVal for AudioSessionType {
    fn from_val(v: &emlite::Val) -> Self {
        AudioSessionType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AudioSessionType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioSessionType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioSessionType> for emlite::Val {
    fn from(s: AudioSessionType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioSessionType {
    pub const AUTO: &str = "auto";
    pub const PLAYBACK: &str = "playback";
    pub const TRANSIENT: &str = "transient";
    pub const TRANSIENT_SOLO: &str = "transient-solo";
    pub const AMBIENT: &str = "ambient";
    pub const PLAY_AND_RECORD: &str = "play-and-record";
}

#[derive(Clone, Debug)]
pub struct AudioSessionState {
    inner: emlite::Val,
}
impl FromVal for AudioSessionState {
    fn from_val(v: &emlite::Val) -> Self {
        AudioSessionState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AudioSessionState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioSessionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioSessionState> for emlite::Val {
    fn from(s: AudioSessionState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioSessionState {
    pub const INACTIVE: &str = "inactive";
    pub const ACTIVE: &str = "active";
    pub const INTERRUPTED: &str = "interrupted";
}

#[derive(Clone, Debug)]
pub struct AutoplayPolicy {
    inner: emlite::Val,
}
impl FromVal for AutoplayPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        AutoplayPolicy { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AutoplayPolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AutoplayPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AutoplayPolicy> for emlite::Val {
    fn from(s: AutoplayPolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AutoplayPolicy {
    pub const ALLOWED: &str = "allowed";
    pub const ALLOWED_MUTED: &str = "allowed-muted";
    pub const DISALLOWED: &str = "disallowed";
}

#[derive(Clone, Debug)]
pub struct AutoplayPolicyMediaType {
    inner: emlite::Val,
}
impl FromVal for AutoplayPolicyMediaType {
    fn from_val(v: &emlite::Val) -> Self {
        AutoplayPolicyMediaType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AutoplayPolicyMediaType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AutoplayPolicyMediaType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AutoplayPolicyMediaType> for emlite::Val {
    fn from(s: AutoplayPolicyMediaType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AutoplayPolicyMediaType {
    pub const MEDIAELEMENT: &str = "mediaelement";
    pub const AUDIOCONTEXT: &str = "audiocontext";
}

#[derive(Clone, Debug)]
pub struct BackgroundFetchResult {
    inner: emlite::Val,
}
impl FromVal for BackgroundFetchResult {
    fn from_val(v: &emlite::Val) -> Self {
        BackgroundFetchResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BackgroundFetchResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BackgroundFetchResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BackgroundFetchResult> for emlite::Val {
    fn from(s: BackgroundFetchResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BackgroundFetchResult {
    pub const NONE: &str = "";
    pub const SUCCESS: &str = "success";
    pub const FAILURE: &str = "failure";
}

#[derive(Clone, Debug)]
pub struct BackgroundFetchFailureReason {
    inner: emlite::Val,
}
impl FromVal for BackgroundFetchFailureReason {
    fn from_val(v: &emlite::Val) -> Self {
        BackgroundFetchFailureReason { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BackgroundFetchFailureReason {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BackgroundFetchFailureReason {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BackgroundFetchFailureReason> for emlite::Val {
    fn from(s: BackgroundFetchFailureReason) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BackgroundFetchFailureReason {
    pub const NONE: &str = "";
    pub const ABORTED: &str = "aborted";
    pub const BAD_STATUS: &str = "bad-status";
    pub const FETCH_ERROR: &str = "fetch-error";
    pub const QUOTA_EXCEEDED: &str = "quota-exceeded";
    pub const DOWNLOAD_TOTAL_EXCEEDED: &str = "download-total-exceeded";
}

#[derive(Clone, Debug)]
pub struct PresentationStyle {
    inner: emlite::Val,
}
impl FromVal for PresentationStyle {
    fn from_val(v: &emlite::Val) -> Self {
        PresentationStyle { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PresentationStyle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PresentationStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PresentationStyle> for emlite::Val {
    fn from(s: PresentationStyle) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PresentationStyle {
    pub const UNSPECIFIED: &str = "unspecified";
    pub const INLINE: &str = "inline";
    pub const ATTACHMENT: &str = "attachment";
}

#[derive(Clone, Debug)]
pub struct CompressionFormat {
    inner: emlite::Val,
}
impl FromVal for CompressionFormat {
    fn from_val(v: &emlite::Val) -> Self {
        CompressionFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CompressionFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CompressionFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CompressionFormat> for emlite::Val {
    fn from(s: CompressionFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CompressionFormat {
    pub const DEFLATE: &str = "deflate";
    pub const DEFLATE_RAW: &str = "deflate-raw";
    pub const GZIP: &str = "gzip";
}

#[derive(Clone, Debug)]
pub struct PressureSource {
    inner: emlite::Val,
}
impl FromVal for PressureSource {
    fn from_val(v: &emlite::Val) -> Self {
        PressureSource { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PressureSource {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PressureSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PressureSource> for emlite::Val {
    fn from(s: PressureSource) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PressureSource {
    pub const CPU: &str = "cpu";
}

#[derive(Clone, Debug)]
pub struct PressureState {
    inner: emlite::Val,
}
impl FromVal for PressureState {
    fn from_val(v: &emlite::Val) -> Self {
        PressureState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PressureState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PressureState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PressureState> for emlite::Val {
    fn from(s: PressureState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PressureState {
    pub const NOMINAL: &str = "nominal";
    pub const FAIR: &str = "fair";
    pub const SERIOUS: &str = "serious";
    pub const CRITICAL: &str = "critical";
}

#[derive(Clone, Debug)]
pub struct ContactProperty {
    inner: emlite::Val,
}
impl FromVal for ContactProperty {
    fn from_val(v: &emlite::Val) -> Self {
        ContactProperty { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ContactProperty {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ContactProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ContactProperty> for emlite::Val {
    fn from(s: ContactProperty) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ContactProperty {
    pub const ADDRESS: &str = "address";
    pub const EMAIL: &str = "email";
    pub const ICON: &str = "icon";
    pub const NAME: &str = "name";
    pub const TEL: &str = "tel";
}

#[derive(Clone, Debug)]
pub struct ContentCategory {
    inner: emlite::Val,
}
impl FromVal for ContentCategory {
    fn from_val(v: &emlite::Val) -> Self {
        ContentCategory { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ContentCategory {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ContentCategory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ContentCategory> for emlite::Val {
    fn from(s: ContentCategory) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ContentCategory {
    pub const NONE: &str = "";
    pub const HOMEPAGE: &str = "homepage";
    pub const ARTICLE: &str = "article";
    pub const VIDEO: &str = "video";
    pub const AUDIO: &str = "audio";
}

#[derive(Clone, Debug)]
pub struct CookieSameSite {
    inner: emlite::Val,
}
impl FromVal for CookieSameSite {
    fn from_val(v: &emlite::Val) -> Self {
        CookieSameSite { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CookieSameSite {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CookieSameSite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CookieSameSite> for emlite::Val {
    fn from(s: CookieSameSite) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CookieSameSite {
    pub const STRICT: &str = "strict";
    pub const LAX: &str = "lax";
    pub const NONE: &str = "none";
}

#[derive(Clone, Debug)]
pub struct CredentialMediationRequirement {
    inner: emlite::Val,
}
impl FromVal for CredentialMediationRequirement {
    fn from_val(v: &emlite::Val) -> Self {
        CredentialMediationRequirement { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CredentialMediationRequirement {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CredentialMediationRequirement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CredentialMediationRequirement> for emlite::Val {
    fn from(s: CredentialMediationRequirement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CredentialMediationRequirement {
    pub const SILENT: &str = "silent";
    pub const OPTIONAL: &str = "optional";
    pub const CONDITIONAL: &str = "conditional";
    pub const REQUIRED: &str = "required";
}

#[derive(Clone, Debug)]
pub struct ScriptingPolicyViolationType {
    inner: emlite::Val,
}
impl FromVal for ScriptingPolicyViolationType {
    fn from_val(v: &emlite::Val) -> Self {
        ScriptingPolicyViolationType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ScriptingPolicyViolationType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ScriptingPolicyViolationType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScriptingPolicyViolationType> for emlite::Val {
    fn from(s: ScriptingPolicyViolationType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScriptingPolicyViolationType {
    pub const EXTERNAL_SCRIPT: &str = "externalScript";
    pub const INLINE_SCRIPT: &str = "inlineScript";
    pub const INLINE_EVENT_HANDLER: &str = "inlineEventHandler";
    pub const EVAL: &str = "eval";
}

#[derive(Clone, Debug)]
pub struct FontFaceLoadStatus {
    inner: emlite::Val,
}
impl FromVal for FontFaceLoadStatus {
    fn from_val(v: &emlite::Val) -> Self {
        FontFaceLoadStatus { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FontFaceLoadStatus {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FontFaceLoadStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FontFaceLoadStatus> for emlite::Val {
    fn from(s: FontFaceLoadStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FontFaceLoadStatus {
    pub const UNLOADED: &str = "unloaded";
    pub const LOADING: &str = "loading";
    pub const LOADED: &str = "loaded";
    pub const ERROR: &str = "error";
}

#[derive(Clone, Debug)]
pub struct FontFaceSetLoadStatus {
    inner: emlite::Val,
}
impl FromVal for FontFaceSetLoadStatus {
    fn from_val(v: &emlite::Val) -> Self {
        FontFaceSetLoadStatus { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FontFaceSetLoadStatus {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FontFaceSetLoadStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FontFaceSetLoadStatus> for emlite::Val {
    fn from(s: FontFaceSetLoadStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FontFaceSetLoadStatus {
    pub const LOADING: &str = "loading";
    pub const LOADED: &str = "loaded";
}

#[derive(Clone, Debug)]
pub struct HighlightType {
    inner: emlite::Val,
}
impl FromVal for HighlightType {
    fn from_val(v: &emlite::Val) -> Self {
        HighlightType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HighlightType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HighlightType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HighlightType> for emlite::Val {
    fn from(s: HighlightType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HighlightType {
    pub const HIGHLIGHT: &str = "highlight";
    pub const SPELLING_ERROR: &str = "spelling-error";
    pub const GRAMMAR_ERROR: &str = "grammar-error";
}

#[derive(Clone, Debug)]
pub struct ChildDisplayType {
    inner: emlite::Val,
}
impl FromVal for ChildDisplayType {
    fn from_val(v: &emlite::Val) -> Self {
        ChildDisplayType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ChildDisplayType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ChildDisplayType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ChildDisplayType> for emlite::Val {
    fn from(s: ChildDisplayType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ChildDisplayType {
    pub const BLOCK: &str = "block";
    pub const NORMAL: &str = "normal";
}

#[derive(Clone, Debug)]
pub struct LayoutSizingMode {
    inner: emlite::Val,
}
impl FromVal for LayoutSizingMode {
    fn from_val(v: &emlite::Val) -> Self {
        LayoutSizingMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LayoutSizingMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LayoutSizingMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LayoutSizingMode> for emlite::Val {
    fn from(s: LayoutSizingMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LayoutSizingMode {
    pub const BLOCK_LIKE: &str = "block-like";
    pub const MANUAL: &str = "manual";
}

#[derive(Clone, Debug)]
pub struct BlockFragmentationType {
    inner: emlite::Val,
}
impl FromVal for BlockFragmentationType {
    fn from_val(v: &emlite::Val) -> Self {
        BlockFragmentationType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BlockFragmentationType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BlockFragmentationType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BlockFragmentationType> for emlite::Val {
    fn from(s: BlockFragmentationType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BlockFragmentationType {
    pub const NONE: &str = "none";
    pub const PAGE: &str = "page";
    pub const COLUMN: &str = "column";
    pub const REGION: &str = "region";
}

#[derive(Clone, Debug)]
pub struct BreakType {
    inner: emlite::Val,
}
impl FromVal for BreakType {
    fn from_val(v: &emlite::Val) -> Self {
        BreakType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BreakType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BreakType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BreakType> for emlite::Val {
    fn from(s: BreakType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BreakType {
    pub const NONE: &str = "none";
    pub const LINE: &str = "line";
    pub const COLUMN: &str = "column";
    pub const PAGE: &str = "page";
    pub const REGION: &str = "region";
}

#[derive(Clone, Debug)]
pub struct SpatialNavigationDirection {
    inner: emlite::Val,
}
impl FromVal for SpatialNavigationDirection {
    fn from_val(v: &emlite::Val) -> Self {
        SpatialNavigationDirection { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SpatialNavigationDirection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SpatialNavigationDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpatialNavigationDirection> for emlite::Val {
    fn from(s: SpatialNavigationDirection) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpatialNavigationDirection {
    pub const UP: &str = "up";
    pub const DOWN: &str = "down";
    pub const LEFT: &str = "left";
    pub const RIGHT: &str = "right";
}

#[derive(Clone, Debug)]
pub struct FocusableAreaSearchMode {
    inner: emlite::Val,
}
impl FromVal for FocusableAreaSearchMode {
    fn from_val(v: &emlite::Val) -> Self {
        FocusableAreaSearchMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FocusableAreaSearchMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FocusableAreaSearchMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FocusableAreaSearchMode> for emlite::Val {
    fn from(s: FocusableAreaSearchMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FocusableAreaSearchMode {
    pub const VISIBLE: &str = "visible";
    pub const ALL: &str = "all";
}

#[derive(Clone, Debug)]
pub struct CSSNumericBaseType {
    inner: emlite::Val,
}
impl FromVal for CSSNumericBaseType {
    fn from_val(v: &emlite::Val) -> Self {
        CSSNumericBaseType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSNumericBaseType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSNumericBaseType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSNumericBaseType> for emlite::Val {
    fn from(s: CSSNumericBaseType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSNumericBaseType {
    pub const LENGTH: &str = "length";
    pub const ANGLE: &str = "angle";
    pub const TIME: &str = "time";
    pub const FREQUENCY: &str = "frequency";
    pub const RESOLUTION: &str = "resolution";
    pub const FLEX: &str = "flex";
    pub const PERCENT: &str = "percent";
}

#[derive(Clone, Debug)]
pub struct CSSMathOperator {
    inner: emlite::Val,
}
impl FromVal for CSSMathOperator {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMathOperator { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSMathOperator {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSMathOperator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSMathOperator> for emlite::Val {
    fn from(s: CSSMathOperator) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSMathOperator {
    pub const SUM: &str = "sum";
    pub const PRODUCT: &str = "product";
    pub const NEGATE: &str = "negate";
    pub const INVERT: &str = "invert";
    pub const MIN: &str = "min";
    pub const MAX: &str = "max";
    pub const CLAMP: &str = "clamp";
}

#[derive(Clone, Debug)]
pub struct ScrollBehavior {
    inner: emlite::Val,
}
impl FromVal for ScrollBehavior {
    fn from_val(v: &emlite::Val) -> Self {
        ScrollBehavior { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ScrollBehavior {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ScrollBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScrollBehavior> for emlite::Val {
    fn from(s: ScrollBehavior) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScrollBehavior {
    pub const AUTO: &str = "auto";
    pub const INSTANT: &str = "instant";
    pub const SMOOTH: &str = "smooth";
}

#[derive(Clone, Debug)]
pub struct ScrollLogicalPosition {
    inner: emlite::Val,
}
impl FromVal for ScrollLogicalPosition {
    fn from_val(v: &emlite::Val) -> Self {
        ScrollLogicalPosition { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ScrollLogicalPosition {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ScrollLogicalPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScrollLogicalPosition> for emlite::Val {
    fn from(s: ScrollLogicalPosition) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScrollLogicalPosition {
    pub const START: &str = "start";
    pub const CENTER: &str = "center";
    pub const END: &str = "end";
    pub const NEAREST: &str = "nearest";
}

#[derive(Clone, Debug)]
pub struct ScrollIntoViewContainer {
    inner: emlite::Val,
}
impl FromVal for ScrollIntoViewContainer {
    fn from_val(v: &emlite::Val) -> Self {
        ScrollIntoViewContainer { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ScrollIntoViewContainer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ScrollIntoViewContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScrollIntoViewContainer> for emlite::Val {
    fn from(s: ScrollIntoViewContainer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScrollIntoViewContainer {
    pub const ALL: &str = "all";
    pub const NEAREST: &str = "nearest";
}

#[derive(Clone, Debug)]
pub struct CSSBoxType {
    inner: emlite::Val,
}
impl FromVal for CSSBoxType {
    fn from_val(v: &emlite::Val) -> Self {
        CSSBoxType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSBoxType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSBoxType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSBoxType> for emlite::Val {
    fn from(s: CSSBoxType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSBoxType {
    pub const MARGIN: &str = "margin";
    pub const BORDER: &str = "border";
    pub const PADDING: &str = "padding";
    pub const CONTENT: &str = "content";
}

#[derive(Clone, Debug)]
pub struct DevicePostureType {
    inner: emlite::Val,
}
impl FromVal for DevicePostureType {
    fn from_val(v: &emlite::Val) -> Self {
        DevicePostureType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DevicePostureType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DevicePostureType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DevicePostureType> for emlite::Val {
    fn from(s: DevicePostureType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DevicePostureType {
    pub const CONTINUOUS: &str = "continuous";
    pub const FOLDED: &str = "folded";
}

#[derive(Clone, Debug)]
pub struct ItemType {
    inner: emlite::Val,
}
impl FromVal for ItemType {
    fn from_val(v: &emlite::Val) -> Self {
        ItemType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ItemType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ItemType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ItemType> for emlite::Val {
    fn from(s: ItemType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ItemType {
    pub const PRODUCT: &str = "product";
    pub const SUBSCRIPTION: &str = "subscription";
}

#[derive(Clone, Debug)]
pub struct ShadowRootMode {
    inner: emlite::Val,
}
impl FromVal for ShadowRootMode {
    fn from_val(v: &emlite::Val) -> Self {
        ShadowRootMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ShadowRootMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ShadowRootMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ShadowRootMode> for emlite::Val {
    fn from(s: ShadowRootMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ShadowRootMode {
    pub const OPEN: &str = "open";
    pub const CLOSED: &str = "closed";
}

#[derive(Clone, Debug)]
pub struct SlotAssignmentMode {
    inner: emlite::Val,
}
impl FromVal for SlotAssignmentMode {
    fn from_val(v: &emlite::Val) -> Self {
        SlotAssignmentMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SlotAssignmentMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SlotAssignmentMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SlotAssignmentMode> for emlite::Val {
    fn from(s: SlotAssignmentMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SlotAssignmentMode {
    pub const MANUAL: &str = "manual";
    pub const NAMED: &str = "named";
}

#[derive(Clone, Debug)]
pub struct UnderlineStyle {
    inner: emlite::Val,
}
impl FromVal for UnderlineStyle {
    fn from_val(v: &emlite::Val) -> Self {
        UnderlineStyle { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for UnderlineStyle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for UnderlineStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<UnderlineStyle> for emlite::Val {
    fn from(s: UnderlineStyle) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl UnderlineStyle {
    pub const NONE: &str = "none";
    pub const SOLID: &str = "solid";
    pub const DOTTED: &str = "dotted";
    pub const DASHED: &str = "dashed";
    pub const WAVY: &str = "wavy";
}

#[derive(Clone, Debug)]
pub struct UnderlineThickness {
    inner: emlite::Val,
}
impl FromVal for UnderlineThickness {
    fn from_val(v: &emlite::Val) -> Self {
        UnderlineThickness { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for UnderlineThickness {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for UnderlineThickness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<UnderlineThickness> for emlite::Val {
    fn from(s: UnderlineThickness) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl UnderlineThickness {
    pub const NONE: &str = "none";
    pub const THIN: &str = "thin";
    pub const THICK: &str = "thick";
}

#[derive(Clone, Debug)]
pub struct MediaKeysRequirement {
    inner: emlite::Val,
}
impl FromVal for MediaKeysRequirement {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeysRequirement { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaKeysRequirement {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaKeysRequirement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaKeysRequirement> for emlite::Val {
    fn from(s: MediaKeysRequirement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaKeysRequirement {
    pub const REQUIRED: &str = "required";
    pub const OPTIONAL: &str = "optional";
    pub const NOT_ALLOWED: &str = "not-allowed";
}

#[derive(Clone, Debug)]
pub struct MediaKeySessionType {
    inner: emlite::Val,
}
impl FromVal for MediaKeySessionType {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeySessionType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaKeySessionType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaKeySessionType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaKeySessionType> for emlite::Val {
    fn from(s: MediaKeySessionType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaKeySessionType {
    pub const TEMPORARY: &str = "temporary";
    pub const PERSISTENT_LICENSE: &str = "persistent-license";
}

#[derive(Clone, Debug)]
pub struct MediaKeySessionClosedReason {
    inner: emlite::Val,
}
impl FromVal for MediaKeySessionClosedReason {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeySessionClosedReason { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaKeySessionClosedReason {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaKeySessionClosedReason {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaKeySessionClosedReason> for emlite::Val {
    fn from(s: MediaKeySessionClosedReason) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaKeySessionClosedReason {
    pub const INTERNAL_ERROR: &str = "internal-error";
    pub const CLOSED_BY_APPLICATION: &str = "closed-by-application";
    pub const RELEASE_ACKNOWLEDGED: &str = "release-acknowledged";
    pub const HARDWARE_CONTEXT_RESET: &str = "hardware-context-reset";
    pub const RESOURCE_EVICTED: &str = "resource-evicted";
}

#[derive(Clone, Debug)]
pub struct MediaKeyStatus {
    inner: emlite::Val,
}
impl FromVal for MediaKeyStatus {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeyStatus { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaKeyStatus {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaKeyStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaKeyStatus> for emlite::Val {
    fn from(s: MediaKeyStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaKeyStatus {
    pub const USABLE: &str = "usable";
    pub const EXPIRED: &str = "expired";
    pub const RELEASED: &str = "released";
    pub const OUTPUT_RESTRICTED: &str = "output-restricted";
    pub const OUTPUT_DOWNSCALED: &str = "output-downscaled";
    pub const USABLE_IN_FUTURE: &str = "usable-in-future";
    pub const STATUS_PENDING: &str = "status-pending";
    pub const INTERNAL_ERROR: &str = "internal-error";
}

#[derive(Clone, Debug)]
pub struct MediaKeyMessageType {
    inner: emlite::Val,
}
impl FromVal for MediaKeyMessageType {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeyMessageType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaKeyMessageType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaKeyMessageType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaKeyMessageType> for emlite::Val {
    fn from(s: MediaKeyMessageType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaKeyMessageType {
    pub const LICENSE_REQUEST: &str = "license-request";
    pub const LICENSE_RENEWAL: &str = "license-renewal";
    pub const LICENSE_RELEASE: &str = "license-release";
    pub const INDIVIDUALIZATION_REQUEST: &str = "individualization-request";
}

#[derive(Clone, Debug)]
pub struct IdentityCredentialRequestOptionsContext {
    inner: emlite::Val,
}
impl FromVal for IdentityCredentialRequestOptionsContext {
    fn from_val(v: &emlite::Val) -> Self {
        IdentityCredentialRequestOptionsContext { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IdentityCredentialRequestOptionsContext {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IdentityCredentialRequestOptionsContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IdentityCredentialRequestOptionsContext> for emlite::Val {
    fn from(s: IdentityCredentialRequestOptionsContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdentityCredentialRequestOptionsContext {
    pub const SIGNIN: &str = "signin";
    pub const SIGNUP: &str = "signup";
    pub const USE_: &str = "use";
    pub const CONTINUE_: &str = "continue";
}

#[derive(Clone, Debug)]
pub struct IdentityCredentialRequestOptionsMode {
    inner: emlite::Val,
}
impl FromVal for IdentityCredentialRequestOptionsMode {
    fn from_val(v: &emlite::Val) -> Self {
        IdentityCredentialRequestOptionsMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IdentityCredentialRequestOptionsMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IdentityCredentialRequestOptionsMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IdentityCredentialRequestOptionsMode> for emlite::Val {
    fn from(s: IdentityCredentialRequestOptionsMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdentityCredentialRequestOptionsMode {
    pub const ACTIVE: &str = "active";
    pub const PASSIVE: &str = "passive";
}

#[derive(Clone, Debug)]
pub struct OpaqueProperty {
    inner: emlite::Val,
}
impl FromVal for OpaqueProperty {
    fn from_val(v: &emlite::Val) -> Self {
        OpaqueProperty { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OpaqueProperty {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OpaqueProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OpaqueProperty> for emlite::Val {
    fn from(s: OpaqueProperty) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OpaqueProperty {
    pub const OPAQUE: &str = "opaque";
}

#[derive(Clone, Debug)]
pub struct FenceReportingDestination {
    inner: emlite::Val,
}
impl FromVal for FenceReportingDestination {
    fn from_val(v: &emlite::Val) -> Self {
        FenceReportingDestination { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FenceReportingDestination {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FenceReportingDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FenceReportingDestination> for emlite::Val {
    fn from(s: FenceReportingDestination) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FenceReportingDestination {
    pub const BUYER: &str = "buyer";
    pub const SELLER: &str = "seller";
    pub const COMPONENT_SELLER: &str = "component-seller";
    pub const DIRECT_SELLER: &str = "direct-seller";
    pub const SHARED_STORAGE_SELECT_URL: &str = "shared-storage-select-url";
}

#[derive(Clone, Debug)]
pub struct RequestDestination {
    inner: emlite::Val,
}
impl FromVal for RequestDestination {
    fn from_val(v: &emlite::Val) -> Self {
        RequestDestination { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RequestDestination {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RequestDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RequestDestination> for emlite::Val {
    fn from(s: RequestDestination) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RequestDestination {
    pub const NONE: &str = "";
    pub const AUDIO: &str = "audio";
    pub const AUDIOWORKLET: &str = "audioworklet";
    pub const DOCUMENT: &str = "document";
    pub const EMBED: &str = "embed";
    pub const FONT: &str = "font";
    pub const FRAME: &str = "frame";
    pub const IFRAME: &str = "iframe";
    pub const IMAGE: &str = "image";
    pub const JSON: &str = "json";
    pub const MANIFEST: &str = "manifest";
    pub const OBJECT: &str = "object";
    pub const PAINTWORKLET: &str = "paintworklet";
    pub const REPORT: &str = "report";
    pub const SCRIPT: &str = "script";
    pub const SHAREDWORKER: &str = "sharedworker";
    pub const STYLE: &str = "style";
    pub const TRACK: &str = "track";
    pub const VIDEO: &str = "video";
    pub const WORKER: &str = "worker";
    pub const XSLT: &str = "xslt";
}

#[derive(Clone, Debug)]
pub struct RequestMode {
    inner: emlite::Val,
}
impl FromVal for RequestMode {
    fn from_val(v: &emlite::Val) -> Self {
        RequestMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RequestMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RequestMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RequestMode> for emlite::Val {
    fn from(s: RequestMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RequestMode {
    pub const NAVIGATE: &str = "navigate";
    pub const SAME_ORIGIN: &str = "same-origin";
    pub const NO_CORS: &str = "no-cors";
    pub const CORS: &str = "cors";
}

#[derive(Clone, Debug)]
pub struct RequestCredentials {
    inner: emlite::Val,
}
impl FromVal for RequestCredentials {
    fn from_val(v: &emlite::Val) -> Self {
        RequestCredentials { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RequestCredentials {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RequestCredentials {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RequestCredentials> for emlite::Val {
    fn from(s: RequestCredentials) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RequestCredentials {
    pub const OMIT: &str = "omit";
    pub const SAME_ORIGIN: &str = "same-origin";
    pub const INCLUDE: &str = "include";
}

#[derive(Clone, Debug)]
pub struct RequestCache {
    inner: emlite::Val,
}
impl FromVal for RequestCache {
    fn from_val(v: &emlite::Val) -> Self {
        RequestCache { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RequestCache {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RequestCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RequestCache> for emlite::Val {
    fn from(s: RequestCache) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RequestCache {
    pub const DEFAULT: &str = "default";
    pub const NO_STORE: &str = "no-store";
    pub const RELOAD: &str = "reload";
    pub const NO_CACHE: &str = "no-cache";
    pub const FORCE_CACHE: &str = "force-cache";
    pub const ONLY_IF_CACHED: &str = "only-if-cached";
}

#[derive(Clone, Debug)]
pub struct RequestRedirect {
    inner: emlite::Val,
}
impl FromVal for RequestRedirect {
    fn from_val(v: &emlite::Val) -> Self {
        RequestRedirect { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RequestRedirect {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RequestRedirect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RequestRedirect> for emlite::Val {
    fn from(s: RequestRedirect) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RequestRedirect {
    pub const FOLLOW: &str = "follow";
    pub const ERROR: &str = "error";
    pub const MANUAL: &str = "manual";
}

#[derive(Clone, Debug)]
pub struct RequestDuplex {
    inner: emlite::Val,
}
impl FromVal for RequestDuplex {
    fn from_val(v: &emlite::Val) -> Self {
        RequestDuplex { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RequestDuplex {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RequestDuplex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RequestDuplex> for emlite::Val {
    fn from(s: RequestDuplex) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RequestDuplex {
    pub const HALF: &str = "half";
}

#[derive(Clone, Debug)]
pub struct RequestPriority {
    inner: emlite::Val,
}
impl FromVal for RequestPriority {
    fn from_val(v: &emlite::Val) -> Self {
        RequestPriority { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RequestPriority {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RequestPriority {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RequestPriority> for emlite::Val {
    fn from(s: RequestPriority) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RequestPriority {
    pub const HIGH: &str = "high";
    pub const LOW: &str = "low";
    pub const AUTO: &str = "auto";
}

#[derive(Clone, Debug)]
pub struct ResponseType {
    inner: emlite::Val,
}
impl FromVal for ResponseType {
    fn from_val(v: &emlite::Val) -> Self {
        ResponseType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ResponseType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ResponseType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ResponseType> for emlite::Val {
    fn from(s: ResponseType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ResponseType {
    pub const BASIC: &str = "basic";
    pub const CORS: &str = "cors";
    pub const DEFAULT: &str = "default";
    pub const ERROR: &str = "error";
    pub const OPAQUE: &str = "opaque";
    pub const OPAQUEREDIRECT: &str = "opaqueredirect";
}

#[derive(Clone, Debug)]
pub struct FileSystemPermissionMode {
    inner: emlite::Val,
}
impl FromVal for FileSystemPermissionMode {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemPermissionMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FileSystemPermissionMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FileSystemPermissionMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystemPermissionMode> for emlite::Val {
    fn from(s: FileSystemPermissionMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemPermissionMode {
    pub const READ: &str = "read";
    pub const READWRITE: &str = "readwrite";
}

#[derive(Clone, Debug)]
pub struct WellKnownDirectory {
    inner: emlite::Val,
}
impl FromVal for WellKnownDirectory {
    fn from_val(v: &emlite::Val) -> Self {
        WellKnownDirectory { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WellKnownDirectory {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WellKnownDirectory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WellKnownDirectory> for emlite::Val {
    fn from(s: WellKnownDirectory) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WellKnownDirectory {
    pub const DESKTOP: &str = "desktop";
    pub const DOCUMENTS: &str = "documents";
    pub const DOWNLOADS: &str = "downloads";
    pub const MUSIC: &str = "music";
    pub const PICTURES: &str = "pictures";
    pub const VIDEOS: &str = "videos";
}

#[derive(Clone, Debug)]
pub struct FileSystemHandleKind {
    inner: emlite::Val,
}
impl FromVal for FileSystemHandleKind {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemHandleKind { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FileSystemHandleKind {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FileSystemHandleKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystemHandleKind> for emlite::Val {
    fn from(s: FileSystemHandleKind) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemHandleKind {
    pub const FILE: &str = "file";
    pub const DIRECTORY: &str = "directory";
}

#[derive(Clone, Debug)]
pub struct WriteCommandType {
    inner: emlite::Val,
}
impl FromVal for WriteCommandType {
    fn from_val(v: &emlite::Val) -> Self {
        WriteCommandType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WriteCommandType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WriteCommandType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WriteCommandType> for emlite::Val {
    fn from(s: WriteCommandType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WriteCommandType {
    pub const WRITE: &str = "write";
    pub const SEEK: &str = "seek";
    pub const TRUNCATE: &str = "truncate";
}

#[derive(Clone, Debug)]
pub struct FullscreenNavigationUI {
    inner: emlite::Val,
}
impl FromVal for FullscreenNavigationUI {
    fn from_val(v: &emlite::Val) -> Self {
        FullscreenNavigationUI { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FullscreenNavigationUI {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FullscreenNavigationUI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FullscreenNavigationUI> for emlite::Val {
    fn from(s: FullscreenNavigationUI) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FullscreenNavigationUI {
    pub const AUTO: &str = "auto";
    pub const SHOW: &str = "show";
    pub const HIDE: &str = "hide";
}

#[derive(Clone, Debug)]
pub struct GamepadHand {
    inner: emlite::Val,
}
impl FromVal for GamepadHand {
    fn from_val(v: &emlite::Val) -> Self {
        GamepadHand { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GamepadHand {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GamepadHand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GamepadHand> for emlite::Val {
    fn from(s: GamepadHand) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GamepadHand {
    pub const NONE: &str = "";
    pub const LEFT: &str = "left";
    pub const RIGHT: &str = "right";
}

#[derive(Clone, Debug)]
pub struct GamepadMappingType {
    inner: emlite::Val,
}
impl FromVal for GamepadMappingType {
    fn from_val(v: &emlite::Val) -> Self {
        GamepadMappingType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GamepadMappingType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GamepadMappingType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GamepadMappingType> for emlite::Val {
    fn from(s: GamepadMappingType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GamepadMappingType {
    pub const NONE: &str = "";
    pub const STANDARD: &str = "standard";
    pub const XR_STANDARD: &str = "xr-standard";
}

#[derive(Clone, Debug)]
pub struct GamepadHapticsResult {
    inner: emlite::Val,
}
impl FromVal for GamepadHapticsResult {
    fn from_val(v: &emlite::Val) -> Self {
        GamepadHapticsResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GamepadHapticsResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GamepadHapticsResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GamepadHapticsResult> for emlite::Val {
    fn from(s: GamepadHapticsResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GamepadHapticsResult {
    pub const COMPLETE: &str = "complete";
    pub const PREEMPTED: &str = "preempted";
}

#[derive(Clone, Debug)]
pub struct GamepadHapticEffectType {
    inner: emlite::Val,
}
impl FromVal for GamepadHapticEffectType {
    fn from_val(v: &emlite::Val) -> Self {
        GamepadHapticEffectType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GamepadHapticEffectType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GamepadHapticEffectType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GamepadHapticEffectType> for emlite::Val {
    fn from(s: GamepadHapticEffectType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GamepadHapticEffectType {
    pub const DUAL_RUMBLE: &str = "dual-rumble";
    pub const TRIGGER_RUMBLE: &str = "trigger-rumble";
}

#[derive(Clone, Debug)]
pub struct GyroscopeLocalCoordinateSystem {
    inner: emlite::Val,
}
impl FromVal for GyroscopeLocalCoordinateSystem {
    fn from_val(v: &emlite::Val) -> Self {
        GyroscopeLocalCoordinateSystem { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GyroscopeLocalCoordinateSystem {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GyroscopeLocalCoordinateSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GyroscopeLocalCoordinateSystem> for emlite::Val {
    fn from(s: GyroscopeLocalCoordinateSystem) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GyroscopeLocalCoordinateSystem {
    pub const DEVICE: &str = "device";
    pub const SCREEN: &str = "screen";
}

#[derive(Clone, Debug)]
pub struct HandwritingRecognitionType {
    inner: emlite::Val,
}
impl FromVal for HandwritingRecognitionType {
    fn from_val(v: &emlite::Val) -> Self {
        HandwritingRecognitionType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HandwritingRecognitionType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HandwritingRecognitionType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HandwritingRecognitionType> for emlite::Val {
    fn from(s: HandwritingRecognitionType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HandwritingRecognitionType {
    pub const TEXT: &str = "text";
    pub const PER_CHARACTER: &str = "per-character";
}

#[derive(Clone, Debug)]
pub struct HandwritingInputType {
    inner: emlite::Val,
}
impl FromVal for HandwritingInputType {
    fn from_val(v: &emlite::Val) -> Self {
        HandwritingInputType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HandwritingInputType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HandwritingInputType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HandwritingInputType> for emlite::Val {
    fn from(s: HandwritingInputType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HandwritingInputType {
    pub const MOUSE: &str = "mouse";
    pub const STYLUS: &str = "stylus";
    pub const TOUCH: &str = "touch";
}

#[derive(Clone, Debug)]
pub struct DocumentReadyState {
    inner: emlite::Val,
}
impl FromVal for DocumentReadyState {
    fn from_val(v: &emlite::Val) -> Self {
        DocumentReadyState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DocumentReadyState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DocumentReadyState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DocumentReadyState> for emlite::Val {
    fn from(s: DocumentReadyState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DocumentReadyState {
    pub const LOADING: &str = "loading";
    pub const INTERACTIVE: &str = "interactive";
    pub const COMPLETE: &str = "complete";
}

#[derive(Clone, Debug)]
pub struct DocumentVisibilityState {
    inner: emlite::Val,
}
impl FromVal for DocumentVisibilityState {
    fn from_val(v: &emlite::Val) -> Self {
        DocumentVisibilityState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DocumentVisibilityState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DocumentVisibilityState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DocumentVisibilityState> for emlite::Val {
    fn from(s: DocumentVisibilityState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DocumentVisibilityState {
    pub const VISIBLE: &str = "visible";
    pub const HIDDEN: &str = "hidden";
}

#[derive(Clone, Debug)]
pub struct CanPlayTypeResult {
    inner: emlite::Val,
}
impl FromVal for CanPlayTypeResult {
    fn from_val(v: &emlite::Val) -> Self {
        CanPlayTypeResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanPlayTypeResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanPlayTypeResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanPlayTypeResult> for emlite::Val {
    fn from(s: CanPlayTypeResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanPlayTypeResult {
    pub const NONE: &str = "";
    pub const MAYBE: &str = "maybe";
    pub const PROBABLY: &str = "probably";
}

#[derive(Clone, Debug)]
pub struct TextTrackMode {
    inner: emlite::Val,
}
impl FromVal for TextTrackMode {
    fn from_val(v: &emlite::Val) -> Self {
        TextTrackMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TextTrackMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TextTrackMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextTrackMode> for emlite::Val {
    fn from(s: TextTrackMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextTrackMode {
    pub const DISABLED: &str = "disabled";
    pub const HIDDEN: &str = "hidden";
    pub const SHOWING: &str = "showing";
}

#[derive(Clone, Debug)]
pub struct TextTrackKind {
    inner: emlite::Val,
}
impl FromVal for TextTrackKind {
    fn from_val(v: &emlite::Val) -> Self {
        TextTrackKind { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TextTrackKind {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TextTrackKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextTrackKind> for emlite::Val {
    fn from(s: TextTrackKind) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextTrackKind {
    pub const SUBTITLES: &str = "subtitles";
    pub const CAPTIONS: &str = "captions";
    pub const DESCRIPTIONS: &str = "descriptions";
    pub const CHAPTERS: &str = "chapters";
    pub const METADATA: &str = "metadata";
}

#[derive(Clone, Debug)]
pub struct SelectionMode {
    inner: emlite::Val,
}
impl FromVal for SelectionMode {
    fn from_val(v: &emlite::Val) -> Self {
        SelectionMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SelectionMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SelectionMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SelectionMode> for emlite::Val {
    fn from(s: SelectionMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SelectionMode {
    pub const SELECT: &str = "select";
    pub const START: &str = "start";
    pub const END: &str = "end";
    pub const PRESERVE: &str = "preserve";
}

#[derive(Clone, Debug)]
pub struct PredefinedColorSpace {
    inner: emlite::Val,
}
impl FromVal for PredefinedColorSpace {
    fn from_val(v: &emlite::Val) -> Self {
        PredefinedColorSpace { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PredefinedColorSpace {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PredefinedColorSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PredefinedColorSpace> for emlite::Val {
    fn from(s: PredefinedColorSpace) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PredefinedColorSpace {
    pub const SRGB: &str = "srgb";
    pub const DISPLAY_P3: &str = "display-p3";
}

#[derive(Clone, Debug)]
pub struct CanvasColorType {
    inner: emlite::Val,
}
impl FromVal for CanvasColorType {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasColorType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasColorType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasColorType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasColorType> for emlite::Val {
    fn from(s: CanvasColorType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasColorType {
    pub const UNORM8: &str = "unorm8";
    pub const FLOAT16: &str = "float16";
}

#[derive(Clone, Debug)]
pub struct CanvasFillRule {
    inner: emlite::Val,
}
impl FromVal for CanvasFillRule {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasFillRule { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasFillRule {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasFillRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasFillRule> for emlite::Val {
    fn from(s: CanvasFillRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasFillRule {
    pub const NONZERO: &str = "nonzero";
    pub const EVENODD: &str = "evenodd";
}

#[derive(Clone, Debug)]
pub struct ImageSmoothingQuality {
    inner: emlite::Val,
}
impl FromVal for ImageSmoothingQuality {
    fn from_val(v: &emlite::Val) -> Self {
        ImageSmoothingQuality { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ImageSmoothingQuality {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ImageSmoothingQuality {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ImageSmoothingQuality> for emlite::Val {
    fn from(s: ImageSmoothingQuality) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageSmoothingQuality {
    pub const LOW: &str = "low";
    pub const MEDIUM: &str = "medium";
    pub const HIGH: &str = "high";
}

#[derive(Clone, Debug)]
pub struct CanvasLineCap {
    inner: emlite::Val,
}
impl FromVal for CanvasLineCap {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasLineCap { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasLineCap {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasLineCap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasLineCap> for emlite::Val {
    fn from(s: CanvasLineCap) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasLineCap {
    pub const BUTT: &str = "butt";
    pub const ROUND: &str = "round";
    pub const SQUARE: &str = "square";
}

#[derive(Clone, Debug)]
pub struct CanvasLineJoin {
    inner: emlite::Val,
}
impl FromVal for CanvasLineJoin {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasLineJoin { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasLineJoin {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasLineJoin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasLineJoin> for emlite::Val {
    fn from(s: CanvasLineJoin) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasLineJoin {
    pub const ROUND: &str = "round";
    pub const BEVEL: &str = "bevel";
    pub const MITER: &str = "miter";
}

#[derive(Clone, Debug)]
pub struct CanvasTextAlign {
    inner: emlite::Val,
}
impl FromVal for CanvasTextAlign {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasTextAlign { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasTextAlign {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasTextAlign {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasTextAlign> for emlite::Val {
    fn from(s: CanvasTextAlign) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasTextAlign {
    pub const START: &str = "start";
    pub const END: &str = "end";
    pub const LEFT: &str = "left";
    pub const RIGHT: &str = "right";
    pub const CENTER: &str = "center";
}

#[derive(Clone, Debug)]
pub struct CanvasTextBaseline {
    inner: emlite::Val,
}
impl FromVal for CanvasTextBaseline {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasTextBaseline { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasTextBaseline {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasTextBaseline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasTextBaseline> for emlite::Val {
    fn from(s: CanvasTextBaseline) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasTextBaseline {
    pub const TOP: &str = "top";
    pub const HANGING: &str = "hanging";
    pub const MIDDLE: &str = "middle";
    pub const ALPHABETIC: &str = "alphabetic";
    pub const IDEOGRAPHIC: &str = "ideographic";
    pub const BOTTOM: &str = "bottom";
}

#[derive(Clone, Debug)]
pub struct CanvasDirection {
    inner: emlite::Val,
}
impl FromVal for CanvasDirection {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasDirection { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasDirection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasDirection> for emlite::Val {
    fn from(s: CanvasDirection) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasDirection {
    pub const LTR: &str = "ltr";
    pub const RTL: &str = "rtl";
    pub const INHERIT: &str = "inherit";
}

#[derive(Clone, Debug)]
pub struct CanvasFontKerning {
    inner: emlite::Val,
}
impl FromVal for CanvasFontKerning {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasFontKerning { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasFontKerning {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasFontKerning {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasFontKerning> for emlite::Val {
    fn from(s: CanvasFontKerning) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasFontKerning {
    pub const AUTO: &str = "auto";
    pub const NORMAL: &str = "normal";
    pub const NONE: &str = "none";
}

#[derive(Clone, Debug)]
pub struct CanvasFontStretch {
    inner: emlite::Val,
}
impl FromVal for CanvasFontStretch {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasFontStretch { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasFontStretch {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasFontStretch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasFontStretch> for emlite::Val {
    fn from(s: CanvasFontStretch) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasFontStretch {
    pub const ULTRA_CONDENSED: &str = "ultra-condensed";
    pub const EXTRA_CONDENSED: &str = "extra-condensed";
    pub const CONDENSED: &str = "condensed";
    pub const SEMI_CONDENSED: &str = "semi-condensed";
    pub const NORMAL: &str = "normal";
    pub const SEMI_EXPANDED: &str = "semi-expanded";
    pub const EXPANDED: &str = "expanded";
    pub const EXTRA_EXPANDED: &str = "extra-expanded";
    pub const ULTRA_EXPANDED: &str = "ultra-expanded";
}

#[derive(Clone, Debug)]
pub struct CanvasFontVariantCaps {
    inner: emlite::Val,
}
impl FromVal for CanvasFontVariantCaps {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasFontVariantCaps { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasFontVariantCaps {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasFontVariantCaps {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasFontVariantCaps> for emlite::Val {
    fn from(s: CanvasFontVariantCaps) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasFontVariantCaps {
    pub const NORMAL: &str = "normal";
    pub const SMALL_CAPS: &str = "small-caps";
    pub const ALL_SMALL_CAPS: &str = "all-small-caps";
    pub const PETITE_CAPS: &str = "petite-caps";
    pub const ALL_PETITE_CAPS: &str = "all-petite-caps";
    pub const UNICASE: &str = "unicase";
    pub const TITLING_CAPS: &str = "titling-caps";
}

#[derive(Clone, Debug)]
pub struct CanvasTextRendering {
    inner: emlite::Val,
}
impl FromVal for CanvasTextRendering {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasTextRendering { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasTextRendering {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasTextRendering {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasTextRendering> for emlite::Val {
    fn from(s: CanvasTextRendering) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasTextRendering {
    pub const AUTO: &str = "auto";
    pub const OPTIMIZE_SPEED: &str = "optimizeSpeed";
    pub const OPTIMIZE_LEGIBILITY: &str = "optimizeLegibility";
    pub const GEOMETRIC_PRECISION: &str = "geometricPrecision";
}

#[derive(Clone, Debug)]
pub struct OffscreenRenderingContextId {
    inner: emlite::Val,
}
impl FromVal for OffscreenRenderingContextId {
    fn from_val(v: &emlite::Val) -> Self {
        OffscreenRenderingContextId { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OffscreenRenderingContextId {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OffscreenRenderingContextId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OffscreenRenderingContextId> for emlite::Val {
    fn from(s: OffscreenRenderingContextId) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OffscreenRenderingContextId {
    pub const _2D: &str = "2d";
    pub const BITMAPRENDERER: &str = "bitmaprenderer";
    pub const WEBGL: &str = "webgl";
    pub const WEBGL2: &str = "webgl2";
    pub const WEBGPU: &str = "webgpu";
}

#[derive(Clone, Debug)]
pub struct ScrollRestoration {
    inner: emlite::Val,
}
impl FromVal for ScrollRestoration {
    fn from_val(v: &emlite::Val) -> Self {
        ScrollRestoration { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ScrollRestoration {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ScrollRestoration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScrollRestoration> for emlite::Val {
    fn from(s: ScrollRestoration) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScrollRestoration {
    pub const AUTO: &str = "auto";
    pub const MANUAL: &str = "manual";
}

#[derive(Clone, Debug)]
pub struct NavigationHistoryBehavior {
    inner: emlite::Val,
}
impl FromVal for NavigationHistoryBehavior {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationHistoryBehavior { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for NavigationHistoryBehavior {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NavigationHistoryBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationHistoryBehavior> for emlite::Val {
    fn from(s: NavigationHistoryBehavior) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationHistoryBehavior {
    pub const AUTO: &str = "auto";
    pub const PUSH: &str = "push";
    pub const REPLACE: &str = "replace";
}

#[derive(Clone, Debug)]
pub struct NavigationType {
    inner: emlite::Val,
}
impl FromVal for NavigationType {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for NavigationType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NavigationType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationType> for emlite::Val {
    fn from(s: NavigationType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationType {
    pub const PUSH: &str = "push";
    pub const REPLACE: &str = "replace";
    pub const RELOAD: &str = "reload";
    pub const TRAVERSE: &str = "traverse";
}

#[derive(Clone, Debug)]
pub struct NavigationFocusReset {
    inner: emlite::Val,
}
impl FromVal for NavigationFocusReset {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationFocusReset { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for NavigationFocusReset {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NavigationFocusReset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationFocusReset> for emlite::Val {
    fn from(s: NavigationFocusReset) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationFocusReset {
    pub const AFTER_TRANSITION: &str = "after-transition";
    pub const MANUAL: &str = "manual";
}

#[derive(Clone, Debug)]
pub struct NavigationScrollBehavior {
    inner: emlite::Val,
}
impl FromVal for NavigationScrollBehavior {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationScrollBehavior { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for NavigationScrollBehavior {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NavigationScrollBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationScrollBehavior> for emlite::Val {
    fn from(s: NavigationScrollBehavior) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationScrollBehavior {
    pub const AFTER_TRANSITION: &str = "after-transition";
    pub const MANUAL: &str = "manual";
}

#[derive(Clone, Debug)]
pub struct DOMParserSupportedType {
    inner: emlite::Val,
}
impl FromVal for DOMParserSupportedType {
    fn from_val(v: &emlite::Val) -> Self {
        DOMParserSupportedType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DOMParserSupportedType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DOMParserSupportedType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DOMParserSupportedType> for emlite::Val {
    fn from(s: DOMParserSupportedType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DOMParserSupportedType {
    pub const TEXT_HTML: &str = "text/html";
    pub const TEXT_XML: &str = "text/xml";
    pub const APPLICATION_XML: &str = "application/xml";
    pub const APPLICATION_XHTML_XML: &str = "application/xhtml+xml";
    pub const IMAGE_SVG_XML: &str = "image/svg+xml";
}

#[derive(Clone, Debug)]
pub struct ImageDataPixelFormat {
    inner: emlite::Val,
}
impl FromVal for ImageDataPixelFormat {
    fn from_val(v: &emlite::Val) -> Self {
        ImageDataPixelFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ImageDataPixelFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ImageDataPixelFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ImageDataPixelFormat> for emlite::Val {
    fn from(s: ImageDataPixelFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageDataPixelFormat {
    pub const RGBA_UNORM8: &str = "rgba-unorm8";
    pub const RGBA_FLOAT16: &str = "rgba-float16";
}

#[derive(Clone, Debug)]
pub struct ImageOrientation {
    inner: emlite::Val,
}
impl FromVal for ImageOrientation {
    fn from_val(v: &emlite::Val) -> Self {
        ImageOrientation { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ImageOrientation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ImageOrientation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ImageOrientation> for emlite::Val {
    fn from(s: ImageOrientation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImageOrientation {
    pub const FROM_IMAGE: &str = "from-image";
    pub const FLIP_Y: &str = "flipY";
}

#[derive(Clone, Debug)]
pub struct PremultiplyAlpha {
    inner: emlite::Val,
}
impl FromVal for PremultiplyAlpha {
    fn from_val(v: &emlite::Val) -> Self {
        PremultiplyAlpha { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PremultiplyAlpha {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PremultiplyAlpha {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PremultiplyAlpha> for emlite::Val {
    fn from(s: PremultiplyAlpha) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PremultiplyAlpha {
    pub const NONE: &str = "none";
    pub const PREMULTIPLY: &str = "premultiply";
    pub const DEFAULT: &str = "default";
}

#[derive(Clone, Debug)]
pub struct ColorSpaceConversion {
    inner: emlite::Val,
}
impl FromVal for ColorSpaceConversion {
    fn from_val(v: &emlite::Val) -> Self {
        ColorSpaceConversion { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ColorSpaceConversion {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ColorSpaceConversion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ColorSpaceConversion> for emlite::Val {
    fn from(s: ColorSpaceConversion) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ColorSpaceConversion {
    pub const NONE: &str = "none";
    pub const DEFAULT: &str = "default";
}

#[derive(Clone, Debug)]
pub struct ResizeQuality {
    inner: emlite::Val,
}
impl FromVal for ResizeQuality {
    fn from_val(v: &emlite::Val) -> Self {
        ResizeQuality { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ResizeQuality {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ResizeQuality {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ResizeQuality> for emlite::Val {
    fn from(s: ResizeQuality) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ResizeQuality {
    pub const PIXELATED: &str = "pixelated";
    pub const LOW: &str = "low";
    pub const MEDIUM: &str = "medium";
    pub const HIGH: &str = "high";
}

#[derive(Clone, Debug)]
pub struct WorkerType {
    inner: emlite::Val,
}
impl FromVal for WorkerType {
    fn from_val(v: &emlite::Val) -> Self {
        WorkerType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WorkerType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WorkerType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WorkerType> for emlite::Val {
    fn from(s: WorkerType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WorkerType {
    pub const CLASSIC: &str = "classic";
    pub const MODULE: &str = "module";
}

#[derive(Clone, Debug)]
pub struct UserIdleState {
    inner: emlite::Val,
}
impl FromVal for UserIdleState {
    fn from_val(v: &emlite::Val) -> Self {
        UserIdleState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for UserIdleState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for UserIdleState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<UserIdleState> for emlite::Val {
    fn from(s: UserIdleState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl UserIdleState {
    pub const ACTIVE: &str = "active";
    pub const IDLE: &str = "idle";
}

#[derive(Clone, Debug)]
pub struct ScreenIdleState {
    inner: emlite::Val,
}
impl FromVal for ScreenIdleState {
    fn from_val(v: &emlite::Val) -> Self {
        ScreenIdleState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ScreenIdleState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ScreenIdleState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScreenIdleState> for emlite::Val {
    fn from(s: ScreenIdleState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScreenIdleState {
    pub const LOCKED: &str = "locked";
    pub const UNLOCKED: &str = "unlocked";
}

#[derive(Clone, Debug)]
pub struct RedEyeReduction {
    inner: emlite::Val,
}
impl FromVal for RedEyeReduction {
    fn from_val(v: &emlite::Val) -> Self {
        RedEyeReduction { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RedEyeReduction {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RedEyeReduction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RedEyeReduction> for emlite::Val {
    fn from(s: RedEyeReduction) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RedEyeReduction {
    pub const NEVER: &str = "never";
    pub const ALWAYS: &str = "always";
    pub const CONTROLLABLE: &str = "controllable";
}

#[derive(Clone, Debug)]
pub struct FillLightMode {
    inner: emlite::Val,
}
impl FromVal for FillLightMode {
    fn from_val(v: &emlite::Val) -> Self {
        FillLightMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FillLightMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FillLightMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FillLightMode> for emlite::Val {
    fn from(s: FillLightMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FillLightMode {
    pub const AUTO: &str = "auto";
    pub const OFF: &str = "off";
    pub const FLASH: &str = "flash";
}

#[derive(Clone, Debug)]
pub struct MeteringMode {
    inner: emlite::Val,
}
impl FromVal for MeteringMode {
    fn from_val(v: &emlite::Val) -> Self {
        MeteringMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MeteringMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MeteringMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MeteringMode> for emlite::Val {
    fn from(s: MeteringMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MeteringMode {
    pub const NONE: &str = "none";
    pub const MANUAL: &str = "manual";
    pub const SINGLE_SHOT: &str = "single-shot";
    pub const CONTINUOUS: &str = "continuous";
}

#[derive(Clone, Debug)]
pub struct LoginStatus {
    inner: emlite::Val,
}
impl FromVal for LoginStatus {
    fn from_val(v: &emlite::Val) -> Self {
        LoginStatus { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LoginStatus {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LoginStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LoginStatus> for emlite::Val {
    fn from(s: LoginStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LoginStatus {
    pub const LOGGED_IN: &str = "logged-in";
    pub const LOGGED_OUT: &str = "logged-out";
}

#[derive(Clone, Debug)]
pub struct ScriptInvokerType {
    inner: emlite::Val,
}
impl FromVal for ScriptInvokerType {
    fn from_val(v: &emlite::Val) -> Self {
        ScriptInvokerType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ScriptInvokerType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ScriptInvokerType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScriptInvokerType> for emlite::Val {
    fn from(s: ScriptInvokerType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScriptInvokerType {
    pub const CLASSIC_SCRIPT: &str = "classic-script";
    pub const MODULE_SCRIPT: &str = "module-script";
    pub const EVENT_LISTENER: &str = "event-listener";
    pub const USER_CALLBACK: &str = "user-callback";
    pub const RESOLVE_PROMISE: &str = "resolve-promise";
    pub const REJECT_PROMISE: &str = "reject-promise";
}

#[derive(Clone, Debug)]
pub struct ScriptWindowAttribution {
    inner: emlite::Val,
}
impl FromVal for ScriptWindowAttribution {
    fn from_val(v: &emlite::Val) -> Self {
        ScriptWindowAttribution { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ScriptWindowAttribution {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ScriptWindowAttribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScriptWindowAttribution> for emlite::Val {
    fn from(s: ScriptWindowAttribution) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScriptWindowAttribution {
    pub const SELF_: &str = "self";
    pub const DESCENDANT: &str = "descendant";
    pub const ANCESTOR: &str = "ancestor";
    pub const SAME_PAGE: &str = "same-page";
    pub const OTHER: &str = "other";
}

#[derive(Clone, Debug)]
pub struct MagnetometerLocalCoordinateSystem {
    inner: emlite::Val,
}
impl FromVal for MagnetometerLocalCoordinateSystem {
    fn from_val(v: &emlite::Val) -> Self {
        MagnetometerLocalCoordinateSystem { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MagnetometerLocalCoordinateSystem {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MagnetometerLocalCoordinateSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MagnetometerLocalCoordinateSystem> for emlite::Val {
    fn from(s: MagnetometerLocalCoordinateSystem) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MagnetometerLocalCoordinateSystem {
    pub const DEVICE: &str = "device";
    pub const SCREEN: &str = "screen";
}

#[derive(Clone, Debug)]
pub struct AppBannerPromptOutcome {
    inner: emlite::Val,
}
impl FromVal for AppBannerPromptOutcome {
    fn from_val(v: &emlite::Val) -> Self {
        AppBannerPromptOutcome { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AppBannerPromptOutcome {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AppBannerPromptOutcome {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AppBannerPromptOutcome> for emlite::Val {
    fn from(s: AppBannerPromptOutcome) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AppBannerPromptOutcome {
    pub const ACCEPTED: &str = "accepted";
    pub const DISMISSED: &str = "dismissed";
}

#[derive(Clone, Debug)]
pub struct MediaDecodingType {
    inner: emlite::Val,
}
impl FromVal for MediaDecodingType {
    fn from_val(v: &emlite::Val) -> Self {
        MediaDecodingType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaDecodingType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaDecodingType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaDecodingType> for emlite::Val {
    fn from(s: MediaDecodingType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaDecodingType {
    pub const FILE: &str = "file";
    pub const MEDIA_SOURCE: &str = "media-source";
    pub const WEBRTC: &str = "webrtc";
}

#[derive(Clone, Debug)]
pub struct MediaEncodingType {
    inner: emlite::Val,
}
impl FromVal for MediaEncodingType {
    fn from_val(v: &emlite::Val) -> Self {
        MediaEncodingType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaEncodingType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaEncodingType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaEncodingType> for emlite::Val {
    fn from(s: MediaEncodingType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaEncodingType {
    pub const RECORD: &str = "record";
    pub const WEBRTC: &str = "webrtc";
}

#[derive(Clone, Debug)]
pub struct HdrMetadataType {
    inner: emlite::Val,
}
impl FromVal for HdrMetadataType {
    fn from_val(v: &emlite::Val) -> Self {
        HdrMetadataType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HdrMetadataType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HdrMetadataType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HdrMetadataType> for emlite::Val {
    fn from(s: HdrMetadataType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HdrMetadataType {
    pub const SMPTE_ST2086: &str = "smpteSt2086";
    pub const SMPTE_ST2094_10: &str = "smpteSt2094-10";
    pub const SMPTE_ST2094_40: &str = "smpteSt2094-40";
}

#[derive(Clone, Debug)]
pub struct ColorGamut {
    inner: emlite::Val,
}
impl FromVal for ColorGamut {
    fn from_val(v: &emlite::Val) -> Self {
        ColorGamut { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ColorGamut {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ColorGamut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ColorGamut> for emlite::Val {
    fn from(s: ColorGamut) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ColorGamut {
    pub const SRGB: &str = "srgb";
    pub const P3: &str = "p3";
    pub const REC2020: &str = "rec2020";
}

#[derive(Clone, Debug)]
pub struct TransferFunction {
    inner: emlite::Val,
}
impl FromVal for TransferFunction {
    fn from_val(v: &emlite::Val) -> Self {
        TransferFunction { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TransferFunction {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TransferFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TransferFunction> for emlite::Val {
    fn from(s: TransferFunction) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TransferFunction {
    pub const SRGB: &str = "srgb";
    pub const PQ: &str = "pq";
    pub const HLG: &str = "hlg";
}

#[derive(Clone, Debug)]
pub struct ReadyState {
    inner: emlite::Val,
}
impl FromVal for ReadyState {
    fn from_val(v: &emlite::Val) -> Self {
        ReadyState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ReadyState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ReadyState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ReadyState> for emlite::Val {
    fn from(s: ReadyState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ReadyState {
    pub const CLOSED: &str = "closed";
    pub const OPEN: &str = "open";
    pub const ENDED: &str = "ended";
}

#[derive(Clone, Debug)]
pub struct EndOfStreamError {
    inner: emlite::Val,
}
impl FromVal for EndOfStreamError {
    fn from_val(v: &emlite::Val) -> Self {
        EndOfStreamError { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for EndOfStreamError {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EndOfStreamError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EndOfStreamError> for emlite::Val {
    fn from(s: EndOfStreamError) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EndOfStreamError {
    pub const NETWORK: &str = "network";
    pub const DECODE: &str = "decode";
}

#[derive(Clone, Debug)]
pub struct AppendMode {
    inner: emlite::Val,
}
impl FromVal for AppendMode {
    fn from_val(v: &emlite::Val) -> Self {
        AppendMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AppendMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AppendMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AppendMode> for emlite::Val {
    fn from(s: AppendMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AppendMode {
    pub const SEGMENTS: &str = "segments";
    pub const SEQUENCE: &str = "sequence";
}

#[derive(Clone, Debug)]
pub struct MockCapturePromptResult {
    inner: emlite::Val,
}
impl FromVal for MockCapturePromptResult {
    fn from_val(v: &emlite::Val) -> Self {
        MockCapturePromptResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MockCapturePromptResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MockCapturePromptResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MockCapturePromptResult> for emlite::Val {
    fn from(s: MockCapturePromptResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MockCapturePromptResult {
    pub const GRANTED: &str = "granted";
    pub const DENIED: &str = "denied";
}

#[derive(Clone, Debug)]
pub struct CaptureAction {
    inner: emlite::Val,
}
impl FromVal for CaptureAction {
    fn from_val(v: &emlite::Val) -> Self {
        CaptureAction { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CaptureAction {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CaptureAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CaptureAction> for emlite::Val {
    fn from(s: CaptureAction) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CaptureAction {
    pub const NEXT: &str = "next";
    pub const PREVIOUS: &str = "previous";
    pub const FIRST: &str = "first";
    pub const LAST: &str = "last";
}

#[derive(Clone, Debug)]
pub struct MediaStreamTrackState {
    inner: emlite::Val,
}
impl FromVal for MediaStreamTrackState {
    fn from_val(v: &emlite::Val) -> Self {
        MediaStreamTrackState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaStreamTrackState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaStreamTrackState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaStreamTrackState> for emlite::Val {
    fn from(s: MediaStreamTrackState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaStreamTrackState {
    pub const LIVE: &str = "live";
    pub const ENDED: &str = "ended";
}

#[derive(Clone, Debug)]
pub struct VideoFacingModeEnum {
    inner: emlite::Val,
}
impl FromVal for VideoFacingModeEnum {
    fn from_val(v: &emlite::Val) -> Self {
        VideoFacingModeEnum { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for VideoFacingModeEnum {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoFacingModeEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoFacingModeEnum> for emlite::Val {
    fn from(s: VideoFacingModeEnum) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoFacingModeEnum {
    pub const USER: &str = "user";
    pub const ENVIRONMENT: &str = "environment";
    pub const LEFT: &str = "left";
    pub const RIGHT: &str = "right";
}

#[derive(Clone, Debug)]
pub struct VideoResizeModeEnum {
    inner: emlite::Val,
}
impl FromVal for VideoResizeModeEnum {
    fn from_val(v: &emlite::Val) -> Self {
        VideoResizeModeEnum { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for VideoResizeModeEnum {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoResizeModeEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoResizeModeEnum> for emlite::Val {
    fn from(s: VideoResizeModeEnum) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoResizeModeEnum {
    pub const NONE: &str = "none";
    pub const CROP_AND_SCALE: &str = "crop-and-scale";
}

#[derive(Clone, Debug)]
pub struct EchoCancellationModeEnum {
    inner: emlite::Val,
}
impl FromVal for EchoCancellationModeEnum {
    fn from_val(v: &emlite::Val) -> Self {
        EchoCancellationModeEnum { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for EchoCancellationModeEnum {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EchoCancellationModeEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EchoCancellationModeEnum> for emlite::Val {
    fn from(s: EchoCancellationModeEnum) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EchoCancellationModeEnum {
    pub const ALL: &str = "all";
    pub const REMOTE_ONLY: &str = "remote-only";
}

#[derive(Clone, Debug)]
pub struct MediaDeviceKind {
    inner: emlite::Val,
}
impl FromVal for MediaDeviceKind {
    fn from_val(v: &emlite::Val) -> Self {
        MediaDeviceKind { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaDeviceKind {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaDeviceKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaDeviceKind> for emlite::Val {
    fn from(s: MediaDeviceKind) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaDeviceKind {
    pub const AUDIOINPUT: &str = "audioinput";
    pub const AUDIOOUTPUT: &str = "audiooutput";
    pub const VIDEOINPUT: &str = "videoinput";
}

#[derive(Clone, Debug)]
pub struct MediaSessionPlaybackState {
    inner: emlite::Val,
}
impl FromVal for MediaSessionPlaybackState {
    fn from_val(v: &emlite::Val) -> Self {
        MediaSessionPlaybackState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaSessionPlaybackState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaSessionPlaybackState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaSessionPlaybackState> for emlite::Val {
    fn from(s: MediaSessionPlaybackState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaSessionPlaybackState {
    pub const NONE: &str = "none";
    pub const PAUSED: &str = "paused";
    pub const PLAYING: &str = "playing";
}

#[derive(Clone, Debug)]
pub struct MediaSessionAction {
    inner: emlite::Val,
}
impl FromVal for MediaSessionAction {
    fn from_val(v: &emlite::Val) -> Self {
        MediaSessionAction { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaSessionAction {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaSessionAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaSessionAction> for emlite::Val {
    fn from(s: MediaSessionAction) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaSessionAction {
    pub const PLAY: &str = "play";
    pub const PAUSE: &str = "pause";
    pub const SEEKBACKWARD: &str = "seekbackward";
    pub const SEEKFORWARD: &str = "seekforward";
    pub const PREVIOUSTRACK: &str = "previoustrack";
    pub const NEXTTRACK: &str = "nexttrack";
    pub const SKIPAD: &str = "skipad";
    pub const STOP: &str = "stop";
    pub const SEEKTO: &str = "seekto";
    pub const TOGGLEMICROPHONE: &str = "togglemicrophone";
    pub const TOGGLECAMERA: &str = "togglecamera";
    pub const TOGGLESCREENSHARE: &str = "togglescreenshare";
    pub const HANGUP: &str = "hangup";
    pub const PREVIOUSSLIDE: &str = "previousslide";
    pub const NEXTSLIDE: &str = "nextslide";
    pub const ENTERPICTUREINPICTURE: &str = "enterpictureinpicture";
    pub const VOICEACTIVITY: &str = "voiceactivity";
}

#[derive(Clone, Debug)]
pub struct BitrateMode {
    inner: emlite::Val,
}
impl FromVal for BitrateMode {
    fn from_val(v: &emlite::Val) -> Self {
        BitrateMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BitrateMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BitrateMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BitrateMode> for emlite::Val {
    fn from(s: BitrateMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BitrateMode {
    pub const CONSTANT: &str = "constant";
    pub const VARIABLE: &str = "variable";
}

#[derive(Clone, Debug)]
pub struct RecordingState {
    inner: emlite::Val,
}
impl FromVal for RecordingState {
    fn from_val(v: &emlite::Val) -> Self {
        RecordingState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RecordingState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RecordingState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RecordingState> for emlite::Val {
    fn from(s: RecordingState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RecordingState {
    pub const INACTIVE: &str = "inactive";
    pub const RECORDING: &str = "recording";
    pub const PAUSED: &str = "paused";
}

#[derive(Clone, Debug)]
pub struct RTCDegradationPreference {
    inner: emlite::Val,
}
impl FromVal for RTCDegradationPreference {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDegradationPreference { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCDegradationPreference {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCDegradationPreference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCDegradationPreference> for emlite::Val {
    fn from(s: RTCDegradationPreference) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCDegradationPreference {
    pub const MAINTAIN_FRAMERATE: &str = "maintain-framerate";
    pub const MAINTAIN_RESOLUTION: &str = "maintain-resolution";
    pub const BALANCED: &str = "balanced";
}

#[derive(Clone, Debug)]
pub struct NavigationTimingType {
    inner: emlite::Val,
}
impl FromVal for NavigationTimingType {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationTimingType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for NavigationTimingType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NavigationTimingType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationTimingType> for emlite::Val {
    fn from(s: NavigationTimingType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationTimingType {
    pub const NAVIGATE: &str = "navigate";
    pub const RELOAD: &str = "reload";
    pub const BACK_FORWARD: &str = "back_forward";
    pub const PRERENDER: &str = "prerender";
}

#[derive(Clone, Debug)]
pub struct ConnectionType {
    inner: emlite::Val,
}
impl FromVal for ConnectionType {
    fn from_val(v: &emlite::Val) -> Self {
        ConnectionType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ConnectionType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ConnectionType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ConnectionType> for emlite::Val {
    fn from(s: ConnectionType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ConnectionType {
    pub const BLUETOOTH: &str = "bluetooth";
    pub const CELLULAR: &str = "cellular";
    pub const ETHERNET: &str = "ethernet";
    pub const MIXED: &str = "mixed";
    pub const NONE: &str = "none";
    pub const OTHER: &str = "other";
    pub const UNKNOWN: &str = "unknown";
    pub const WIFI: &str = "wifi";
    pub const WIMAX: &str = "wimax";
}

#[derive(Clone, Debug)]
pub struct EffectiveConnectionType {
    inner: emlite::Val,
}
impl FromVal for EffectiveConnectionType {
    fn from_val(v: &emlite::Val) -> Self {
        EffectiveConnectionType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for EffectiveConnectionType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EffectiveConnectionType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EffectiveConnectionType> for emlite::Val {
    fn from(s: EffectiveConnectionType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EffectiveConnectionType {
    pub const _2G: &str = "2g";
    pub const _3G: &str = "3g";
    pub const _4G: &str = "4g";
    pub const SLOW_2G: &str = "slow-2g";
}

#[derive(Clone, Debug)]
pub struct NotificationPermission {
    inner: emlite::Val,
}
impl FromVal for NotificationPermission {
    fn from_val(v: &emlite::Val) -> Self {
        NotificationPermission { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for NotificationPermission {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NotificationPermission {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NotificationPermission> for emlite::Val {
    fn from(s: NotificationPermission) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NotificationPermission {
    pub const DEFAULT: &str = "default";
    pub const DENIED: &str = "denied";
    pub const GRANTED: &str = "granted";
}

#[derive(Clone, Debug)]
pub struct NotificationDirection {
    inner: emlite::Val,
}
impl FromVal for NotificationDirection {
    fn from_val(v: &emlite::Val) -> Self {
        NotificationDirection { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for NotificationDirection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NotificationDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NotificationDirection> for emlite::Val {
    fn from(s: NotificationDirection) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NotificationDirection {
    pub const AUTO: &str = "auto";
    pub const LTR: &str = "ltr";
    pub const RTL: &str = "rtl";
}

#[derive(Clone, Debug)]
pub struct OrientationSensorLocalCoordinateSystem {
    inner: emlite::Val,
}
impl FromVal for OrientationSensorLocalCoordinateSystem {
    fn from_val(v: &emlite::Val) -> Self {
        OrientationSensorLocalCoordinateSystem { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OrientationSensorLocalCoordinateSystem {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OrientationSensorLocalCoordinateSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OrientationSensorLocalCoordinateSystem> for emlite::Val {
    fn from(s: OrientationSensorLocalCoordinateSystem) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OrientationSensorLocalCoordinateSystem {
    pub const DEVICE: &str = "device";
    pub const SCREEN: &str = "screen";
}

#[derive(Clone, Debug)]
pub struct ClientLifecycleState {
    inner: emlite::Val,
}
impl FromVal for ClientLifecycleState {
    fn from_val(v: &emlite::Val) -> Self {
        ClientLifecycleState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ClientLifecycleState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ClientLifecycleState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ClientLifecycleState> for emlite::Val {
    fn from(s: ClientLifecycleState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ClientLifecycleState {
    pub const ACTIVE: &str = "active";
    pub const FROZEN: &str = "frozen";
}

#[derive(Clone, Debug)]
pub struct PaymentDelegation {
    inner: emlite::Val,
}
impl FromVal for PaymentDelegation {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentDelegation { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PaymentDelegation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PaymentDelegation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaymentDelegation> for emlite::Val {
    fn from(s: PaymentDelegation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaymentDelegation {
    pub const SHIPPING_ADDRESS: &str = "shippingAddress";
    pub const PAYER_NAME: &str = "payerName";
    pub const PAYER_PHONE: &str = "payerPhone";
    pub const PAYER_EMAIL: &str = "payerEmail";
}

#[derive(Clone, Debug)]
pub struct PaymentShippingType {
    inner: emlite::Val,
}
impl FromVal for PaymentShippingType {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentShippingType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PaymentShippingType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PaymentShippingType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaymentShippingType> for emlite::Val {
    fn from(s: PaymentShippingType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaymentShippingType {
    pub const SHIPPING: &str = "shipping";
    pub const DELIVERY: &str = "delivery";
    pub const PICKUP: &str = "pickup";
}

#[derive(Clone, Debug)]
pub struct PaymentComplete {
    inner: emlite::Val,
}
impl FromVal for PaymentComplete {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentComplete { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PaymentComplete {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PaymentComplete {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaymentComplete> for emlite::Val {
    fn from(s: PaymentComplete) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaymentComplete {
    pub const FAIL: &str = "fail";
    pub const SUCCESS: &str = "success";
    pub const UNKNOWN: &str = "unknown";
}

#[derive(Clone, Debug)]
pub struct PermissionState {
    inner: emlite::Val,
}
impl FromVal for PermissionState {
    fn from_val(v: &emlite::Val) -> Self {
        PermissionState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PermissionState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PermissionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PermissionState> for emlite::Val {
    fn from(s: PermissionState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PermissionState {
    pub const GRANTED: &str = "granted";
    pub const DENIED: &str = "denied";
    pub const PROMPT: &str = "prompt";
}

#[derive(Clone, Debug)]
pub struct PointerAxis {
    inner: emlite::Val,
}
impl FromVal for PointerAxis {
    fn from_val(v: &emlite::Val) -> Self {
        PointerAxis { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PointerAxis {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PointerAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PointerAxis> for emlite::Val {
    fn from(s: PointerAxis) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PointerAxis {
    pub const BLOCK: &str = "block";
    pub const INLINE: &str = "inline";
    pub const X: &str = "x";
    pub const Y: &str = "y";
}

#[derive(Clone, Debug)]
pub struct PresentationConnectionState {
    inner: emlite::Val,
}
impl FromVal for PresentationConnectionState {
    fn from_val(v: &emlite::Val) -> Self {
        PresentationConnectionState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PresentationConnectionState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PresentationConnectionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PresentationConnectionState> for emlite::Val {
    fn from(s: PresentationConnectionState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PresentationConnectionState {
    pub const CONNECTING: &str = "connecting";
    pub const CONNECTED: &str = "connected";
    pub const CLOSED: &str = "closed";
    pub const TERMINATED: &str = "terminated";
}

#[derive(Clone, Debug)]
pub struct PresentationConnectionCloseReason {
    inner: emlite::Val,
}
impl FromVal for PresentationConnectionCloseReason {
    fn from_val(v: &emlite::Val) -> Self {
        PresentationConnectionCloseReason { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PresentationConnectionCloseReason {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PresentationConnectionCloseReason {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PresentationConnectionCloseReason> for emlite::Val {
    fn from(s: PresentationConnectionCloseReason) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PresentationConnectionCloseReason {
    pub const ERROR: &str = "error";
    pub const CLOSED: &str = "closed";
    pub const WENTAWAY: &str = "wentaway";
}

#[derive(Clone, Debug)]
pub struct PrivateAttributionAggregationProtocol {
    inner: emlite::Val,
}
impl FromVal for PrivateAttributionAggregationProtocol {
    fn from_val(v: &emlite::Val) -> Self {
        PrivateAttributionAggregationProtocol { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PrivateAttributionAggregationProtocol {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PrivateAttributionAggregationProtocol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PrivateAttributionAggregationProtocol> for emlite::Val {
    fn from(s: PrivateAttributionAggregationProtocol) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PrivateAttributionAggregationProtocol {
    pub const DAP_12_HISTOGRAM: &str = "dap-12-histogram";
    pub const TEE_00: &str = "tee-00";
}

#[derive(Clone, Debug)]
pub struct AttributionLogic {
    inner: emlite::Val,
}
impl FromVal for AttributionLogic {
    fn from_val(v: &emlite::Val) -> Self {
        AttributionLogic { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AttributionLogic {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AttributionLogic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AttributionLogic> for emlite::Val {
    fn from(s: AttributionLogic) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AttributionLogic {
    pub const LAST_TOUCH: &str = "last-touch";
}

#[derive(Clone, Debug)]
pub struct IPAddressSpace {
    inner: emlite::Val,
}
impl FromVal for IPAddressSpace {
    fn from_val(v: &emlite::Val) -> Self {
        IPAddressSpace { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IPAddressSpace {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IPAddressSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IPAddressSpace> for emlite::Val {
    fn from(s: IPAddressSpace) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IPAddressSpace {
    pub const PUBLIC: &str = "public";
    pub const PRIVATE: &str = "private";
    pub const LOCAL: &str = "local";
}

#[derive(Clone, Debug)]
pub struct PushEncryptionKeyName {
    inner: emlite::Val,
}
impl FromVal for PushEncryptionKeyName {
    fn from_val(v: &emlite::Val) -> Self {
        PushEncryptionKeyName { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PushEncryptionKeyName {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PushEncryptionKeyName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PushEncryptionKeyName> for emlite::Val {
    fn from(s: PushEncryptionKeyName) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PushEncryptionKeyName {
    pub const P256DH: &str = "p256dh";
    pub const AUTH: &str = "auth";
}

#[derive(Clone, Debug)]
pub struct ReferrerPolicy {
    inner: emlite::Val,
}
impl FromVal for ReferrerPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        ReferrerPolicy { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ReferrerPolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ReferrerPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ReferrerPolicy> for emlite::Val {
    fn from(s: ReferrerPolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ReferrerPolicy {
    pub const NONE: &str = "";
    pub const NO_REFERRER: &str = "no-referrer";
    pub const NO_REFERRER_WHEN_DOWNGRADE: &str = "no-referrer-when-downgrade";
    pub const SAME_ORIGIN: &str = "same-origin";
    pub const ORIGIN: &str = "origin";
    pub const STRICT_ORIGIN: &str = "strict-origin";
    pub const ORIGIN_WHEN_CROSS_ORIGIN: &str = "origin-when-cross-origin";
    pub const STRICT_ORIGIN_WHEN_CROSS_ORIGIN: &str = "strict-origin-when-cross-origin";
    pub const UNSAFE_URL: &str = "unsafe-url";
}

#[derive(Clone, Debug)]
pub struct RemotePlaybackState {
    inner: emlite::Val,
}
impl FromVal for RemotePlaybackState {
    fn from_val(v: &emlite::Val) -> Self {
        RemotePlaybackState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RemotePlaybackState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RemotePlaybackState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RemotePlaybackState> for emlite::Val {
    fn from(s: RemotePlaybackState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RemotePlaybackState {
    pub const CONNECTING: &str = "connecting";
    pub const CONNECTED: &str = "connected";
    pub const DISCONNECTED: &str = "disconnected";
}

#[derive(Clone, Debug)]
pub struct ResizeObserverBoxOptions {
    inner: emlite::Val,
}
impl FromVal for ResizeObserverBoxOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ResizeObserverBoxOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ResizeObserverBoxOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ResizeObserverBoxOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ResizeObserverBoxOptions> for emlite::Val {
    fn from(s: ResizeObserverBoxOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ResizeObserverBoxOptions {
    pub const BORDER_BOX: &str = "border-box";
    pub const CONTENT_BOX: &str = "content-box";
    pub const DEVICE_PIXEL_CONTENT_BOX: &str = "device-pixel-content-box";
}

#[derive(Clone, Debug)]
pub struct RenderBlockingStatusType {
    inner: emlite::Val,
}
impl FromVal for RenderBlockingStatusType {
    fn from_val(v: &emlite::Val) -> Self {
        RenderBlockingStatusType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RenderBlockingStatusType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RenderBlockingStatusType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RenderBlockingStatusType> for emlite::Val {
    fn from(s: RenderBlockingStatusType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RenderBlockingStatusType {
    pub const BLOCKING: &str = "blocking";
    pub const NON_BLOCKING: &str = "non-blocking";
}

#[derive(Clone, Debug)]
pub struct SameSiteCookiesType {
    inner: emlite::Val,
}
impl FromVal for SameSiteCookiesType {
    fn from_val(v: &emlite::Val) -> Self {
        SameSiteCookiesType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SameSiteCookiesType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SameSiteCookiesType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SameSiteCookiesType> for emlite::Val {
    fn from(s: SameSiteCookiesType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SameSiteCookiesType {
    pub const ALL: &str = "all";
    pub const NONE: &str = "none";
}

#[derive(Clone, Debug)]
pub struct SanitizerPresets {
    inner: emlite::Val,
}
impl FromVal for SanitizerPresets {
    fn from_val(v: &emlite::Val) -> Self {
        SanitizerPresets { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SanitizerPresets {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SanitizerPresets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SanitizerPresets> for emlite::Val {
    fn from(s: SanitizerPresets) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SanitizerPresets {
    pub const DEFAULT: &str = "default";
}

#[derive(Clone, Debug)]
pub struct TaskPriority {
    inner: emlite::Val,
}
impl FromVal for TaskPriority {
    fn from_val(v: &emlite::Val) -> Self {
        TaskPriority { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TaskPriority {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TaskPriority {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TaskPriority> for emlite::Val {
    fn from(s: TaskPriority) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TaskPriority {
    pub const USER_BLOCKING: &str = "user-blocking";
    pub const USER_VISIBLE: &str = "user-visible";
    pub const BACKGROUND: &str = "background";
}

#[derive(Clone, Debug)]
pub struct CaptureStartFocusBehavior {
    inner: emlite::Val,
}
impl FromVal for CaptureStartFocusBehavior {
    fn from_val(v: &emlite::Val) -> Self {
        CaptureStartFocusBehavior { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CaptureStartFocusBehavior {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CaptureStartFocusBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CaptureStartFocusBehavior> for emlite::Val {
    fn from(s: CaptureStartFocusBehavior) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CaptureStartFocusBehavior {
    pub const FOCUS_CAPTURING_APPLICATION: &str = "focus-capturing-application";
    pub const FOCUS_CAPTURED_SURFACE: &str = "focus-captured-surface";
    pub const NO_FOCUS_CHANGE: &str = "no-focus-change";
}

#[derive(Clone, Debug)]
pub struct SelfCapturePreferenceEnum {
    inner: emlite::Val,
}
impl FromVal for SelfCapturePreferenceEnum {
    fn from_val(v: &emlite::Val) -> Self {
        SelfCapturePreferenceEnum { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SelfCapturePreferenceEnum {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SelfCapturePreferenceEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SelfCapturePreferenceEnum> for emlite::Val {
    fn from(s: SelfCapturePreferenceEnum) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SelfCapturePreferenceEnum {
    pub const INCLUDE: &str = "include";
    pub const EXCLUDE: &str = "exclude";
}

#[derive(Clone, Debug)]
pub struct SystemAudioPreferenceEnum {
    inner: emlite::Val,
}
impl FromVal for SystemAudioPreferenceEnum {
    fn from_val(v: &emlite::Val) -> Self {
        SystemAudioPreferenceEnum { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SystemAudioPreferenceEnum {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SystemAudioPreferenceEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SystemAudioPreferenceEnum> for emlite::Val {
    fn from(s: SystemAudioPreferenceEnum) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SystemAudioPreferenceEnum {
    pub const INCLUDE: &str = "include";
    pub const EXCLUDE: &str = "exclude";
}

#[derive(Clone, Debug)]
pub struct WindowAudioPreferenceEnum {
    inner: emlite::Val,
}
impl FromVal for WindowAudioPreferenceEnum {
    fn from_val(v: &emlite::Val) -> Self {
        WindowAudioPreferenceEnum { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WindowAudioPreferenceEnum {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WindowAudioPreferenceEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WindowAudioPreferenceEnum> for emlite::Val {
    fn from(s: WindowAudioPreferenceEnum) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WindowAudioPreferenceEnum {
    pub const SYSTEM: &str = "system";
    pub const WINDOW: &str = "window";
    pub const EXCLUDE: &str = "exclude";
}

#[derive(Clone, Debug)]
pub struct SurfaceSwitchingPreferenceEnum {
    inner: emlite::Val,
}
impl FromVal for SurfaceSwitchingPreferenceEnum {
    fn from_val(v: &emlite::Val) -> Self {
        SurfaceSwitchingPreferenceEnum { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SurfaceSwitchingPreferenceEnum {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SurfaceSwitchingPreferenceEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SurfaceSwitchingPreferenceEnum> for emlite::Val {
    fn from(s: SurfaceSwitchingPreferenceEnum) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SurfaceSwitchingPreferenceEnum {
    pub const INCLUDE: &str = "include";
    pub const EXCLUDE: &str = "exclude";
}

#[derive(Clone, Debug)]
pub struct MonitorTypeSurfacesEnum {
    inner: emlite::Val,
}
impl FromVal for MonitorTypeSurfacesEnum {
    fn from_val(v: &emlite::Val) -> Self {
        MonitorTypeSurfacesEnum { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MonitorTypeSurfacesEnum {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MonitorTypeSurfacesEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MonitorTypeSurfacesEnum> for emlite::Val {
    fn from(s: MonitorTypeSurfacesEnum) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MonitorTypeSurfacesEnum {
    pub const INCLUDE: &str = "include";
    pub const EXCLUDE: &str = "exclude";
}

#[derive(Clone, Debug)]
pub struct DisplayCaptureSurfaceType {
    inner: emlite::Val,
}
impl FromVal for DisplayCaptureSurfaceType {
    fn from_val(v: &emlite::Val) -> Self {
        DisplayCaptureSurfaceType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DisplayCaptureSurfaceType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DisplayCaptureSurfaceType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DisplayCaptureSurfaceType> for emlite::Val {
    fn from(s: DisplayCaptureSurfaceType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DisplayCaptureSurfaceType {
    pub const MONITOR: &str = "monitor";
    pub const WINDOW: &str = "window";
    pub const BROWSER: &str = "browser";
}

#[derive(Clone, Debug)]
pub struct CursorCaptureConstraint {
    inner: emlite::Val,
}
impl FromVal for CursorCaptureConstraint {
    fn from_val(v: &emlite::Val) -> Self {
        CursorCaptureConstraint { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CursorCaptureConstraint {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CursorCaptureConstraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CursorCaptureConstraint> for emlite::Val {
    fn from(s: CursorCaptureConstraint) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CursorCaptureConstraint {
    pub const NEVER: &str = "never";
    pub const ALWAYS: &str = "always";
    pub const MOTION: &str = "motion";
}

#[derive(Clone, Debug)]
pub struct OrientationLockType {
    inner: emlite::Val,
}
impl FromVal for OrientationLockType {
    fn from_val(v: &emlite::Val) -> Self {
        OrientationLockType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OrientationLockType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OrientationLockType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OrientationLockType> for emlite::Val {
    fn from(s: OrientationLockType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OrientationLockType {
    pub const ANY: &str = "any";
    pub const NATURAL: &str = "natural";
    pub const LANDSCAPE: &str = "landscape";
    pub const PORTRAIT: &str = "portrait";
    pub const PORTRAIT_PRIMARY: &str = "portrait-primary";
    pub const PORTRAIT_SECONDARY: &str = "portrait-secondary";
    pub const LANDSCAPE_PRIMARY: &str = "landscape-primary";
    pub const LANDSCAPE_SECONDARY: &str = "landscape-secondary";
}

#[derive(Clone, Debug)]
pub struct OrientationType {
    inner: emlite::Val,
}
impl FromVal for OrientationType {
    fn from_val(v: &emlite::Val) -> Self {
        OrientationType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OrientationType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OrientationType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OrientationType> for emlite::Val {
    fn from(s: OrientationType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OrientationType {
    pub const PORTRAIT_PRIMARY: &str = "portrait-primary";
    pub const PORTRAIT_SECONDARY: &str = "portrait-secondary";
    pub const LANDSCAPE_PRIMARY: &str = "landscape-primary";
    pub const LANDSCAPE_SECONDARY: &str = "landscape-secondary";
}

#[derive(Clone, Debug)]
pub struct WakeLockType {
    inner: emlite::Val,
}
impl FromVal for WakeLockType {
    fn from_val(v: &emlite::Val) -> Self {
        WakeLockType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WakeLockType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WakeLockType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WakeLockType> for emlite::Val {
    fn from(s: WakeLockType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WakeLockType {
    pub const SCREEN: &str = "screen";
}

#[derive(Clone, Debug)]
pub struct ScrollAxis {
    inner: emlite::Val,
}
impl FromVal for ScrollAxis {
    fn from_val(v: &emlite::Val) -> Self {
        ScrollAxis { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ScrollAxis {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ScrollAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScrollAxis> for emlite::Val {
    fn from(s: ScrollAxis) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScrollAxis {
    pub const BLOCK: &str = "block";
    pub const INLINE: &str = "inline";
    pub const X: &str = "x";
    pub const Y: &str = "y";
}

#[derive(Clone, Debug)]
pub struct SecurePaymentConfirmationAvailability {
    inner: emlite::Val,
}
impl FromVal for SecurePaymentConfirmationAvailability {
    fn from_val(v: &emlite::Val) -> Self {
        SecurePaymentConfirmationAvailability { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SecurePaymentConfirmationAvailability {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SecurePaymentConfirmationAvailability {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SecurePaymentConfirmationAvailability> for emlite::Val {
    fn from(s: SecurePaymentConfirmationAvailability) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SecurePaymentConfirmationAvailability {
    pub const AVAILABLE: &str = "available";
    pub const UNAVAILABLE_UNKNOWN_REASON: &str = "unavailable-unknown-reason";
    pub const UNAVAILABLE_FEATURE_NOT_ENABLED: &str = "unavailable-feature-not-enabled";
    pub const UNAVAILABLE_NO_PERMISSION_POLICY: &str = "unavailable-no-permission-policy";
    pub const UNAVAILABLE_NO_USER_VERIFYING_PLATFORM_AUTHENTICATOR: &str =
        "unavailable-no-user-verifying-platform-authenticator";
}

#[derive(Clone, Debug)]
pub struct ParityType {
    inner: emlite::Val,
}
impl FromVal for ParityType {
    fn from_val(v: &emlite::Val) -> Self {
        ParityType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ParityType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ParityType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ParityType> for emlite::Val {
    fn from(s: ParityType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ParityType {
    pub const NONE: &str = "none";
    pub const EVEN: &str = "even";
    pub const ODD: &str = "odd";
}

#[derive(Clone, Debug)]
pub struct FlowControlType {
    inner: emlite::Val,
}
impl FromVal for FlowControlType {
    fn from_val(v: &emlite::Val) -> Self {
        FlowControlType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FlowControlType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FlowControlType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FlowControlType> for emlite::Val {
    fn from(s: FlowControlType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FlowControlType {
    pub const NONE: &str = "none";
    pub const HARDWARE: &str = "hardware";
}

#[derive(Clone, Debug)]
pub struct ServiceWorkerState {
    inner: emlite::Val,
}
impl FromVal for ServiceWorkerState {
    fn from_val(v: &emlite::Val) -> Self {
        ServiceWorkerState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ServiceWorkerState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ServiceWorkerState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ServiceWorkerState> for emlite::Val {
    fn from(s: ServiceWorkerState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ServiceWorkerState {
    pub const PARSED: &str = "parsed";
    pub const INSTALLING: &str = "installing";
    pub const INSTALLED: &str = "installed";
    pub const ACTIVATING: &str = "activating";
    pub const ACTIVATED: &str = "activated";
    pub const REDUNDANT: &str = "redundant";
}

#[derive(Clone, Debug)]
pub struct ServiceWorkerUpdateViaCache {
    inner: emlite::Val,
}
impl FromVal for ServiceWorkerUpdateViaCache {
    fn from_val(v: &emlite::Val) -> Self {
        ServiceWorkerUpdateViaCache { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ServiceWorkerUpdateViaCache {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ServiceWorkerUpdateViaCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ServiceWorkerUpdateViaCache> for emlite::Val {
    fn from(s: ServiceWorkerUpdateViaCache) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ServiceWorkerUpdateViaCache {
    pub const IMPORTS: &str = "imports";
    pub const ALL: &str = "all";
    pub const NONE: &str = "none";
}

#[derive(Clone, Debug)]
pub struct FrameType {
    inner: emlite::Val,
}
impl FromVal for FrameType {
    fn from_val(v: &emlite::Val) -> Self {
        FrameType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FrameType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FrameType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FrameType> for emlite::Val {
    fn from(s: FrameType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FrameType {
    pub const AUXILIARY: &str = "auxiliary";
    pub const TOP_LEVEL: &str = "top-level";
    pub const NESTED: &str = "nested";
    pub const NONE: &str = "none";
}

#[derive(Clone, Debug)]
pub struct ClientType {
    inner: emlite::Val,
}
impl FromVal for ClientType {
    fn from_val(v: &emlite::Val) -> Self {
        ClientType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ClientType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ClientType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ClientType> for emlite::Val {
    fn from(s: ClientType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ClientType {
    pub const WINDOW: &str = "window";
    pub const WORKER: &str = "worker";
    pub const SHAREDWORKER: &str = "sharedworker";
    pub const ALL: &str = "all";
}

#[derive(Clone, Debug)]
pub struct RunningStatus {
    inner: emlite::Val,
}
impl FromVal for RunningStatus {
    fn from_val(v: &emlite::Val) -> Self {
        RunningStatus { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RunningStatus {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RunningStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RunningStatus> for emlite::Val {
    fn from(s: RunningStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RunningStatus {
    pub const RUNNING: &str = "running";
    pub const NOT_RUNNING: &str = "not-running";
}

#[derive(Clone, Debug)]
pub struct RouterSourceEnum {
    inner: emlite::Val,
}
impl FromVal for RouterSourceEnum {
    fn from_val(v: &emlite::Val) -> Self {
        RouterSourceEnum { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RouterSourceEnum {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RouterSourceEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RouterSourceEnum> for emlite::Val {
    fn from(s: RouterSourceEnum) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RouterSourceEnum {
    pub const CACHE: &str = "cache";
    pub const FETCH_EVENT: &str = "fetch-event";
    pub const NETWORK: &str = "network";
    pub const RACE_NETWORK_AND_FETCH_HANDLER: &str = "race-network-and-fetch-handler";
}

#[derive(Clone, Debug)]
pub struct LandmarkType {
    inner: emlite::Val,
}
impl FromVal for LandmarkType {
    fn from_val(v: &emlite::Val) -> Self {
        LandmarkType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LandmarkType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LandmarkType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LandmarkType> for emlite::Val {
    fn from(s: LandmarkType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LandmarkType {
    pub const MOUTH: &str = "mouth";
    pub const EYE: &str = "eye";
    pub const NOSE: &str = "nose";
}

#[derive(Clone, Debug)]
pub struct BarcodeFormat {
    inner: emlite::Val,
}
impl FromVal for BarcodeFormat {
    fn from_val(v: &emlite::Val) -> Self {
        BarcodeFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BarcodeFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BarcodeFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BarcodeFormat> for emlite::Val {
    fn from(s: BarcodeFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BarcodeFormat {
    pub const AZTEC: &str = "aztec";
    pub const CODE_128: &str = "code_128";
    pub const CODE_39: &str = "code_39";
    pub const CODE_93: &str = "code_93";
    pub const CODABAR: &str = "codabar";
    pub const DATA_MATRIX: &str = "data_matrix";
    pub const EAN_13: &str = "ean_13";
    pub const EAN_8: &str = "ean_8";
    pub const ITF: &str = "itf";
    pub const PDF417: &str = "pdf417";
    pub const QR_CODE: &str = "qr_code";
    pub const UNKNOWN: &str = "unknown";
    pub const UPC_A: &str = "upc_a";
    pub const UPC_E: &str = "upc_e";
}

#[derive(Clone, Debug)]
pub struct SpeechRecognitionErrorCode {
    inner: emlite::Val,
}
impl FromVal for SpeechRecognitionErrorCode {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechRecognitionErrorCode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SpeechRecognitionErrorCode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SpeechRecognitionErrorCode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechRecognitionErrorCode> for emlite::Val {
    fn from(s: SpeechRecognitionErrorCode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechRecognitionErrorCode {
    pub const NO_SPEECH: &str = "no-speech";
    pub const ABORTED: &str = "aborted";
    pub const AUDIO_CAPTURE: &str = "audio-capture";
    pub const NETWORK: &str = "network";
    pub const NOT_ALLOWED: &str = "not-allowed";
    pub const SERVICE_NOT_ALLOWED: &str = "service-not-allowed";
    pub const LANGUAGE_NOT_SUPPORTED: &str = "language-not-supported";
    pub const PHRASES_NOT_SUPPORTED: &str = "phrases-not-supported";
}

#[derive(Clone, Debug)]
pub struct AvailabilityStatus {
    inner: emlite::Val,
}
impl FromVal for AvailabilityStatus {
    fn from_val(v: &emlite::Val) -> Self {
        AvailabilityStatus { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AvailabilityStatus {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AvailabilityStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AvailabilityStatus> for emlite::Val {
    fn from(s: AvailabilityStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AvailabilityStatus {
    pub const UNAVAILABLE: &str = "unavailable";
    pub const DOWNLOADABLE: &str = "downloadable";
    pub const DOWNLOADING: &str = "downloading";
    pub const AVAILABLE: &str = "available";
}

#[derive(Clone, Debug)]
pub struct SpeechSynthesisErrorCode {
    inner: emlite::Val,
}
impl FromVal for SpeechSynthesisErrorCode {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechSynthesisErrorCode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SpeechSynthesisErrorCode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SpeechSynthesisErrorCode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechSynthesisErrorCode> for emlite::Val {
    fn from(s: SpeechSynthesisErrorCode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechSynthesisErrorCode {
    pub const CANCELED: &str = "canceled";
    pub const INTERRUPTED: &str = "interrupted";
    pub const AUDIO_BUSY: &str = "audio-busy";
    pub const AUDIO_HARDWARE: &str = "audio-hardware";
    pub const NETWORK: &str = "network";
    pub const SYNTHESIS_UNAVAILABLE: &str = "synthesis-unavailable";
    pub const SYNTHESIS_FAILED: &str = "synthesis-failed";
    pub const LANGUAGE_UNAVAILABLE: &str = "language-unavailable";
    pub const VOICE_UNAVAILABLE: &str = "voice-unavailable";
    pub const TEXT_TOO_LONG: &str = "text-too-long";
    pub const INVALID_ARGUMENT: &str = "invalid-argument";
    pub const NOT_ALLOWED: &str = "not-allowed";
}

#[derive(Clone, Debug)]
pub struct ReadableStreamReaderMode {
    inner: emlite::Val,
}
impl FromVal for ReadableStreamReaderMode {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableStreamReaderMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ReadableStreamReaderMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ReadableStreamReaderMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ReadableStreamReaderMode> for emlite::Val {
    fn from(s: ReadableStreamReaderMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ReadableStreamReaderMode {
    pub const BYOB: &str = "byob";
}

#[derive(Clone, Debug)]
pub struct ReadableStreamType {
    inner: emlite::Val,
}
impl FromVal for ReadableStreamType {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableStreamType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ReadableStreamType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ReadableStreamType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ReadableStreamType> for emlite::Val {
    fn from(s: ReadableStreamType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ReadableStreamType {
    pub const BYTES: &str = "bytes";
}

#[derive(Clone, Debug)]
pub struct TouchType {
    inner: emlite::Val,
}
impl FromVal for TouchType {
    fn from_val(v: &emlite::Val) -> Self {
        TouchType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TouchType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TouchType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TouchType> for emlite::Val {
    fn from(s: TouchType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TouchType {
    pub const DIRECT: &str = "direct";
    pub const STYLUS: &str = "stylus";
}

#[derive(Clone, Debug)]
pub struct RefreshPolicy {
    inner: emlite::Val,
}
impl FromVal for RefreshPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        RefreshPolicy { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RefreshPolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RefreshPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RefreshPolicy> for emlite::Val {
    fn from(s: RefreshPolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RefreshPolicy {
    pub const NONE: &str = "none";
    pub const REFRESH: &str = "refresh";
}

#[derive(Clone, Debug)]
pub struct TokenVersion {
    inner: emlite::Val,
}
impl FromVal for TokenVersion {
    fn from_val(v: &emlite::Val) -> Self {
        TokenVersion { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TokenVersion {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TokenVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TokenVersion> for emlite::Val {
    fn from(s: TokenVersion) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TokenVersion {
    pub const _1: &str = "1";
}

#[derive(Clone, Debug)]
pub struct OperationType {
    inner: emlite::Val,
}
impl FromVal for OperationType {
    fn from_val(v: &emlite::Val) -> Self {
        OperationType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OperationType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OperationType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OperationType> for emlite::Val {
    fn from(s: OperationType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OperationType {
    pub const TOKEN_REQUEST: &str = "token-request";
    pub const SEND_REDEMPTION_RECORD: &str = "send-redemption-record";
    pub const TOKEN_REDEMPTION: &str = "token-redemption";
}

#[derive(Clone, Debug)]
pub struct KAnonStatus {
    inner: emlite::Val,
}
impl FromVal for KAnonStatus {
    fn from_val(v: &emlite::Val) -> Self {
        KAnonStatus { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for KAnonStatus {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for KAnonStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<KAnonStatus> for emlite::Val {
    fn from(s: KAnonStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl KAnonStatus {
    pub const PASSED_AND_ENFORCED: &str = "passedAndEnforced";
    pub const PASSED_NOT_ENFORCED: &str = "passedNotEnforced";
    pub const BELOW_THRESHOLD: &str = "belowThreshold";
    pub const NOT_CALCULATED: &str = "notCalculated";
}

#[derive(Clone, Debug)]
pub struct ImportExportKind {
    inner: emlite::Val,
}
impl FromVal for ImportExportKind {
    fn from_val(v: &emlite::Val) -> Self {
        ImportExportKind { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ImportExportKind {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ImportExportKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ImportExportKind> for emlite::Val {
    fn from(s: ImportExportKind) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ImportExportKind {
    pub const FUNCTION: &str = "function";
    pub const TABLE: &str = "table";
    pub const MEMORY: &str = "memory";
    pub const GLOBAL: &str = "global";
}

#[derive(Clone, Debug)]
pub struct TableKind {
    inner: emlite::Val,
}
impl FromVal for TableKind {
    fn from_val(v: &emlite::Val) -> Self {
        TableKind { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TableKind {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TableKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TableKind> for emlite::Val {
    fn from(s: TableKind) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TableKind {
    pub const EXTERNREF: &str = "externref";
    pub const ANYFUNC: &str = "anyfunc";
}

#[derive(Clone, Debug)]
pub struct ValueType {
    inner: emlite::Val,
}
impl FromVal for ValueType {
    fn from_val(v: &emlite::Val) -> Self {
        ValueType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ValueType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ValueType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ValueType> for emlite::Val {
    fn from(s: ValueType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ValueType {
    pub const I32_: &str = "i32";
    pub const I64_: &str = "i64";
    pub const F32_: &str = "f32";
    pub const F64_: &str = "f64";
    pub const V128: &str = "v128";
    pub const EXTERNREF: &str = "externref";
    pub const ANYFUNC: &str = "anyfunc";
}

#[derive(Clone, Debug)]
pub struct IterationCompositeOperation {
    inner: emlite::Val,
}
impl FromVal for IterationCompositeOperation {
    fn from_val(v: &emlite::Val) -> Self {
        IterationCompositeOperation { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IterationCompositeOperation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IterationCompositeOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IterationCompositeOperation> for emlite::Val {
    fn from(s: IterationCompositeOperation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IterationCompositeOperation {
    pub const REPLACE: &str = "replace";
    pub const ACCUMULATE: &str = "accumulate";
}

#[derive(Clone, Debug)]
pub struct AnimationTriggerBehavior {
    inner: emlite::Val,
}
impl FromVal for AnimationTriggerBehavior {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationTriggerBehavior { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AnimationTriggerBehavior {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AnimationTriggerBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AnimationTriggerBehavior> for emlite::Val {
    fn from(s: AnimationTriggerBehavior) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AnimationTriggerBehavior {
    pub const ONCE: &str = "once";
    pub const REPEAT: &str = "repeat";
    pub const ALTERNATE: &str = "alternate";
    pub const STATE: &str = "state";
}

#[derive(Clone, Debug)]
pub struct AnimationPlayState {
    inner: emlite::Val,
}
impl FromVal for AnimationPlayState {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationPlayState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AnimationPlayState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AnimationPlayState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AnimationPlayState> for emlite::Val {
    fn from(s: AnimationPlayState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AnimationPlayState {
    pub const IDLE: &str = "idle";
    pub const RUNNING: &str = "running";
    pub const PAUSED: &str = "paused";
    pub const FINISHED: &str = "finished";
}

#[derive(Clone, Debug)]
pub struct AnimationReplaceState {
    inner: emlite::Val,
}
impl FromVal for AnimationReplaceState {
    fn from_val(v: &emlite::Val) -> Self {
        AnimationReplaceState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AnimationReplaceState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AnimationReplaceState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AnimationReplaceState> for emlite::Val {
    fn from(s: AnimationReplaceState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AnimationReplaceState {
    pub const ACTIVE: &str = "active";
    pub const REMOVED: &str = "removed";
    pub const PERSISTED: &str = "persisted";
}

#[derive(Clone, Debug)]
pub struct FillMode {
    inner: emlite::Val,
}
impl FromVal for FillMode {
    fn from_val(v: &emlite::Val) -> Self {
        FillMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FillMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FillMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FillMode> for emlite::Val {
    fn from(s: FillMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FillMode {
    pub const NONE: &str = "none";
    pub const FORWARDS: &str = "forwards";
    pub const BACKWARDS: &str = "backwards";
    pub const BOTH: &str = "both";
    pub const AUTO: &str = "auto";
}

#[derive(Clone, Debug)]
pub struct PlaybackDirection {
    inner: emlite::Val,
}
impl FromVal for PlaybackDirection {
    fn from_val(v: &emlite::Val) -> Self {
        PlaybackDirection { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PlaybackDirection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PlaybackDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PlaybackDirection> for emlite::Val {
    fn from(s: PlaybackDirection) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PlaybackDirection {
    pub const NORMAL: &str = "normal";
    pub const REVERSE: &str = "reverse";
    pub const ALTERNATE: &str = "alternate";
    pub const ALTERNATE_REVERSE: &str = "alternate-reverse";
}

#[derive(Clone, Debug)]
pub struct CompositeOperation {
    inner: emlite::Val,
}
impl FromVal for CompositeOperation {
    fn from_val(v: &emlite::Val) -> Self {
        CompositeOperation { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CompositeOperation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CompositeOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CompositeOperation> for emlite::Val {
    fn from(s: CompositeOperation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CompositeOperation {
    pub const REPLACE: &str = "replace";
    pub const ADD: &str = "add";
    pub const ACCUMULATE: &str = "accumulate";
}

#[derive(Clone, Debug)]
pub struct CompositeOperationOrAuto {
    inner: emlite::Val,
}
impl FromVal for CompositeOperationOrAuto {
    fn from_val(v: &emlite::Val) -> Self {
        CompositeOperationOrAuto { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CompositeOperationOrAuto {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CompositeOperationOrAuto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CompositeOperationOrAuto> for emlite::Val {
    fn from(s: CompositeOperationOrAuto) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CompositeOperationOrAuto {
    pub const REPLACE: &str = "replace";
    pub const ADD: &str = "add";
    pub const ACCUMULATE: &str = "accumulate";
    pub const AUTO: &str = "auto";
}

#[derive(Clone, Debug)]
pub struct LockMode {
    inner: emlite::Val,
}
impl FromVal for LockMode {
    fn from_val(v: &emlite::Val) -> Self {
        LockMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LockMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LockMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LockMode> for emlite::Val {
    fn from(s: LockMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LockMode {
    pub const SHARED: &str = "shared";
    pub const EXCLUSIVE: &str = "exclusive";
}

#[derive(Clone, Debug)]
pub struct OTPCredentialTransportType {
    inner: emlite::Val,
}
impl FromVal for OTPCredentialTransportType {
    fn from_val(v: &emlite::Val) -> Self {
        OTPCredentialTransportType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OTPCredentialTransportType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OTPCredentialTransportType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OTPCredentialTransportType> for emlite::Val {
    fn from(s: OTPCredentialTransportType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OTPCredentialTransportType {
    pub const SMS: &str = "sms";
}

#[derive(Clone, Debug)]
pub struct AudioContextState {
    inner: emlite::Val,
}
impl FromVal for AudioContextState {
    fn from_val(v: &emlite::Val) -> Self {
        AudioContextState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AudioContextState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioContextState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioContextState> for emlite::Val {
    fn from(s: AudioContextState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioContextState {
    pub const SUSPENDED: &str = "suspended";
    pub const RUNNING: &str = "running";
    pub const CLOSED: &str = "closed";
    pub const INTERRUPTED: &str = "interrupted";
}

#[derive(Clone, Debug)]
pub struct AudioContextRenderSizeCategory {
    inner: emlite::Val,
}
impl FromVal for AudioContextRenderSizeCategory {
    fn from_val(v: &emlite::Val) -> Self {
        AudioContextRenderSizeCategory { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AudioContextRenderSizeCategory {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioContextRenderSizeCategory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioContextRenderSizeCategory> for emlite::Val {
    fn from(s: AudioContextRenderSizeCategory) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioContextRenderSizeCategory {
    pub const DEFAULT: &str = "default";
    pub const HARDWARE: &str = "hardware";
}

#[derive(Clone, Debug)]
pub struct AudioContextLatencyCategory {
    inner: emlite::Val,
}
impl FromVal for AudioContextLatencyCategory {
    fn from_val(v: &emlite::Val) -> Self {
        AudioContextLatencyCategory { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AudioContextLatencyCategory {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioContextLatencyCategory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioContextLatencyCategory> for emlite::Val {
    fn from(s: AudioContextLatencyCategory) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioContextLatencyCategory {
    pub const BALANCED: &str = "balanced";
    pub const INTERACTIVE: &str = "interactive";
    pub const PLAYBACK: &str = "playback";
}

#[derive(Clone, Debug)]
pub struct AudioSinkType {
    inner: emlite::Val,
}
impl FromVal for AudioSinkType {
    fn from_val(v: &emlite::Val) -> Self {
        AudioSinkType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AudioSinkType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioSinkType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioSinkType> for emlite::Val {
    fn from(s: AudioSinkType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioSinkType {
    pub const NONE: &str = "none";
}

#[derive(Clone, Debug)]
pub struct ChannelCountMode {
    inner: emlite::Val,
}
impl FromVal for ChannelCountMode {
    fn from_val(v: &emlite::Val) -> Self {
        ChannelCountMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ChannelCountMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ChannelCountMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ChannelCountMode> for emlite::Val {
    fn from(s: ChannelCountMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ChannelCountMode {
    pub const MAX: &str = "max";
    pub const CLAMPED_MAX: &str = "clamped-max";
    pub const EXPLICIT: &str = "explicit";
}

#[derive(Clone, Debug)]
pub struct ChannelInterpretation {
    inner: emlite::Val,
}
impl FromVal for ChannelInterpretation {
    fn from_val(v: &emlite::Val) -> Self {
        ChannelInterpretation { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ChannelInterpretation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ChannelInterpretation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ChannelInterpretation> for emlite::Val {
    fn from(s: ChannelInterpretation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ChannelInterpretation {
    pub const SPEAKERS: &str = "speakers";
    pub const DISCRETE: &str = "discrete";
}

#[derive(Clone, Debug)]
pub struct AutomationRate {
    inner: emlite::Val,
}
impl FromVal for AutomationRate {
    fn from_val(v: &emlite::Val) -> Self {
        AutomationRate { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AutomationRate {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AutomationRate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AutomationRate> for emlite::Val {
    fn from(s: AutomationRate) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AutomationRate {
    pub const A_RATE: &str = "a-rate";
    pub const K_RATE: &str = "k-rate";
}

#[derive(Clone, Debug)]
pub struct BiquadFilterType {
    inner: emlite::Val,
}
impl FromVal for BiquadFilterType {
    fn from_val(v: &emlite::Val) -> Self {
        BiquadFilterType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BiquadFilterType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BiquadFilterType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BiquadFilterType> for emlite::Val {
    fn from(s: BiquadFilterType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BiquadFilterType {
    pub const LOWPASS: &str = "lowpass";
    pub const HIGHPASS: &str = "highpass";
    pub const BANDPASS: &str = "bandpass";
    pub const LOWSHELF: &str = "lowshelf";
    pub const HIGHSHELF: &str = "highshelf";
    pub const PEAKING: &str = "peaking";
    pub const NOTCH: &str = "notch";
    pub const ALLPASS: &str = "allpass";
}

#[derive(Clone, Debug)]
pub struct OscillatorType {
    inner: emlite::Val,
}
impl FromVal for OscillatorType {
    fn from_val(v: &emlite::Val) -> Self {
        OscillatorType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OscillatorType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OscillatorType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OscillatorType> for emlite::Val {
    fn from(s: OscillatorType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OscillatorType {
    pub const SINE: &str = "sine";
    pub const SQUARE: &str = "square";
    pub const SAWTOOTH: &str = "sawtooth";
    pub const TRIANGLE: &str = "triangle";
    pub const CUSTOM: &str = "custom";
}

#[derive(Clone, Debug)]
pub struct PanningModelType {
    inner: emlite::Val,
}
impl FromVal for PanningModelType {
    fn from_val(v: &emlite::Val) -> Self {
        PanningModelType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PanningModelType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PanningModelType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PanningModelType> for emlite::Val {
    fn from(s: PanningModelType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PanningModelType {
    pub const EQUALPOWER: &str = "equalpower";
    pub const HRTF: &str = "HRTF";
}

#[derive(Clone, Debug)]
pub struct DistanceModelType {
    inner: emlite::Val,
}
impl FromVal for DistanceModelType {
    fn from_val(v: &emlite::Val) -> Self {
        DistanceModelType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DistanceModelType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DistanceModelType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DistanceModelType> for emlite::Val {
    fn from(s: DistanceModelType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DistanceModelType {
    pub const LINEAR: &str = "linear";
    pub const INVERSE: &str = "inverse";
    pub const EXPONENTIAL: &str = "exponential";
}

#[derive(Clone, Debug)]
pub struct OverSampleType {
    inner: emlite::Val,
}
impl FromVal for OverSampleType {
    fn from_val(v: &emlite::Val) -> Self {
        OverSampleType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OverSampleType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OverSampleType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OverSampleType> for emlite::Val {
    fn from(s: OverSampleType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OverSampleType {
    pub const NONE: &str = "none";
    pub const _2X: &str = "2x";
    pub const _4X: &str = "4x";
}

#[derive(Clone, Debug)]
pub struct AuthenticatorAttachment {
    inner: emlite::Val,
}
impl FromVal for AuthenticatorAttachment {
    fn from_val(v: &emlite::Val) -> Self {
        AuthenticatorAttachment { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AuthenticatorAttachment {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AuthenticatorAttachment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AuthenticatorAttachment> for emlite::Val {
    fn from(s: AuthenticatorAttachment) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AuthenticatorAttachment {
    pub const PLATFORM: &str = "platform";
    pub const CROSS_PLATFORM: &str = "cross-platform";
}

#[derive(Clone, Debug)]
pub struct ResidentKeyRequirement {
    inner: emlite::Val,
}
impl FromVal for ResidentKeyRequirement {
    fn from_val(v: &emlite::Val) -> Self {
        ResidentKeyRequirement { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ResidentKeyRequirement {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ResidentKeyRequirement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ResidentKeyRequirement> for emlite::Val {
    fn from(s: ResidentKeyRequirement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ResidentKeyRequirement {
    pub const DISCOURAGED: &str = "discouraged";
    pub const PREFERRED: &str = "preferred";
    pub const REQUIRED: &str = "required";
}

#[derive(Clone, Debug)]
pub struct AttestationConveyancePreference {
    inner: emlite::Val,
}
impl FromVal for AttestationConveyancePreference {
    fn from_val(v: &emlite::Val) -> Self {
        AttestationConveyancePreference { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AttestationConveyancePreference {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AttestationConveyancePreference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AttestationConveyancePreference> for emlite::Val {
    fn from(s: AttestationConveyancePreference) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AttestationConveyancePreference {
    pub const NONE: &str = "none";
    pub const INDIRECT: &str = "indirect";
    pub const DIRECT: &str = "direct";
    pub const ENTERPRISE: &str = "enterprise";
}

#[derive(Clone, Debug)]
pub struct TokenBindingStatus {
    inner: emlite::Val,
}
impl FromVal for TokenBindingStatus {
    fn from_val(v: &emlite::Val) -> Self {
        TokenBindingStatus { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TokenBindingStatus {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TokenBindingStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TokenBindingStatus> for emlite::Val {
    fn from(s: TokenBindingStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TokenBindingStatus {
    pub const PRESENT: &str = "present";
    pub const SUPPORTED: &str = "supported";
}

#[derive(Clone, Debug)]
pub struct PublicKeyCredentialType {
    inner: emlite::Val,
}
impl FromVal for PublicKeyCredentialType {
    fn from_val(v: &emlite::Val) -> Self {
        PublicKeyCredentialType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PublicKeyCredentialType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PublicKeyCredentialType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PublicKeyCredentialType> for emlite::Val {
    fn from(s: PublicKeyCredentialType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PublicKeyCredentialType {
    pub const PUBLIC_KEY: &str = "public-key";
}

#[derive(Clone, Debug)]
pub struct AuthenticatorTransport {
    inner: emlite::Val,
}
impl FromVal for AuthenticatorTransport {
    fn from_val(v: &emlite::Val) -> Self {
        AuthenticatorTransport { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AuthenticatorTransport {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AuthenticatorTransport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AuthenticatorTransport> for emlite::Val {
    fn from(s: AuthenticatorTransport) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AuthenticatorTransport {
    pub const USB: &str = "usb";
    pub const NFC: &str = "nfc";
    pub const BLE: &str = "ble";
    pub const SMART_CARD: &str = "smart-card";
    pub const HYBRID: &str = "hybrid";
    pub const INTERNAL: &str = "internal";
}

#[derive(Clone, Debug)]
pub struct UserVerificationRequirement {
    inner: emlite::Val,
}
impl FromVal for UserVerificationRequirement {
    fn from_val(v: &emlite::Val) -> Self {
        UserVerificationRequirement { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for UserVerificationRequirement {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for UserVerificationRequirement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<UserVerificationRequirement> for emlite::Val {
    fn from(s: UserVerificationRequirement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl UserVerificationRequirement {
    pub const REQUIRED: &str = "required";
    pub const PREFERRED: &str = "preferred";
    pub const DISCOURAGED: &str = "discouraged";
}

#[derive(Clone, Debug)]
pub struct ClientCapability {
    inner: emlite::Val,
}
impl FromVal for ClientCapability {
    fn from_val(v: &emlite::Val) -> Self {
        ClientCapability { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ClientCapability {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ClientCapability {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ClientCapability> for emlite::Val {
    fn from(s: ClientCapability) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ClientCapability {
    pub const CONDITIONAL_CREATE: &str = "conditionalCreate";
    pub const CONDITIONAL_GET: &str = "conditionalGet";
    pub const HYBRID_TRANSPORT: &str = "hybridTransport";
    pub const PASSKEY_PLATFORM_AUTHENTICATOR: &str = "passkeyPlatformAuthenticator";
    pub const USER_VERIFYING_PLATFORM_AUTHENTICATOR: &str = "userVerifyingPlatformAuthenticator";
    pub const RELATED_ORIGINS: &str = "relatedOrigins";
    pub const SIGNAL_ALL_ACCEPTED_CREDENTIALS: &str = "signalAllAcceptedCredentials";
    pub const SIGNAL_CURRENT_USER_DETAILS: &str = "signalCurrentUserDetails";
    pub const SIGNAL_UNKNOWN_CREDENTIAL: &str = "signalUnknownCredential";
}

#[derive(Clone, Debug)]
pub struct PublicKeyCredentialHint {
    inner: emlite::Val,
}
impl FromVal for PublicKeyCredentialHint {
    fn from_val(v: &emlite::Val) -> Self {
        PublicKeyCredentialHint { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PublicKeyCredentialHint {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PublicKeyCredentialHint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PublicKeyCredentialHint> for emlite::Val {
    fn from(s: PublicKeyCredentialHint) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PublicKeyCredentialHint {
    pub const SECURITY_KEY: &str = "security-key";
    pub const CLIENT_DEVICE: &str = "client-device";
    pub const HYBRID: &str = "hybrid";
}

#[derive(Clone, Debug)]
pub struct LargeBlobSupport {
    inner: emlite::Val,
}
impl FromVal for LargeBlobSupport {
    fn from_val(v: &emlite::Val) -> Self {
        LargeBlobSupport { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LargeBlobSupport {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LargeBlobSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LargeBlobSupport> for emlite::Val {
    fn from(s: LargeBlobSupport) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LargeBlobSupport {
    pub const REQUIRED: &str = "required";
    pub const PREFERRED: &str = "preferred";
}

#[derive(Clone, Debug)]
pub struct AacBitstreamFormat {
    inner: emlite::Val,
}
impl FromVal for AacBitstreamFormat {
    fn from_val(v: &emlite::Val) -> Self {
        AacBitstreamFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AacBitstreamFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AacBitstreamFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AacBitstreamFormat> for emlite::Val {
    fn from(s: AacBitstreamFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AacBitstreamFormat {
    pub const AAC: &str = "aac";
    pub const ADTS: &str = "adts";
}

#[derive(Clone, Debug)]
pub struct AvcBitstreamFormat {
    inner: emlite::Val,
}
impl FromVal for AvcBitstreamFormat {
    fn from_val(v: &emlite::Val) -> Self {
        AvcBitstreamFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AvcBitstreamFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AvcBitstreamFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AvcBitstreamFormat> for emlite::Val {
    fn from(s: AvcBitstreamFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AvcBitstreamFormat {
    pub const ANNEXB: &str = "annexb";
    pub const AVC: &str = "avc";
}

#[derive(Clone, Debug)]
pub struct HevcBitstreamFormat {
    inner: emlite::Val,
}
impl FromVal for HevcBitstreamFormat {
    fn from_val(v: &emlite::Val) -> Self {
        HevcBitstreamFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HevcBitstreamFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HevcBitstreamFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HevcBitstreamFormat> for emlite::Val {
    fn from(s: HevcBitstreamFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HevcBitstreamFormat {
    pub const ANNEXB: &str = "annexb";
    pub const HEVC: &str = "hevc";
}

#[derive(Clone, Debug)]
pub struct OpusBitstreamFormat {
    inner: emlite::Val,
}
impl FromVal for OpusBitstreamFormat {
    fn from_val(v: &emlite::Val) -> Self {
        OpusBitstreamFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OpusBitstreamFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OpusBitstreamFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OpusBitstreamFormat> for emlite::Val {
    fn from(s: OpusBitstreamFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OpusBitstreamFormat {
    pub const OPUS: &str = "opus";
    pub const OGG: &str = "ogg";
}

#[derive(Clone, Debug)]
pub struct OpusSignal {
    inner: emlite::Val,
}
impl FromVal for OpusSignal {
    fn from_val(v: &emlite::Val) -> Self {
        OpusSignal { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OpusSignal {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OpusSignal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OpusSignal> for emlite::Val {
    fn from(s: OpusSignal) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OpusSignal {
    pub const AUTO: &str = "auto";
    pub const MUSIC: &str = "music";
    pub const VOICE: &str = "voice";
}

#[derive(Clone, Debug)]
pub struct OpusApplication {
    inner: emlite::Val,
}
impl FromVal for OpusApplication {
    fn from_val(v: &emlite::Val) -> Self {
        OpusApplication { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for OpusApplication {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for OpusApplication {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<OpusApplication> for emlite::Val {
    fn from(s: OpusApplication) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OpusApplication {
    pub const VOIP: &str = "voip";
    pub const AUDIO: &str = "audio";
    pub const LOWDELAY: &str = "lowdelay";
}

#[derive(Clone, Debug)]
pub struct HardwareAcceleration {
    inner: emlite::Val,
}
impl FromVal for HardwareAcceleration {
    fn from_val(v: &emlite::Val) -> Self {
        HardwareAcceleration { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HardwareAcceleration {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HardwareAcceleration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HardwareAcceleration> for emlite::Val {
    fn from(s: HardwareAcceleration) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HardwareAcceleration {
    pub const NO_PREFERENCE: &str = "no-preference";
    pub const PREFER_HARDWARE: &str = "prefer-hardware";
    pub const PREFER_SOFTWARE: &str = "prefer-software";
}

#[derive(Clone, Debug)]
pub struct AlphaOption {
    inner: emlite::Val,
}
impl FromVal for AlphaOption {
    fn from_val(v: &emlite::Val) -> Self {
        AlphaOption { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AlphaOption {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AlphaOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AlphaOption> for emlite::Val {
    fn from(s: AlphaOption) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AlphaOption {
    pub const KEEP: &str = "keep";
    pub const DISCARD: &str = "discard";
}

#[derive(Clone, Debug)]
pub struct LatencyMode {
    inner: emlite::Val,
}
impl FromVal for LatencyMode {
    fn from_val(v: &emlite::Val) -> Self {
        LatencyMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LatencyMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LatencyMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LatencyMode> for emlite::Val {
    fn from(s: LatencyMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LatencyMode {
    pub const QUALITY: &str = "quality";
    pub const REALTIME: &str = "realtime";
}

#[derive(Clone, Debug)]
pub struct VideoEncoderBitrateMode {
    inner: emlite::Val,
}
impl FromVal for VideoEncoderBitrateMode {
    fn from_val(v: &emlite::Val) -> Self {
        VideoEncoderBitrateMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for VideoEncoderBitrateMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoEncoderBitrateMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoEncoderBitrateMode> for emlite::Val {
    fn from(s: VideoEncoderBitrateMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoEncoderBitrateMode {
    pub const CONSTANT: &str = "constant";
    pub const VARIABLE: &str = "variable";
    pub const QUANTIZER: &str = "quantizer";
}

#[derive(Clone, Debug)]
pub struct CodecState {
    inner: emlite::Val,
}
impl FromVal for CodecState {
    fn from_val(v: &emlite::Val) -> Self {
        CodecState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CodecState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CodecState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CodecState> for emlite::Val {
    fn from(s: CodecState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CodecState {
    pub const UNCONFIGURED: &str = "unconfigured";
    pub const CONFIGURED: &str = "configured";
    pub const CLOSED: &str = "closed";
}

#[derive(Clone, Debug)]
pub struct EncodedAudioChunkType {
    inner: emlite::Val,
}
impl FromVal for EncodedAudioChunkType {
    fn from_val(v: &emlite::Val) -> Self {
        EncodedAudioChunkType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for EncodedAudioChunkType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EncodedAudioChunkType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EncodedAudioChunkType> for emlite::Val {
    fn from(s: EncodedAudioChunkType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EncodedAudioChunkType {
    pub const KEY: &str = "key";
    pub const DELTA: &str = "delta";
}

#[derive(Clone, Debug)]
pub struct EncodedVideoChunkType {
    inner: emlite::Val,
}
impl FromVal for EncodedVideoChunkType {
    fn from_val(v: &emlite::Val) -> Self {
        EncodedVideoChunkType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for EncodedVideoChunkType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EncodedVideoChunkType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EncodedVideoChunkType> for emlite::Val {
    fn from(s: EncodedVideoChunkType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EncodedVideoChunkType {
    pub const KEY: &str = "key";
    pub const DELTA: &str = "delta";
}

#[derive(Clone, Debug)]
pub struct AudioSampleFormat {
    inner: emlite::Val,
}
impl FromVal for AudioSampleFormat {
    fn from_val(v: &emlite::Val) -> Self {
        AudioSampleFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AudioSampleFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioSampleFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioSampleFormat> for emlite::Val {
    fn from(s: AudioSampleFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioSampleFormat {
    pub const U8_: &str = "u8";
    pub const S16: &str = "s16";
    pub const S32: &str = "s32";
    pub const F32_: &str = "f32";
    pub const U8_PLANAR: &str = "u8-planar";
    pub const S16_PLANAR: &str = "s16-planar";
    pub const S32_PLANAR: &str = "s32-planar";
    pub const F32_PLANAR: &str = "f32-planar";
}

#[derive(Clone, Debug)]
pub struct VideoPixelFormat {
    inner: emlite::Val,
}
impl FromVal for VideoPixelFormat {
    fn from_val(v: &emlite::Val) -> Self {
        VideoPixelFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for VideoPixelFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoPixelFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoPixelFormat> for emlite::Val {
    fn from(s: VideoPixelFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoPixelFormat {
    pub const I420: &str = "I420";
    pub const I420_P10: &str = "I420P10";
    pub const I420_P12: &str = "I420P12";
    pub const I420_A: &str = "I420A";
    pub const I420_AP10: &str = "I420AP10";
    pub const I420_AP12: &str = "I420AP12";
    pub const I422: &str = "I422";
    pub const I422_P10: &str = "I422P10";
    pub const I422_P12: &str = "I422P12";
    pub const I422_A: &str = "I422A";
    pub const I422_AP10: &str = "I422AP10";
    pub const I422_AP12: &str = "I422AP12";
    pub const I444: &str = "I444";
    pub const I444_P10: &str = "I444P10";
    pub const I444_P12: &str = "I444P12";
    pub const I444_A: &str = "I444A";
    pub const I444_AP10: &str = "I444AP10";
    pub const I444_AP12: &str = "I444AP12";
    pub const NV12: &str = "NV12";
    pub const RGBA: &str = "RGBA";
    pub const RGBX: &str = "RGBX";
    pub const BGRA: &str = "BGRA";
    pub const BGRX: &str = "BGRX";
}

#[derive(Clone, Debug)]
pub struct VideoColorPrimaries {
    inner: emlite::Val,
}
impl FromVal for VideoColorPrimaries {
    fn from_val(v: &emlite::Val) -> Self {
        VideoColorPrimaries { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for VideoColorPrimaries {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoColorPrimaries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoColorPrimaries> for emlite::Val {
    fn from(s: VideoColorPrimaries) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoColorPrimaries {
    pub const BT709: &str = "bt709";
    pub const BT470BG: &str = "bt470bg";
    pub const SMPTE170M: &str = "smpte170m";
    pub const BT2020: &str = "bt2020";
    pub const SMPTE432: &str = "smpte432";
}

#[derive(Clone, Debug)]
pub struct VideoTransferCharacteristics {
    inner: emlite::Val,
}
impl FromVal for VideoTransferCharacteristics {
    fn from_val(v: &emlite::Val) -> Self {
        VideoTransferCharacteristics { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for VideoTransferCharacteristics {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoTransferCharacteristics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoTransferCharacteristics> for emlite::Val {
    fn from(s: VideoTransferCharacteristics) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoTransferCharacteristics {
    pub const BT709: &str = "bt709";
    pub const SMPTE170M: &str = "smpte170m";
    pub const IEC61966_2_1: &str = "iec61966-2-1";
    pub const LINEAR: &str = "linear";
    pub const PQ: &str = "pq";
    pub const HLG: &str = "hlg";
}

#[derive(Clone, Debug)]
pub struct VideoMatrixCoefficients {
    inner: emlite::Val,
}
impl FromVal for VideoMatrixCoefficients {
    fn from_val(v: &emlite::Val) -> Self {
        VideoMatrixCoefficients { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for VideoMatrixCoefficients {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoMatrixCoefficients {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoMatrixCoefficients> for emlite::Val {
    fn from(s: VideoMatrixCoefficients) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoMatrixCoefficients {
    pub const RGB: &str = "rgb";
    pub const BT709: &str = "bt709";
    pub const BT470BG: &str = "bt470bg";
    pub const SMPTE170M: &str = "smpte170m";
    pub const BT2020_NCL: &str = "bt2020-ncl";
}

#[derive(Clone, Debug)]
pub struct KeyType {
    inner: emlite::Val,
}
impl FromVal for KeyType {
    fn from_val(v: &emlite::Val) -> Self {
        KeyType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for KeyType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for KeyType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<KeyType> for emlite::Val {
    fn from(s: KeyType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl KeyType {
    pub const PUBLIC: &str = "public";
    pub const PRIVATE: &str = "private";
    pub const SECRET: &str = "secret";
}

#[derive(Clone, Debug)]
pub struct KeyUsage {
    inner: emlite::Val,
}
impl FromVal for KeyUsage {
    fn from_val(v: &emlite::Val) -> Self {
        KeyUsage { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for KeyUsage {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for KeyUsage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<KeyUsage> for emlite::Val {
    fn from(s: KeyUsage) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl KeyUsage {
    pub const ENCRYPT: &str = "encrypt";
    pub const DECRYPT: &str = "decrypt";
    pub const SIGN: &str = "sign";
    pub const VERIFY: &str = "verify";
    pub const DERIVE_KEY: &str = "deriveKey";
    pub const DERIVE_BITS: &str = "deriveBits";
    pub const WRAP_KEY: &str = "wrapKey";
    pub const UNWRAP_KEY: &str = "unwrapKey";
}

#[derive(Clone, Debug)]
pub struct KeyFormat {
    inner: emlite::Val,
}
impl FromVal for KeyFormat {
    fn from_val(v: &emlite::Val) -> Self {
        KeyFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for KeyFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for KeyFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<KeyFormat> for emlite::Val {
    fn from(s: KeyFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl KeyFormat {
    pub const RAW_: &str = "raw";
    pub const SPKI: &str = "spki";
    pub const PKCS8: &str = "pkcs8";
    pub const JWK: &str = "jwk";
}

#[derive(Clone, Debug)]
pub struct WebGLPowerPreference {
    inner: emlite::Val,
}
impl FromVal for WebGLPowerPreference {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLPowerPreference { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WebGLPowerPreference {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebGLPowerPreference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebGLPowerPreference> for emlite::Val {
    fn from(s: WebGLPowerPreference) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebGLPowerPreference {
    pub const DEFAULT: &str = "default";
    pub const LOW_POWER: &str = "low-power";
    pub const HIGH_PERFORMANCE: &str = "high-performance";
}

#[derive(Clone, Debug)]
pub struct GPUPowerPreference {
    inner: emlite::Val,
}
impl FromVal for GPUPowerPreference {
    fn from_val(v: &emlite::Val) -> Self {
        GPUPowerPreference { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUPowerPreference {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUPowerPreference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUPowerPreference> for emlite::Val {
    fn from(s: GPUPowerPreference) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUPowerPreference {
    pub const LOW_POWER: &str = "low-power";
    pub const HIGH_PERFORMANCE: &str = "high-performance";
}

#[derive(Clone, Debug)]
pub struct GPUFeatureName {
    inner: emlite::Val,
}
impl FromVal for GPUFeatureName {
    fn from_val(v: &emlite::Val) -> Self {
        GPUFeatureName { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUFeatureName {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUFeatureName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUFeatureName> for emlite::Val {
    fn from(s: GPUFeatureName) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUFeatureName {
    pub const CORE_FEATURES_AND_LIMITS: &str = "core-features-and-limits";
    pub const DEPTH_CLIP_CONTROL: &str = "depth-clip-control";
    pub const DEPTH32FLOAT_STENCIL8: &str = "depth32float-stencil8";
    pub const TEXTURE_COMPRESSION_BC: &str = "texture-compression-bc";
    pub const TEXTURE_COMPRESSION_BC_SLICED_3D: &str = "texture-compression-bc-sliced-3d";
    pub const TEXTURE_COMPRESSION_ETC2: &str = "texture-compression-etc2";
    pub const TEXTURE_COMPRESSION_ASTC: &str = "texture-compression-astc";
    pub const TEXTURE_COMPRESSION_ASTC_SLICED_3D: &str = "texture-compression-astc-sliced-3d";
    pub const TIMESTAMP_QUERY: &str = "timestamp-query";
    pub const INDIRECT_FIRST_INSTANCE: &str = "indirect-first-instance";
    pub const SHADER_F16: &str = "shader-f16";
    pub const RG11B10UFLOAT_RENDERABLE: &str = "rg11b10ufloat-renderable";
    pub const BGRA8UNORM_STORAGE: &str = "bgra8unorm-storage";
    pub const FLOAT32_FILTERABLE: &str = "float32-filterable";
    pub const FLOAT32_BLENDABLE: &str = "float32-blendable";
    pub const CLIP_DISTANCES: &str = "clip-distances";
    pub const DUAL_SOURCE_BLENDING: &str = "dual-source-blending";
    pub const SUBGROUPS: &str = "subgroups";
    pub const TEXTURE_FORMATS_TIER1: &str = "texture-formats-tier1";
    pub const TEXTURE_FORMATS_TIER2: &str = "texture-formats-tier2";
}

#[derive(Clone, Debug)]
pub struct GPUBufferMapState {
    inner: emlite::Val,
}
impl FromVal for GPUBufferMapState {
    fn from_val(v: &emlite::Val) -> Self {
        GPUBufferMapState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUBufferMapState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUBufferMapState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUBufferMapState> for emlite::Val {
    fn from(s: GPUBufferMapState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUBufferMapState {
    pub const UNMAPPED: &str = "unmapped";
    pub const PENDING: &str = "pending";
    pub const MAPPED: &str = "mapped";
}

#[derive(Clone, Debug)]
pub struct GPUTextureDimension {
    inner: emlite::Val,
}
impl FromVal for GPUTextureDimension {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTextureDimension { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUTextureDimension {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUTextureDimension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUTextureDimension> for emlite::Val {
    fn from(s: GPUTextureDimension) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUTextureDimension {
    pub const _1D: &str = "1d";
    pub const _2D: &str = "2d";
    pub const _3D: &str = "3d";
}

#[derive(Clone, Debug)]
pub struct GPUTextureViewDimension {
    inner: emlite::Val,
}
impl FromVal for GPUTextureViewDimension {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTextureViewDimension { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUTextureViewDimension {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUTextureViewDimension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUTextureViewDimension> for emlite::Val {
    fn from(s: GPUTextureViewDimension) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUTextureViewDimension {
    pub const _1D: &str = "1d";
    pub const _2D: &str = "2d";
    pub const _2D_ARRAY: &str = "2d-array";
    pub const CUBE: &str = "cube";
    pub const CUBE_ARRAY: &str = "cube-array";
    pub const _3D: &str = "3d";
}

#[derive(Clone, Debug)]
pub struct GPUTextureAspect {
    inner: emlite::Val,
}
impl FromVal for GPUTextureAspect {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTextureAspect { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUTextureAspect {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUTextureAspect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUTextureAspect> for emlite::Val {
    fn from(s: GPUTextureAspect) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUTextureAspect {
    pub const ALL: &str = "all";
    pub const STENCIL_ONLY: &str = "stencil-only";
    pub const DEPTH_ONLY: &str = "depth-only";
}

#[derive(Clone, Debug)]
pub struct GPUTextureFormat {
    inner: emlite::Val,
}
impl FromVal for GPUTextureFormat {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTextureFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUTextureFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUTextureFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUTextureFormat> for emlite::Val {
    fn from(s: GPUTextureFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUTextureFormat {
    pub const R8UNORM: &str = "r8unorm";
    pub const R8SNORM: &str = "r8snorm";
    pub const R8UINT: &str = "r8uint";
    pub const R8SINT: &str = "r8sint";
    pub const R16UNORM: &str = "r16unorm";
    pub const R16SNORM: &str = "r16snorm";
    pub const R16UINT: &str = "r16uint";
    pub const R16SINT: &str = "r16sint";
    pub const R16FLOAT: &str = "r16float";
    pub const RG8UNORM: &str = "rg8unorm";
    pub const RG8SNORM: &str = "rg8snorm";
    pub const RG8UINT: &str = "rg8uint";
    pub const RG8SINT: &str = "rg8sint";
    pub const R32UINT: &str = "r32uint";
    pub const R32SINT: &str = "r32sint";
    pub const R32FLOAT: &str = "r32float";
    pub const RG16UNORM: &str = "rg16unorm";
    pub const RG16SNORM: &str = "rg16snorm";
    pub const RG16UINT: &str = "rg16uint";
    pub const RG16SINT: &str = "rg16sint";
    pub const RG16FLOAT: &str = "rg16float";
    pub const RGBA8UNORM: &str = "rgba8unorm";
    pub const RGBA8UNORM_SRGB: &str = "rgba8unorm-srgb";
    pub const RGBA8SNORM: &str = "rgba8snorm";
    pub const RGBA8UINT: &str = "rgba8uint";
    pub const RGBA8SINT: &str = "rgba8sint";
    pub const BGRA8UNORM: &str = "bgra8unorm";
    pub const BGRA8UNORM_SRGB: &str = "bgra8unorm-srgb";
    pub const RGB9E5UFLOAT: &str = "rgb9e5ufloat";
    pub const RGB10A2UINT: &str = "rgb10a2uint";
    pub const RGB10A2UNORM: &str = "rgb10a2unorm";
    pub const RG11B10UFLOAT: &str = "rg11b10ufloat";
    pub const RG32UINT: &str = "rg32uint";
    pub const RG32SINT: &str = "rg32sint";
    pub const RG32FLOAT: &str = "rg32float";
    pub const RGBA16UNORM: &str = "rgba16unorm";
    pub const RGBA16SNORM: &str = "rgba16snorm";
    pub const RGBA16UINT: &str = "rgba16uint";
    pub const RGBA16SINT: &str = "rgba16sint";
    pub const RGBA16FLOAT: &str = "rgba16float";
    pub const RGBA32UINT: &str = "rgba32uint";
    pub const RGBA32SINT: &str = "rgba32sint";
    pub const RGBA32FLOAT: &str = "rgba32float";
    pub const STENCIL8: &str = "stencil8";
    pub const DEPTH16UNORM: &str = "depth16unorm";
    pub const DEPTH24PLUS: &str = "depth24plus";
    pub const DEPTH24PLUS_STENCIL8: &str = "depth24plus-stencil8";
    pub const DEPTH32FLOAT: &str = "depth32float";
    pub const DEPTH32FLOAT_STENCIL8: &str = "depth32float-stencil8";
    pub const BC1_RGBA_UNORM: &str = "bc1-rgba-unorm";
    pub const BC1_RGBA_UNORM_SRGB: &str = "bc1-rgba-unorm-srgb";
    pub const BC2_RGBA_UNORM: &str = "bc2-rgba-unorm";
    pub const BC2_RGBA_UNORM_SRGB: &str = "bc2-rgba-unorm-srgb";
    pub const BC3_RGBA_UNORM: &str = "bc3-rgba-unorm";
    pub const BC3_RGBA_UNORM_SRGB: &str = "bc3-rgba-unorm-srgb";
    pub const BC4_R_UNORM: &str = "bc4-r-unorm";
    pub const BC4_R_SNORM: &str = "bc4-r-snorm";
    pub const BC5_RG_UNORM: &str = "bc5-rg-unorm";
    pub const BC5_RG_SNORM: &str = "bc5-rg-snorm";
    pub const BC6H_RGB_UFLOAT: &str = "bc6h-rgb-ufloat";
    pub const BC6H_RGB_FLOAT: &str = "bc6h-rgb-float";
    pub const BC7_RGBA_UNORM: &str = "bc7-rgba-unorm";
    pub const BC7_RGBA_UNORM_SRGB: &str = "bc7-rgba-unorm-srgb";
    pub const ETC2_RGB8UNORM: &str = "etc2-rgb8unorm";
    pub const ETC2_RGB8UNORM_SRGB: &str = "etc2-rgb8unorm-srgb";
    pub const ETC2_RGB8A1UNORM: &str = "etc2-rgb8a1unorm";
    pub const ETC2_RGB8A1UNORM_SRGB: &str = "etc2-rgb8a1unorm-srgb";
    pub const ETC2_RGBA8UNORM: &str = "etc2-rgba8unorm";
    pub const ETC2_RGBA8UNORM_SRGB: &str = "etc2-rgba8unorm-srgb";
    pub const EAC_R11UNORM: &str = "eac-r11unorm";
    pub const EAC_R11SNORM: &str = "eac-r11snorm";
    pub const EAC_RG11UNORM: &str = "eac-rg11unorm";
    pub const EAC_RG11SNORM: &str = "eac-rg11snorm";
    pub const ASTC_4X4_UNORM: &str = "astc-4x4-unorm";
    pub const ASTC_4X4_UNORM_SRGB: &str = "astc-4x4-unorm-srgb";
    pub const ASTC_5X4_UNORM: &str = "astc-5x4-unorm";
    pub const ASTC_5X4_UNORM_SRGB: &str = "astc-5x4-unorm-srgb";
    pub const ASTC_5X5_UNORM: &str = "astc-5x5-unorm";
    pub const ASTC_5X5_UNORM_SRGB: &str = "astc-5x5-unorm-srgb";
    pub const ASTC_6X5_UNORM: &str = "astc-6x5-unorm";
    pub const ASTC_6X5_UNORM_SRGB: &str = "astc-6x5-unorm-srgb";
    pub const ASTC_6X6_UNORM: &str = "astc-6x6-unorm";
    pub const ASTC_6X6_UNORM_SRGB: &str = "astc-6x6-unorm-srgb";
    pub const ASTC_8X5_UNORM: &str = "astc-8x5-unorm";
    pub const ASTC_8X5_UNORM_SRGB: &str = "astc-8x5-unorm-srgb";
    pub const ASTC_8X6_UNORM: &str = "astc-8x6-unorm";
    pub const ASTC_8X6_UNORM_SRGB: &str = "astc-8x6-unorm-srgb";
    pub const ASTC_8X8_UNORM: &str = "astc-8x8-unorm";
    pub const ASTC_8X8_UNORM_SRGB: &str = "astc-8x8-unorm-srgb";
    pub const ASTC_10X5_UNORM: &str = "astc-10x5-unorm";
    pub const ASTC_10X5_UNORM_SRGB: &str = "astc-10x5-unorm-srgb";
    pub const ASTC_10X6_UNORM: &str = "astc-10x6-unorm";
    pub const ASTC_10X6_UNORM_SRGB: &str = "astc-10x6-unorm-srgb";
    pub const ASTC_10X8_UNORM: &str = "astc-10x8-unorm";
    pub const ASTC_10X8_UNORM_SRGB: &str = "astc-10x8-unorm-srgb";
    pub const ASTC_10X10_UNORM: &str = "astc-10x10-unorm";
    pub const ASTC_10X10_UNORM_SRGB: &str = "astc-10x10-unorm-srgb";
    pub const ASTC_12X10_UNORM: &str = "astc-12x10-unorm";
    pub const ASTC_12X10_UNORM_SRGB: &str = "astc-12x10-unorm-srgb";
    pub const ASTC_12X12_UNORM: &str = "astc-12x12-unorm";
    pub const ASTC_12X12_UNORM_SRGB: &str = "astc-12x12-unorm-srgb";
}

#[derive(Clone, Debug)]
pub struct GPUAddressMode {
    inner: emlite::Val,
}
impl FromVal for GPUAddressMode {
    fn from_val(v: &emlite::Val) -> Self {
        GPUAddressMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUAddressMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUAddressMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUAddressMode> for emlite::Val {
    fn from(s: GPUAddressMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUAddressMode {
    pub const CLAMP_TO_EDGE: &str = "clamp-to-edge";
    pub const REPEAT: &str = "repeat";
    pub const MIRROR_REPEAT: &str = "mirror-repeat";
}

#[derive(Clone, Debug)]
pub struct GPUFilterMode {
    inner: emlite::Val,
}
impl FromVal for GPUFilterMode {
    fn from_val(v: &emlite::Val) -> Self {
        GPUFilterMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUFilterMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUFilterMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUFilterMode> for emlite::Val {
    fn from(s: GPUFilterMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUFilterMode {
    pub const NEAREST: &str = "nearest";
    pub const LINEAR: &str = "linear";
}

#[derive(Clone, Debug)]
pub struct GPUMipmapFilterMode {
    inner: emlite::Val,
}
impl FromVal for GPUMipmapFilterMode {
    fn from_val(v: &emlite::Val) -> Self {
        GPUMipmapFilterMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUMipmapFilterMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUMipmapFilterMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUMipmapFilterMode> for emlite::Val {
    fn from(s: GPUMipmapFilterMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUMipmapFilterMode {
    pub const NEAREST: &str = "nearest";
    pub const LINEAR: &str = "linear";
}

#[derive(Clone, Debug)]
pub struct GPUCompareFunction {
    inner: emlite::Val,
}
impl FromVal for GPUCompareFunction {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCompareFunction { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUCompareFunction {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUCompareFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUCompareFunction> for emlite::Val {
    fn from(s: GPUCompareFunction) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUCompareFunction {
    pub const NEVER: &str = "never";
    pub const LESS: &str = "less";
    pub const EQUAL: &str = "equal";
    pub const LESS_EQUAL: &str = "less-equal";
    pub const GREATER: &str = "greater";
    pub const NOT_EQUAL: &str = "not-equal";
    pub const GREATER_EQUAL: &str = "greater-equal";
    pub const ALWAYS: &str = "always";
}

#[derive(Clone, Debug)]
pub struct GPUBufferBindingType {
    inner: emlite::Val,
}
impl FromVal for GPUBufferBindingType {
    fn from_val(v: &emlite::Val) -> Self {
        GPUBufferBindingType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUBufferBindingType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUBufferBindingType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUBufferBindingType> for emlite::Val {
    fn from(s: GPUBufferBindingType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUBufferBindingType {
    pub const UNIFORM: &str = "uniform";
    pub const STORAGE: &str = "storage";
    pub const READ_ONLY_STORAGE: &str = "read-only-storage";
}

#[derive(Clone, Debug)]
pub struct GPUSamplerBindingType {
    inner: emlite::Val,
}
impl FromVal for GPUSamplerBindingType {
    fn from_val(v: &emlite::Val) -> Self {
        GPUSamplerBindingType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUSamplerBindingType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUSamplerBindingType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUSamplerBindingType> for emlite::Val {
    fn from(s: GPUSamplerBindingType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUSamplerBindingType {
    pub const FILTERING: &str = "filtering";
    pub const NON_FILTERING: &str = "non-filtering";
    pub const COMPARISON: &str = "comparison";
}

#[derive(Clone, Debug)]
pub struct GPUTextureSampleType {
    inner: emlite::Val,
}
impl FromVal for GPUTextureSampleType {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTextureSampleType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUTextureSampleType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUTextureSampleType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUTextureSampleType> for emlite::Val {
    fn from(s: GPUTextureSampleType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUTextureSampleType {
    pub const FLOAT: &str = "float";
    pub const UNFILTERABLE_FLOAT: &str = "unfilterable-float";
    pub const DEPTH: &str = "depth";
    pub const SINT: &str = "sint";
    pub const UINT: &str = "uint";
}

#[derive(Clone, Debug)]
pub struct GPUStorageTextureAccess {
    inner: emlite::Val,
}
impl FromVal for GPUStorageTextureAccess {
    fn from_val(v: &emlite::Val) -> Self {
        GPUStorageTextureAccess { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUStorageTextureAccess {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUStorageTextureAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUStorageTextureAccess> for emlite::Val {
    fn from(s: GPUStorageTextureAccess) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUStorageTextureAccess {
    pub const WRITE_ONLY: &str = "write-only";
    pub const READ_ONLY: &str = "read-only";
    pub const READ_WRITE: &str = "read-write";
}

#[derive(Clone, Debug)]
pub struct GPUCompilationMessageType {
    inner: emlite::Val,
}
impl FromVal for GPUCompilationMessageType {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCompilationMessageType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUCompilationMessageType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUCompilationMessageType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUCompilationMessageType> for emlite::Val {
    fn from(s: GPUCompilationMessageType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUCompilationMessageType {
    pub const ERROR: &str = "error";
    pub const WARNING: &str = "warning";
    pub const INFO: &str = "info";
}

#[derive(Clone, Debug)]
pub struct GPUPipelineErrorReason {
    inner: emlite::Val,
}
impl FromVal for GPUPipelineErrorReason {
    fn from_val(v: &emlite::Val) -> Self {
        GPUPipelineErrorReason { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUPipelineErrorReason {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUPipelineErrorReason {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUPipelineErrorReason> for emlite::Val {
    fn from(s: GPUPipelineErrorReason) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUPipelineErrorReason {
    pub const VALIDATION: &str = "validation";
    pub const INTERNAL: &str = "internal";
}

#[derive(Clone, Debug)]
pub struct GPUAutoLayoutMode {
    inner: emlite::Val,
}
impl FromVal for GPUAutoLayoutMode {
    fn from_val(v: &emlite::Val) -> Self {
        GPUAutoLayoutMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUAutoLayoutMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUAutoLayoutMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUAutoLayoutMode> for emlite::Val {
    fn from(s: GPUAutoLayoutMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUAutoLayoutMode {
    pub const AUTO: &str = "auto";
}

#[derive(Clone, Debug)]
pub struct GPUPrimitiveTopology {
    inner: emlite::Val,
}
impl FromVal for GPUPrimitiveTopology {
    fn from_val(v: &emlite::Val) -> Self {
        GPUPrimitiveTopology { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUPrimitiveTopology {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUPrimitiveTopology {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUPrimitiveTopology> for emlite::Val {
    fn from(s: GPUPrimitiveTopology) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUPrimitiveTopology {
    pub const POINT_LIST: &str = "point-list";
    pub const LINE_LIST: &str = "line-list";
    pub const LINE_STRIP: &str = "line-strip";
    pub const TRIANGLE_LIST: &str = "triangle-list";
    pub const TRIANGLE_STRIP: &str = "triangle-strip";
}

#[derive(Clone, Debug)]
pub struct GPUFrontFace {
    inner: emlite::Val,
}
impl FromVal for GPUFrontFace {
    fn from_val(v: &emlite::Val) -> Self {
        GPUFrontFace { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUFrontFace {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUFrontFace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUFrontFace> for emlite::Val {
    fn from(s: GPUFrontFace) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUFrontFace {
    pub const CCW: &str = "ccw";
    pub const CW: &str = "cw";
}

#[derive(Clone, Debug)]
pub struct GPUCullMode {
    inner: emlite::Val,
}
impl FromVal for GPUCullMode {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCullMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUCullMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUCullMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUCullMode> for emlite::Val {
    fn from(s: GPUCullMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUCullMode {
    pub const NONE: &str = "none";
    pub const FRONT: &str = "front";
    pub const BACK: &str = "back";
}

#[derive(Clone, Debug)]
pub struct GPUBlendFactor {
    inner: emlite::Val,
}
impl FromVal for GPUBlendFactor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUBlendFactor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUBlendFactor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUBlendFactor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUBlendFactor> for emlite::Val {
    fn from(s: GPUBlendFactor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUBlendFactor {
    pub const ZERO: &str = "zero";
    pub const ONE: &str = "one";
    pub const SRC: &str = "src";
    pub const ONE_MINUS_SRC: &str = "one-minus-src";
    pub const SRC_ALPHA: &str = "src-alpha";
    pub const ONE_MINUS_SRC_ALPHA: &str = "one-minus-src-alpha";
    pub const DST: &str = "dst";
    pub const ONE_MINUS_DST: &str = "one-minus-dst";
    pub const DST_ALPHA: &str = "dst-alpha";
    pub const ONE_MINUS_DST_ALPHA: &str = "one-minus-dst-alpha";
    pub const SRC_ALPHA_SATURATED: &str = "src-alpha-saturated";
    pub const CONSTANT: &str = "constant";
    pub const ONE_MINUS_CONSTANT: &str = "one-minus-constant";
    pub const SRC1: &str = "src1";
    pub const ONE_MINUS_SRC1: &str = "one-minus-src1";
    pub const SRC1_ALPHA: &str = "src1-alpha";
    pub const ONE_MINUS_SRC1_ALPHA: &str = "one-minus-src1-alpha";
}

#[derive(Clone, Debug)]
pub struct GPUBlendOperation {
    inner: emlite::Val,
}
impl FromVal for GPUBlendOperation {
    fn from_val(v: &emlite::Val) -> Self {
        GPUBlendOperation { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUBlendOperation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUBlendOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUBlendOperation> for emlite::Val {
    fn from(s: GPUBlendOperation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUBlendOperation {
    pub const ADD: &str = "add";
    pub const SUBTRACT: &str = "subtract";
    pub const REVERSE_SUBTRACT: &str = "reverse-subtract";
    pub const MIN: &str = "min";
    pub const MAX: &str = "max";
}

#[derive(Clone, Debug)]
pub struct GPUStencilOperation {
    inner: emlite::Val,
}
impl FromVal for GPUStencilOperation {
    fn from_val(v: &emlite::Val) -> Self {
        GPUStencilOperation { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUStencilOperation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUStencilOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUStencilOperation> for emlite::Val {
    fn from(s: GPUStencilOperation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUStencilOperation {
    pub const KEEP: &str = "keep";
    pub const ZERO: &str = "zero";
    pub const REPLACE: &str = "replace";
    pub const INVERT: &str = "invert";
    pub const INCREMENT_CLAMP: &str = "increment-clamp";
    pub const DECREMENT_CLAMP: &str = "decrement-clamp";
    pub const INCREMENT_WRAP: &str = "increment-wrap";
    pub const DECREMENT_WRAP: &str = "decrement-wrap";
}

#[derive(Clone, Debug)]
pub struct GPUIndexFormat {
    inner: emlite::Val,
}
impl FromVal for GPUIndexFormat {
    fn from_val(v: &emlite::Val) -> Self {
        GPUIndexFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUIndexFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUIndexFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUIndexFormat> for emlite::Val {
    fn from(s: GPUIndexFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUIndexFormat {
    pub const UINT16: &str = "uint16";
    pub const UINT32: &str = "uint32";
}

#[derive(Clone, Debug)]
pub struct GPUVertexFormat {
    inner: emlite::Val,
}
impl FromVal for GPUVertexFormat {
    fn from_val(v: &emlite::Val) -> Self {
        GPUVertexFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUVertexFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUVertexFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUVertexFormat> for emlite::Val {
    fn from(s: GPUVertexFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUVertexFormat {
    pub const UINT8: &str = "uint8";
    pub const UINT8X2: &str = "uint8x2";
    pub const UINT8X4: &str = "uint8x4";
    pub const SINT8: &str = "sint8";
    pub const SINT8X2: &str = "sint8x2";
    pub const SINT8X4: &str = "sint8x4";
    pub const UNORM8: &str = "unorm8";
    pub const UNORM8X2: &str = "unorm8x2";
    pub const UNORM8X4: &str = "unorm8x4";
    pub const SNORM8: &str = "snorm8";
    pub const SNORM8X2: &str = "snorm8x2";
    pub const SNORM8X4: &str = "snorm8x4";
    pub const UINT16: &str = "uint16";
    pub const UINT16X2: &str = "uint16x2";
    pub const UINT16X4: &str = "uint16x4";
    pub const SINT16: &str = "sint16";
    pub const SINT16X2: &str = "sint16x2";
    pub const SINT16X4: &str = "sint16x4";
    pub const UNORM16: &str = "unorm16";
    pub const UNORM16X2: &str = "unorm16x2";
    pub const UNORM16X4: &str = "unorm16x4";
    pub const SNORM16: &str = "snorm16";
    pub const SNORM16X2: &str = "snorm16x2";
    pub const SNORM16X4: &str = "snorm16x4";
    pub const FLOAT16: &str = "float16";
    pub const FLOAT16X2: &str = "float16x2";
    pub const FLOAT16X4: &str = "float16x4";
    pub const FLOAT32: &str = "float32";
    pub const FLOAT32X2: &str = "float32x2";
    pub const FLOAT32X3: &str = "float32x3";
    pub const FLOAT32X4: &str = "float32x4";
    pub const UINT32: &str = "uint32";
    pub const UINT32X2: &str = "uint32x2";
    pub const UINT32X3: &str = "uint32x3";
    pub const UINT32X4: &str = "uint32x4";
    pub const SINT32: &str = "sint32";
    pub const SINT32X2: &str = "sint32x2";
    pub const SINT32X3: &str = "sint32x3";
    pub const SINT32X4: &str = "sint32x4";
    pub const UNORM10_10_10_2: &str = "unorm10-10-10-2";
    pub const UNORM8X4_BGRA: &str = "unorm8x4-bgra";
}

#[derive(Clone, Debug)]
pub struct GPUVertexStepMode {
    inner: emlite::Val,
}
impl FromVal for GPUVertexStepMode {
    fn from_val(v: &emlite::Val) -> Self {
        GPUVertexStepMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUVertexStepMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUVertexStepMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUVertexStepMode> for emlite::Val {
    fn from(s: GPUVertexStepMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUVertexStepMode {
    pub const VERTEX: &str = "vertex";
    pub const INSTANCE: &str = "instance";
}

#[derive(Clone, Debug)]
pub struct GPULoadOp {
    inner: emlite::Val,
}
impl FromVal for GPULoadOp {
    fn from_val(v: &emlite::Val) -> Self {
        GPULoadOp { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPULoadOp {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPULoadOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPULoadOp> for emlite::Val {
    fn from(s: GPULoadOp) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPULoadOp {
    pub const LOAD: &str = "load";
    pub const CLEAR: &str = "clear";
}

#[derive(Clone, Debug)]
pub struct GPUStoreOp {
    inner: emlite::Val,
}
impl FromVal for GPUStoreOp {
    fn from_val(v: &emlite::Val) -> Self {
        GPUStoreOp { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUStoreOp {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUStoreOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUStoreOp> for emlite::Val {
    fn from(s: GPUStoreOp) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUStoreOp {
    pub const STORE: &str = "store";
    pub const DISCARD: &str = "discard";
}

#[derive(Clone, Debug)]
pub struct GPUQueryType {
    inner: emlite::Val,
}
impl FromVal for GPUQueryType {
    fn from_val(v: &emlite::Val) -> Self {
        GPUQueryType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUQueryType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUQueryType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUQueryType> for emlite::Val {
    fn from(s: GPUQueryType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUQueryType {
    pub const OCCLUSION: &str = "occlusion";
    pub const TIMESTAMP: &str = "timestamp";
}

#[derive(Clone, Debug)]
pub struct GPUCanvasAlphaMode {
    inner: emlite::Val,
}
impl FromVal for GPUCanvasAlphaMode {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCanvasAlphaMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUCanvasAlphaMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUCanvasAlphaMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUCanvasAlphaMode> for emlite::Val {
    fn from(s: GPUCanvasAlphaMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUCanvasAlphaMode {
    pub const OPAQUE: &str = "opaque";
    pub const PREMULTIPLIED: &str = "premultiplied";
}

#[derive(Clone, Debug)]
pub struct GPUCanvasToneMappingMode {
    inner: emlite::Val,
}
impl FromVal for GPUCanvasToneMappingMode {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCanvasToneMappingMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUCanvasToneMappingMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUCanvasToneMappingMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUCanvasToneMappingMode> for emlite::Val {
    fn from(s: GPUCanvasToneMappingMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUCanvasToneMappingMode {
    pub const STANDARD: &str = "standard";
    pub const EXTENDED: &str = "extended";
}

#[derive(Clone, Debug)]
pub struct GPUDeviceLostReason {
    inner: emlite::Val,
}
impl FromVal for GPUDeviceLostReason {
    fn from_val(v: &emlite::Val) -> Self {
        GPUDeviceLostReason { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUDeviceLostReason {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUDeviceLostReason {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUDeviceLostReason> for emlite::Val {
    fn from(s: GPUDeviceLostReason) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUDeviceLostReason {
    pub const UNKNOWN: &str = "unknown";
    pub const DESTROYED: &str = "destroyed";
}

#[derive(Clone, Debug)]
pub struct GPUErrorFilter {
    inner: emlite::Val,
}
impl FromVal for GPUErrorFilter {
    fn from_val(v: &emlite::Val) -> Self {
        GPUErrorFilter { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPUErrorFilter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUErrorFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUErrorFilter> for emlite::Val {
    fn from(s: GPUErrorFilter) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUErrorFilter {
    pub const VALIDATION: &str = "validation";
    pub const OUT_OF_MEMORY: &str = "out-of-memory";
    pub const INTERNAL: &str = "internal";
}

#[derive(Clone, Debug)]
pub struct HIDUnitSystem {
    inner: emlite::Val,
}
impl FromVal for HIDUnitSystem {
    fn from_val(v: &emlite::Val) -> Self {
        HIDUnitSystem { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HIDUnitSystem {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HIDUnitSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HIDUnitSystem> for emlite::Val {
    fn from(s: HIDUnitSystem) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HIDUnitSystem {
    pub const NONE: &str = "none";
    pub const SI_LINEAR: &str = "si-linear";
    pub const SI_ROTATION: &str = "si-rotation";
    pub const ENGLISH_LINEAR: &str = "english-linear";
    pub const ENGLISH_ROTATION: &str = "english-rotation";
    pub const VENDOR_DEFINED: &str = "vendor-defined";
    pub const RESERVED: &str = "reserved";
}

#[derive(Clone, Debug)]
pub struct MIDIPortType {
    inner: emlite::Val,
}
impl FromVal for MIDIPortType {
    fn from_val(v: &emlite::Val) -> Self {
        MIDIPortType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MIDIPortType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MIDIPortType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MIDIPortType> for emlite::Val {
    fn from(s: MIDIPortType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MIDIPortType {
    pub const INPUT: &str = "input";
    pub const OUTPUT: &str = "output";
}

#[derive(Clone, Debug)]
pub struct MIDIPortDeviceState {
    inner: emlite::Val,
}
impl FromVal for MIDIPortDeviceState {
    fn from_val(v: &emlite::Val) -> Self {
        MIDIPortDeviceState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MIDIPortDeviceState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MIDIPortDeviceState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MIDIPortDeviceState> for emlite::Val {
    fn from(s: MIDIPortDeviceState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MIDIPortDeviceState {
    pub const DISCONNECTED: &str = "disconnected";
    pub const CONNECTED: &str = "connected";
}

#[derive(Clone, Debug)]
pub struct MIDIPortConnectionState {
    inner: emlite::Val,
}
impl FromVal for MIDIPortConnectionState {
    fn from_val(v: &emlite::Val) -> Self {
        MIDIPortConnectionState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MIDIPortConnectionState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MIDIPortConnectionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MIDIPortConnectionState> for emlite::Val {
    fn from(s: MIDIPortConnectionState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MIDIPortConnectionState {
    pub const OPEN: &str = "open";
    pub const CLOSED: &str = "closed";
    pub const PENDING: &str = "pending";
}

#[derive(Clone, Debug)]
pub struct MLPowerPreference {
    inner: emlite::Val,
}
impl FromVal for MLPowerPreference {
    fn from_val(v: &emlite::Val) -> Self {
        MLPowerPreference { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLPowerPreference {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLPowerPreference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLPowerPreference> for emlite::Val {
    fn from(s: MLPowerPreference) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLPowerPreference {
    pub const DEFAULT: &str = "default";
    pub const HIGH_PERFORMANCE: &str = "high-performance";
    pub const LOW_POWER: &str = "low-power";
}

#[derive(Clone, Debug)]
pub struct MLInputOperandLayout {
    inner: emlite::Val,
}
impl FromVal for MLInputOperandLayout {
    fn from_val(v: &emlite::Val) -> Self {
        MLInputOperandLayout { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLInputOperandLayout {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLInputOperandLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLInputOperandLayout> for emlite::Val {
    fn from(s: MLInputOperandLayout) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLInputOperandLayout {
    pub const NCHW: &str = "nchw";
    pub const NHWC: &str = "nhwc";
}

#[derive(Clone, Debug)]
pub struct MLOperandDataType {
    inner: emlite::Val,
}
impl FromVal for MLOperandDataType {
    fn from_val(v: &emlite::Val) -> Self {
        MLOperandDataType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLOperandDataType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLOperandDataType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLOperandDataType> for emlite::Val {
    fn from(s: MLOperandDataType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLOperandDataType {
    pub const FLOAT32: &str = "float32";
    pub const FLOAT16: &str = "float16";
    pub const INT32: &str = "int32";
    pub const UINT32: &str = "uint32";
    pub const INT64: &str = "int64";
    pub const UINT64: &str = "uint64";
    pub const INT8: &str = "int8";
    pub const UINT8: &str = "uint8";
}

#[derive(Clone, Debug)]
pub struct MLConv2dFilterOperandLayout {
    inner: emlite::Val,
}
impl FromVal for MLConv2dFilterOperandLayout {
    fn from_val(v: &emlite::Val) -> Self {
        MLConv2dFilterOperandLayout { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLConv2dFilterOperandLayout {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLConv2dFilterOperandLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLConv2dFilterOperandLayout> for emlite::Val {
    fn from(s: MLConv2dFilterOperandLayout) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLConv2dFilterOperandLayout {
    pub const OIHW: &str = "oihw";
    pub const HWIO: &str = "hwio";
    pub const OHWI: &str = "ohwi";
    pub const IHWO: &str = "ihwo";
}

#[derive(Clone, Debug)]
pub struct MLConvTranspose2dFilterOperandLayout {
    inner: emlite::Val,
}
impl FromVal for MLConvTranspose2dFilterOperandLayout {
    fn from_val(v: &emlite::Val) -> Self {
        MLConvTranspose2dFilterOperandLayout { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLConvTranspose2dFilterOperandLayout {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLConvTranspose2dFilterOperandLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLConvTranspose2dFilterOperandLayout> for emlite::Val {
    fn from(s: MLConvTranspose2dFilterOperandLayout) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLConvTranspose2dFilterOperandLayout {
    pub const IOHW: &str = "iohw";
    pub const HWOI: &str = "hwoi";
    pub const OHWI: &str = "ohwi";
}

#[derive(Clone, Debug)]
pub struct MLGruWeightLayout {
    inner: emlite::Val,
}
impl FromVal for MLGruWeightLayout {
    fn from_val(v: &emlite::Val) -> Self {
        MLGruWeightLayout { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLGruWeightLayout {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLGruWeightLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLGruWeightLayout> for emlite::Val {
    fn from(s: MLGruWeightLayout) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLGruWeightLayout {
    pub const ZRN: &str = "zrn";
    pub const RZN: &str = "rzn";
}

#[derive(Clone, Debug)]
pub struct MLRecurrentNetworkActivation {
    inner: emlite::Val,
}
impl FromVal for MLRecurrentNetworkActivation {
    fn from_val(v: &emlite::Val) -> Self {
        MLRecurrentNetworkActivation { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLRecurrentNetworkActivation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLRecurrentNetworkActivation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLRecurrentNetworkActivation> for emlite::Val {
    fn from(s: MLRecurrentNetworkActivation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLRecurrentNetworkActivation {
    pub const RELU: &str = "relu";
    pub const SIGMOID: &str = "sigmoid";
    pub const TANH: &str = "tanh";
}

#[derive(Clone, Debug)]
pub struct MLRecurrentNetworkDirection {
    inner: emlite::Val,
}
impl FromVal for MLRecurrentNetworkDirection {
    fn from_val(v: &emlite::Val) -> Self {
        MLRecurrentNetworkDirection { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLRecurrentNetworkDirection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLRecurrentNetworkDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLRecurrentNetworkDirection> for emlite::Val {
    fn from(s: MLRecurrentNetworkDirection) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLRecurrentNetworkDirection {
    pub const FORWARD: &str = "forward";
    pub const BACKWARD: &str = "backward";
    pub const BOTH: &str = "both";
}

#[derive(Clone, Debug)]
pub struct MLLstmWeightLayout {
    inner: emlite::Val,
}
impl FromVal for MLLstmWeightLayout {
    fn from_val(v: &emlite::Val) -> Self {
        MLLstmWeightLayout { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLLstmWeightLayout {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLLstmWeightLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLLstmWeightLayout> for emlite::Val {
    fn from(s: MLLstmWeightLayout) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLLstmWeightLayout {
    pub const IOFG: &str = "iofg";
    pub const IFGO: &str = "ifgo";
}

#[derive(Clone, Debug)]
pub struct MLPaddingMode {
    inner: emlite::Val,
}
impl FromVal for MLPaddingMode {
    fn from_val(v: &emlite::Val) -> Self {
        MLPaddingMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLPaddingMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLPaddingMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLPaddingMode> for emlite::Val {
    fn from(s: MLPaddingMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLPaddingMode {
    pub const CONSTANT: &str = "constant";
    pub const EDGE: &str = "edge";
    pub const REFLECTION: &str = "reflection";
}

#[derive(Clone, Debug)]
pub struct MLRoundingType {
    inner: emlite::Val,
}
impl FromVal for MLRoundingType {
    fn from_val(v: &emlite::Val) -> Self {
        MLRoundingType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLRoundingType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLRoundingType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLRoundingType> for emlite::Val {
    fn from(s: MLRoundingType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLRoundingType {
    pub const FLOOR: &str = "floor";
    pub const CEIL: &str = "ceil";
}

#[derive(Clone, Debug)]
pub struct MLInterpolationMode {
    inner: emlite::Val,
}
impl FromVal for MLInterpolationMode {
    fn from_val(v: &emlite::Val) -> Self {
        MLInterpolationMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MLInterpolationMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLInterpolationMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLInterpolationMode> for emlite::Val {
    fn from(s: MLInterpolationMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLInterpolationMode {
    pub const NEAREST_NEIGHBOR: &str = "nearest-neighbor";
    pub const LINEAR: &str = "linear";
}

#[derive(Clone, Debug)]
pub struct SFrameTransformRole {
    inner: emlite::Val,
}
impl FromVal for SFrameTransformRole {
    fn from_val(v: &emlite::Val) -> Self {
        SFrameTransformRole { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SFrameTransformRole {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SFrameTransformRole {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SFrameTransformRole> for emlite::Val {
    fn from(s: SFrameTransformRole) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SFrameTransformRole {
    pub const ENCRYPT: &str = "encrypt";
    pub const DECRYPT: &str = "decrypt";
}

#[derive(Clone, Debug)]
pub struct SFrameTransformErrorEventType {
    inner: emlite::Val,
}
impl FromVal for SFrameTransformErrorEventType {
    fn from_val(v: &emlite::Val) -> Self {
        SFrameTransformErrorEventType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SFrameTransformErrorEventType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SFrameTransformErrorEventType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SFrameTransformErrorEventType> for emlite::Val {
    fn from(s: SFrameTransformErrorEventType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SFrameTransformErrorEventType {
    pub const AUTHENTICATION: &str = "authentication";
    pub const KEY_ID: &str = "keyID";
    pub const SYNTAX: &str = "syntax";
}

#[derive(Clone, Debug)]
pub struct RTCEncodedVideoFrameType {
    inner: emlite::Val,
}
impl FromVal for RTCEncodedVideoFrameType {
    fn from_val(v: &emlite::Val) -> Self {
        RTCEncodedVideoFrameType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCEncodedVideoFrameType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCEncodedVideoFrameType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCEncodedVideoFrameType> for emlite::Val {
    fn from(s: RTCEncodedVideoFrameType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCEncodedVideoFrameType {
    pub const EMPTY: &str = "empty";
    pub const KEY: &str = "key";
    pub const DELTA: &str = "delta";
}

#[derive(Clone, Debug)]
pub struct RTCErrorDetailTypeIdp {
    inner: emlite::Val,
}
impl FromVal for RTCErrorDetailTypeIdp {
    fn from_val(v: &emlite::Val) -> Self {
        RTCErrorDetailTypeIdp { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCErrorDetailTypeIdp {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCErrorDetailTypeIdp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCErrorDetailTypeIdp> for emlite::Val {
    fn from(s: RTCErrorDetailTypeIdp) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCErrorDetailTypeIdp {
    pub const IDP_BAD_SCRIPT_FAILURE: &str = "idp-bad-script-failure";
    pub const IDP_EXECUTION_FAILURE: &str = "idp-execution-failure";
    pub const IDP_LOAD_FAILURE: &str = "idp-load-failure";
    pub const IDP_NEED_LOGIN: &str = "idp-need-login";
    pub const IDP_TIMEOUT: &str = "idp-timeout";
    pub const IDP_TLS_FAILURE: &str = "idp-tls-failure";
    pub const IDP_TOKEN_EXPIRED: &str = "idp-token-expired";
    pub const IDP_TOKEN_INVALID: &str = "idp-token-invalid";
}

#[derive(Clone, Debug)]
pub struct RTCPriorityType {
    inner: emlite::Val,
}
impl FromVal for RTCPriorityType {
    fn from_val(v: &emlite::Val) -> Self {
        RTCPriorityType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCPriorityType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCPriorityType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCPriorityType> for emlite::Val {
    fn from(s: RTCPriorityType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCPriorityType {
    pub const VERY_LOW: &str = "very-low";
    pub const LOW: &str = "low";
    pub const MEDIUM: &str = "medium";
    pub const HIGH: &str = "high";
}

#[derive(Clone, Debug)]
pub struct RTCStatsType {
    inner: emlite::Val,
}
impl FromVal for RTCStatsType {
    fn from_val(v: &emlite::Val) -> Self {
        RTCStatsType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCStatsType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCStatsType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCStatsType> for emlite::Val {
    fn from(s: RTCStatsType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCStatsType {
    pub const CODEC: &str = "codec";
    pub const INBOUND_RTP: &str = "inbound-rtp";
    pub const OUTBOUND_RTP: &str = "outbound-rtp";
    pub const REMOTE_INBOUND_RTP: &str = "remote-inbound-rtp";
    pub const REMOTE_OUTBOUND_RTP: &str = "remote-outbound-rtp";
    pub const MEDIA_SOURCE: &str = "media-source";
    pub const MEDIA_PLAYOUT: &str = "media-playout";
    pub const PEER_CONNECTION: &str = "peer-connection";
    pub const DATA_CHANNEL: &str = "data-channel";
    pub const TRANSPORT: &str = "transport";
    pub const CANDIDATE_PAIR: &str = "candidate-pair";
    pub const LOCAL_CANDIDATE: &str = "local-candidate";
    pub const REMOTE_CANDIDATE: &str = "remote-candidate";
    pub const CERTIFICATE: &str = "certificate";
}

#[derive(Clone, Debug)]
pub struct RTCQualityLimitationReason {
    inner: emlite::Val,
}
impl FromVal for RTCQualityLimitationReason {
    fn from_val(v: &emlite::Val) -> Self {
        RTCQualityLimitationReason { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCQualityLimitationReason {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCQualityLimitationReason {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCQualityLimitationReason> for emlite::Val {
    fn from(s: RTCQualityLimitationReason) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCQualityLimitationReason {
    pub const NONE: &str = "none";
    pub const CPU: &str = "cpu";
    pub const BANDWIDTH: &str = "bandwidth";
    pub const OTHER: &str = "other";
}

#[derive(Clone, Debug)]
pub struct RTCDtlsRole {
    inner: emlite::Val,
}
impl FromVal for RTCDtlsRole {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDtlsRole { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCDtlsRole {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCDtlsRole {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCDtlsRole> for emlite::Val {
    fn from(s: RTCDtlsRole) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCDtlsRole {
    pub const CLIENT: &str = "client";
    pub const SERVER: &str = "server";
    pub const UNKNOWN: &str = "unknown";
}

#[derive(Clone, Debug)]
pub struct RTCStatsIceCandidatePairState {
    inner: emlite::Val,
}
impl FromVal for RTCStatsIceCandidatePairState {
    fn from_val(v: &emlite::Val) -> Self {
        RTCStatsIceCandidatePairState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCStatsIceCandidatePairState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCStatsIceCandidatePairState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCStatsIceCandidatePairState> for emlite::Val {
    fn from(s: RTCStatsIceCandidatePairState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCStatsIceCandidatePairState {
    pub const FROZEN: &str = "frozen";
    pub const WAITING: &str = "waiting";
    pub const IN_PROGRESS: &str = "in-progress";
    pub const FAILED: &str = "failed";
    pub const SUCCEEDED: &str = "succeeded";
}

#[derive(Clone, Debug)]
pub struct RTCIceTransportPolicy {
    inner: emlite::Val,
}
impl FromVal for RTCIceTransportPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceTransportPolicy { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceTransportPolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceTransportPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceTransportPolicy> for emlite::Val {
    fn from(s: RTCIceTransportPolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceTransportPolicy {
    pub const RELAY: &str = "relay";
    pub const ALL: &str = "all";
}

#[derive(Clone, Debug)]
pub struct RTCBundlePolicy {
    inner: emlite::Val,
}
impl FromVal for RTCBundlePolicy {
    fn from_val(v: &emlite::Val) -> Self {
        RTCBundlePolicy { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCBundlePolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCBundlePolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCBundlePolicy> for emlite::Val {
    fn from(s: RTCBundlePolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCBundlePolicy {
    pub const BALANCED: &str = "balanced";
    pub const MAX_COMPAT: &str = "max-compat";
    pub const MAX_BUNDLE: &str = "max-bundle";
}

#[derive(Clone, Debug)]
pub struct RTCRtcpMuxPolicy {
    inner: emlite::Val,
}
impl FromVal for RTCRtcpMuxPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtcpMuxPolicy { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCRtcpMuxPolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCRtcpMuxPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCRtcpMuxPolicy> for emlite::Val {
    fn from(s: RTCRtcpMuxPolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCRtcpMuxPolicy {
    pub const REQUIRE: &str = "require";
}

#[derive(Clone, Debug)]
pub struct RTCSignalingState {
    inner: emlite::Val,
}
impl FromVal for RTCSignalingState {
    fn from_val(v: &emlite::Val) -> Self {
        RTCSignalingState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCSignalingState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCSignalingState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCSignalingState> for emlite::Val {
    fn from(s: RTCSignalingState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCSignalingState {
    pub const STABLE: &str = "stable";
    pub const HAVE_LOCAL_OFFER: &str = "have-local-offer";
    pub const HAVE_REMOTE_OFFER: &str = "have-remote-offer";
    pub const HAVE_LOCAL_PRANSWER: &str = "have-local-pranswer";
    pub const HAVE_REMOTE_PRANSWER: &str = "have-remote-pranswer";
    pub const CLOSED: &str = "closed";
}

#[derive(Clone, Debug)]
pub struct RTCIceGatheringState {
    inner: emlite::Val,
}
impl FromVal for RTCIceGatheringState {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceGatheringState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceGatheringState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceGatheringState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceGatheringState> for emlite::Val {
    fn from(s: RTCIceGatheringState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceGatheringState {
    pub const NEW: &str = "new";
    pub const GATHERING: &str = "gathering";
    pub const COMPLETE: &str = "complete";
}

#[derive(Clone, Debug)]
pub struct RTCPeerConnectionState {
    inner: emlite::Val,
}
impl FromVal for RTCPeerConnectionState {
    fn from_val(v: &emlite::Val) -> Self {
        RTCPeerConnectionState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCPeerConnectionState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCPeerConnectionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCPeerConnectionState> for emlite::Val {
    fn from(s: RTCPeerConnectionState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCPeerConnectionState {
    pub const CLOSED: &str = "closed";
    pub const FAILED: &str = "failed";
    pub const DISCONNECTED: &str = "disconnected";
    pub const NEW: &str = "new";
    pub const CONNECTING: &str = "connecting";
    pub const CONNECTED: &str = "connected";
}

#[derive(Clone, Debug)]
pub struct RTCIceConnectionState {
    inner: emlite::Val,
}
impl FromVal for RTCIceConnectionState {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceConnectionState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceConnectionState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceConnectionState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceConnectionState> for emlite::Val {
    fn from(s: RTCIceConnectionState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceConnectionState {
    pub const CLOSED: &str = "closed";
    pub const FAILED: &str = "failed";
    pub const DISCONNECTED: &str = "disconnected";
    pub const NEW: &str = "new";
    pub const CHECKING: &str = "checking";
    pub const COMPLETED: &str = "completed";
    pub const CONNECTED: &str = "connected";
}

#[derive(Clone, Debug)]
pub struct RTCSdpType {
    inner: emlite::Val,
}
impl FromVal for RTCSdpType {
    fn from_val(v: &emlite::Val) -> Self {
        RTCSdpType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCSdpType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCSdpType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCSdpType> for emlite::Val {
    fn from(s: RTCSdpType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCSdpType {
    pub const OFFER: &str = "offer";
    pub const PRANSWER: &str = "pranswer";
    pub const ANSWER: &str = "answer";
    pub const ROLLBACK: &str = "rollback";
}

#[derive(Clone, Debug)]
pub struct RTCIceProtocol {
    inner: emlite::Val,
}
impl FromVal for RTCIceProtocol {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceProtocol { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceProtocol {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceProtocol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceProtocol> for emlite::Val {
    fn from(s: RTCIceProtocol) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceProtocol {
    pub const UDP: &str = "udp";
    pub const TCP: &str = "tcp";
}

#[derive(Clone, Debug)]
pub struct RTCIceTcpCandidateType {
    inner: emlite::Val,
}
impl FromVal for RTCIceTcpCandidateType {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceTcpCandidateType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceTcpCandidateType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceTcpCandidateType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceTcpCandidateType> for emlite::Val {
    fn from(s: RTCIceTcpCandidateType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceTcpCandidateType {
    pub const ACTIVE: &str = "active";
    pub const PASSIVE: &str = "passive";
    pub const SO: &str = "so";
}

#[derive(Clone, Debug)]
pub struct RTCIceCandidateType {
    inner: emlite::Val,
}
impl FromVal for RTCIceCandidateType {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceCandidateType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceCandidateType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceCandidateType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceCandidateType> for emlite::Val {
    fn from(s: RTCIceCandidateType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceCandidateType {
    pub const HOST: &str = "host";
    pub const SRFLX: &str = "srflx";
    pub const PRFLX: &str = "prflx";
    pub const RELAY: &str = "relay";
}

#[derive(Clone, Debug)]
pub struct RTCIceServerTransportProtocol {
    inner: emlite::Val,
}
impl FromVal for RTCIceServerTransportProtocol {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceServerTransportProtocol { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceServerTransportProtocol {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceServerTransportProtocol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceServerTransportProtocol> for emlite::Val {
    fn from(s: RTCIceServerTransportProtocol) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceServerTransportProtocol {
    pub const UDP: &str = "udp";
    pub const TCP: &str = "tcp";
    pub const TLS: &str = "tls";
}

#[derive(Clone, Debug)]
pub struct RTCRtpTransceiverDirection {
    inner: emlite::Val,
}
impl FromVal for RTCRtpTransceiverDirection {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpTransceiverDirection { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCRtpTransceiverDirection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCRtpTransceiverDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCRtpTransceiverDirection> for emlite::Val {
    fn from(s: RTCRtpTransceiverDirection) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCRtpTransceiverDirection {
    pub const SENDRECV: &str = "sendrecv";
    pub const SENDONLY: &str = "sendonly";
    pub const RECVONLY: &str = "recvonly";
    pub const INACTIVE: &str = "inactive";
    pub const STOPPED: &str = "stopped";
}

#[derive(Clone, Debug)]
pub struct RTCDtlsTransportState {
    inner: emlite::Val,
}
impl FromVal for RTCDtlsTransportState {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDtlsTransportState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCDtlsTransportState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCDtlsTransportState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCDtlsTransportState> for emlite::Val {
    fn from(s: RTCDtlsTransportState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCDtlsTransportState {
    pub const NEW: &str = "new";
    pub const CONNECTING: &str = "connecting";
    pub const CONNECTED: &str = "connected";
    pub const CLOSED: &str = "closed";
    pub const FAILED: &str = "failed";
}

#[derive(Clone, Debug)]
pub struct RTCIceGathererState {
    inner: emlite::Val,
}
impl FromVal for RTCIceGathererState {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceGathererState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceGathererState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceGathererState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceGathererState> for emlite::Val {
    fn from(s: RTCIceGathererState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceGathererState {
    pub const NEW: &str = "new";
    pub const GATHERING: &str = "gathering";
    pub const COMPLETE: &str = "complete";
}

#[derive(Clone, Debug)]
pub struct RTCIceTransportState {
    inner: emlite::Val,
}
impl FromVal for RTCIceTransportState {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceTransportState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceTransportState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceTransportState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceTransportState> for emlite::Val {
    fn from(s: RTCIceTransportState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceTransportState {
    pub const CLOSED: &str = "closed";
    pub const FAILED: &str = "failed";
    pub const DISCONNECTED: &str = "disconnected";
    pub const NEW: &str = "new";
    pub const CHECKING: &str = "checking";
    pub const COMPLETED: &str = "completed";
    pub const CONNECTED: &str = "connected";
}

#[derive(Clone, Debug)]
pub struct RTCIceRole {
    inner: emlite::Val,
}
impl FromVal for RTCIceRole {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceRole { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceRole {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceRole {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceRole> for emlite::Val {
    fn from(s: RTCIceRole) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceRole {
    pub const UNKNOWN: &str = "unknown";
    pub const CONTROLLING: &str = "controlling";
    pub const CONTROLLED: &str = "controlled";
}

#[derive(Clone, Debug)]
pub struct RTCIceComponent {
    inner: emlite::Val,
}
impl FromVal for RTCIceComponent {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceComponent { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCIceComponent {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceComponent> for emlite::Val {
    fn from(s: RTCIceComponent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIceComponent {
    pub const RTP: &str = "rtp";
    pub const RTCP: &str = "rtcp";
}

#[derive(Clone, Debug)]
pub struct RTCSctpTransportState {
    inner: emlite::Val,
}
impl FromVal for RTCSctpTransportState {
    fn from_val(v: &emlite::Val) -> Self {
        RTCSctpTransportState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCSctpTransportState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCSctpTransportState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCSctpTransportState> for emlite::Val {
    fn from(s: RTCSctpTransportState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCSctpTransportState {
    pub const CONNECTING: &str = "connecting";
    pub const CONNECTED: &str = "connected";
    pub const CLOSED: &str = "closed";
}

#[derive(Clone, Debug)]
pub struct RTCDataChannelState {
    inner: emlite::Val,
}
impl FromVal for RTCDataChannelState {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDataChannelState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCDataChannelState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCDataChannelState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCDataChannelState> for emlite::Val {
    fn from(s: RTCDataChannelState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCDataChannelState {
    pub const CONNECTING: &str = "connecting";
    pub const OPEN: &str = "open";
    pub const CLOSING: &str = "closing";
    pub const CLOSED: &str = "closed";
}

#[derive(Clone, Debug)]
pub struct RTCErrorDetailType {
    inner: emlite::Val,
}
impl FromVal for RTCErrorDetailType {
    fn from_val(v: &emlite::Val) -> Self {
        RTCErrorDetailType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCErrorDetailType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCErrorDetailType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCErrorDetailType> for emlite::Val {
    fn from(s: RTCErrorDetailType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCErrorDetailType {
    pub const DATA_CHANNEL_FAILURE: &str = "data-channel-failure";
    pub const DTLS_FAILURE: &str = "dtls-failure";
    pub const FINGERPRINT_FAILURE: &str = "fingerprint-failure";
    pub const SCTP_FAILURE: &str = "sctp-failure";
    pub const SDP_SYNTAX_ERROR: &str = "sdp-syntax-error";
    pub const HARDWARE_ENCODER_NOT_AVAILABLE: &str = "hardware-encoder-not-available";
    pub const HARDWARE_ENCODER_ERROR: &str = "hardware-encoder-error";
}

#[derive(Clone, Debug)]
pub struct BinaryType {
    inner: emlite::Val,
}
impl FromVal for BinaryType {
    fn from_val(v: &emlite::Val) -> Self {
        BinaryType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BinaryType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BinaryType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BinaryType> for emlite::Val {
    fn from(s: BinaryType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BinaryType {
    pub const BLOB: &str = "blob";
    pub const ARRAYBUFFER: &str = "arraybuffer";
}

#[derive(Clone, Debug)]
pub struct WebTransportReliabilityMode {
    inner: emlite::Val,
}
impl FromVal for WebTransportReliabilityMode {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportReliabilityMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WebTransportReliabilityMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebTransportReliabilityMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportReliabilityMode> for emlite::Val {
    fn from(s: WebTransportReliabilityMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportReliabilityMode {
    pub const PENDING: &str = "pending";
    pub const RELIABLE_ONLY: &str = "reliable-only";
    pub const SUPPORTS_UNRELIABLE: &str = "supports-unreliable";
}

#[derive(Clone, Debug)]
pub struct WebTransportCongestionControl {
    inner: emlite::Val,
}
impl FromVal for WebTransportCongestionControl {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportCongestionControl { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WebTransportCongestionControl {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebTransportCongestionControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportCongestionControl> for emlite::Val {
    fn from(s: WebTransportCongestionControl) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportCongestionControl {
    pub const DEFAULT: &str = "default";
    pub const THROUGHPUT: &str = "throughput";
    pub const LOW_LATENCY: &str = "low-latency";
}

#[derive(Clone, Debug)]
pub struct WebTransportErrorSource {
    inner: emlite::Val,
}
impl FromVal for WebTransportErrorSource {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportErrorSource { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WebTransportErrorSource {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebTransportErrorSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportErrorSource> for emlite::Val {
    fn from(s: WebTransportErrorSource) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportErrorSource {
    pub const STREAM: &str = "stream";
    pub const SESSION: &str = "session";
}

#[derive(Clone, Debug)]
pub struct USBTransferStatus {
    inner: emlite::Val,
}
impl FromVal for USBTransferStatus {
    fn from_val(v: &emlite::Val) -> Self {
        USBTransferStatus { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for USBTransferStatus {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for USBTransferStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<USBTransferStatus> for emlite::Val {
    fn from(s: USBTransferStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl USBTransferStatus {
    pub const OK: &str = "ok";
    pub const STALL: &str = "stall";
    pub const BABBLE: &str = "babble";
}

#[derive(Clone, Debug)]
pub struct USBRequestType {
    inner: emlite::Val,
}
impl FromVal for USBRequestType {
    fn from_val(v: &emlite::Val) -> Self {
        USBRequestType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for USBRequestType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for USBRequestType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<USBRequestType> for emlite::Val {
    fn from(s: USBRequestType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl USBRequestType {
    pub const STANDARD: &str = "standard";
    pub const CLASS: &str = "class";
    pub const VENDOR: &str = "vendor";
}

#[derive(Clone, Debug)]
pub struct USBRecipient {
    inner: emlite::Val,
}
impl FromVal for USBRecipient {
    fn from_val(v: &emlite::Val) -> Self {
        USBRecipient { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for USBRecipient {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for USBRecipient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<USBRecipient> for emlite::Val {
    fn from(s: USBRecipient) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl USBRecipient {
    pub const DEVICE: &str = "device";
    pub const INTERFACE: &str = "interface";
    pub const ENDPOINT: &str = "endpoint";
    pub const OTHER: &str = "other";
}

#[derive(Clone, Debug)]
pub struct USBDirection {
    inner: emlite::Val,
}
impl FromVal for USBDirection {
    fn from_val(v: &emlite::Val) -> Self {
        USBDirection { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for USBDirection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for USBDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<USBDirection> for emlite::Val {
    fn from(s: USBDirection) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl USBDirection {
    pub const IN_: &str = "in";
    pub const OUT: &str = "out";
}

#[derive(Clone, Debug)]
pub struct USBEndpointType {
    inner: emlite::Val,
}
impl FromVal for USBEndpointType {
    fn from_val(v: &emlite::Val) -> Self {
        USBEndpointType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for USBEndpointType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for USBEndpointType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<USBEndpointType> for emlite::Val {
    fn from(s: USBEndpointType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl USBEndpointType {
    pub const BULK: &str = "bulk";
    pub const INTERRUPT: &str = "interrupt";
    pub const ISOCHRONOUS: &str = "isochronous";
}

#[derive(Clone, Debug)]
pub struct AutoKeyword {
    inner: emlite::Val,
}
impl FromVal for AutoKeyword {
    fn from_val(v: &emlite::Val) -> Self {
        AutoKeyword { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AutoKeyword {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AutoKeyword {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AutoKeyword> for emlite::Val {
    fn from(s: AutoKeyword) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AutoKeyword {
    pub const AUTO: &str = "auto";
}

#[derive(Clone, Debug)]
pub struct DirectionSetting {
    inner: emlite::Val,
}
impl FromVal for DirectionSetting {
    fn from_val(v: &emlite::Val) -> Self {
        DirectionSetting { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DirectionSetting {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DirectionSetting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DirectionSetting> for emlite::Val {
    fn from(s: DirectionSetting) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DirectionSetting {
    pub const NONE: &str = "";
    pub const RL: &str = "rl";
    pub const LR: &str = "lr";
}

#[derive(Clone, Debug)]
pub struct LineAlignSetting {
    inner: emlite::Val,
}
impl FromVal for LineAlignSetting {
    fn from_val(v: &emlite::Val) -> Self {
        LineAlignSetting { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LineAlignSetting {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LineAlignSetting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LineAlignSetting> for emlite::Val {
    fn from(s: LineAlignSetting) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LineAlignSetting {
    pub const START: &str = "start";
    pub const CENTER: &str = "center";
    pub const END: &str = "end";
}

#[derive(Clone, Debug)]
pub struct PositionAlignSetting {
    inner: emlite::Val,
}
impl FromVal for PositionAlignSetting {
    fn from_val(v: &emlite::Val) -> Self {
        PositionAlignSetting { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PositionAlignSetting {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PositionAlignSetting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PositionAlignSetting> for emlite::Val {
    fn from(s: PositionAlignSetting) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PositionAlignSetting {
    pub const LINE_LEFT: &str = "line-left";
    pub const CENTER: &str = "center";
    pub const LINE_RIGHT: &str = "line-right";
    pub const AUTO: &str = "auto";
}

#[derive(Clone, Debug)]
pub struct AlignSetting {
    inner: emlite::Val,
}
impl FromVal for AlignSetting {
    fn from_val(v: &emlite::Val) -> Self {
        AlignSetting { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AlignSetting {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AlignSetting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AlignSetting> for emlite::Val {
    fn from(s: AlignSetting) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AlignSetting {
    pub const START: &str = "start";
    pub const CENTER: &str = "center";
    pub const END: &str = "end";
    pub const LEFT: &str = "left";
    pub const RIGHT: &str = "right";
}

#[derive(Clone, Debug)]
pub struct ScrollSetting {
    inner: emlite::Val,
}
impl FromVal for ScrollSetting {
    fn from_val(v: &emlite::Val) -> Self {
        ScrollSetting { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ScrollSetting {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ScrollSetting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScrollSetting> for emlite::Val {
    fn from(s: ScrollSetting) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScrollSetting {
    pub const NONE: &str = "";
    pub const UP: &str = "up";
}

#[derive(Clone, Debug)]
pub struct XREnvironmentBlendMode {
    inner: emlite::Val,
}
impl FromVal for XREnvironmentBlendMode {
    fn from_val(v: &emlite::Val) -> Self {
        XREnvironmentBlendMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XREnvironmentBlendMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XREnvironmentBlendMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XREnvironmentBlendMode> for emlite::Val {
    fn from(s: XREnvironmentBlendMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XREnvironmentBlendMode {
    pub const OPAQUE: &str = "opaque";
    pub const ALPHA_BLEND: &str = "alpha-blend";
    pub const ADDITIVE: &str = "additive";
}

#[derive(Clone, Debug)]
pub struct XRInteractionMode {
    inner: emlite::Val,
}
impl FromVal for XRInteractionMode {
    fn from_val(v: &emlite::Val) -> Self {
        XRInteractionMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRInteractionMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRInteractionMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRInteractionMode> for emlite::Val {
    fn from(s: XRInteractionMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRInteractionMode {
    pub const SCREEN_SPACE: &str = "screen-space";
    pub const WORLD_SPACE: &str = "world-space";
}

#[derive(Clone, Debug)]
pub struct XRDepthType {
    inner: emlite::Val,
}
impl FromVal for XRDepthType {
    fn from_val(v: &emlite::Val) -> Self {
        XRDepthType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRDepthType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRDepthType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRDepthType> for emlite::Val {
    fn from(s: XRDepthType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRDepthType {
    pub const RAW_: &str = "raw";
    pub const SMOOTH: &str = "smooth";
}

#[derive(Clone, Debug)]
pub struct XRDepthUsage {
    inner: emlite::Val,
}
impl FromVal for XRDepthUsage {
    fn from_val(v: &emlite::Val) -> Self {
        XRDepthUsage { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRDepthUsage {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRDepthUsage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRDepthUsage> for emlite::Val {
    fn from(s: XRDepthUsage) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRDepthUsage {
    pub const CPU_OPTIMIZED: &str = "cpu-optimized";
    pub const GPU_OPTIMIZED: &str = "gpu-optimized";
}

#[derive(Clone, Debug)]
pub struct XRDepthDataFormat {
    inner: emlite::Val,
}
impl FromVal for XRDepthDataFormat {
    fn from_val(v: &emlite::Val) -> Self {
        XRDepthDataFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRDepthDataFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRDepthDataFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRDepthDataFormat> for emlite::Val {
    fn from(s: XRDepthDataFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRDepthDataFormat {
    pub const LUMINANCE_ALPHA: &str = "luminance-alpha";
    pub const FLOAT32: &str = "float32";
    pub const UNSIGNED_SHORT: &str = "unsigned-short";
}

#[derive(Clone, Debug)]
pub struct XRDOMOverlayType {
    inner: emlite::Val,
}
impl FromVal for XRDOMOverlayType {
    fn from_val(v: &emlite::Val) -> Self {
        XRDOMOverlayType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRDOMOverlayType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRDOMOverlayType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRDOMOverlayType> for emlite::Val {
    fn from(s: XRDOMOverlayType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRDOMOverlayType {
    pub const SCREEN: &str = "screen";
    pub const FLOATING: &str = "floating";
    pub const HEAD_LOCKED: &str = "head-locked";
}

#[derive(Clone, Debug)]
pub struct XRHandJoint {
    inner: emlite::Val,
}
impl FromVal for XRHandJoint {
    fn from_val(v: &emlite::Val) -> Self {
        XRHandJoint { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRHandJoint {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRHandJoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRHandJoint> for emlite::Val {
    fn from(s: XRHandJoint) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRHandJoint {
    pub const WRIST: &str = "wrist";
    pub const THUMB_METACARPAL: &str = "thumb-metacarpal";
    pub const THUMB_PHALANX_PROXIMAL: &str = "thumb-phalanx-proximal";
    pub const THUMB_PHALANX_DISTAL: &str = "thumb-phalanx-distal";
    pub const THUMB_TIP: &str = "thumb-tip";
    pub const INDEX_FINGER_METACARPAL: &str = "index-finger-metacarpal";
    pub const INDEX_FINGER_PHALANX_PROXIMAL: &str = "index-finger-phalanx-proximal";
    pub const INDEX_FINGER_PHALANX_INTERMEDIATE: &str = "index-finger-phalanx-intermediate";
    pub const INDEX_FINGER_PHALANX_DISTAL: &str = "index-finger-phalanx-distal";
    pub const INDEX_FINGER_TIP: &str = "index-finger-tip";
    pub const MIDDLE_FINGER_METACARPAL: &str = "middle-finger-metacarpal";
    pub const MIDDLE_FINGER_PHALANX_PROXIMAL: &str = "middle-finger-phalanx-proximal";
    pub const MIDDLE_FINGER_PHALANX_INTERMEDIATE: &str = "middle-finger-phalanx-intermediate";
    pub const MIDDLE_FINGER_PHALANX_DISTAL: &str = "middle-finger-phalanx-distal";
    pub const MIDDLE_FINGER_TIP: &str = "middle-finger-tip";
    pub const RING_FINGER_METACARPAL: &str = "ring-finger-metacarpal";
    pub const RING_FINGER_PHALANX_PROXIMAL: &str = "ring-finger-phalanx-proximal";
    pub const RING_FINGER_PHALANX_INTERMEDIATE: &str = "ring-finger-phalanx-intermediate";
    pub const RING_FINGER_PHALANX_DISTAL: &str = "ring-finger-phalanx-distal";
    pub const RING_FINGER_TIP: &str = "ring-finger-tip";
    pub const PINKY_FINGER_METACARPAL: &str = "pinky-finger-metacarpal";
    pub const PINKY_FINGER_PHALANX_PROXIMAL: &str = "pinky-finger-phalanx-proximal";
    pub const PINKY_FINGER_PHALANX_INTERMEDIATE: &str = "pinky-finger-phalanx-intermediate";
    pub const PINKY_FINGER_PHALANX_DISTAL: &str = "pinky-finger-phalanx-distal";
    pub const PINKY_FINGER_TIP: &str = "pinky-finger-tip";
}

#[derive(Clone, Debug)]
pub struct XRHitTestTrackableType {
    inner: emlite::Val,
}
impl FromVal for XRHitTestTrackableType {
    fn from_val(v: &emlite::Val) -> Self {
        XRHitTestTrackableType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRHitTestTrackableType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRHitTestTrackableType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRHitTestTrackableType> for emlite::Val {
    fn from(s: XRHitTestTrackableType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRHitTestTrackableType {
    pub const POINT: &str = "point";
    pub const PLANE: &str = "plane";
    pub const MESH: &str = "mesh";
}

#[derive(Clone, Debug)]
pub struct XRReflectionFormat {
    inner: emlite::Val,
}
impl FromVal for XRReflectionFormat {
    fn from_val(v: &emlite::Val) -> Self {
        XRReflectionFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRReflectionFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRReflectionFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRReflectionFormat> for emlite::Val {
    fn from(s: XRReflectionFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRReflectionFormat {
    pub const SRGBA8: &str = "srgba8";
    pub const RGBA16F: &str = "rgba16f";
}

#[derive(Clone, Debug)]
pub struct XRPlaneOrientation {
    inner: emlite::Val,
}
impl FromVal for XRPlaneOrientation {
    fn from_val(v: &emlite::Val) -> Self {
        XRPlaneOrientation { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRPlaneOrientation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRPlaneOrientation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRPlaneOrientation> for emlite::Val {
    fn from(s: XRPlaneOrientation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRPlaneOrientation {
    pub const HORIZONTAL: &str = "horizontal";
    pub const VERTICAL: &str = "vertical";
}

#[derive(Clone, Debug)]
pub struct XRSessionMode {
    inner: emlite::Val,
}
impl FromVal for XRSessionMode {
    fn from_val(v: &emlite::Val) -> Self {
        XRSessionMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRSessionMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRSessionMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRSessionMode> for emlite::Val {
    fn from(s: XRSessionMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRSessionMode {
    pub const INLINE: &str = "inline";
    pub const IMMERSIVE_VR: &str = "immersive-vr";
    pub const IMMERSIVE_AR: &str = "immersive-ar";
}

#[derive(Clone, Debug)]
pub struct XRVisibilityState {
    inner: emlite::Val,
}
impl FromVal for XRVisibilityState {
    fn from_val(v: &emlite::Val) -> Self {
        XRVisibilityState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRVisibilityState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRVisibilityState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRVisibilityState> for emlite::Val {
    fn from(s: XRVisibilityState) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRVisibilityState {
    pub const VISIBLE: &str = "visible";
    pub const VISIBLE_BLURRED: &str = "visible-blurred";
    pub const HIDDEN: &str = "hidden";
}

#[derive(Clone, Debug)]
pub struct XRReferenceSpaceType {
    inner: emlite::Val,
}
impl FromVal for XRReferenceSpaceType {
    fn from_val(v: &emlite::Val) -> Self {
        XRReferenceSpaceType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRReferenceSpaceType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRReferenceSpaceType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRReferenceSpaceType> for emlite::Val {
    fn from(s: XRReferenceSpaceType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRReferenceSpaceType {
    pub const VIEWER: &str = "viewer";
    pub const LOCAL: &str = "local";
    pub const LOCAL_FLOOR: &str = "local-floor";
    pub const BOUNDED_FLOOR: &str = "bounded-floor";
    pub const UNBOUNDED: &str = "unbounded";
}

#[derive(Clone, Debug)]
pub struct XREye {
    inner: emlite::Val,
}
impl FromVal for XREye {
    fn from_val(v: &emlite::Val) -> Self {
        XREye { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XREye {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XREye {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XREye> for emlite::Val {
    fn from(s: XREye) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XREye {
    pub const NONE: &str = "none";
    pub const LEFT: &str = "left";
    pub const RIGHT: &str = "right";
}

#[derive(Clone, Debug)]
pub struct XRHandedness {
    inner: emlite::Val,
}
impl FromVal for XRHandedness {
    fn from_val(v: &emlite::Val) -> Self {
        XRHandedness { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRHandedness {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRHandedness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRHandedness> for emlite::Val {
    fn from(s: XRHandedness) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRHandedness {
    pub const NONE: &str = "none";
    pub const LEFT: &str = "left";
    pub const RIGHT: &str = "right";
}

#[derive(Clone, Debug)]
pub struct XRTargetRayMode {
    inner: emlite::Val,
}
impl FromVal for XRTargetRayMode {
    fn from_val(v: &emlite::Val) -> Self {
        XRTargetRayMode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRTargetRayMode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRTargetRayMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRTargetRayMode> for emlite::Val {
    fn from(s: XRTargetRayMode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRTargetRayMode {
    pub const GAZE: &str = "gaze";
    pub const TRACKED_POINTER: &str = "tracked-pointer";
    pub const SCREEN: &str = "screen";
    pub const TRANSIENT_POINTER: &str = "transient-pointer";
}

#[derive(Clone, Debug)]
pub struct XRLayerLayout {
    inner: emlite::Val,
}
impl FromVal for XRLayerLayout {
    fn from_val(v: &emlite::Val) -> Self {
        XRLayerLayout { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRLayerLayout {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRLayerLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRLayerLayout> for emlite::Val {
    fn from(s: XRLayerLayout) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRLayerLayout {
    pub const DEFAULT: &str = "default";
    pub const MONO: &str = "mono";
    pub const STEREO: &str = "stereo";
    pub const STEREO_LEFT_RIGHT: &str = "stereo-left-right";
    pub const STEREO_TOP_BOTTOM: &str = "stereo-top-bottom";
}

#[derive(Clone, Debug)]
pub struct XRLayerQuality {
    inner: emlite::Val,
}
impl FromVal for XRLayerQuality {
    fn from_val(v: &emlite::Val) -> Self {
        XRLayerQuality { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRLayerQuality {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRLayerQuality {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRLayerQuality> for emlite::Val {
    fn from(s: XRLayerQuality) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRLayerQuality {
    pub const DEFAULT: &str = "default";
    pub const TEXT_OPTIMIZED: &str = "text-optimized";
    pub const GRAPHICS_OPTIMIZED: &str = "graphics-optimized";
}

#[derive(Clone, Debug)]
pub struct XRTextureType {
    inner: emlite::Val,
}
impl FromVal for XRTextureType {
    fn from_val(v: &emlite::Val) -> Self {
        XRTextureType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRTextureType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRTextureType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRTextureType> for emlite::Val {
    fn from(s: XRTextureType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRTextureType {
    pub const TEXTURE: &str = "texture";
    pub const TEXTURE_ARRAY: &str = "texture-array";
}

#[derive(Clone, Debug)]
pub struct SummarizerType {
    inner: emlite::Val,
}
impl FromVal for SummarizerType {
    fn from_val(v: &emlite::Val) -> Self {
        SummarizerType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SummarizerType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SummarizerType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SummarizerType> for emlite::Val {
    fn from(s: SummarizerType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SummarizerType {
    pub const TLDR: &str = "tldr";
    pub const TEASER: &str = "teaser";
    pub const KEY_POINTS: &str = "key-points";
    pub const HEADLINE: &str = "headline";
}

#[derive(Clone, Debug)]
pub struct SummarizerFormat {
    inner: emlite::Val,
}
impl FromVal for SummarizerFormat {
    fn from_val(v: &emlite::Val) -> Self {
        SummarizerFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SummarizerFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SummarizerFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SummarizerFormat> for emlite::Val {
    fn from(s: SummarizerFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SummarizerFormat {
    pub const PLAIN_TEXT: &str = "plain-text";
    pub const MARKDOWN: &str = "markdown";
}

#[derive(Clone, Debug)]
pub struct SummarizerLength {
    inner: emlite::Val,
}
impl FromVal for SummarizerLength {
    fn from_val(v: &emlite::Val) -> Self {
        SummarizerLength { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SummarizerLength {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SummarizerLength {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SummarizerLength> for emlite::Val {
    fn from(s: SummarizerLength) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SummarizerLength {
    pub const SHORT: &str = "short";
    pub const MEDIUM: &str = "medium";
    pub const LONG: &str = "long";
}

#[derive(Clone, Debug)]
pub struct WriterTone {
    inner: emlite::Val,
}
impl FromVal for WriterTone {
    fn from_val(v: &emlite::Val) -> Self {
        WriterTone { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WriterTone {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WriterTone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WriterTone> for emlite::Val {
    fn from(s: WriterTone) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WriterTone {
    pub const FORMAL: &str = "formal";
    pub const NEUTRAL: &str = "neutral";
    pub const CASUAL: &str = "casual";
}

#[derive(Clone, Debug)]
pub struct WriterFormat {
    inner: emlite::Val,
}
impl FromVal for WriterFormat {
    fn from_val(v: &emlite::Val) -> Self {
        WriterFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WriterFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WriterFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WriterFormat> for emlite::Val {
    fn from(s: WriterFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WriterFormat {
    pub const PLAIN_TEXT: &str = "plain-text";
    pub const MARKDOWN: &str = "markdown";
}

#[derive(Clone, Debug)]
pub struct WriterLength {
    inner: emlite::Val,
}
impl FromVal for WriterLength {
    fn from_val(v: &emlite::Val) -> Self {
        WriterLength { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WriterLength {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WriterLength {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WriterLength> for emlite::Val {
    fn from(s: WriterLength) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WriterLength {
    pub const SHORT: &str = "short";
    pub const MEDIUM: &str = "medium";
    pub const LONG: &str = "long";
}

#[derive(Clone, Debug)]
pub struct RewriterTone {
    inner: emlite::Val,
}
impl FromVal for RewriterTone {
    fn from_val(v: &emlite::Val) -> Self {
        RewriterTone { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RewriterTone {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RewriterTone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RewriterTone> for emlite::Val {
    fn from(s: RewriterTone) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RewriterTone {
    pub const AS_IS: &str = "as-is";
    pub const MORE_FORMAL: &str = "more-formal";
    pub const MORE_CASUAL: &str = "more-casual";
}

#[derive(Clone, Debug)]
pub struct RewriterFormat {
    inner: emlite::Val,
}
impl FromVal for RewriterFormat {
    fn from_val(v: &emlite::Val) -> Self {
        RewriterFormat { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RewriterFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RewriterFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RewriterFormat> for emlite::Val {
    fn from(s: RewriterFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RewriterFormat {
    pub const AS_IS: &str = "as-is";
    pub const PLAIN_TEXT: &str = "plain-text";
    pub const MARKDOWN: &str = "markdown";
}

#[derive(Clone, Debug)]
pub struct RewriterLength {
    inner: emlite::Val,
}
impl FromVal for RewriterLength {
    fn from_val(v: &emlite::Val) -> Self {
        RewriterLength { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RewriterLength {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RewriterLength {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RewriterLength> for emlite::Val {
    fn from(s: RewriterLength) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RewriterLength {
    pub const AS_IS: &str = "as-is";
    pub const SHORTER: &str = "shorter";
    pub const LONGER: &str = "longer";
}

#[derive(Clone, Debug)]
pub struct Availability {
    inner: emlite::Val,
}
impl FromVal for Availability {
    fn from_val(v: &emlite::Val) -> Self {
        Availability { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for Availability {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Availability {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Availability> for emlite::Val {
    fn from(s: Availability) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Availability {
    pub const UNAVAILABLE: &str = "unavailable";
    pub const DOWNLOADABLE: &str = "downloadable";
    pub const DOWNLOADING: &str = "downloading";
    pub const AVAILABLE: &str = "available";
}

#[derive(Clone, Debug)]
pub struct XMLHttpRequestResponseType {
    inner: emlite::Val,
}
impl FromVal for XMLHttpRequestResponseType {
    fn from_val(v: &emlite::Val) -> Self {
        XMLHttpRequestResponseType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XMLHttpRequestResponseType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XMLHttpRequestResponseType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XMLHttpRequestResponseType> for emlite::Val {
    fn from(s: XMLHttpRequestResponseType) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XMLHttpRequestResponseType {
    pub const NONE: &str = "";
    pub const ARRAYBUFFER: &str = "arraybuffer";
    pub const BLOB: &str = "blob";
    pub const DOCUMENT: &str = "document";
    pub const JSON: &str = "json";
    pub const TEXT: &str = "text";
}
