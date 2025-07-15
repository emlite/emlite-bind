use super::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SecurityPolicyViolationEventDisposition {
    ENFORCE,
    REPORT,
}
impl FromVal for SecurityPolicyViolationEventDisposition {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "enforce" => Self::ENFORCE,
            "report" => Self::REPORT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SecurityPolicyViolationEventDisposition> for emlite::Val {
    fn from(s: SecurityPolicyViolationEventDisposition) -> emlite::Val {
        match s {
            SecurityPolicyViolationEventDisposition::ENFORCE => emlite::Val::from("enforce"),
            SecurityPolicyViolationEventDisposition::REPORT => emlite::Val::from("report"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EndingType {
    TRANSPARENT,
    NATIVE,
}
impl FromVal for EndingType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "transparent" => Self::TRANSPARENT,
            "native" => Self::NATIVE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<EndingType> for emlite::Val {
    fn from(s: EndingType) -> emlite::Val {
        match s {
            EndingType::TRANSPARENT => emlite::Val::from("transparent"),
            EndingType::NATIVE => emlite::Val::from("native"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IDBRequestReadyState {
    PENDING,
    DONE,
}
impl FromVal for IDBRequestReadyState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "pending" => Self::PENDING,
            "done" => Self::DONE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<IDBRequestReadyState> for emlite::Val {
    fn from(s: IDBRequestReadyState) -> emlite::Val {
        match s {
            IDBRequestReadyState::PENDING => emlite::Val::from("pending"),
            IDBRequestReadyState::DONE => emlite::Val::from("done"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IDBTransactionDurability {
    DEFAULT,
    STRICT,
    RELAXED,
}
impl FromVal for IDBTransactionDurability {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "default" => Self::DEFAULT,
            "strict" => Self::STRICT,
            "relaxed" => Self::RELAXED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<IDBTransactionDurability> for emlite::Val {
    fn from(s: IDBTransactionDurability) -> emlite::Val {
        match s {
            IDBTransactionDurability::DEFAULT => emlite::Val::from("default"),
            IDBTransactionDurability::STRICT => emlite::Val::from("strict"),
            IDBTransactionDurability::RELAXED => emlite::Val::from("relaxed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IDBCursorDirection {
    NEXT,
    NEXTUNIQUE,
    PREV,
    PREVUNIQUE,
}
impl FromVal for IDBCursorDirection {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "next" => Self::NEXT,
            "nextunique" => Self::NEXTUNIQUE,
            "prev" => Self::PREV,
            "prevunique" => Self::PREVUNIQUE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<IDBCursorDirection> for emlite::Val {
    fn from(s: IDBCursorDirection) -> emlite::Val {
        match s {
            IDBCursorDirection::NEXT => emlite::Val::from("next"),
            IDBCursorDirection::NEXTUNIQUE => emlite::Val::from("nextunique"),
            IDBCursorDirection::PREV => emlite::Val::from("prev"),
            IDBCursorDirection::PREVUNIQUE => emlite::Val::from("prevunique"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IDBTransactionMode {
    READONLY,
    READWRITE,
    VERSIONCHANGE,
}
impl FromVal for IDBTransactionMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "readonly" => Self::READONLY,
            "readwrite" => Self::READWRITE,
            "versionchange" => Self::VERSIONCHANGE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<IDBTransactionMode> for emlite::Val {
    fn from(s: IDBTransactionMode) -> emlite::Val {
        match s {
            IDBTransactionMode::READONLY => emlite::Val::from("readonly"),
            IDBTransactionMode::READWRITE => emlite::Val::from("readwrite"),
            IDBTransactionMode::VERSIONCHANGE => emlite::Val::from("versionchange"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AccelerometerLocalCoordinateSystem {
    DEVICE,
    SCREEN,
}
impl FromVal for AccelerometerLocalCoordinateSystem {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "device" => Self::DEVICE,
            "screen" => Self::SCREEN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AccelerometerLocalCoordinateSystem> for emlite::Val {
    fn from(s: AccelerometerLocalCoordinateSystem) -> emlite::Val {
        match s {
            AccelerometerLocalCoordinateSystem::DEVICE => emlite::Val::from("device"),
            AccelerometerLocalCoordinateSystem::SCREEN => emlite::Val::from("screen"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AudioSessionType {
    AUTO,
    PLAYBACK,
    TRANSIENT,
    TRANSIENT_SOLO,
    AMBIENT,
    PLAY_AND_RECORD,
}
impl FromVal for AudioSessionType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            "playback" => Self::PLAYBACK,
            "transient" => Self::TRANSIENT,
            "transient-solo" => Self::TRANSIENT_SOLO,
            "ambient" => Self::AMBIENT,
            "play-and-record" => Self::PLAY_AND_RECORD,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AudioSessionType> for emlite::Val {
    fn from(s: AudioSessionType) -> emlite::Val {
        match s {
            AudioSessionType::AUTO => emlite::Val::from("auto"),
            AudioSessionType::PLAYBACK => emlite::Val::from("playback"),
            AudioSessionType::TRANSIENT => emlite::Val::from("transient"),
            AudioSessionType::TRANSIENT_SOLO => emlite::Val::from("transient-solo"),
            AudioSessionType::AMBIENT => emlite::Val::from("ambient"),
            AudioSessionType::PLAY_AND_RECORD => emlite::Val::from("play-and-record"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AudioSessionState {
    INACTIVE,
    ACTIVE,
    INTERRUPTED,
}
impl FromVal for AudioSessionState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "inactive" => Self::INACTIVE,
            "active" => Self::ACTIVE,
            "interrupted" => Self::INTERRUPTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AudioSessionState> for emlite::Val {
    fn from(s: AudioSessionState) -> emlite::Val {
        match s {
            AudioSessionState::INACTIVE => emlite::Val::from("inactive"),
            AudioSessionState::ACTIVE => emlite::Val::from("active"),
            AudioSessionState::INTERRUPTED => emlite::Val::from("interrupted"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AutoplayPolicy {
    ALLOWED,
    ALLOWED_MUTED,
    DISALLOWED,
}
impl FromVal for AutoplayPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "allowed" => Self::ALLOWED,
            "allowed-muted" => Self::ALLOWED_MUTED,
            "disallowed" => Self::DISALLOWED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AutoplayPolicy> for emlite::Val {
    fn from(s: AutoplayPolicy) -> emlite::Val {
        match s {
            AutoplayPolicy::ALLOWED => emlite::Val::from("allowed"),
            AutoplayPolicy::ALLOWED_MUTED => emlite::Val::from("allowed-muted"),
            AutoplayPolicy::DISALLOWED => emlite::Val::from("disallowed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AutoplayPolicyMediaType {
    MEDIAELEMENT,
    AUDIOCONTEXT,
}
impl FromVal for AutoplayPolicyMediaType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "mediaelement" => Self::MEDIAELEMENT,
            "audiocontext" => Self::AUDIOCONTEXT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AutoplayPolicyMediaType> for emlite::Val {
    fn from(s: AutoplayPolicyMediaType) -> emlite::Val {
        match s {
            AutoplayPolicyMediaType::MEDIAELEMENT => emlite::Val::from("mediaelement"),
            AutoplayPolicyMediaType::AUDIOCONTEXT => emlite::Val::from("audiocontext"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BackgroundFetchResult {
    NONE,
    SUCCESS,
    FAILURE,
}
impl FromVal for BackgroundFetchResult {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "success" => Self::SUCCESS,
            "failure" => Self::FAILURE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<BackgroundFetchResult> for emlite::Val {
    fn from(s: BackgroundFetchResult) -> emlite::Val {
        match s {
            BackgroundFetchResult::NONE => emlite::Val::from(""),
            BackgroundFetchResult::SUCCESS => emlite::Val::from("success"),
            BackgroundFetchResult::FAILURE => emlite::Val::from("failure"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BackgroundFetchFailureReason {
    NONE,
    ABORTED,
    BAD_STATUS,
    FETCH_ERROR,
    QUOTA_EXCEEDED,
    DOWNLOAD_TOTAL_EXCEEDED,
}
impl FromVal for BackgroundFetchFailureReason {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "aborted" => Self::ABORTED,
            "bad-status" => Self::BAD_STATUS,
            "fetch-error" => Self::FETCH_ERROR,
            "quota-exceeded" => Self::QUOTA_EXCEEDED,
            "download-total-exceeded" => Self::DOWNLOAD_TOTAL_EXCEEDED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<BackgroundFetchFailureReason> for emlite::Val {
    fn from(s: BackgroundFetchFailureReason) -> emlite::Val {
        match s {
            BackgroundFetchFailureReason::NONE => emlite::Val::from(""),
            BackgroundFetchFailureReason::ABORTED => emlite::Val::from("aborted"),
            BackgroundFetchFailureReason::BAD_STATUS => emlite::Val::from("bad-status"),
            BackgroundFetchFailureReason::FETCH_ERROR => emlite::Val::from("fetch-error"),
            BackgroundFetchFailureReason::QUOTA_EXCEEDED => emlite::Val::from("quota-exceeded"),
            BackgroundFetchFailureReason::DOWNLOAD_TOTAL_EXCEEDED => {
                emlite::Val::from("download-total-exceeded")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PresentationStyle {
    UNSPECIFIED,
    INLINE,
    ATTACHMENT,
}
impl FromVal for PresentationStyle {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "unspecified" => Self::UNSPECIFIED,
            "inline" => Self::INLINE,
            "attachment" => Self::ATTACHMENT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PresentationStyle> for emlite::Val {
    fn from(s: PresentationStyle) -> emlite::Val {
        match s {
            PresentationStyle::UNSPECIFIED => emlite::Val::from("unspecified"),
            PresentationStyle::INLINE => emlite::Val::from("inline"),
            PresentationStyle::ATTACHMENT => emlite::Val::from("attachment"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CompressionFormat {
    DEFLATE,
    DEFLATE_RAW,
    GZIP,
}
impl FromVal for CompressionFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "deflate" => Self::DEFLATE,
            "deflate-raw" => Self::DEFLATE_RAW,
            "gzip" => Self::GZIP,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CompressionFormat> for emlite::Val {
    fn from(s: CompressionFormat) -> emlite::Val {
        match s {
            CompressionFormat::DEFLATE => emlite::Val::from("deflate"),
            CompressionFormat::DEFLATE_RAW => emlite::Val::from("deflate-raw"),
            CompressionFormat::GZIP => emlite::Val::from("gzip"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PressureSource {
    CPU,
}
impl FromVal for PressureSource {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "cpu" => Self::CPU,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PressureSource> for emlite::Val {
    fn from(s: PressureSource) -> emlite::Val {
        match s {
            PressureSource::CPU => emlite::Val::from("cpu"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PressureState {
    NOMINAL,
    FAIR,
    SERIOUS,
    CRITICAL,
}
impl FromVal for PressureState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "nominal" => Self::NOMINAL,
            "fair" => Self::FAIR,
            "serious" => Self::SERIOUS,
            "critical" => Self::CRITICAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PressureState> for emlite::Val {
    fn from(s: PressureState) -> emlite::Val {
        match s {
            PressureState::NOMINAL => emlite::Val::from("nominal"),
            PressureState::FAIR => emlite::Val::from("fair"),
            PressureState::SERIOUS => emlite::Val::from("serious"),
            PressureState::CRITICAL => emlite::Val::from("critical"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ContactProperty {
    ADDRESS,
    EMAIL,
    ICON,
    NAME,
    TEL,
}
impl FromVal for ContactProperty {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "address" => Self::ADDRESS,
            "email" => Self::EMAIL,
            "icon" => Self::ICON,
            "name" => Self::NAME,
            "tel" => Self::TEL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ContactProperty> for emlite::Val {
    fn from(s: ContactProperty) -> emlite::Val {
        match s {
            ContactProperty::ADDRESS => emlite::Val::from("address"),
            ContactProperty::EMAIL => emlite::Val::from("email"),
            ContactProperty::ICON => emlite::Val::from("icon"),
            ContactProperty::NAME => emlite::Val::from("name"),
            ContactProperty::TEL => emlite::Val::from("tel"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ContentCategory {
    NONE,
    HOMEPAGE,
    ARTICLE,
    VIDEO,
    AUDIO,
}
impl FromVal for ContentCategory {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "homepage" => Self::HOMEPAGE,
            "article" => Self::ARTICLE,
            "video" => Self::VIDEO,
            "audio" => Self::AUDIO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ContentCategory> for emlite::Val {
    fn from(s: ContentCategory) -> emlite::Val {
        match s {
            ContentCategory::NONE => emlite::Val::from(""),
            ContentCategory::HOMEPAGE => emlite::Val::from("homepage"),
            ContentCategory::ARTICLE => emlite::Val::from("article"),
            ContentCategory::VIDEO => emlite::Val::from("video"),
            ContentCategory::AUDIO => emlite::Val::from("audio"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CookieSameSite {
    STRICT,
    LAX,
    NONE,
}
impl FromVal for CookieSameSite {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "strict" => Self::STRICT,
            "lax" => Self::LAX,
            "none" => Self::NONE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CookieSameSite> for emlite::Val {
    fn from(s: CookieSameSite) -> emlite::Val {
        match s {
            CookieSameSite::STRICT => emlite::Val::from("strict"),
            CookieSameSite::LAX => emlite::Val::from("lax"),
            CookieSameSite::NONE => emlite::Val::from("none"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CredentialMediationRequirement {
    SILENT,
    OPTIONAL,
    CONDITIONAL,
    REQUIRED,
}
impl FromVal for CredentialMediationRequirement {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "silent" => Self::SILENT,
            "optional" => Self::OPTIONAL,
            "conditional" => Self::CONDITIONAL,
            "required" => Self::REQUIRED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CredentialMediationRequirement> for emlite::Val {
    fn from(s: CredentialMediationRequirement) -> emlite::Val {
        match s {
            CredentialMediationRequirement::SILENT => emlite::Val::from("silent"),
            CredentialMediationRequirement::OPTIONAL => emlite::Val::from("optional"),
            CredentialMediationRequirement::CONDITIONAL => emlite::Val::from("conditional"),
            CredentialMediationRequirement::REQUIRED => emlite::Val::from("required"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScriptingPolicyViolationType {
    EXTERNAL_SCRIPT,
    INLINE_SCRIPT,
    INLINE_EVENT_HANDLER,
    EVAL,
}
impl FromVal for ScriptingPolicyViolationType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "externalScript" => Self::EXTERNAL_SCRIPT,
            "inlineScript" => Self::INLINE_SCRIPT,
            "inlineEventHandler" => Self::INLINE_EVENT_HANDLER,
            "eval" => Self::EVAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ScriptingPolicyViolationType> for emlite::Val {
    fn from(s: ScriptingPolicyViolationType) -> emlite::Val {
        match s {
            ScriptingPolicyViolationType::EXTERNAL_SCRIPT => emlite::Val::from("externalScript"),
            ScriptingPolicyViolationType::INLINE_SCRIPT => emlite::Val::from("inlineScript"),
            ScriptingPolicyViolationType::INLINE_EVENT_HANDLER => {
                emlite::Val::from("inlineEventHandler")
            }
            ScriptingPolicyViolationType::EVAL => emlite::Val::from("eval"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FontFaceLoadStatus {
    UNLOADED,
    LOADING,
    LOADED,
    ERROR,
}
impl FromVal for FontFaceLoadStatus {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "unloaded" => Self::UNLOADED,
            "loading" => Self::LOADING,
            "loaded" => Self::LOADED,
            "error" => Self::ERROR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FontFaceLoadStatus> for emlite::Val {
    fn from(s: FontFaceLoadStatus) -> emlite::Val {
        match s {
            FontFaceLoadStatus::UNLOADED => emlite::Val::from("unloaded"),
            FontFaceLoadStatus::LOADING => emlite::Val::from("loading"),
            FontFaceLoadStatus::LOADED => emlite::Val::from("loaded"),
            FontFaceLoadStatus::ERROR => emlite::Val::from("error"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FontFaceSetLoadStatus {
    LOADING,
    LOADED,
}
impl FromVal for FontFaceSetLoadStatus {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "loading" => Self::LOADING,
            "loaded" => Self::LOADED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FontFaceSetLoadStatus> for emlite::Val {
    fn from(s: FontFaceSetLoadStatus) -> emlite::Val {
        match s {
            FontFaceSetLoadStatus::LOADING => emlite::Val::from("loading"),
            FontFaceSetLoadStatus::LOADED => emlite::Val::from("loaded"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum HighlightType {
    HIGHLIGHT,
    SPELLING_ERROR,
    GRAMMAR_ERROR,
}
impl FromVal for HighlightType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "highlight" => Self::HIGHLIGHT,
            "spelling-error" => Self::SPELLING_ERROR,
            "grammar-error" => Self::GRAMMAR_ERROR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<HighlightType> for emlite::Val {
    fn from(s: HighlightType) -> emlite::Val {
        match s {
            HighlightType::HIGHLIGHT => emlite::Val::from("highlight"),
            HighlightType::SPELLING_ERROR => emlite::Val::from("spelling-error"),
            HighlightType::GRAMMAR_ERROR => emlite::Val::from("grammar-error"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ChildDisplayType {
    BLOCK,
    NORMAL,
}
impl FromVal for ChildDisplayType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "block" => Self::BLOCK,
            "normal" => Self::NORMAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ChildDisplayType> for emlite::Val {
    fn from(s: ChildDisplayType) -> emlite::Val {
        match s {
            ChildDisplayType::BLOCK => emlite::Val::from("block"),
            ChildDisplayType::NORMAL => emlite::Val::from("normal"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LayoutSizingMode {
    BLOCK_LIKE,
    MANUAL,
}
impl FromVal for LayoutSizingMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "block-like" => Self::BLOCK_LIKE,
            "manual" => Self::MANUAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<LayoutSizingMode> for emlite::Val {
    fn from(s: LayoutSizingMode) -> emlite::Val {
        match s {
            LayoutSizingMode::BLOCK_LIKE => emlite::Val::from("block-like"),
            LayoutSizingMode::MANUAL => emlite::Val::from("manual"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BlockFragmentationType {
    NONE,
    PAGE,
    COLUMN,
    REGION,
}
impl FromVal for BlockFragmentationType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "page" => Self::PAGE,
            "column" => Self::COLUMN,
            "region" => Self::REGION,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<BlockFragmentationType> for emlite::Val {
    fn from(s: BlockFragmentationType) -> emlite::Val {
        match s {
            BlockFragmentationType::NONE => emlite::Val::from("none"),
            BlockFragmentationType::PAGE => emlite::Val::from("page"),
            BlockFragmentationType::COLUMN => emlite::Val::from("column"),
            BlockFragmentationType::REGION => emlite::Val::from("region"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BreakType {
    NONE,
    LINE,
    COLUMN,
    PAGE,
    REGION,
}
impl FromVal for BreakType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "line" => Self::LINE,
            "column" => Self::COLUMN,
            "page" => Self::PAGE,
            "region" => Self::REGION,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<BreakType> for emlite::Val {
    fn from(s: BreakType) -> emlite::Val {
        match s {
            BreakType::NONE => emlite::Val::from("none"),
            BreakType::LINE => emlite::Val::from("line"),
            BreakType::COLUMN => emlite::Val::from("column"),
            BreakType::PAGE => emlite::Val::from("page"),
            BreakType::REGION => emlite::Val::from("region"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SpatialNavigationDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl FromVal for SpatialNavigationDirection {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "up" => Self::UP,
            "down" => Self::DOWN,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SpatialNavigationDirection> for emlite::Val {
    fn from(s: SpatialNavigationDirection) -> emlite::Val {
        match s {
            SpatialNavigationDirection::UP => emlite::Val::from("up"),
            SpatialNavigationDirection::DOWN => emlite::Val::from("down"),
            SpatialNavigationDirection::LEFT => emlite::Val::from("left"),
            SpatialNavigationDirection::RIGHT => emlite::Val::from("right"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FocusableAreaSearchMode {
    VISIBLE,
    ALL,
}
impl FromVal for FocusableAreaSearchMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "visible" => Self::VISIBLE,
            "all" => Self::ALL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FocusableAreaSearchMode> for emlite::Val {
    fn from(s: FocusableAreaSearchMode) -> emlite::Val {
        match s {
            FocusableAreaSearchMode::VISIBLE => emlite::Val::from("visible"),
            FocusableAreaSearchMode::ALL => emlite::Val::from("all"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CSSNumericBaseType {
    LENGTH,
    ANGLE,
    TIME,
    FREQUENCY,
    RESOLUTION,
    FLEX,
    PERCENT,
}
impl FromVal for CSSNumericBaseType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "length" => Self::LENGTH,
            "angle" => Self::ANGLE,
            "time" => Self::TIME,
            "frequency" => Self::FREQUENCY,
            "resolution" => Self::RESOLUTION,
            "flex" => Self::FLEX,
            "percent" => Self::PERCENT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CSSNumericBaseType> for emlite::Val {
    fn from(s: CSSNumericBaseType) -> emlite::Val {
        match s {
            CSSNumericBaseType::LENGTH => emlite::Val::from("length"),
            CSSNumericBaseType::ANGLE => emlite::Val::from("angle"),
            CSSNumericBaseType::TIME => emlite::Val::from("time"),
            CSSNumericBaseType::FREQUENCY => emlite::Val::from("frequency"),
            CSSNumericBaseType::RESOLUTION => emlite::Val::from("resolution"),
            CSSNumericBaseType::FLEX => emlite::Val::from("flex"),
            CSSNumericBaseType::PERCENT => emlite::Val::from("percent"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CSSMathOperator {
    SUM,
    PRODUCT,
    NEGATE,
    INVERT,
    MIN,
    MAX,
    CLAMP,
}
impl FromVal for CSSMathOperator {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "sum" => Self::SUM,
            "product" => Self::PRODUCT,
            "negate" => Self::NEGATE,
            "invert" => Self::INVERT,
            "min" => Self::MIN,
            "max" => Self::MAX,
            "clamp" => Self::CLAMP,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CSSMathOperator> for emlite::Val {
    fn from(s: CSSMathOperator) -> emlite::Val {
        match s {
            CSSMathOperator::SUM => emlite::Val::from("sum"),
            CSSMathOperator::PRODUCT => emlite::Val::from("product"),
            CSSMathOperator::NEGATE => emlite::Val::from("negate"),
            CSSMathOperator::INVERT => emlite::Val::from("invert"),
            CSSMathOperator::MIN => emlite::Val::from("min"),
            CSSMathOperator::MAX => emlite::Val::from("max"),
            CSSMathOperator::CLAMP => emlite::Val::from("clamp"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScrollBehavior {
    AUTO,
    INSTANT,
    SMOOTH,
}
impl FromVal for ScrollBehavior {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            "instant" => Self::INSTANT,
            "smooth" => Self::SMOOTH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ScrollBehavior> for emlite::Val {
    fn from(s: ScrollBehavior) -> emlite::Val {
        match s {
            ScrollBehavior::AUTO => emlite::Val::from("auto"),
            ScrollBehavior::INSTANT => emlite::Val::from("instant"),
            ScrollBehavior::SMOOTH => emlite::Val::from("smooth"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScrollLogicalPosition {
    START,
    CENTER,
    END,
    NEAREST,
}
impl FromVal for ScrollLogicalPosition {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "start" => Self::START,
            "center" => Self::CENTER,
            "end" => Self::END,
            "nearest" => Self::NEAREST,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ScrollLogicalPosition> for emlite::Val {
    fn from(s: ScrollLogicalPosition) -> emlite::Val {
        match s {
            ScrollLogicalPosition::START => emlite::Val::from("start"),
            ScrollLogicalPosition::CENTER => emlite::Val::from("center"),
            ScrollLogicalPosition::END => emlite::Val::from("end"),
            ScrollLogicalPosition::NEAREST => emlite::Val::from("nearest"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScrollIntoViewContainer {
    ALL,
    NEAREST,
}
impl FromVal for ScrollIntoViewContainer {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "all" => Self::ALL,
            "nearest" => Self::NEAREST,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ScrollIntoViewContainer> for emlite::Val {
    fn from(s: ScrollIntoViewContainer) -> emlite::Val {
        match s {
            ScrollIntoViewContainer::ALL => emlite::Val::from("all"),
            ScrollIntoViewContainer::NEAREST => emlite::Val::from("nearest"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CSSBoxType {
    MARGIN,
    BORDER,
    PADDING,
    CONTENT,
}
impl FromVal for CSSBoxType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "margin" => Self::MARGIN,
            "border" => Self::BORDER,
            "padding" => Self::PADDING,
            "content" => Self::CONTENT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CSSBoxType> for emlite::Val {
    fn from(s: CSSBoxType) -> emlite::Val {
        match s {
            CSSBoxType::MARGIN => emlite::Val::from("margin"),
            CSSBoxType::BORDER => emlite::Val::from("border"),
            CSSBoxType::PADDING => emlite::Val::from("padding"),
            CSSBoxType::CONTENT => emlite::Val::from("content"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DevicePostureType {
    CONTINUOUS,
    FOLDED,
}
impl FromVal for DevicePostureType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "continuous" => Self::CONTINUOUS,
            "folded" => Self::FOLDED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<DevicePostureType> for emlite::Val {
    fn from(s: DevicePostureType) -> emlite::Val {
        match s {
            DevicePostureType::CONTINUOUS => emlite::Val::from("continuous"),
            DevicePostureType::FOLDED => emlite::Val::from("folded"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ItemType {
    PRODUCT,
    SUBSCRIPTION,
}
impl FromVal for ItemType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "product" => Self::PRODUCT,
            "subscription" => Self::SUBSCRIPTION,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ItemType> for emlite::Val {
    fn from(s: ItemType) -> emlite::Val {
        match s {
            ItemType::PRODUCT => emlite::Val::from("product"),
            ItemType::SUBSCRIPTION => emlite::Val::from("subscription"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ShadowRootMode {
    OPEN,
    CLOSED,
}
impl FromVal for ShadowRootMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "open" => Self::OPEN,
            "closed" => Self::CLOSED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ShadowRootMode> for emlite::Val {
    fn from(s: ShadowRootMode) -> emlite::Val {
        match s {
            ShadowRootMode::OPEN => emlite::Val::from("open"),
            ShadowRootMode::CLOSED => emlite::Val::from("closed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SlotAssignmentMode {
    MANUAL,
    NAMED,
}
impl FromVal for SlotAssignmentMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "manual" => Self::MANUAL,
            "named" => Self::NAMED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SlotAssignmentMode> for emlite::Val {
    fn from(s: SlotAssignmentMode) -> emlite::Val {
        match s {
            SlotAssignmentMode::MANUAL => emlite::Val::from("manual"),
            SlotAssignmentMode::NAMED => emlite::Val::from("named"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum UnderlineStyle {
    NONE,
    SOLID,
    DOTTED,
    DASHED,
    WAVY,
}
impl FromVal for UnderlineStyle {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "solid" => Self::SOLID,
            "dotted" => Self::DOTTED,
            "dashed" => Self::DASHED,
            "wavy" => Self::WAVY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<UnderlineStyle> for emlite::Val {
    fn from(s: UnderlineStyle) -> emlite::Val {
        match s {
            UnderlineStyle::NONE => emlite::Val::from("none"),
            UnderlineStyle::SOLID => emlite::Val::from("solid"),
            UnderlineStyle::DOTTED => emlite::Val::from("dotted"),
            UnderlineStyle::DASHED => emlite::Val::from("dashed"),
            UnderlineStyle::WAVY => emlite::Val::from("wavy"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum UnderlineThickness {
    NONE,
    THIN,
    THICK,
}
impl FromVal for UnderlineThickness {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "thin" => Self::THIN,
            "thick" => Self::THICK,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<UnderlineThickness> for emlite::Val {
    fn from(s: UnderlineThickness) -> emlite::Val {
        match s {
            UnderlineThickness::NONE => emlite::Val::from("none"),
            UnderlineThickness::THIN => emlite::Val::from("thin"),
            UnderlineThickness::THICK => emlite::Val::from("thick"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaKeysRequirement {
    REQUIRED,
    OPTIONAL,
    NOT_ALLOWED,
}
impl FromVal for MediaKeysRequirement {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "required" => Self::REQUIRED,
            "optional" => Self::OPTIONAL,
            "not-allowed" => Self::NOT_ALLOWED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaKeysRequirement> for emlite::Val {
    fn from(s: MediaKeysRequirement) -> emlite::Val {
        match s {
            MediaKeysRequirement::REQUIRED => emlite::Val::from("required"),
            MediaKeysRequirement::OPTIONAL => emlite::Val::from("optional"),
            MediaKeysRequirement::NOT_ALLOWED => emlite::Val::from("not-allowed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaKeySessionType {
    TEMPORARY,
    PERSISTENT_LICENSE,
}
impl FromVal for MediaKeySessionType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "temporary" => Self::TEMPORARY,
            "persistent-license" => Self::PERSISTENT_LICENSE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaKeySessionType> for emlite::Val {
    fn from(s: MediaKeySessionType) -> emlite::Val {
        match s {
            MediaKeySessionType::TEMPORARY => emlite::Val::from("temporary"),
            MediaKeySessionType::PERSISTENT_LICENSE => emlite::Val::from("persistent-license"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaKeySessionClosedReason {
    INTERNAL_ERROR,
    CLOSED_BY_APPLICATION,
    RELEASE_ACKNOWLEDGED,
    HARDWARE_CONTEXT_RESET,
    RESOURCE_EVICTED,
}
impl FromVal for MediaKeySessionClosedReason {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "internal-error" => Self::INTERNAL_ERROR,
            "closed-by-application" => Self::CLOSED_BY_APPLICATION,
            "release-acknowledged" => Self::RELEASE_ACKNOWLEDGED,
            "hardware-context-reset" => Self::HARDWARE_CONTEXT_RESET,
            "resource-evicted" => Self::RESOURCE_EVICTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaKeySessionClosedReason> for emlite::Val {
    fn from(s: MediaKeySessionClosedReason) -> emlite::Val {
        match s {
            MediaKeySessionClosedReason::INTERNAL_ERROR => emlite::Val::from("internal-error"),
            MediaKeySessionClosedReason::CLOSED_BY_APPLICATION => {
                emlite::Val::from("closed-by-application")
            }
            MediaKeySessionClosedReason::RELEASE_ACKNOWLEDGED => {
                emlite::Val::from("release-acknowledged")
            }
            MediaKeySessionClosedReason::HARDWARE_CONTEXT_RESET => {
                emlite::Val::from("hardware-context-reset")
            }
            MediaKeySessionClosedReason::RESOURCE_EVICTED => emlite::Val::from("resource-evicted"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaKeyStatus {
    USABLE,
    EXPIRED,
    RELEASED,
    OUTPUT_RESTRICTED,
    OUTPUT_DOWNSCALED,
    USABLE_IN_FUTURE,
    STATUS_PENDING,
    INTERNAL_ERROR,
}
impl FromVal for MediaKeyStatus {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "usable" => Self::USABLE,
            "expired" => Self::EXPIRED,
            "released" => Self::RELEASED,
            "output-restricted" => Self::OUTPUT_RESTRICTED,
            "output-downscaled" => Self::OUTPUT_DOWNSCALED,
            "usable-in-future" => Self::USABLE_IN_FUTURE,
            "status-pending" => Self::STATUS_PENDING,
            "internal-error" => Self::INTERNAL_ERROR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaKeyStatus> for emlite::Val {
    fn from(s: MediaKeyStatus) -> emlite::Val {
        match s {
            MediaKeyStatus::USABLE => emlite::Val::from("usable"),
            MediaKeyStatus::EXPIRED => emlite::Val::from("expired"),
            MediaKeyStatus::RELEASED => emlite::Val::from("released"),
            MediaKeyStatus::OUTPUT_RESTRICTED => emlite::Val::from("output-restricted"),
            MediaKeyStatus::OUTPUT_DOWNSCALED => emlite::Val::from("output-downscaled"),
            MediaKeyStatus::USABLE_IN_FUTURE => emlite::Val::from("usable-in-future"),
            MediaKeyStatus::STATUS_PENDING => emlite::Val::from("status-pending"),
            MediaKeyStatus::INTERNAL_ERROR => emlite::Val::from("internal-error"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaKeyMessageType {
    LICENSE_REQUEST,
    LICENSE_RENEWAL,
    LICENSE_RELEASE,
    INDIVIDUALIZATION_REQUEST,
}
impl FromVal for MediaKeyMessageType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "license-request" => Self::LICENSE_REQUEST,
            "license-renewal" => Self::LICENSE_RENEWAL,
            "license-release" => Self::LICENSE_RELEASE,
            "individualization-request" => Self::INDIVIDUALIZATION_REQUEST,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaKeyMessageType> for emlite::Val {
    fn from(s: MediaKeyMessageType) -> emlite::Val {
        match s {
            MediaKeyMessageType::LICENSE_REQUEST => emlite::Val::from("license-request"),
            MediaKeyMessageType::LICENSE_RENEWAL => emlite::Val::from("license-renewal"),
            MediaKeyMessageType::LICENSE_RELEASE => emlite::Val::from("license-release"),
            MediaKeyMessageType::INDIVIDUALIZATION_REQUEST => {
                emlite::Val::from("individualization-request")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IdentityCredentialRequestOptionsContext {
    SIGNIN,
    SIGNUP,
    USE_,
    CONTINUE_,
}
impl FromVal for IdentityCredentialRequestOptionsContext {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "signin" => Self::SIGNIN,
            "signup" => Self::SIGNUP,
            "use" => Self::USE_,
            "continue" => Self::CONTINUE_,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<IdentityCredentialRequestOptionsContext> for emlite::Val {
    fn from(s: IdentityCredentialRequestOptionsContext) -> emlite::Val {
        match s {
            IdentityCredentialRequestOptionsContext::SIGNIN => emlite::Val::from("signin"),
            IdentityCredentialRequestOptionsContext::SIGNUP => emlite::Val::from("signup"),
            IdentityCredentialRequestOptionsContext::USE_ => emlite::Val::from("use"),
            IdentityCredentialRequestOptionsContext::CONTINUE_ => emlite::Val::from("continue"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IdentityCredentialRequestOptionsMode {
    ACTIVE,
    PASSIVE,
}
impl FromVal for IdentityCredentialRequestOptionsMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "active" => Self::ACTIVE,
            "passive" => Self::PASSIVE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<IdentityCredentialRequestOptionsMode> for emlite::Val {
    fn from(s: IdentityCredentialRequestOptionsMode) -> emlite::Val {
        match s {
            IdentityCredentialRequestOptionsMode::ACTIVE => emlite::Val::from("active"),
            IdentityCredentialRequestOptionsMode::PASSIVE => emlite::Val::from("passive"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OpaqueProperty {
    OPAQUE,
}
impl FromVal for OpaqueProperty {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "opaque" => Self::OPAQUE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OpaqueProperty> for emlite::Val {
    fn from(s: OpaqueProperty) -> emlite::Val {
        match s {
            OpaqueProperty::OPAQUE => emlite::Val::from("opaque"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FenceReportingDestination {
    BUYER,
    SELLER,
    COMPONENT_SELLER,
    DIRECT_SELLER,
    SHARED_STORAGE_SELECT_URL,
}
impl FromVal for FenceReportingDestination {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "buyer" => Self::BUYER,
            "seller" => Self::SELLER,
            "component-seller" => Self::COMPONENT_SELLER,
            "direct-seller" => Self::DIRECT_SELLER,
            "shared-storage-select-url" => Self::SHARED_STORAGE_SELECT_URL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FenceReportingDestination> for emlite::Val {
    fn from(s: FenceReportingDestination) -> emlite::Val {
        match s {
            FenceReportingDestination::BUYER => emlite::Val::from("buyer"),
            FenceReportingDestination::SELLER => emlite::Val::from("seller"),
            FenceReportingDestination::COMPONENT_SELLER => emlite::Val::from("component-seller"),
            FenceReportingDestination::DIRECT_SELLER => emlite::Val::from("direct-seller"),
            FenceReportingDestination::SHARED_STORAGE_SELECT_URL => {
                emlite::Val::from("shared-storage-select-url")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RequestDestination {
    NONE,
    AUDIO,
    AUDIOWORKLET,
    DOCUMENT,
    EMBED,
    FONT,
    FRAME,
    IFRAME,
    IMAGE,
    JSON,
    MANIFEST,
    OBJECT,
    PAINTWORKLET,
    REPORT,
    SCRIPT,
    SHAREDWORKER,
    STYLE,
    TRACK,
    VIDEO,
    WORKER,
    XSLT,
}
impl FromVal for RequestDestination {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "audio" => Self::AUDIO,
            "audioworklet" => Self::AUDIOWORKLET,
            "document" => Self::DOCUMENT,
            "embed" => Self::EMBED,
            "font" => Self::FONT,
            "frame" => Self::FRAME,
            "iframe" => Self::IFRAME,
            "image" => Self::IMAGE,
            "json" => Self::JSON,
            "manifest" => Self::MANIFEST,
            "object" => Self::OBJECT,
            "paintworklet" => Self::PAINTWORKLET,
            "report" => Self::REPORT,
            "script" => Self::SCRIPT,
            "sharedworker" => Self::SHAREDWORKER,
            "style" => Self::STYLE,
            "track" => Self::TRACK,
            "video" => Self::VIDEO,
            "worker" => Self::WORKER,
            "xslt" => Self::XSLT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RequestDestination> for emlite::Val {
    fn from(s: RequestDestination) -> emlite::Val {
        match s {
            RequestDestination::NONE => emlite::Val::from(""),
            RequestDestination::AUDIO => emlite::Val::from("audio"),
            RequestDestination::AUDIOWORKLET => emlite::Val::from("audioworklet"),
            RequestDestination::DOCUMENT => emlite::Val::from("document"),
            RequestDestination::EMBED => emlite::Val::from("embed"),
            RequestDestination::FONT => emlite::Val::from("font"),
            RequestDestination::FRAME => emlite::Val::from("frame"),
            RequestDestination::IFRAME => emlite::Val::from("iframe"),
            RequestDestination::IMAGE => emlite::Val::from("image"),
            RequestDestination::JSON => emlite::Val::from("json"),
            RequestDestination::MANIFEST => emlite::Val::from("manifest"),
            RequestDestination::OBJECT => emlite::Val::from("object"),
            RequestDestination::PAINTWORKLET => emlite::Val::from("paintworklet"),
            RequestDestination::REPORT => emlite::Val::from("report"),
            RequestDestination::SCRIPT => emlite::Val::from("script"),
            RequestDestination::SHAREDWORKER => emlite::Val::from("sharedworker"),
            RequestDestination::STYLE => emlite::Val::from("style"),
            RequestDestination::TRACK => emlite::Val::from("track"),
            RequestDestination::VIDEO => emlite::Val::from("video"),
            RequestDestination::WORKER => emlite::Val::from("worker"),
            RequestDestination::XSLT => emlite::Val::from("xslt"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RequestMode {
    NAVIGATE,
    SAME_ORIGIN,
    NO_CORS,
    CORS,
}
impl FromVal for RequestMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "navigate" => Self::NAVIGATE,
            "same-origin" => Self::SAME_ORIGIN,
            "no-cors" => Self::NO_CORS,
            "cors" => Self::CORS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RequestMode> for emlite::Val {
    fn from(s: RequestMode) -> emlite::Val {
        match s {
            RequestMode::NAVIGATE => emlite::Val::from("navigate"),
            RequestMode::SAME_ORIGIN => emlite::Val::from("same-origin"),
            RequestMode::NO_CORS => emlite::Val::from("no-cors"),
            RequestMode::CORS => emlite::Val::from("cors"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RequestCredentials {
    OMIT,
    SAME_ORIGIN,
    INCLUDE,
}
impl FromVal for RequestCredentials {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "omit" => Self::OMIT,
            "same-origin" => Self::SAME_ORIGIN,
            "include" => Self::INCLUDE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RequestCredentials> for emlite::Val {
    fn from(s: RequestCredentials) -> emlite::Val {
        match s {
            RequestCredentials::OMIT => emlite::Val::from("omit"),
            RequestCredentials::SAME_ORIGIN => emlite::Val::from("same-origin"),
            RequestCredentials::INCLUDE => emlite::Val::from("include"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RequestCache {
    DEFAULT,
    NO_STORE,
    RELOAD,
    NO_CACHE,
    FORCE_CACHE,
    ONLY_IF_CACHED,
}
impl FromVal for RequestCache {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "default" => Self::DEFAULT,
            "no-store" => Self::NO_STORE,
            "reload" => Self::RELOAD,
            "no-cache" => Self::NO_CACHE,
            "force-cache" => Self::FORCE_CACHE,
            "only-if-cached" => Self::ONLY_IF_CACHED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RequestCache> for emlite::Val {
    fn from(s: RequestCache) -> emlite::Val {
        match s {
            RequestCache::DEFAULT => emlite::Val::from("default"),
            RequestCache::NO_STORE => emlite::Val::from("no-store"),
            RequestCache::RELOAD => emlite::Val::from("reload"),
            RequestCache::NO_CACHE => emlite::Val::from("no-cache"),
            RequestCache::FORCE_CACHE => emlite::Val::from("force-cache"),
            RequestCache::ONLY_IF_CACHED => emlite::Val::from("only-if-cached"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RequestRedirect {
    FOLLOW,
    ERROR,
    MANUAL,
}
impl FromVal for RequestRedirect {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "follow" => Self::FOLLOW,
            "error" => Self::ERROR,
            "manual" => Self::MANUAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RequestRedirect> for emlite::Val {
    fn from(s: RequestRedirect) -> emlite::Val {
        match s {
            RequestRedirect::FOLLOW => emlite::Val::from("follow"),
            RequestRedirect::ERROR => emlite::Val::from("error"),
            RequestRedirect::MANUAL => emlite::Val::from("manual"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RequestDuplex {
    HALF,
}
impl FromVal for RequestDuplex {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "half" => Self::HALF,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RequestDuplex> for emlite::Val {
    fn from(s: RequestDuplex) -> emlite::Val {
        match s {
            RequestDuplex::HALF => emlite::Val::from("half"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RequestPriority {
    HIGH,
    LOW,
    AUTO,
}
impl FromVal for RequestPriority {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "high" => Self::HIGH,
            "low" => Self::LOW,
            "auto" => Self::AUTO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RequestPriority> for emlite::Val {
    fn from(s: RequestPriority) -> emlite::Val {
        match s {
            RequestPriority::HIGH => emlite::Val::from("high"),
            RequestPriority::LOW => emlite::Val::from("low"),
            RequestPriority::AUTO => emlite::Val::from("auto"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ResponseType {
    BASIC,
    CORS,
    DEFAULT,
    ERROR,
    OPAQUE,
    OPAQUEREDIRECT,
}
impl FromVal for ResponseType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "basic" => Self::BASIC,
            "cors" => Self::CORS,
            "default" => Self::DEFAULT,
            "error" => Self::ERROR,
            "opaque" => Self::OPAQUE,
            "opaqueredirect" => Self::OPAQUEREDIRECT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ResponseType> for emlite::Val {
    fn from(s: ResponseType) -> emlite::Val {
        match s {
            ResponseType::BASIC => emlite::Val::from("basic"),
            ResponseType::CORS => emlite::Val::from("cors"),
            ResponseType::DEFAULT => emlite::Val::from("default"),
            ResponseType::ERROR => emlite::Val::from("error"),
            ResponseType::OPAQUE => emlite::Val::from("opaque"),
            ResponseType::OPAQUEREDIRECT => emlite::Val::from("opaqueredirect"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FileSystemPermissionMode {
    READ,
    READWRITE,
}
impl FromVal for FileSystemPermissionMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "read" => Self::READ,
            "readwrite" => Self::READWRITE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FileSystemPermissionMode> for emlite::Val {
    fn from(s: FileSystemPermissionMode) -> emlite::Val {
        match s {
            FileSystemPermissionMode::READ => emlite::Val::from("read"),
            FileSystemPermissionMode::READWRITE => emlite::Val::from("readwrite"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WellKnownDirectory {
    DESKTOP,
    DOCUMENTS,
    DOWNLOADS,
    MUSIC,
    PICTURES,
    VIDEOS,
}
impl FromVal for WellKnownDirectory {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "desktop" => Self::DESKTOP,
            "documents" => Self::DOCUMENTS,
            "downloads" => Self::DOWNLOADS,
            "music" => Self::MUSIC,
            "pictures" => Self::PICTURES,
            "videos" => Self::VIDEOS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WellKnownDirectory> for emlite::Val {
    fn from(s: WellKnownDirectory) -> emlite::Val {
        match s {
            WellKnownDirectory::DESKTOP => emlite::Val::from("desktop"),
            WellKnownDirectory::DOCUMENTS => emlite::Val::from("documents"),
            WellKnownDirectory::DOWNLOADS => emlite::Val::from("downloads"),
            WellKnownDirectory::MUSIC => emlite::Val::from("music"),
            WellKnownDirectory::PICTURES => emlite::Val::from("pictures"),
            WellKnownDirectory::VIDEOS => emlite::Val::from("videos"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FileSystemHandleKind {
    FILE,
    DIRECTORY,
}
impl FromVal for FileSystemHandleKind {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "file" => Self::FILE,
            "directory" => Self::DIRECTORY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FileSystemHandleKind> for emlite::Val {
    fn from(s: FileSystemHandleKind) -> emlite::Val {
        match s {
            FileSystemHandleKind::FILE => emlite::Val::from("file"),
            FileSystemHandleKind::DIRECTORY => emlite::Val::from("directory"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WriteCommandType {
    WRITE,
    SEEK,
    TRUNCATE,
}
impl FromVal for WriteCommandType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "write" => Self::WRITE,
            "seek" => Self::SEEK,
            "truncate" => Self::TRUNCATE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WriteCommandType> for emlite::Val {
    fn from(s: WriteCommandType) -> emlite::Val {
        match s {
            WriteCommandType::WRITE => emlite::Val::from("write"),
            WriteCommandType::SEEK => emlite::Val::from("seek"),
            WriteCommandType::TRUNCATE => emlite::Val::from("truncate"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FullscreenNavigationUI {
    AUTO,
    SHOW,
    HIDE,
}
impl FromVal for FullscreenNavigationUI {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            "show" => Self::SHOW,
            "hide" => Self::HIDE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FullscreenNavigationUI> for emlite::Val {
    fn from(s: FullscreenNavigationUI) -> emlite::Val {
        match s {
            FullscreenNavigationUI::AUTO => emlite::Val::from("auto"),
            FullscreenNavigationUI::SHOW => emlite::Val::from("show"),
            FullscreenNavigationUI::HIDE => emlite::Val::from("hide"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GamepadHand {
    NONE,
    LEFT,
    RIGHT,
}
impl FromVal for GamepadHand {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GamepadHand> for emlite::Val {
    fn from(s: GamepadHand) -> emlite::Val {
        match s {
            GamepadHand::NONE => emlite::Val::from(""),
            GamepadHand::LEFT => emlite::Val::from("left"),
            GamepadHand::RIGHT => emlite::Val::from("right"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GamepadMappingType {
    NONE,
    STANDARD,
    XR_STANDARD,
}
impl FromVal for GamepadMappingType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "standard" => Self::STANDARD,
            "xr-standard" => Self::XR_STANDARD,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GamepadMappingType> for emlite::Val {
    fn from(s: GamepadMappingType) -> emlite::Val {
        match s {
            GamepadMappingType::NONE => emlite::Val::from(""),
            GamepadMappingType::STANDARD => emlite::Val::from("standard"),
            GamepadMappingType::XR_STANDARD => emlite::Val::from("xr-standard"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GamepadHapticsResult {
    COMPLETE,
    PREEMPTED,
}
impl FromVal for GamepadHapticsResult {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "complete" => Self::COMPLETE,
            "preempted" => Self::PREEMPTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GamepadHapticsResult> for emlite::Val {
    fn from(s: GamepadHapticsResult) -> emlite::Val {
        match s {
            GamepadHapticsResult::COMPLETE => emlite::Val::from("complete"),
            GamepadHapticsResult::PREEMPTED => emlite::Val::from("preempted"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GamepadHapticEffectType {
    DUAL_RUMBLE,
    TRIGGER_RUMBLE,
}
impl FromVal for GamepadHapticEffectType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "dual-rumble" => Self::DUAL_RUMBLE,
            "trigger-rumble" => Self::TRIGGER_RUMBLE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GamepadHapticEffectType> for emlite::Val {
    fn from(s: GamepadHapticEffectType) -> emlite::Val {
        match s {
            GamepadHapticEffectType::DUAL_RUMBLE => emlite::Val::from("dual-rumble"),
            GamepadHapticEffectType::TRIGGER_RUMBLE => emlite::Val::from("trigger-rumble"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GyroscopeLocalCoordinateSystem {
    DEVICE,
    SCREEN,
}
impl FromVal for GyroscopeLocalCoordinateSystem {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "device" => Self::DEVICE,
            "screen" => Self::SCREEN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GyroscopeLocalCoordinateSystem> for emlite::Val {
    fn from(s: GyroscopeLocalCoordinateSystem) -> emlite::Val {
        match s {
            GyroscopeLocalCoordinateSystem::DEVICE => emlite::Val::from("device"),
            GyroscopeLocalCoordinateSystem::SCREEN => emlite::Val::from("screen"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum HandwritingRecognitionType {
    TEXT,
    PER_CHARACTER,
}
impl FromVal for HandwritingRecognitionType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "text" => Self::TEXT,
            "per-character" => Self::PER_CHARACTER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<HandwritingRecognitionType> for emlite::Val {
    fn from(s: HandwritingRecognitionType) -> emlite::Val {
        match s {
            HandwritingRecognitionType::TEXT => emlite::Val::from("text"),
            HandwritingRecognitionType::PER_CHARACTER => emlite::Val::from("per-character"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum HandwritingInputType {
    MOUSE,
    STYLUS,
    TOUCH,
}
impl FromVal for HandwritingInputType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "mouse" => Self::MOUSE,
            "stylus" => Self::STYLUS,
            "touch" => Self::TOUCH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<HandwritingInputType> for emlite::Val {
    fn from(s: HandwritingInputType) -> emlite::Val {
        match s {
            HandwritingInputType::MOUSE => emlite::Val::from("mouse"),
            HandwritingInputType::STYLUS => emlite::Val::from("stylus"),
            HandwritingInputType::TOUCH => emlite::Val::from("touch"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DocumentReadyState {
    LOADING,
    INTERACTIVE,
    COMPLETE,
}
impl FromVal for DocumentReadyState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "loading" => Self::LOADING,
            "interactive" => Self::INTERACTIVE,
            "complete" => Self::COMPLETE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<DocumentReadyState> for emlite::Val {
    fn from(s: DocumentReadyState) -> emlite::Val {
        match s {
            DocumentReadyState::LOADING => emlite::Val::from("loading"),
            DocumentReadyState::INTERACTIVE => emlite::Val::from("interactive"),
            DocumentReadyState::COMPLETE => emlite::Val::from("complete"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DocumentVisibilityState {
    VISIBLE,
    HIDDEN,
}
impl FromVal for DocumentVisibilityState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "visible" => Self::VISIBLE,
            "hidden" => Self::HIDDEN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<DocumentVisibilityState> for emlite::Val {
    fn from(s: DocumentVisibilityState) -> emlite::Val {
        match s {
            DocumentVisibilityState::VISIBLE => emlite::Val::from("visible"),
            DocumentVisibilityState::HIDDEN => emlite::Val::from("hidden"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanPlayTypeResult {
    NONE,
    MAYBE,
    PROBABLY,
}
impl FromVal for CanPlayTypeResult {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "maybe" => Self::MAYBE,
            "probably" => Self::PROBABLY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanPlayTypeResult> for emlite::Val {
    fn from(s: CanPlayTypeResult) -> emlite::Val {
        match s {
            CanPlayTypeResult::NONE => emlite::Val::from(""),
            CanPlayTypeResult::MAYBE => emlite::Val::from("maybe"),
            CanPlayTypeResult::PROBABLY => emlite::Val::from("probably"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TextTrackMode {
    DISABLED,
    HIDDEN,
    SHOWING,
}
impl FromVal for TextTrackMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "disabled" => Self::DISABLED,
            "hidden" => Self::HIDDEN,
            "showing" => Self::SHOWING,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<TextTrackMode> for emlite::Val {
    fn from(s: TextTrackMode) -> emlite::Val {
        match s {
            TextTrackMode::DISABLED => emlite::Val::from("disabled"),
            TextTrackMode::HIDDEN => emlite::Val::from("hidden"),
            TextTrackMode::SHOWING => emlite::Val::from("showing"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TextTrackKind {
    SUBTITLES,
    CAPTIONS,
    DESCRIPTIONS,
    CHAPTERS,
    METADATA,
}
impl FromVal for TextTrackKind {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "subtitles" => Self::SUBTITLES,
            "captions" => Self::CAPTIONS,
            "descriptions" => Self::DESCRIPTIONS,
            "chapters" => Self::CHAPTERS,
            "metadata" => Self::METADATA,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<TextTrackKind> for emlite::Val {
    fn from(s: TextTrackKind) -> emlite::Val {
        match s {
            TextTrackKind::SUBTITLES => emlite::Val::from("subtitles"),
            TextTrackKind::CAPTIONS => emlite::Val::from("captions"),
            TextTrackKind::DESCRIPTIONS => emlite::Val::from("descriptions"),
            TextTrackKind::CHAPTERS => emlite::Val::from("chapters"),
            TextTrackKind::METADATA => emlite::Val::from("metadata"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SelectionMode {
    SELECT,
    START,
    END,
    PRESERVE,
}
impl FromVal for SelectionMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "select" => Self::SELECT,
            "start" => Self::START,
            "end" => Self::END,
            "preserve" => Self::PRESERVE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SelectionMode> for emlite::Val {
    fn from(s: SelectionMode) -> emlite::Val {
        match s {
            SelectionMode::SELECT => emlite::Val::from("select"),
            SelectionMode::START => emlite::Val::from("start"),
            SelectionMode::END => emlite::Val::from("end"),
            SelectionMode::PRESERVE => emlite::Val::from("preserve"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PredefinedColorSpace {
    SRGB,
    DISPLAY_P3,
}
impl FromVal for PredefinedColorSpace {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "srgb" => Self::SRGB,
            "display-p3" => Self::DISPLAY_P3,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PredefinedColorSpace> for emlite::Val {
    fn from(s: PredefinedColorSpace) -> emlite::Val {
        match s {
            PredefinedColorSpace::SRGB => emlite::Val::from("srgb"),
            PredefinedColorSpace::DISPLAY_P3 => emlite::Val::from("display-p3"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasColorType {
    UNORM8,
    FLOAT16,
}
impl FromVal for CanvasColorType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "unorm8" => Self::UNORM8,
            "float16" => Self::FLOAT16,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasColorType> for emlite::Val {
    fn from(s: CanvasColorType) -> emlite::Val {
        match s {
            CanvasColorType::UNORM8 => emlite::Val::from("unorm8"),
            CanvasColorType::FLOAT16 => emlite::Val::from("float16"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasFillRule {
    NONZERO,
    EVENODD,
}
impl FromVal for CanvasFillRule {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "nonzero" => Self::NONZERO,
            "evenodd" => Self::EVENODD,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasFillRule> for emlite::Val {
    fn from(s: CanvasFillRule) -> emlite::Val {
        match s {
            CanvasFillRule::NONZERO => emlite::Val::from("nonzero"),
            CanvasFillRule::EVENODD => emlite::Val::from("evenodd"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ImageSmoothingQuality {
    LOW,
    MEDIUM,
    HIGH,
}
impl FromVal for ImageSmoothingQuality {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "low" => Self::LOW,
            "medium" => Self::MEDIUM,
            "high" => Self::HIGH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ImageSmoothingQuality> for emlite::Val {
    fn from(s: ImageSmoothingQuality) -> emlite::Val {
        match s {
            ImageSmoothingQuality::LOW => emlite::Val::from("low"),
            ImageSmoothingQuality::MEDIUM => emlite::Val::from("medium"),
            ImageSmoothingQuality::HIGH => emlite::Val::from("high"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasLineCap {
    BUTT,
    ROUND,
    SQUARE,
}
impl FromVal for CanvasLineCap {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "butt" => Self::BUTT,
            "round" => Self::ROUND,
            "square" => Self::SQUARE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasLineCap> for emlite::Val {
    fn from(s: CanvasLineCap) -> emlite::Val {
        match s {
            CanvasLineCap::BUTT => emlite::Val::from("butt"),
            CanvasLineCap::ROUND => emlite::Val::from("round"),
            CanvasLineCap::SQUARE => emlite::Val::from("square"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasLineJoin {
    ROUND,
    BEVEL,
    MITER,
}
impl FromVal for CanvasLineJoin {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "round" => Self::ROUND,
            "bevel" => Self::BEVEL,
            "miter" => Self::MITER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasLineJoin> for emlite::Val {
    fn from(s: CanvasLineJoin) -> emlite::Val {
        match s {
            CanvasLineJoin::ROUND => emlite::Val::from("round"),
            CanvasLineJoin::BEVEL => emlite::Val::from("bevel"),
            CanvasLineJoin::MITER => emlite::Val::from("miter"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasTextAlign {
    START,
    END,
    LEFT,
    RIGHT,
    CENTER,
}
impl FromVal for CanvasTextAlign {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "start" => Self::START,
            "end" => Self::END,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
            "center" => Self::CENTER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasTextAlign> for emlite::Val {
    fn from(s: CanvasTextAlign) -> emlite::Val {
        match s {
            CanvasTextAlign::START => emlite::Val::from("start"),
            CanvasTextAlign::END => emlite::Val::from("end"),
            CanvasTextAlign::LEFT => emlite::Val::from("left"),
            CanvasTextAlign::RIGHT => emlite::Val::from("right"),
            CanvasTextAlign::CENTER => emlite::Val::from("center"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasTextBaseline {
    TOP,
    HANGING,
    MIDDLE,
    ALPHABETIC,
    IDEOGRAPHIC,
    BOTTOM,
}
impl FromVal for CanvasTextBaseline {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "top" => Self::TOP,
            "hanging" => Self::HANGING,
            "middle" => Self::MIDDLE,
            "alphabetic" => Self::ALPHABETIC,
            "ideographic" => Self::IDEOGRAPHIC,
            "bottom" => Self::BOTTOM,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasTextBaseline> for emlite::Val {
    fn from(s: CanvasTextBaseline) -> emlite::Val {
        match s {
            CanvasTextBaseline::TOP => emlite::Val::from("top"),
            CanvasTextBaseline::HANGING => emlite::Val::from("hanging"),
            CanvasTextBaseline::MIDDLE => emlite::Val::from("middle"),
            CanvasTextBaseline::ALPHABETIC => emlite::Val::from("alphabetic"),
            CanvasTextBaseline::IDEOGRAPHIC => emlite::Val::from("ideographic"),
            CanvasTextBaseline::BOTTOM => emlite::Val::from("bottom"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasDirection {
    LTR,
    RTL,
    INHERIT,
}
impl FromVal for CanvasDirection {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "ltr" => Self::LTR,
            "rtl" => Self::RTL,
            "inherit" => Self::INHERIT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasDirection> for emlite::Val {
    fn from(s: CanvasDirection) -> emlite::Val {
        match s {
            CanvasDirection::LTR => emlite::Val::from("ltr"),
            CanvasDirection::RTL => emlite::Val::from("rtl"),
            CanvasDirection::INHERIT => emlite::Val::from("inherit"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasFontKerning {
    AUTO,
    NORMAL,
    NONE,
}
impl FromVal for CanvasFontKerning {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            "normal" => Self::NORMAL,
            "none" => Self::NONE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasFontKerning> for emlite::Val {
    fn from(s: CanvasFontKerning) -> emlite::Val {
        match s {
            CanvasFontKerning::AUTO => emlite::Val::from("auto"),
            CanvasFontKerning::NORMAL => emlite::Val::from("normal"),
            CanvasFontKerning::NONE => emlite::Val::from("none"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasFontStretch {
    ULTRA_CONDENSED,
    EXTRA_CONDENSED,
    CONDENSED,
    SEMI_CONDENSED,
    NORMAL,
    SEMI_EXPANDED,
    EXPANDED,
    EXTRA_EXPANDED,
    ULTRA_EXPANDED,
}
impl FromVal for CanvasFontStretch {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "ultra-condensed" => Self::ULTRA_CONDENSED,
            "extra-condensed" => Self::EXTRA_CONDENSED,
            "condensed" => Self::CONDENSED,
            "semi-condensed" => Self::SEMI_CONDENSED,
            "normal" => Self::NORMAL,
            "semi-expanded" => Self::SEMI_EXPANDED,
            "expanded" => Self::EXPANDED,
            "extra-expanded" => Self::EXTRA_EXPANDED,
            "ultra-expanded" => Self::ULTRA_EXPANDED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasFontStretch> for emlite::Val {
    fn from(s: CanvasFontStretch) -> emlite::Val {
        match s {
            CanvasFontStretch::ULTRA_CONDENSED => emlite::Val::from("ultra-condensed"),
            CanvasFontStretch::EXTRA_CONDENSED => emlite::Val::from("extra-condensed"),
            CanvasFontStretch::CONDENSED => emlite::Val::from("condensed"),
            CanvasFontStretch::SEMI_CONDENSED => emlite::Val::from("semi-condensed"),
            CanvasFontStretch::NORMAL => emlite::Val::from("normal"),
            CanvasFontStretch::SEMI_EXPANDED => emlite::Val::from("semi-expanded"),
            CanvasFontStretch::EXPANDED => emlite::Val::from("expanded"),
            CanvasFontStretch::EXTRA_EXPANDED => emlite::Val::from("extra-expanded"),
            CanvasFontStretch::ULTRA_EXPANDED => emlite::Val::from("ultra-expanded"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasFontVariantCaps {
    NORMAL,
    SMALL_CAPS,
    ALL_SMALL_CAPS,
    PETITE_CAPS,
    ALL_PETITE_CAPS,
    UNICASE,
    TITLING_CAPS,
}
impl FromVal for CanvasFontVariantCaps {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "normal" => Self::NORMAL,
            "small-caps" => Self::SMALL_CAPS,
            "all-small-caps" => Self::ALL_SMALL_CAPS,
            "petite-caps" => Self::PETITE_CAPS,
            "all-petite-caps" => Self::ALL_PETITE_CAPS,
            "unicase" => Self::UNICASE,
            "titling-caps" => Self::TITLING_CAPS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasFontVariantCaps> for emlite::Val {
    fn from(s: CanvasFontVariantCaps) -> emlite::Val {
        match s {
            CanvasFontVariantCaps::NORMAL => emlite::Val::from("normal"),
            CanvasFontVariantCaps::SMALL_CAPS => emlite::Val::from("small-caps"),
            CanvasFontVariantCaps::ALL_SMALL_CAPS => emlite::Val::from("all-small-caps"),
            CanvasFontVariantCaps::PETITE_CAPS => emlite::Val::from("petite-caps"),
            CanvasFontVariantCaps::ALL_PETITE_CAPS => emlite::Val::from("all-petite-caps"),
            CanvasFontVariantCaps::UNICASE => emlite::Val::from("unicase"),
            CanvasFontVariantCaps::TITLING_CAPS => emlite::Val::from("titling-caps"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasTextRendering {
    AUTO,
    OPTIMIZE_SPEED,
    OPTIMIZE_LEGIBILITY,
    GEOMETRIC_PRECISION,
}
impl FromVal for CanvasTextRendering {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            "optimizeSpeed" => Self::OPTIMIZE_SPEED,
            "optimizeLegibility" => Self::OPTIMIZE_LEGIBILITY,
            "geometricPrecision" => Self::GEOMETRIC_PRECISION,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CanvasTextRendering> for emlite::Val {
    fn from(s: CanvasTextRendering) -> emlite::Val {
        match s {
            CanvasTextRendering::AUTO => emlite::Val::from("auto"),
            CanvasTextRendering::OPTIMIZE_SPEED => emlite::Val::from("optimizeSpeed"),
            CanvasTextRendering::OPTIMIZE_LEGIBILITY => emlite::Val::from("optimizeLegibility"),
            CanvasTextRendering::GEOMETRIC_PRECISION => emlite::Val::from("geometricPrecision"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OffscreenRenderingContextId {
    _2D,
    BITMAPRENDERER,
    WEBGL,
    WEBGL2,
    WEBGPU,
}
impl FromVal for OffscreenRenderingContextId {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "2d" => Self::_2D,
            "bitmaprenderer" => Self::BITMAPRENDERER,
            "webgl" => Self::WEBGL,
            "webgl2" => Self::WEBGL2,
            "webgpu" => Self::WEBGPU,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OffscreenRenderingContextId> for emlite::Val {
    fn from(s: OffscreenRenderingContextId) -> emlite::Val {
        match s {
            OffscreenRenderingContextId::_2D => emlite::Val::from("2d"),
            OffscreenRenderingContextId::BITMAPRENDERER => emlite::Val::from("bitmaprenderer"),
            OffscreenRenderingContextId::WEBGL => emlite::Val::from("webgl"),
            OffscreenRenderingContextId::WEBGL2 => emlite::Val::from("webgl2"),
            OffscreenRenderingContextId::WEBGPU => emlite::Val::from("webgpu"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScrollRestoration {
    AUTO,
    MANUAL,
}
impl FromVal for ScrollRestoration {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            "manual" => Self::MANUAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ScrollRestoration> for emlite::Val {
    fn from(s: ScrollRestoration) -> emlite::Val {
        match s {
            ScrollRestoration::AUTO => emlite::Val::from("auto"),
            ScrollRestoration::MANUAL => emlite::Val::from("manual"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum NavigationHistoryBehavior {
    AUTO,
    PUSH,
    REPLACE,
}
impl FromVal for NavigationHistoryBehavior {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            "push" => Self::PUSH,
            "replace" => Self::REPLACE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<NavigationHistoryBehavior> for emlite::Val {
    fn from(s: NavigationHistoryBehavior) -> emlite::Val {
        match s {
            NavigationHistoryBehavior::AUTO => emlite::Val::from("auto"),
            NavigationHistoryBehavior::PUSH => emlite::Val::from("push"),
            NavigationHistoryBehavior::REPLACE => emlite::Val::from("replace"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum NavigationType {
    PUSH,
    REPLACE,
    RELOAD,
    TRAVERSE,
}
impl FromVal for NavigationType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "push" => Self::PUSH,
            "replace" => Self::REPLACE,
            "reload" => Self::RELOAD,
            "traverse" => Self::TRAVERSE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<NavigationType> for emlite::Val {
    fn from(s: NavigationType) -> emlite::Val {
        match s {
            NavigationType::PUSH => emlite::Val::from("push"),
            NavigationType::REPLACE => emlite::Val::from("replace"),
            NavigationType::RELOAD => emlite::Val::from("reload"),
            NavigationType::TRAVERSE => emlite::Val::from("traverse"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum NavigationFocusReset {
    AFTER_TRANSITION,
    MANUAL,
}
impl FromVal for NavigationFocusReset {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "after-transition" => Self::AFTER_TRANSITION,
            "manual" => Self::MANUAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<NavigationFocusReset> for emlite::Val {
    fn from(s: NavigationFocusReset) -> emlite::Val {
        match s {
            NavigationFocusReset::AFTER_TRANSITION => emlite::Val::from("after-transition"),
            NavigationFocusReset::MANUAL => emlite::Val::from("manual"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum NavigationScrollBehavior {
    AFTER_TRANSITION,
    MANUAL,
}
impl FromVal for NavigationScrollBehavior {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "after-transition" => Self::AFTER_TRANSITION,
            "manual" => Self::MANUAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<NavigationScrollBehavior> for emlite::Val {
    fn from(s: NavigationScrollBehavior) -> emlite::Val {
        match s {
            NavigationScrollBehavior::AFTER_TRANSITION => emlite::Val::from("after-transition"),
            NavigationScrollBehavior::MANUAL => emlite::Val::from("manual"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DOMParserSupportedType {
    TEXT_HTML,
    TEXT_XML,
    APPLICATION_XML,
    APPLICATION_XHTML_XML,
    IMAGE_SVG_XML,
}
impl FromVal for DOMParserSupportedType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "text/html" => Self::TEXT_HTML,
            "text/xml" => Self::TEXT_XML,
            "application/xml" => Self::APPLICATION_XML,
            "application/xhtml+xml" => Self::APPLICATION_XHTML_XML,
            "image/svg+xml" => Self::IMAGE_SVG_XML,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<DOMParserSupportedType> for emlite::Val {
    fn from(s: DOMParserSupportedType) -> emlite::Val {
        match s {
            DOMParserSupportedType::TEXT_HTML => emlite::Val::from("text/html"),
            DOMParserSupportedType::TEXT_XML => emlite::Val::from("text/xml"),
            DOMParserSupportedType::APPLICATION_XML => emlite::Val::from("application/xml"),
            DOMParserSupportedType::APPLICATION_XHTML_XML => {
                emlite::Val::from("application/xhtml+xml")
            }
            DOMParserSupportedType::IMAGE_SVG_XML => emlite::Val::from("image/svg+xml"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ImageDataPixelFormat {
    RGBA_UNORM8,
    RGBA_FLOAT16,
}
impl FromVal for ImageDataPixelFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "rgba-unorm8" => Self::RGBA_UNORM8,
            "rgba-float16" => Self::RGBA_FLOAT16,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ImageDataPixelFormat> for emlite::Val {
    fn from(s: ImageDataPixelFormat) -> emlite::Val {
        match s {
            ImageDataPixelFormat::RGBA_UNORM8 => emlite::Val::from("rgba-unorm8"),
            ImageDataPixelFormat::RGBA_FLOAT16 => emlite::Val::from("rgba-float16"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ImageOrientation {
    FROM_IMAGE,
    FLIP_Y,
}
impl FromVal for ImageOrientation {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "from-image" => Self::FROM_IMAGE,
            "flipY" => Self::FLIP_Y,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ImageOrientation> for emlite::Val {
    fn from(s: ImageOrientation) -> emlite::Val {
        match s {
            ImageOrientation::FROM_IMAGE => emlite::Val::from("from-image"),
            ImageOrientation::FLIP_Y => emlite::Val::from("flipY"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PremultiplyAlpha {
    NONE,
    PREMULTIPLY,
    DEFAULT,
}
impl FromVal for PremultiplyAlpha {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "premultiply" => Self::PREMULTIPLY,
            "default" => Self::DEFAULT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PremultiplyAlpha> for emlite::Val {
    fn from(s: PremultiplyAlpha) -> emlite::Val {
        match s {
            PremultiplyAlpha::NONE => emlite::Val::from("none"),
            PremultiplyAlpha::PREMULTIPLY => emlite::Val::from("premultiply"),
            PremultiplyAlpha::DEFAULT => emlite::Val::from("default"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ColorSpaceConversion {
    NONE,
    DEFAULT,
}
impl FromVal for ColorSpaceConversion {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "default" => Self::DEFAULT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ColorSpaceConversion> for emlite::Val {
    fn from(s: ColorSpaceConversion) -> emlite::Val {
        match s {
            ColorSpaceConversion::NONE => emlite::Val::from("none"),
            ColorSpaceConversion::DEFAULT => emlite::Val::from("default"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ResizeQuality {
    PIXELATED,
    LOW,
    MEDIUM,
    HIGH,
}
impl FromVal for ResizeQuality {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "pixelated" => Self::PIXELATED,
            "low" => Self::LOW,
            "medium" => Self::MEDIUM,
            "high" => Self::HIGH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ResizeQuality> for emlite::Val {
    fn from(s: ResizeQuality) -> emlite::Val {
        match s {
            ResizeQuality::PIXELATED => emlite::Val::from("pixelated"),
            ResizeQuality::LOW => emlite::Val::from("low"),
            ResizeQuality::MEDIUM => emlite::Val::from("medium"),
            ResizeQuality::HIGH => emlite::Val::from("high"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WorkerType {
    CLASSIC,
    MODULE,
}
impl FromVal for WorkerType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "classic" => Self::CLASSIC,
            "module" => Self::MODULE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WorkerType> for emlite::Val {
    fn from(s: WorkerType) -> emlite::Val {
        match s {
            WorkerType::CLASSIC => emlite::Val::from("classic"),
            WorkerType::MODULE => emlite::Val::from("module"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum UserIdleState {
    ACTIVE,
    IDLE,
}
impl FromVal for UserIdleState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "active" => Self::ACTIVE,
            "idle" => Self::IDLE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<UserIdleState> for emlite::Val {
    fn from(s: UserIdleState) -> emlite::Val {
        match s {
            UserIdleState::ACTIVE => emlite::Val::from("active"),
            UserIdleState::IDLE => emlite::Val::from("idle"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScreenIdleState {
    LOCKED,
    UNLOCKED,
}
impl FromVal for ScreenIdleState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "locked" => Self::LOCKED,
            "unlocked" => Self::UNLOCKED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ScreenIdleState> for emlite::Val {
    fn from(s: ScreenIdleState) -> emlite::Val {
        match s {
            ScreenIdleState::LOCKED => emlite::Val::from("locked"),
            ScreenIdleState::UNLOCKED => emlite::Val::from("unlocked"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RedEyeReduction {
    NEVER,
    ALWAYS,
    CONTROLLABLE,
}
impl FromVal for RedEyeReduction {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "never" => Self::NEVER,
            "always" => Self::ALWAYS,
            "controllable" => Self::CONTROLLABLE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RedEyeReduction> for emlite::Val {
    fn from(s: RedEyeReduction) -> emlite::Val {
        match s {
            RedEyeReduction::NEVER => emlite::Val::from("never"),
            RedEyeReduction::ALWAYS => emlite::Val::from("always"),
            RedEyeReduction::CONTROLLABLE => emlite::Val::from("controllable"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FillLightMode {
    AUTO,
    OFF,
    FLASH,
}
impl FromVal for FillLightMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            "off" => Self::OFF,
            "flash" => Self::FLASH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FillLightMode> for emlite::Val {
    fn from(s: FillLightMode) -> emlite::Val {
        match s {
            FillLightMode::AUTO => emlite::Val::from("auto"),
            FillLightMode::OFF => emlite::Val::from("off"),
            FillLightMode::FLASH => emlite::Val::from("flash"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MeteringMode {
    NONE,
    MANUAL,
    SINGLE_SHOT,
    CONTINUOUS,
}
impl FromVal for MeteringMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "manual" => Self::MANUAL,
            "single-shot" => Self::SINGLE_SHOT,
            "continuous" => Self::CONTINUOUS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MeteringMode> for emlite::Val {
    fn from(s: MeteringMode) -> emlite::Val {
        match s {
            MeteringMode::NONE => emlite::Val::from("none"),
            MeteringMode::MANUAL => emlite::Val::from("manual"),
            MeteringMode::SINGLE_SHOT => emlite::Val::from("single-shot"),
            MeteringMode::CONTINUOUS => emlite::Val::from("continuous"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LoginStatus {
    LOGGED_IN,
    LOGGED_OUT,
}
impl FromVal for LoginStatus {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "logged-in" => Self::LOGGED_IN,
            "logged-out" => Self::LOGGED_OUT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<LoginStatus> for emlite::Val {
    fn from(s: LoginStatus) -> emlite::Val {
        match s {
            LoginStatus::LOGGED_IN => emlite::Val::from("logged-in"),
            LoginStatus::LOGGED_OUT => emlite::Val::from("logged-out"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScriptInvokerType {
    CLASSIC_SCRIPT,
    MODULE_SCRIPT,
    EVENT_LISTENER,
    USER_CALLBACK,
    RESOLVE_PROMISE,
    REJECT_PROMISE,
}
impl FromVal for ScriptInvokerType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "classic-script" => Self::CLASSIC_SCRIPT,
            "module-script" => Self::MODULE_SCRIPT,
            "event-listener" => Self::EVENT_LISTENER,
            "user-callback" => Self::USER_CALLBACK,
            "resolve-promise" => Self::RESOLVE_PROMISE,
            "reject-promise" => Self::REJECT_PROMISE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ScriptInvokerType> for emlite::Val {
    fn from(s: ScriptInvokerType) -> emlite::Val {
        match s {
            ScriptInvokerType::CLASSIC_SCRIPT => emlite::Val::from("classic-script"),
            ScriptInvokerType::MODULE_SCRIPT => emlite::Val::from("module-script"),
            ScriptInvokerType::EVENT_LISTENER => emlite::Val::from("event-listener"),
            ScriptInvokerType::USER_CALLBACK => emlite::Val::from("user-callback"),
            ScriptInvokerType::RESOLVE_PROMISE => emlite::Val::from("resolve-promise"),
            ScriptInvokerType::REJECT_PROMISE => emlite::Val::from("reject-promise"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScriptWindowAttribution {
    SELF_,
    DESCENDANT,
    ANCESTOR,
    SAME_PAGE,
    OTHER,
}
impl FromVal for ScriptWindowAttribution {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "self" => Self::SELF_,
            "descendant" => Self::DESCENDANT,
            "ancestor" => Self::ANCESTOR,
            "same-page" => Self::SAME_PAGE,
            "other" => Self::OTHER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ScriptWindowAttribution> for emlite::Val {
    fn from(s: ScriptWindowAttribution) -> emlite::Val {
        match s {
            ScriptWindowAttribution::SELF_ => emlite::Val::from("self"),
            ScriptWindowAttribution::DESCENDANT => emlite::Val::from("descendant"),
            ScriptWindowAttribution::ANCESTOR => emlite::Val::from("ancestor"),
            ScriptWindowAttribution::SAME_PAGE => emlite::Val::from("same-page"),
            ScriptWindowAttribution::OTHER => emlite::Val::from("other"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MagnetometerLocalCoordinateSystem {
    DEVICE,
    SCREEN,
}
impl FromVal for MagnetometerLocalCoordinateSystem {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "device" => Self::DEVICE,
            "screen" => Self::SCREEN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MagnetometerLocalCoordinateSystem> for emlite::Val {
    fn from(s: MagnetometerLocalCoordinateSystem) -> emlite::Val {
        match s {
            MagnetometerLocalCoordinateSystem::DEVICE => emlite::Val::from("device"),
            MagnetometerLocalCoordinateSystem::SCREEN => emlite::Val::from("screen"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AppBannerPromptOutcome {
    ACCEPTED,
    DISMISSED,
}
impl FromVal for AppBannerPromptOutcome {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "accepted" => Self::ACCEPTED,
            "dismissed" => Self::DISMISSED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AppBannerPromptOutcome> for emlite::Val {
    fn from(s: AppBannerPromptOutcome) -> emlite::Val {
        match s {
            AppBannerPromptOutcome::ACCEPTED => emlite::Val::from("accepted"),
            AppBannerPromptOutcome::DISMISSED => emlite::Val::from("dismissed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaDecodingType {
    FILE,
    MEDIA_SOURCE,
    WEBRTC,
}
impl FromVal for MediaDecodingType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "file" => Self::FILE,
            "media-source" => Self::MEDIA_SOURCE,
            "webrtc" => Self::WEBRTC,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaDecodingType> for emlite::Val {
    fn from(s: MediaDecodingType) -> emlite::Val {
        match s {
            MediaDecodingType::FILE => emlite::Val::from("file"),
            MediaDecodingType::MEDIA_SOURCE => emlite::Val::from("media-source"),
            MediaDecodingType::WEBRTC => emlite::Val::from("webrtc"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaEncodingType {
    RECORD,
    WEBRTC,
}
impl FromVal for MediaEncodingType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "record" => Self::RECORD,
            "webrtc" => Self::WEBRTC,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaEncodingType> for emlite::Val {
    fn from(s: MediaEncodingType) -> emlite::Val {
        match s {
            MediaEncodingType::RECORD => emlite::Val::from("record"),
            MediaEncodingType::WEBRTC => emlite::Val::from("webrtc"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum HdrMetadataType {
    SMPTE_ST2086,
    SMPTE_ST2094_10,
    SMPTE_ST2094_40,
}
impl FromVal for HdrMetadataType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "smpteSt2086" => Self::SMPTE_ST2086,
            "smpteSt2094-10" => Self::SMPTE_ST2094_10,
            "smpteSt2094-40" => Self::SMPTE_ST2094_40,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<HdrMetadataType> for emlite::Val {
    fn from(s: HdrMetadataType) -> emlite::Val {
        match s {
            HdrMetadataType::SMPTE_ST2086 => emlite::Val::from("smpteSt2086"),
            HdrMetadataType::SMPTE_ST2094_10 => emlite::Val::from("smpteSt2094-10"),
            HdrMetadataType::SMPTE_ST2094_40 => emlite::Val::from("smpteSt2094-40"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ColorGamut {
    SRGB,
    P3,
    REC2020,
}
impl FromVal for ColorGamut {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "srgb" => Self::SRGB,
            "p3" => Self::P3,
            "rec2020" => Self::REC2020,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ColorGamut> for emlite::Val {
    fn from(s: ColorGamut) -> emlite::Val {
        match s {
            ColorGamut::SRGB => emlite::Val::from("srgb"),
            ColorGamut::P3 => emlite::Val::from("p3"),
            ColorGamut::REC2020 => emlite::Val::from("rec2020"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TransferFunction {
    SRGB,
    PQ,
    HLG,
}
impl FromVal for TransferFunction {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "srgb" => Self::SRGB,
            "pq" => Self::PQ,
            "hlg" => Self::HLG,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<TransferFunction> for emlite::Val {
    fn from(s: TransferFunction) -> emlite::Val {
        match s {
            TransferFunction::SRGB => emlite::Val::from("srgb"),
            TransferFunction::PQ => emlite::Val::from("pq"),
            TransferFunction::HLG => emlite::Val::from("hlg"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ReadyState {
    CLOSED,
    OPEN,
    ENDED,
}
impl FromVal for ReadyState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "closed" => Self::CLOSED,
            "open" => Self::OPEN,
            "ended" => Self::ENDED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ReadyState> for emlite::Val {
    fn from(s: ReadyState) -> emlite::Val {
        match s {
            ReadyState::CLOSED => emlite::Val::from("closed"),
            ReadyState::OPEN => emlite::Val::from("open"),
            ReadyState::ENDED => emlite::Val::from("ended"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EndOfStreamError {
    NETWORK,
    DECODE,
}
impl FromVal for EndOfStreamError {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "network" => Self::NETWORK,
            "decode" => Self::DECODE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<EndOfStreamError> for emlite::Val {
    fn from(s: EndOfStreamError) -> emlite::Val {
        match s {
            EndOfStreamError::NETWORK => emlite::Val::from("network"),
            EndOfStreamError::DECODE => emlite::Val::from("decode"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AppendMode {
    SEGMENTS,
    SEQUENCE,
}
impl FromVal for AppendMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "segments" => Self::SEGMENTS,
            "sequence" => Self::SEQUENCE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AppendMode> for emlite::Val {
    fn from(s: AppendMode) -> emlite::Val {
        match s {
            AppendMode::SEGMENTS => emlite::Val::from("segments"),
            AppendMode::SEQUENCE => emlite::Val::from("sequence"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MockCapturePromptResult {
    GRANTED,
    DENIED,
}
impl FromVal for MockCapturePromptResult {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "granted" => Self::GRANTED,
            "denied" => Self::DENIED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MockCapturePromptResult> for emlite::Val {
    fn from(s: MockCapturePromptResult) -> emlite::Val {
        match s {
            MockCapturePromptResult::GRANTED => emlite::Val::from("granted"),
            MockCapturePromptResult::DENIED => emlite::Val::from("denied"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CaptureAction {
    NEXT,
    PREVIOUS,
    FIRST,
    LAST,
}
impl FromVal for CaptureAction {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "next" => Self::NEXT,
            "previous" => Self::PREVIOUS,
            "first" => Self::FIRST,
            "last" => Self::LAST,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CaptureAction> for emlite::Val {
    fn from(s: CaptureAction) -> emlite::Val {
        match s {
            CaptureAction::NEXT => emlite::Val::from("next"),
            CaptureAction::PREVIOUS => emlite::Val::from("previous"),
            CaptureAction::FIRST => emlite::Val::from("first"),
            CaptureAction::LAST => emlite::Val::from("last"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaStreamTrackState {
    LIVE,
    ENDED,
}
impl FromVal for MediaStreamTrackState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "live" => Self::LIVE,
            "ended" => Self::ENDED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaStreamTrackState> for emlite::Val {
    fn from(s: MediaStreamTrackState) -> emlite::Val {
        match s {
            MediaStreamTrackState::LIVE => emlite::Val::from("live"),
            MediaStreamTrackState::ENDED => emlite::Val::from("ended"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum VideoFacingModeEnum {
    USER,
    ENVIRONMENT,
    LEFT,
    RIGHT,
}
impl FromVal for VideoFacingModeEnum {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "user" => Self::USER,
            "environment" => Self::ENVIRONMENT,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<VideoFacingModeEnum> for emlite::Val {
    fn from(s: VideoFacingModeEnum) -> emlite::Val {
        match s {
            VideoFacingModeEnum::USER => emlite::Val::from("user"),
            VideoFacingModeEnum::ENVIRONMENT => emlite::Val::from("environment"),
            VideoFacingModeEnum::LEFT => emlite::Val::from("left"),
            VideoFacingModeEnum::RIGHT => emlite::Val::from("right"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum VideoResizeModeEnum {
    NONE,
    CROP_AND_SCALE,
}
impl FromVal for VideoResizeModeEnum {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "crop-and-scale" => Self::CROP_AND_SCALE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<VideoResizeModeEnum> for emlite::Val {
    fn from(s: VideoResizeModeEnum) -> emlite::Val {
        match s {
            VideoResizeModeEnum::NONE => emlite::Val::from("none"),
            VideoResizeModeEnum::CROP_AND_SCALE => emlite::Val::from("crop-and-scale"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EchoCancellationModeEnum {
    ALL,
    REMOTE_ONLY,
}
impl FromVal for EchoCancellationModeEnum {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "all" => Self::ALL,
            "remote-only" => Self::REMOTE_ONLY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<EchoCancellationModeEnum> for emlite::Val {
    fn from(s: EchoCancellationModeEnum) -> emlite::Val {
        match s {
            EchoCancellationModeEnum::ALL => emlite::Val::from("all"),
            EchoCancellationModeEnum::REMOTE_ONLY => emlite::Val::from("remote-only"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaDeviceKind {
    AUDIOINPUT,
    AUDIOOUTPUT,
    VIDEOINPUT,
}
impl FromVal for MediaDeviceKind {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "audioinput" => Self::AUDIOINPUT,
            "audiooutput" => Self::AUDIOOUTPUT,
            "videoinput" => Self::VIDEOINPUT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaDeviceKind> for emlite::Val {
    fn from(s: MediaDeviceKind) -> emlite::Val {
        match s {
            MediaDeviceKind::AUDIOINPUT => emlite::Val::from("audioinput"),
            MediaDeviceKind::AUDIOOUTPUT => emlite::Val::from("audiooutput"),
            MediaDeviceKind::VIDEOINPUT => emlite::Val::from("videoinput"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaSessionPlaybackState {
    NONE,
    PAUSED,
    PLAYING,
}
impl FromVal for MediaSessionPlaybackState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "paused" => Self::PAUSED,
            "playing" => Self::PLAYING,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaSessionPlaybackState> for emlite::Val {
    fn from(s: MediaSessionPlaybackState) -> emlite::Val {
        match s {
            MediaSessionPlaybackState::NONE => emlite::Val::from("none"),
            MediaSessionPlaybackState::PAUSED => emlite::Val::from("paused"),
            MediaSessionPlaybackState::PLAYING => emlite::Val::from("playing"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaSessionAction {
    PLAY,
    PAUSE,
    SEEKBACKWARD,
    SEEKFORWARD,
    PREVIOUSTRACK,
    NEXTTRACK,
    SKIPAD,
    STOP,
    SEEKTO,
    TOGGLEMICROPHONE,
    TOGGLECAMERA,
    TOGGLESCREENSHARE,
    HANGUP,
    PREVIOUSSLIDE,
    NEXTSLIDE,
    ENTERPICTUREINPICTURE,
    VOICEACTIVITY,
}
impl FromVal for MediaSessionAction {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "play" => Self::PLAY,
            "pause" => Self::PAUSE,
            "seekbackward" => Self::SEEKBACKWARD,
            "seekforward" => Self::SEEKFORWARD,
            "previoustrack" => Self::PREVIOUSTRACK,
            "nexttrack" => Self::NEXTTRACK,
            "skipad" => Self::SKIPAD,
            "stop" => Self::STOP,
            "seekto" => Self::SEEKTO,
            "togglemicrophone" => Self::TOGGLEMICROPHONE,
            "togglecamera" => Self::TOGGLECAMERA,
            "togglescreenshare" => Self::TOGGLESCREENSHARE,
            "hangup" => Self::HANGUP,
            "previousslide" => Self::PREVIOUSSLIDE,
            "nextslide" => Self::NEXTSLIDE,
            "enterpictureinpicture" => Self::ENTERPICTUREINPICTURE,
            "voiceactivity" => Self::VOICEACTIVITY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MediaSessionAction> for emlite::Val {
    fn from(s: MediaSessionAction) -> emlite::Val {
        match s {
            MediaSessionAction::PLAY => emlite::Val::from("play"),
            MediaSessionAction::PAUSE => emlite::Val::from("pause"),
            MediaSessionAction::SEEKBACKWARD => emlite::Val::from("seekbackward"),
            MediaSessionAction::SEEKFORWARD => emlite::Val::from("seekforward"),
            MediaSessionAction::PREVIOUSTRACK => emlite::Val::from("previoustrack"),
            MediaSessionAction::NEXTTRACK => emlite::Val::from("nexttrack"),
            MediaSessionAction::SKIPAD => emlite::Val::from("skipad"),
            MediaSessionAction::STOP => emlite::Val::from("stop"),
            MediaSessionAction::SEEKTO => emlite::Val::from("seekto"),
            MediaSessionAction::TOGGLEMICROPHONE => emlite::Val::from("togglemicrophone"),
            MediaSessionAction::TOGGLECAMERA => emlite::Val::from("togglecamera"),
            MediaSessionAction::TOGGLESCREENSHARE => emlite::Val::from("togglescreenshare"),
            MediaSessionAction::HANGUP => emlite::Val::from("hangup"),
            MediaSessionAction::PREVIOUSSLIDE => emlite::Val::from("previousslide"),
            MediaSessionAction::NEXTSLIDE => emlite::Val::from("nextslide"),
            MediaSessionAction::ENTERPICTUREINPICTURE => emlite::Val::from("enterpictureinpicture"),
            MediaSessionAction::VOICEACTIVITY => emlite::Val::from("voiceactivity"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BitrateMode {
    CONSTANT,
    VARIABLE,
}
impl FromVal for BitrateMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "constant" => Self::CONSTANT,
            "variable" => Self::VARIABLE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<BitrateMode> for emlite::Val {
    fn from(s: BitrateMode) -> emlite::Val {
        match s {
            BitrateMode::CONSTANT => emlite::Val::from("constant"),
            BitrateMode::VARIABLE => emlite::Val::from("variable"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RecordingState {
    INACTIVE,
    RECORDING,
    PAUSED,
}
impl FromVal for RecordingState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "inactive" => Self::INACTIVE,
            "recording" => Self::RECORDING,
            "paused" => Self::PAUSED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RecordingState> for emlite::Val {
    fn from(s: RecordingState) -> emlite::Val {
        match s {
            RecordingState::INACTIVE => emlite::Val::from("inactive"),
            RecordingState::RECORDING => emlite::Val::from("recording"),
            RecordingState::PAUSED => emlite::Val::from("paused"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCDegradationPreference {
    MAINTAIN_FRAMERATE,
    MAINTAIN_RESOLUTION,
    BALANCED,
}
impl FromVal for RTCDegradationPreference {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "maintain-framerate" => Self::MAINTAIN_FRAMERATE,
            "maintain-resolution" => Self::MAINTAIN_RESOLUTION,
            "balanced" => Self::BALANCED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCDegradationPreference> for emlite::Val {
    fn from(s: RTCDegradationPreference) -> emlite::Val {
        match s {
            RTCDegradationPreference::MAINTAIN_FRAMERATE => emlite::Val::from("maintain-framerate"),
            RTCDegradationPreference::MAINTAIN_RESOLUTION => {
                emlite::Val::from("maintain-resolution")
            }
            RTCDegradationPreference::BALANCED => emlite::Val::from("balanced"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum NavigationTimingType {
    NAVIGATE,
    RELOAD,
    BACK_FORWARD,
    PRERENDER,
}
impl FromVal for NavigationTimingType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "navigate" => Self::NAVIGATE,
            "reload" => Self::RELOAD,
            "back_forward" => Self::BACK_FORWARD,
            "prerender" => Self::PRERENDER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<NavigationTimingType> for emlite::Val {
    fn from(s: NavigationTimingType) -> emlite::Val {
        match s {
            NavigationTimingType::NAVIGATE => emlite::Val::from("navigate"),
            NavigationTimingType::RELOAD => emlite::Val::from("reload"),
            NavigationTimingType::BACK_FORWARD => emlite::Val::from("back_forward"),
            NavigationTimingType::PRERENDER => emlite::Val::from("prerender"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ConnectionType {
    BLUETOOTH,
    CELLULAR,
    ETHERNET,
    MIXED,
    NONE,
    OTHER,
    UNKNOWN,
    WIFI,
    WIMAX,
}
impl FromVal for ConnectionType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "bluetooth" => Self::BLUETOOTH,
            "cellular" => Self::CELLULAR,
            "ethernet" => Self::ETHERNET,
            "mixed" => Self::MIXED,
            "none" => Self::NONE,
            "other" => Self::OTHER,
            "unknown" => Self::UNKNOWN,
            "wifi" => Self::WIFI,
            "wimax" => Self::WIMAX,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ConnectionType> for emlite::Val {
    fn from(s: ConnectionType) -> emlite::Val {
        match s {
            ConnectionType::BLUETOOTH => emlite::Val::from("bluetooth"),
            ConnectionType::CELLULAR => emlite::Val::from("cellular"),
            ConnectionType::ETHERNET => emlite::Val::from("ethernet"),
            ConnectionType::MIXED => emlite::Val::from("mixed"),
            ConnectionType::NONE => emlite::Val::from("none"),
            ConnectionType::OTHER => emlite::Val::from("other"),
            ConnectionType::UNKNOWN => emlite::Val::from("unknown"),
            ConnectionType::WIFI => emlite::Val::from("wifi"),
            ConnectionType::WIMAX => emlite::Val::from("wimax"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EffectiveConnectionType {
    _2G,
    _3G,
    _4G,
    SLOW_2G,
}
impl FromVal for EffectiveConnectionType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "2g" => Self::_2G,
            "3g" => Self::_3G,
            "4g" => Self::_4G,
            "slow-2g" => Self::SLOW_2G,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<EffectiveConnectionType> for emlite::Val {
    fn from(s: EffectiveConnectionType) -> emlite::Val {
        match s {
            EffectiveConnectionType::_2G => emlite::Val::from("2g"),
            EffectiveConnectionType::_3G => emlite::Val::from("3g"),
            EffectiveConnectionType::_4G => emlite::Val::from("4g"),
            EffectiveConnectionType::SLOW_2G => emlite::Val::from("slow-2g"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum NotificationPermission {
    DEFAULT,
    DENIED,
    GRANTED,
}
impl FromVal for NotificationPermission {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "default" => Self::DEFAULT,
            "denied" => Self::DENIED,
            "granted" => Self::GRANTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<NotificationPermission> for emlite::Val {
    fn from(s: NotificationPermission) -> emlite::Val {
        match s {
            NotificationPermission::DEFAULT => emlite::Val::from("default"),
            NotificationPermission::DENIED => emlite::Val::from("denied"),
            NotificationPermission::GRANTED => emlite::Val::from("granted"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum NotificationDirection {
    AUTO,
    LTR,
    RTL,
}
impl FromVal for NotificationDirection {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            "ltr" => Self::LTR,
            "rtl" => Self::RTL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<NotificationDirection> for emlite::Val {
    fn from(s: NotificationDirection) -> emlite::Val {
        match s {
            NotificationDirection::AUTO => emlite::Val::from("auto"),
            NotificationDirection::LTR => emlite::Val::from("ltr"),
            NotificationDirection::RTL => emlite::Val::from("rtl"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OrientationSensorLocalCoordinateSystem {
    DEVICE,
    SCREEN,
}
impl FromVal for OrientationSensorLocalCoordinateSystem {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "device" => Self::DEVICE,
            "screen" => Self::SCREEN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OrientationSensorLocalCoordinateSystem> for emlite::Val {
    fn from(s: OrientationSensorLocalCoordinateSystem) -> emlite::Val {
        match s {
            OrientationSensorLocalCoordinateSystem::DEVICE => emlite::Val::from("device"),
            OrientationSensorLocalCoordinateSystem::SCREEN => emlite::Val::from("screen"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ClientLifecycleState {
    ACTIVE,
    FROZEN,
}
impl FromVal for ClientLifecycleState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "active" => Self::ACTIVE,
            "frozen" => Self::FROZEN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ClientLifecycleState> for emlite::Val {
    fn from(s: ClientLifecycleState) -> emlite::Val {
        match s {
            ClientLifecycleState::ACTIVE => emlite::Val::from("active"),
            ClientLifecycleState::FROZEN => emlite::Val::from("frozen"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PaymentDelegation {
    SHIPPING_ADDRESS,
    PAYER_NAME,
    PAYER_PHONE,
    PAYER_EMAIL,
}
impl FromVal for PaymentDelegation {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "shippingAddress" => Self::SHIPPING_ADDRESS,
            "payerName" => Self::PAYER_NAME,
            "payerPhone" => Self::PAYER_PHONE,
            "payerEmail" => Self::PAYER_EMAIL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PaymentDelegation> for emlite::Val {
    fn from(s: PaymentDelegation) -> emlite::Val {
        match s {
            PaymentDelegation::SHIPPING_ADDRESS => emlite::Val::from("shippingAddress"),
            PaymentDelegation::PAYER_NAME => emlite::Val::from("payerName"),
            PaymentDelegation::PAYER_PHONE => emlite::Val::from("payerPhone"),
            PaymentDelegation::PAYER_EMAIL => emlite::Val::from("payerEmail"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PaymentShippingType {
    SHIPPING,
    DELIVERY,
    PICKUP,
}
impl FromVal for PaymentShippingType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "shipping" => Self::SHIPPING,
            "delivery" => Self::DELIVERY,
            "pickup" => Self::PICKUP,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PaymentShippingType> for emlite::Val {
    fn from(s: PaymentShippingType) -> emlite::Val {
        match s {
            PaymentShippingType::SHIPPING => emlite::Val::from("shipping"),
            PaymentShippingType::DELIVERY => emlite::Val::from("delivery"),
            PaymentShippingType::PICKUP => emlite::Val::from("pickup"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PaymentComplete {
    FAIL,
    SUCCESS,
    UNKNOWN,
}
impl FromVal for PaymentComplete {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "fail" => Self::FAIL,
            "success" => Self::SUCCESS,
            "unknown" => Self::UNKNOWN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PaymentComplete> for emlite::Val {
    fn from(s: PaymentComplete) -> emlite::Val {
        match s {
            PaymentComplete::FAIL => emlite::Val::from("fail"),
            PaymentComplete::SUCCESS => emlite::Val::from("success"),
            PaymentComplete::UNKNOWN => emlite::Val::from("unknown"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PermissionState {
    GRANTED,
    DENIED,
    PROMPT,
}
impl FromVal for PermissionState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "granted" => Self::GRANTED,
            "denied" => Self::DENIED,
            "prompt" => Self::PROMPT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PermissionState> for emlite::Val {
    fn from(s: PermissionState) -> emlite::Val {
        match s {
            PermissionState::GRANTED => emlite::Val::from("granted"),
            PermissionState::DENIED => emlite::Val::from("denied"),
            PermissionState::PROMPT => emlite::Val::from("prompt"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PointerAxis {
    BLOCK,
    INLINE,
    X,
    Y,
}
impl FromVal for PointerAxis {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "block" => Self::BLOCK,
            "inline" => Self::INLINE,
            "x" => Self::X,
            "y" => Self::Y,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PointerAxis> for emlite::Val {
    fn from(s: PointerAxis) -> emlite::Val {
        match s {
            PointerAxis::BLOCK => emlite::Val::from("block"),
            PointerAxis::INLINE => emlite::Val::from("inline"),
            PointerAxis::X => emlite::Val::from("x"),
            PointerAxis::Y => emlite::Val::from("y"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PresentationConnectionState {
    CONNECTING,
    CONNECTED,
    CLOSED,
    TERMINATED,
}
impl FromVal for PresentationConnectionState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "connecting" => Self::CONNECTING,
            "connected" => Self::CONNECTED,
            "closed" => Self::CLOSED,
            "terminated" => Self::TERMINATED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PresentationConnectionState> for emlite::Val {
    fn from(s: PresentationConnectionState) -> emlite::Val {
        match s {
            PresentationConnectionState::CONNECTING => emlite::Val::from("connecting"),
            PresentationConnectionState::CONNECTED => emlite::Val::from("connected"),
            PresentationConnectionState::CLOSED => emlite::Val::from("closed"),
            PresentationConnectionState::TERMINATED => emlite::Val::from("terminated"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PresentationConnectionCloseReason {
    ERROR,
    CLOSED,
    WENTAWAY,
}
impl FromVal for PresentationConnectionCloseReason {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "error" => Self::ERROR,
            "closed" => Self::CLOSED,
            "wentaway" => Self::WENTAWAY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PresentationConnectionCloseReason> for emlite::Val {
    fn from(s: PresentationConnectionCloseReason) -> emlite::Val {
        match s {
            PresentationConnectionCloseReason::ERROR => emlite::Val::from("error"),
            PresentationConnectionCloseReason::CLOSED => emlite::Val::from("closed"),
            PresentationConnectionCloseReason::WENTAWAY => emlite::Val::from("wentaway"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PrivateAttributionAggregationProtocol {
    DAP_12_HISTOGRAM,
    TEE_00,
}
impl FromVal for PrivateAttributionAggregationProtocol {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "dap-12-histogram" => Self::DAP_12_HISTOGRAM,
            "tee-00" => Self::TEE_00,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PrivateAttributionAggregationProtocol> for emlite::Val {
    fn from(s: PrivateAttributionAggregationProtocol) -> emlite::Val {
        match s {
            PrivateAttributionAggregationProtocol::DAP_12_HISTOGRAM => {
                emlite::Val::from("dap-12-histogram")
            }
            PrivateAttributionAggregationProtocol::TEE_00 => emlite::Val::from("tee-00"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AttributionLogic {
    LAST_TOUCH,
}
impl FromVal for AttributionLogic {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "last-touch" => Self::LAST_TOUCH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AttributionLogic> for emlite::Val {
    fn from(s: AttributionLogic) -> emlite::Val {
        match s {
            AttributionLogic::LAST_TOUCH => emlite::Val::from("last-touch"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IPAddressSpace {
    PUBLIC,
    PRIVATE,
    LOCAL,
}
impl FromVal for IPAddressSpace {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "public" => Self::PUBLIC,
            "private" => Self::PRIVATE,
            "local" => Self::LOCAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<IPAddressSpace> for emlite::Val {
    fn from(s: IPAddressSpace) -> emlite::Val {
        match s {
            IPAddressSpace::PUBLIC => emlite::Val::from("public"),
            IPAddressSpace::PRIVATE => emlite::Val::from("private"),
            IPAddressSpace::LOCAL => emlite::Val::from("local"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PushEncryptionKeyName {
    P256DH,
    AUTH,
}
impl FromVal for PushEncryptionKeyName {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "p256dh" => Self::P256DH,
            "auth" => Self::AUTH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PushEncryptionKeyName> for emlite::Val {
    fn from(s: PushEncryptionKeyName) -> emlite::Val {
        match s {
            PushEncryptionKeyName::P256DH => emlite::Val::from("p256dh"),
            PushEncryptionKeyName::AUTH => emlite::Val::from("auth"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ReferrerPolicy {
    NONE,
    NO_REFERRER,
    NO_REFERRER_WHEN_DOWNGRADE,
    SAME_ORIGIN,
    ORIGIN,
    STRICT_ORIGIN,
    ORIGIN_WHEN_CROSS_ORIGIN,
    STRICT_ORIGIN_WHEN_CROSS_ORIGIN,
    UNSAFE_URL,
}
impl FromVal for ReferrerPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "no-referrer" => Self::NO_REFERRER,
            "no-referrer-when-downgrade" => Self::NO_REFERRER_WHEN_DOWNGRADE,
            "same-origin" => Self::SAME_ORIGIN,
            "origin" => Self::ORIGIN,
            "strict-origin" => Self::STRICT_ORIGIN,
            "origin-when-cross-origin" => Self::ORIGIN_WHEN_CROSS_ORIGIN,
            "strict-origin-when-cross-origin" => Self::STRICT_ORIGIN_WHEN_CROSS_ORIGIN,
            "unsafe-url" => Self::UNSAFE_URL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ReferrerPolicy> for emlite::Val {
    fn from(s: ReferrerPolicy) -> emlite::Val {
        match s {
            ReferrerPolicy::NONE => emlite::Val::from(""),
            ReferrerPolicy::NO_REFERRER => emlite::Val::from("no-referrer"),
            ReferrerPolicy::NO_REFERRER_WHEN_DOWNGRADE => {
                emlite::Val::from("no-referrer-when-downgrade")
            }
            ReferrerPolicy::SAME_ORIGIN => emlite::Val::from("same-origin"),
            ReferrerPolicy::ORIGIN => emlite::Val::from("origin"),
            ReferrerPolicy::STRICT_ORIGIN => emlite::Val::from("strict-origin"),
            ReferrerPolicy::ORIGIN_WHEN_CROSS_ORIGIN => {
                emlite::Val::from("origin-when-cross-origin")
            }
            ReferrerPolicy::STRICT_ORIGIN_WHEN_CROSS_ORIGIN => {
                emlite::Val::from("strict-origin-when-cross-origin")
            }
            ReferrerPolicy::UNSAFE_URL => emlite::Val::from("unsafe-url"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RemotePlaybackState {
    CONNECTING,
    CONNECTED,
    DISCONNECTED,
}
impl FromVal for RemotePlaybackState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "connecting" => Self::CONNECTING,
            "connected" => Self::CONNECTED,
            "disconnected" => Self::DISCONNECTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RemotePlaybackState> for emlite::Val {
    fn from(s: RemotePlaybackState) -> emlite::Val {
        match s {
            RemotePlaybackState::CONNECTING => emlite::Val::from("connecting"),
            RemotePlaybackState::CONNECTED => emlite::Val::from("connected"),
            RemotePlaybackState::DISCONNECTED => emlite::Val::from("disconnected"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ResizeObserverBoxOptions {
    BORDER_BOX,
    CONTENT_BOX,
    DEVICE_PIXEL_CONTENT_BOX,
}
impl FromVal for ResizeObserverBoxOptions {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "border-box" => Self::BORDER_BOX,
            "content-box" => Self::CONTENT_BOX,
            "device-pixel-content-box" => Self::DEVICE_PIXEL_CONTENT_BOX,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ResizeObserverBoxOptions> for emlite::Val {
    fn from(s: ResizeObserverBoxOptions) -> emlite::Val {
        match s {
            ResizeObserverBoxOptions::BORDER_BOX => emlite::Val::from("border-box"),
            ResizeObserverBoxOptions::CONTENT_BOX => emlite::Val::from("content-box"),
            ResizeObserverBoxOptions::DEVICE_PIXEL_CONTENT_BOX => {
                emlite::Val::from("device-pixel-content-box")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RenderBlockingStatusType {
    BLOCKING,
    NON_BLOCKING,
}
impl FromVal for RenderBlockingStatusType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "blocking" => Self::BLOCKING,
            "non-blocking" => Self::NON_BLOCKING,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RenderBlockingStatusType> for emlite::Val {
    fn from(s: RenderBlockingStatusType) -> emlite::Val {
        match s {
            RenderBlockingStatusType::BLOCKING => emlite::Val::from("blocking"),
            RenderBlockingStatusType::NON_BLOCKING => emlite::Val::from("non-blocking"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SameSiteCookiesType {
    ALL,
    NONE,
}
impl FromVal for SameSiteCookiesType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "all" => Self::ALL,
            "none" => Self::NONE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SameSiteCookiesType> for emlite::Val {
    fn from(s: SameSiteCookiesType) -> emlite::Val {
        match s {
            SameSiteCookiesType::ALL => emlite::Val::from("all"),
            SameSiteCookiesType::NONE => emlite::Val::from("none"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SanitizerPresets {
    DEFAULT,
}
impl FromVal for SanitizerPresets {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "default" => Self::DEFAULT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SanitizerPresets> for emlite::Val {
    fn from(s: SanitizerPresets) -> emlite::Val {
        match s {
            SanitizerPresets::DEFAULT => emlite::Val::from("default"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TaskPriority {
    USER_BLOCKING,
    USER_VISIBLE,
    BACKGROUND,
}
impl FromVal for TaskPriority {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "user-blocking" => Self::USER_BLOCKING,
            "user-visible" => Self::USER_VISIBLE,
            "background" => Self::BACKGROUND,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<TaskPriority> for emlite::Val {
    fn from(s: TaskPriority) -> emlite::Val {
        match s {
            TaskPriority::USER_BLOCKING => emlite::Val::from("user-blocking"),
            TaskPriority::USER_VISIBLE => emlite::Val::from("user-visible"),
            TaskPriority::BACKGROUND => emlite::Val::from("background"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CaptureStartFocusBehavior {
    FOCUS_CAPTURING_APPLICATION,
    FOCUS_CAPTURED_SURFACE,
    NO_FOCUS_CHANGE,
}
impl FromVal for CaptureStartFocusBehavior {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "focus-capturing-application" => Self::FOCUS_CAPTURING_APPLICATION,
            "focus-captured-surface" => Self::FOCUS_CAPTURED_SURFACE,
            "no-focus-change" => Self::NO_FOCUS_CHANGE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CaptureStartFocusBehavior> for emlite::Val {
    fn from(s: CaptureStartFocusBehavior) -> emlite::Val {
        match s {
            CaptureStartFocusBehavior::FOCUS_CAPTURING_APPLICATION => {
                emlite::Val::from("focus-capturing-application")
            }
            CaptureStartFocusBehavior::FOCUS_CAPTURED_SURFACE => {
                emlite::Val::from("focus-captured-surface")
            }
            CaptureStartFocusBehavior::NO_FOCUS_CHANGE => emlite::Val::from("no-focus-change"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SelfCapturePreferenceEnum {
    INCLUDE,
    EXCLUDE,
}
impl FromVal for SelfCapturePreferenceEnum {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "include" => Self::INCLUDE,
            "exclude" => Self::EXCLUDE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SelfCapturePreferenceEnum> for emlite::Val {
    fn from(s: SelfCapturePreferenceEnum) -> emlite::Val {
        match s {
            SelfCapturePreferenceEnum::INCLUDE => emlite::Val::from("include"),
            SelfCapturePreferenceEnum::EXCLUDE => emlite::Val::from("exclude"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SystemAudioPreferenceEnum {
    INCLUDE,
    EXCLUDE,
}
impl FromVal for SystemAudioPreferenceEnum {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "include" => Self::INCLUDE,
            "exclude" => Self::EXCLUDE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SystemAudioPreferenceEnum> for emlite::Val {
    fn from(s: SystemAudioPreferenceEnum) -> emlite::Val {
        match s {
            SystemAudioPreferenceEnum::INCLUDE => emlite::Val::from("include"),
            SystemAudioPreferenceEnum::EXCLUDE => emlite::Val::from("exclude"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WindowAudioPreferenceEnum {
    SYSTEM,
    WINDOW,
    EXCLUDE,
}
impl FromVal for WindowAudioPreferenceEnum {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "system" => Self::SYSTEM,
            "window" => Self::WINDOW,
            "exclude" => Self::EXCLUDE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WindowAudioPreferenceEnum> for emlite::Val {
    fn from(s: WindowAudioPreferenceEnum) -> emlite::Val {
        match s {
            WindowAudioPreferenceEnum::SYSTEM => emlite::Val::from("system"),
            WindowAudioPreferenceEnum::WINDOW => emlite::Val::from("window"),
            WindowAudioPreferenceEnum::EXCLUDE => emlite::Val::from("exclude"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SurfaceSwitchingPreferenceEnum {
    INCLUDE,
    EXCLUDE,
}
impl FromVal for SurfaceSwitchingPreferenceEnum {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "include" => Self::INCLUDE,
            "exclude" => Self::EXCLUDE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SurfaceSwitchingPreferenceEnum> for emlite::Val {
    fn from(s: SurfaceSwitchingPreferenceEnum) -> emlite::Val {
        match s {
            SurfaceSwitchingPreferenceEnum::INCLUDE => emlite::Val::from("include"),
            SurfaceSwitchingPreferenceEnum::EXCLUDE => emlite::Val::from("exclude"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MonitorTypeSurfacesEnum {
    INCLUDE,
    EXCLUDE,
}
impl FromVal for MonitorTypeSurfacesEnum {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "include" => Self::INCLUDE,
            "exclude" => Self::EXCLUDE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MonitorTypeSurfacesEnum> for emlite::Val {
    fn from(s: MonitorTypeSurfacesEnum) -> emlite::Val {
        match s {
            MonitorTypeSurfacesEnum::INCLUDE => emlite::Val::from("include"),
            MonitorTypeSurfacesEnum::EXCLUDE => emlite::Val::from("exclude"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DisplayCaptureSurfaceType {
    MONITOR,
    WINDOW,
    BROWSER,
}
impl FromVal for DisplayCaptureSurfaceType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "monitor" => Self::MONITOR,
            "window" => Self::WINDOW,
            "browser" => Self::BROWSER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<DisplayCaptureSurfaceType> for emlite::Val {
    fn from(s: DisplayCaptureSurfaceType) -> emlite::Val {
        match s {
            DisplayCaptureSurfaceType::MONITOR => emlite::Val::from("monitor"),
            DisplayCaptureSurfaceType::WINDOW => emlite::Val::from("window"),
            DisplayCaptureSurfaceType::BROWSER => emlite::Val::from("browser"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CursorCaptureConstraint {
    NEVER,
    ALWAYS,
    MOTION,
}
impl FromVal for CursorCaptureConstraint {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "never" => Self::NEVER,
            "always" => Self::ALWAYS,
            "motion" => Self::MOTION,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CursorCaptureConstraint> for emlite::Val {
    fn from(s: CursorCaptureConstraint) -> emlite::Val {
        match s {
            CursorCaptureConstraint::NEVER => emlite::Val::from("never"),
            CursorCaptureConstraint::ALWAYS => emlite::Val::from("always"),
            CursorCaptureConstraint::MOTION => emlite::Val::from("motion"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OrientationLockType {
    ANY,
    NATURAL,
    LANDSCAPE,
    PORTRAIT,
    PORTRAIT_PRIMARY,
    PORTRAIT_SECONDARY,
    LANDSCAPE_PRIMARY,
    LANDSCAPE_SECONDARY,
}
impl FromVal for OrientationLockType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "any" => Self::ANY,
            "natural" => Self::NATURAL,
            "landscape" => Self::LANDSCAPE,
            "portrait" => Self::PORTRAIT,
            "portrait-primary" => Self::PORTRAIT_PRIMARY,
            "portrait-secondary" => Self::PORTRAIT_SECONDARY,
            "landscape-primary" => Self::LANDSCAPE_PRIMARY,
            "landscape-secondary" => Self::LANDSCAPE_SECONDARY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OrientationLockType> for emlite::Val {
    fn from(s: OrientationLockType) -> emlite::Val {
        match s {
            OrientationLockType::ANY => emlite::Val::from("any"),
            OrientationLockType::NATURAL => emlite::Val::from("natural"),
            OrientationLockType::LANDSCAPE => emlite::Val::from("landscape"),
            OrientationLockType::PORTRAIT => emlite::Val::from("portrait"),
            OrientationLockType::PORTRAIT_PRIMARY => emlite::Val::from("portrait-primary"),
            OrientationLockType::PORTRAIT_SECONDARY => emlite::Val::from("portrait-secondary"),
            OrientationLockType::LANDSCAPE_PRIMARY => emlite::Val::from("landscape-primary"),
            OrientationLockType::LANDSCAPE_SECONDARY => emlite::Val::from("landscape-secondary"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OrientationType {
    PORTRAIT_PRIMARY,
    PORTRAIT_SECONDARY,
    LANDSCAPE_PRIMARY,
    LANDSCAPE_SECONDARY,
}
impl FromVal for OrientationType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "portrait-primary" => Self::PORTRAIT_PRIMARY,
            "portrait-secondary" => Self::PORTRAIT_SECONDARY,
            "landscape-primary" => Self::LANDSCAPE_PRIMARY,
            "landscape-secondary" => Self::LANDSCAPE_SECONDARY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OrientationType> for emlite::Val {
    fn from(s: OrientationType) -> emlite::Val {
        match s {
            OrientationType::PORTRAIT_PRIMARY => emlite::Val::from("portrait-primary"),
            OrientationType::PORTRAIT_SECONDARY => emlite::Val::from("portrait-secondary"),
            OrientationType::LANDSCAPE_PRIMARY => emlite::Val::from("landscape-primary"),
            OrientationType::LANDSCAPE_SECONDARY => emlite::Val::from("landscape-secondary"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WakeLockType {
    SCREEN,
}
impl FromVal for WakeLockType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "screen" => Self::SCREEN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WakeLockType> for emlite::Val {
    fn from(s: WakeLockType) -> emlite::Val {
        match s {
            WakeLockType::SCREEN => emlite::Val::from("screen"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScrollAxis {
    BLOCK,
    INLINE,
    X,
    Y,
}
impl FromVal for ScrollAxis {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "block" => Self::BLOCK,
            "inline" => Self::INLINE,
            "x" => Self::X,
            "y" => Self::Y,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ScrollAxis> for emlite::Val {
    fn from(s: ScrollAxis) -> emlite::Val {
        match s {
            ScrollAxis::BLOCK => emlite::Val::from("block"),
            ScrollAxis::INLINE => emlite::Val::from("inline"),
            ScrollAxis::X => emlite::Val::from("x"),
            ScrollAxis::Y => emlite::Val::from("y"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SecurePaymentConfirmationAvailability {
    AVAILABLE,
    UNAVAILABLE_UNKNOWN_REASON,
    UNAVAILABLE_FEATURE_NOT_ENABLED,
    UNAVAILABLE_NO_PERMISSION_POLICY,
    UNAVAILABLE_NO_USER_VERIFYING_PLATFORM_AUTHENTICATOR,
}
impl FromVal for SecurePaymentConfirmationAvailability {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "available" => Self::AVAILABLE,
            "unavailable-unknown-reason" => Self::UNAVAILABLE_UNKNOWN_REASON,
            "unavailable-feature-not-enabled" => Self::UNAVAILABLE_FEATURE_NOT_ENABLED,
            "unavailable-no-permission-policy" => Self::UNAVAILABLE_NO_PERMISSION_POLICY,
            "unavailable-no-user-verifying-platform-authenticator" => {
                Self::UNAVAILABLE_NO_USER_VERIFYING_PLATFORM_AUTHENTICATOR
            }
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SecurePaymentConfirmationAvailability> for emlite::Val {
    fn from(s: SecurePaymentConfirmationAvailability) -> emlite::Val {
        match s {
            SecurePaymentConfirmationAvailability::AVAILABLE => emlite::Val::from("available"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_UNKNOWN_REASON => emlite::Val::from("unavailable-unknown-reason"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_FEATURE_NOT_ENABLED => emlite::Val::from("unavailable-feature-not-enabled"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_NO_PERMISSION_POLICY => emlite::Val::from("unavailable-no-permission-policy"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_NO_USER_VERIFYING_PLATFORM_AUTHENTICATOR => emlite::Val::from("unavailable-no-user-verifying-platform-authenticator"),
         }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ParityType {
    NONE,
    EVEN,
    ODD,
}
impl FromVal for ParityType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "even" => Self::EVEN,
            "odd" => Self::ODD,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ParityType> for emlite::Val {
    fn from(s: ParityType) -> emlite::Val {
        match s {
            ParityType::NONE => emlite::Val::from("none"),
            ParityType::EVEN => emlite::Val::from("even"),
            ParityType::ODD => emlite::Val::from("odd"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FlowControlType {
    NONE,
    HARDWARE,
}
impl FromVal for FlowControlType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "hardware" => Self::HARDWARE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FlowControlType> for emlite::Val {
    fn from(s: FlowControlType) -> emlite::Val {
        match s {
            FlowControlType::NONE => emlite::Val::from("none"),
            FlowControlType::HARDWARE => emlite::Val::from("hardware"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ServiceWorkerState {
    PARSED,
    INSTALLING,
    INSTALLED,
    ACTIVATING,
    ACTIVATED,
    REDUNDANT,
}
impl FromVal for ServiceWorkerState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "parsed" => Self::PARSED,
            "installing" => Self::INSTALLING,
            "installed" => Self::INSTALLED,
            "activating" => Self::ACTIVATING,
            "activated" => Self::ACTIVATED,
            "redundant" => Self::REDUNDANT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ServiceWorkerState> for emlite::Val {
    fn from(s: ServiceWorkerState) -> emlite::Val {
        match s {
            ServiceWorkerState::PARSED => emlite::Val::from("parsed"),
            ServiceWorkerState::INSTALLING => emlite::Val::from("installing"),
            ServiceWorkerState::INSTALLED => emlite::Val::from("installed"),
            ServiceWorkerState::ACTIVATING => emlite::Val::from("activating"),
            ServiceWorkerState::ACTIVATED => emlite::Val::from("activated"),
            ServiceWorkerState::REDUNDANT => emlite::Val::from("redundant"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ServiceWorkerUpdateViaCache {
    IMPORTS,
    ALL,
    NONE,
}
impl FromVal for ServiceWorkerUpdateViaCache {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "imports" => Self::IMPORTS,
            "all" => Self::ALL,
            "none" => Self::NONE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ServiceWorkerUpdateViaCache> for emlite::Val {
    fn from(s: ServiceWorkerUpdateViaCache) -> emlite::Val {
        match s {
            ServiceWorkerUpdateViaCache::IMPORTS => emlite::Val::from("imports"),
            ServiceWorkerUpdateViaCache::ALL => emlite::Val::from("all"),
            ServiceWorkerUpdateViaCache::NONE => emlite::Val::from("none"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FrameType {
    AUXILIARY,
    TOP_LEVEL,
    NESTED,
    NONE,
}
impl FromVal for FrameType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auxiliary" => Self::AUXILIARY,
            "top-level" => Self::TOP_LEVEL,
            "nested" => Self::NESTED,
            "none" => Self::NONE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FrameType> for emlite::Val {
    fn from(s: FrameType) -> emlite::Val {
        match s {
            FrameType::AUXILIARY => emlite::Val::from("auxiliary"),
            FrameType::TOP_LEVEL => emlite::Val::from("top-level"),
            FrameType::NESTED => emlite::Val::from("nested"),
            FrameType::NONE => emlite::Val::from("none"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ClientType {
    WINDOW,
    WORKER,
    SHAREDWORKER,
    ALL,
}
impl FromVal for ClientType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "window" => Self::WINDOW,
            "worker" => Self::WORKER,
            "sharedworker" => Self::SHAREDWORKER,
            "all" => Self::ALL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ClientType> for emlite::Val {
    fn from(s: ClientType) -> emlite::Val {
        match s {
            ClientType::WINDOW => emlite::Val::from("window"),
            ClientType::WORKER => emlite::Val::from("worker"),
            ClientType::SHAREDWORKER => emlite::Val::from("sharedworker"),
            ClientType::ALL => emlite::Val::from("all"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RunningStatus {
    RUNNING,
    NOT_RUNNING,
}
impl FromVal for RunningStatus {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "running" => Self::RUNNING,
            "not-running" => Self::NOT_RUNNING,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RunningStatus> for emlite::Val {
    fn from(s: RunningStatus) -> emlite::Val {
        match s {
            RunningStatus::RUNNING => emlite::Val::from("running"),
            RunningStatus::NOT_RUNNING => emlite::Val::from("not-running"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RouterSourceEnum {
    CACHE,
    FETCH_EVENT,
    NETWORK,
    RACE_NETWORK_AND_FETCH_HANDLER,
}
impl FromVal for RouterSourceEnum {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "cache" => Self::CACHE,
            "fetch-event" => Self::FETCH_EVENT,
            "network" => Self::NETWORK,
            "race-network-and-fetch-handler" => Self::RACE_NETWORK_AND_FETCH_HANDLER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RouterSourceEnum> for emlite::Val {
    fn from(s: RouterSourceEnum) -> emlite::Val {
        match s {
            RouterSourceEnum::CACHE => emlite::Val::from("cache"),
            RouterSourceEnum::FETCH_EVENT => emlite::Val::from("fetch-event"),
            RouterSourceEnum::NETWORK => emlite::Val::from("network"),
            RouterSourceEnum::RACE_NETWORK_AND_FETCH_HANDLER => {
                emlite::Val::from("race-network-and-fetch-handler")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LandmarkType {
    MOUTH,
    EYE,
    NOSE,
}
impl FromVal for LandmarkType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "mouth" => Self::MOUTH,
            "eye" => Self::EYE,
            "nose" => Self::NOSE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<LandmarkType> for emlite::Val {
    fn from(s: LandmarkType) -> emlite::Val {
        match s {
            LandmarkType::MOUTH => emlite::Val::from("mouth"),
            LandmarkType::EYE => emlite::Val::from("eye"),
            LandmarkType::NOSE => emlite::Val::from("nose"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BarcodeFormat {
    AZTEC,
    CODE_128,
    CODE_39,
    CODE_93,
    CODABAR,
    DATA_MATRIX,
    EAN_13,
    EAN_8,
    ITF,
    PDF417,
    QR_CODE,
    UNKNOWN,
    UPC_A,
    UPC_E,
}
impl FromVal for BarcodeFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "aztec" => Self::AZTEC,
            "code_128" => Self::CODE_128,
            "code_39" => Self::CODE_39,
            "code_93" => Self::CODE_93,
            "codabar" => Self::CODABAR,
            "data_matrix" => Self::DATA_MATRIX,
            "ean_13" => Self::EAN_13,
            "ean_8" => Self::EAN_8,
            "itf" => Self::ITF,
            "pdf417" => Self::PDF417,
            "qr_code" => Self::QR_CODE,
            "unknown" => Self::UNKNOWN,
            "upc_a" => Self::UPC_A,
            "upc_e" => Self::UPC_E,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<BarcodeFormat> for emlite::Val {
    fn from(s: BarcodeFormat) -> emlite::Val {
        match s {
            BarcodeFormat::AZTEC => emlite::Val::from("aztec"),
            BarcodeFormat::CODE_128 => emlite::Val::from("code_128"),
            BarcodeFormat::CODE_39 => emlite::Val::from("code_39"),
            BarcodeFormat::CODE_93 => emlite::Val::from("code_93"),
            BarcodeFormat::CODABAR => emlite::Val::from("codabar"),
            BarcodeFormat::DATA_MATRIX => emlite::Val::from("data_matrix"),
            BarcodeFormat::EAN_13 => emlite::Val::from("ean_13"),
            BarcodeFormat::EAN_8 => emlite::Val::from("ean_8"),
            BarcodeFormat::ITF => emlite::Val::from("itf"),
            BarcodeFormat::PDF417 => emlite::Val::from("pdf417"),
            BarcodeFormat::QR_CODE => emlite::Val::from("qr_code"),
            BarcodeFormat::UNKNOWN => emlite::Val::from("unknown"),
            BarcodeFormat::UPC_A => emlite::Val::from("upc_a"),
            BarcodeFormat::UPC_E => emlite::Val::from("upc_e"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SpeechRecognitionErrorCode {
    NO_SPEECH,
    ABORTED,
    AUDIO_CAPTURE,
    NETWORK,
    NOT_ALLOWED,
    SERVICE_NOT_ALLOWED,
    LANGUAGE_NOT_SUPPORTED,
    PHRASES_NOT_SUPPORTED,
}
impl FromVal for SpeechRecognitionErrorCode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "no-speech" => Self::NO_SPEECH,
            "aborted" => Self::ABORTED,
            "audio-capture" => Self::AUDIO_CAPTURE,
            "network" => Self::NETWORK,
            "not-allowed" => Self::NOT_ALLOWED,
            "service-not-allowed" => Self::SERVICE_NOT_ALLOWED,
            "language-not-supported" => Self::LANGUAGE_NOT_SUPPORTED,
            "phrases-not-supported" => Self::PHRASES_NOT_SUPPORTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SpeechRecognitionErrorCode> for emlite::Val {
    fn from(s: SpeechRecognitionErrorCode) -> emlite::Val {
        match s {
            SpeechRecognitionErrorCode::NO_SPEECH => emlite::Val::from("no-speech"),
            SpeechRecognitionErrorCode::ABORTED => emlite::Val::from("aborted"),
            SpeechRecognitionErrorCode::AUDIO_CAPTURE => emlite::Val::from("audio-capture"),
            SpeechRecognitionErrorCode::NETWORK => emlite::Val::from("network"),
            SpeechRecognitionErrorCode::NOT_ALLOWED => emlite::Val::from("not-allowed"),
            SpeechRecognitionErrorCode::SERVICE_NOT_ALLOWED => {
                emlite::Val::from("service-not-allowed")
            }
            SpeechRecognitionErrorCode::LANGUAGE_NOT_SUPPORTED => {
                emlite::Val::from("language-not-supported")
            }
            SpeechRecognitionErrorCode::PHRASES_NOT_SUPPORTED => {
                emlite::Val::from("phrases-not-supported")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AvailabilityStatus {
    UNAVAILABLE,
    DOWNLOADABLE,
    DOWNLOADING,
    AVAILABLE,
}
impl FromVal for AvailabilityStatus {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "unavailable" => Self::UNAVAILABLE,
            "downloadable" => Self::DOWNLOADABLE,
            "downloading" => Self::DOWNLOADING,
            "available" => Self::AVAILABLE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AvailabilityStatus> for emlite::Val {
    fn from(s: AvailabilityStatus) -> emlite::Val {
        match s {
            AvailabilityStatus::UNAVAILABLE => emlite::Val::from("unavailable"),
            AvailabilityStatus::DOWNLOADABLE => emlite::Val::from("downloadable"),
            AvailabilityStatus::DOWNLOADING => emlite::Val::from("downloading"),
            AvailabilityStatus::AVAILABLE => emlite::Val::from("available"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SpeechSynthesisErrorCode {
    CANCELED,
    INTERRUPTED,
    AUDIO_BUSY,
    AUDIO_HARDWARE,
    NETWORK,
    SYNTHESIS_UNAVAILABLE,
    SYNTHESIS_FAILED,
    LANGUAGE_UNAVAILABLE,
    VOICE_UNAVAILABLE,
    TEXT_TOO_LONG,
    INVALID_ARGUMENT,
    NOT_ALLOWED,
}
impl FromVal for SpeechSynthesisErrorCode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "canceled" => Self::CANCELED,
            "interrupted" => Self::INTERRUPTED,
            "audio-busy" => Self::AUDIO_BUSY,
            "audio-hardware" => Self::AUDIO_HARDWARE,
            "network" => Self::NETWORK,
            "synthesis-unavailable" => Self::SYNTHESIS_UNAVAILABLE,
            "synthesis-failed" => Self::SYNTHESIS_FAILED,
            "language-unavailable" => Self::LANGUAGE_UNAVAILABLE,
            "voice-unavailable" => Self::VOICE_UNAVAILABLE,
            "text-too-long" => Self::TEXT_TOO_LONG,
            "invalid-argument" => Self::INVALID_ARGUMENT,
            "not-allowed" => Self::NOT_ALLOWED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SpeechSynthesisErrorCode> for emlite::Val {
    fn from(s: SpeechSynthesisErrorCode) -> emlite::Val {
        match s {
            SpeechSynthesisErrorCode::CANCELED => emlite::Val::from("canceled"),
            SpeechSynthesisErrorCode::INTERRUPTED => emlite::Val::from("interrupted"),
            SpeechSynthesisErrorCode::AUDIO_BUSY => emlite::Val::from("audio-busy"),
            SpeechSynthesisErrorCode::AUDIO_HARDWARE => emlite::Val::from("audio-hardware"),
            SpeechSynthesisErrorCode::NETWORK => emlite::Val::from("network"),
            SpeechSynthesisErrorCode::SYNTHESIS_UNAVAILABLE => {
                emlite::Val::from("synthesis-unavailable")
            }
            SpeechSynthesisErrorCode::SYNTHESIS_FAILED => emlite::Val::from("synthesis-failed"),
            SpeechSynthesisErrorCode::LANGUAGE_UNAVAILABLE => {
                emlite::Val::from("language-unavailable")
            }
            SpeechSynthesisErrorCode::VOICE_UNAVAILABLE => emlite::Val::from("voice-unavailable"),
            SpeechSynthesisErrorCode::TEXT_TOO_LONG => emlite::Val::from("text-too-long"),
            SpeechSynthesisErrorCode::INVALID_ARGUMENT => emlite::Val::from("invalid-argument"),
            SpeechSynthesisErrorCode::NOT_ALLOWED => emlite::Val::from("not-allowed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ReadableStreamReaderMode {
    BYOB,
}
impl FromVal for ReadableStreamReaderMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "byob" => Self::BYOB,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ReadableStreamReaderMode> for emlite::Val {
    fn from(s: ReadableStreamReaderMode) -> emlite::Val {
        match s {
            ReadableStreamReaderMode::BYOB => emlite::Val::from("byob"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ReadableStreamType {
    BYTES,
}
impl FromVal for ReadableStreamType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "bytes" => Self::BYTES,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ReadableStreamType> for emlite::Val {
    fn from(s: ReadableStreamType) -> emlite::Val {
        match s {
            ReadableStreamType::BYTES => emlite::Val::from("bytes"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TouchType {
    DIRECT,
    STYLUS,
}
impl FromVal for TouchType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "direct" => Self::DIRECT,
            "stylus" => Self::STYLUS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<TouchType> for emlite::Val {
    fn from(s: TouchType) -> emlite::Val {
        match s {
            TouchType::DIRECT => emlite::Val::from("direct"),
            TouchType::STYLUS => emlite::Val::from("stylus"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RefreshPolicy {
    NONE,
    REFRESH,
}
impl FromVal for RefreshPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "refresh" => Self::REFRESH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RefreshPolicy> for emlite::Val {
    fn from(s: RefreshPolicy) -> emlite::Val {
        match s {
            RefreshPolicy::NONE => emlite::Val::from("none"),
            RefreshPolicy::REFRESH => emlite::Val::from("refresh"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TokenVersion {
    _1,
}
impl FromVal for TokenVersion {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "1" => Self::_1,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<TokenVersion> for emlite::Val {
    fn from(s: TokenVersion) -> emlite::Val {
        match s {
            TokenVersion::_1 => emlite::Val::from("1"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OperationType {
    TOKEN_REQUEST,
    SEND_REDEMPTION_RECORD,
    TOKEN_REDEMPTION,
}
impl FromVal for OperationType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "token-request" => Self::TOKEN_REQUEST,
            "send-redemption-record" => Self::SEND_REDEMPTION_RECORD,
            "token-redemption" => Self::TOKEN_REDEMPTION,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OperationType> for emlite::Val {
    fn from(s: OperationType) -> emlite::Val {
        match s {
            OperationType::TOKEN_REQUEST => emlite::Val::from("token-request"),
            OperationType::SEND_REDEMPTION_RECORD => emlite::Val::from("send-redemption-record"),
            OperationType::TOKEN_REDEMPTION => emlite::Val::from("token-redemption"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum KAnonStatus {
    PASSED_AND_ENFORCED,
    PASSED_NOT_ENFORCED,
    BELOW_THRESHOLD,
    NOT_CALCULATED,
}
impl FromVal for KAnonStatus {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "passedAndEnforced" => Self::PASSED_AND_ENFORCED,
            "passedNotEnforced" => Self::PASSED_NOT_ENFORCED,
            "belowThreshold" => Self::BELOW_THRESHOLD,
            "notCalculated" => Self::NOT_CALCULATED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<KAnonStatus> for emlite::Val {
    fn from(s: KAnonStatus) -> emlite::Val {
        match s {
            KAnonStatus::PASSED_AND_ENFORCED => emlite::Val::from("passedAndEnforced"),
            KAnonStatus::PASSED_NOT_ENFORCED => emlite::Val::from("passedNotEnforced"),
            KAnonStatus::BELOW_THRESHOLD => emlite::Val::from("belowThreshold"),
            KAnonStatus::NOT_CALCULATED => emlite::Val::from("notCalculated"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ImportExportKind {
    FUNCTION,
    TABLE,
    MEMORY,
    GLOBAL,
}
impl FromVal for ImportExportKind {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "function" => Self::FUNCTION,
            "table" => Self::TABLE,
            "memory" => Self::MEMORY,
            "global" => Self::GLOBAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ImportExportKind> for emlite::Val {
    fn from(s: ImportExportKind) -> emlite::Val {
        match s {
            ImportExportKind::FUNCTION => emlite::Val::from("function"),
            ImportExportKind::TABLE => emlite::Val::from("table"),
            ImportExportKind::MEMORY => emlite::Val::from("memory"),
            ImportExportKind::GLOBAL => emlite::Val::from("global"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TableKind {
    EXTERNREF,
    ANYFUNC,
}
impl FromVal for TableKind {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "externref" => Self::EXTERNREF,
            "anyfunc" => Self::ANYFUNC,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<TableKind> for emlite::Val {
    fn from(s: TableKind) -> emlite::Val {
        match s {
            TableKind::EXTERNREF => emlite::Val::from("externref"),
            TableKind::ANYFUNC => emlite::Val::from("anyfunc"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ValueType {
    I32_,
    I64_,
    F32_,
    F64_,
    V128,
    EXTERNREF,
    ANYFUNC,
}
impl FromVal for ValueType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "i32" => Self::I32_,
            "i64" => Self::I64_,
            "f32" => Self::F32_,
            "f64" => Self::F64_,
            "v128" => Self::V128,
            "externref" => Self::EXTERNREF,
            "anyfunc" => Self::ANYFUNC,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ValueType> for emlite::Val {
    fn from(s: ValueType) -> emlite::Val {
        match s {
            ValueType::I32_ => emlite::Val::from("i32"),
            ValueType::I64_ => emlite::Val::from("i64"),
            ValueType::F32_ => emlite::Val::from("f32"),
            ValueType::F64_ => emlite::Val::from("f64"),
            ValueType::V128 => emlite::Val::from("v128"),
            ValueType::EXTERNREF => emlite::Val::from("externref"),
            ValueType::ANYFUNC => emlite::Val::from("anyfunc"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IterationCompositeOperation {
    REPLACE,
    ACCUMULATE,
}
impl FromVal for IterationCompositeOperation {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "replace" => Self::REPLACE,
            "accumulate" => Self::ACCUMULATE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<IterationCompositeOperation> for emlite::Val {
    fn from(s: IterationCompositeOperation) -> emlite::Val {
        match s {
            IterationCompositeOperation::REPLACE => emlite::Val::from("replace"),
            IterationCompositeOperation::ACCUMULATE => emlite::Val::from("accumulate"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AnimationTriggerBehavior {
    ONCE,
    REPEAT,
    ALTERNATE,
    STATE,
}
impl FromVal for AnimationTriggerBehavior {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "once" => Self::ONCE,
            "repeat" => Self::REPEAT,
            "alternate" => Self::ALTERNATE,
            "state" => Self::STATE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AnimationTriggerBehavior> for emlite::Val {
    fn from(s: AnimationTriggerBehavior) -> emlite::Val {
        match s {
            AnimationTriggerBehavior::ONCE => emlite::Val::from("once"),
            AnimationTriggerBehavior::REPEAT => emlite::Val::from("repeat"),
            AnimationTriggerBehavior::ALTERNATE => emlite::Val::from("alternate"),
            AnimationTriggerBehavior::STATE => emlite::Val::from("state"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AnimationPlayState {
    IDLE,
    RUNNING,
    PAUSED,
    FINISHED,
}
impl FromVal for AnimationPlayState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "idle" => Self::IDLE,
            "running" => Self::RUNNING,
            "paused" => Self::PAUSED,
            "finished" => Self::FINISHED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AnimationPlayState> for emlite::Val {
    fn from(s: AnimationPlayState) -> emlite::Val {
        match s {
            AnimationPlayState::IDLE => emlite::Val::from("idle"),
            AnimationPlayState::RUNNING => emlite::Val::from("running"),
            AnimationPlayState::PAUSED => emlite::Val::from("paused"),
            AnimationPlayState::FINISHED => emlite::Val::from("finished"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AnimationReplaceState {
    ACTIVE,
    REMOVED,
    PERSISTED,
}
impl FromVal for AnimationReplaceState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "active" => Self::ACTIVE,
            "removed" => Self::REMOVED,
            "persisted" => Self::PERSISTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AnimationReplaceState> for emlite::Val {
    fn from(s: AnimationReplaceState) -> emlite::Val {
        match s {
            AnimationReplaceState::ACTIVE => emlite::Val::from("active"),
            AnimationReplaceState::REMOVED => emlite::Val::from("removed"),
            AnimationReplaceState::PERSISTED => emlite::Val::from("persisted"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FillMode {
    NONE,
    FORWARDS,
    BACKWARDS,
    BOTH,
    AUTO,
}
impl FromVal for FillMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "forwards" => Self::FORWARDS,
            "backwards" => Self::BACKWARDS,
            "both" => Self::BOTH,
            "auto" => Self::AUTO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<FillMode> for emlite::Val {
    fn from(s: FillMode) -> emlite::Val {
        match s {
            FillMode::NONE => emlite::Val::from("none"),
            FillMode::FORWARDS => emlite::Val::from("forwards"),
            FillMode::BACKWARDS => emlite::Val::from("backwards"),
            FillMode::BOTH => emlite::Val::from("both"),
            FillMode::AUTO => emlite::Val::from("auto"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PlaybackDirection {
    NORMAL,
    REVERSE,
    ALTERNATE,
    ALTERNATE_REVERSE,
}
impl FromVal for PlaybackDirection {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "normal" => Self::NORMAL,
            "reverse" => Self::REVERSE,
            "alternate" => Self::ALTERNATE,
            "alternate-reverse" => Self::ALTERNATE_REVERSE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PlaybackDirection> for emlite::Val {
    fn from(s: PlaybackDirection) -> emlite::Val {
        match s {
            PlaybackDirection::NORMAL => emlite::Val::from("normal"),
            PlaybackDirection::REVERSE => emlite::Val::from("reverse"),
            PlaybackDirection::ALTERNATE => emlite::Val::from("alternate"),
            PlaybackDirection::ALTERNATE_REVERSE => emlite::Val::from("alternate-reverse"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CompositeOperation {
    REPLACE,
    ADD,
    ACCUMULATE,
}
impl FromVal for CompositeOperation {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "replace" => Self::REPLACE,
            "add" => Self::ADD,
            "accumulate" => Self::ACCUMULATE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CompositeOperation> for emlite::Val {
    fn from(s: CompositeOperation) -> emlite::Val {
        match s {
            CompositeOperation::REPLACE => emlite::Val::from("replace"),
            CompositeOperation::ADD => emlite::Val::from("add"),
            CompositeOperation::ACCUMULATE => emlite::Val::from("accumulate"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CompositeOperationOrAuto {
    REPLACE,
    ADD,
    ACCUMULATE,
    AUTO,
}
impl FromVal for CompositeOperationOrAuto {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "replace" => Self::REPLACE,
            "add" => Self::ADD,
            "accumulate" => Self::ACCUMULATE,
            "auto" => Self::AUTO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CompositeOperationOrAuto> for emlite::Val {
    fn from(s: CompositeOperationOrAuto) -> emlite::Val {
        match s {
            CompositeOperationOrAuto::REPLACE => emlite::Val::from("replace"),
            CompositeOperationOrAuto::ADD => emlite::Val::from("add"),
            CompositeOperationOrAuto::ACCUMULATE => emlite::Val::from("accumulate"),
            CompositeOperationOrAuto::AUTO => emlite::Val::from("auto"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LockMode {
    SHARED,
    EXCLUSIVE,
}
impl FromVal for LockMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "shared" => Self::SHARED,
            "exclusive" => Self::EXCLUSIVE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<LockMode> for emlite::Val {
    fn from(s: LockMode) -> emlite::Val {
        match s {
            LockMode::SHARED => emlite::Val::from("shared"),
            LockMode::EXCLUSIVE => emlite::Val::from("exclusive"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OTPCredentialTransportType {
    SMS,
}
impl FromVal for OTPCredentialTransportType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "sms" => Self::SMS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OTPCredentialTransportType> for emlite::Val {
    fn from(s: OTPCredentialTransportType) -> emlite::Val {
        match s {
            OTPCredentialTransportType::SMS => emlite::Val::from("sms"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AudioContextState {
    SUSPENDED,
    RUNNING,
    CLOSED,
    INTERRUPTED,
}
impl FromVal for AudioContextState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "suspended" => Self::SUSPENDED,
            "running" => Self::RUNNING,
            "closed" => Self::CLOSED,
            "interrupted" => Self::INTERRUPTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AudioContextState> for emlite::Val {
    fn from(s: AudioContextState) -> emlite::Val {
        match s {
            AudioContextState::SUSPENDED => emlite::Val::from("suspended"),
            AudioContextState::RUNNING => emlite::Val::from("running"),
            AudioContextState::CLOSED => emlite::Val::from("closed"),
            AudioContextState::INTERRUPTED => emlite::Val::from("interrupted"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AudioContextRenderSizeCategory {
    DEFAULT,
    HARDWARE,
}
impl FromVal for AudioContextRenderSizeCategory {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "default" => Self::DEFAULT,
            "hardware" => Self::HARDWARE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AudioContextRenderSizeCategory> for emlite::Val {
    fn from(s: AudioContextRenderSizeCategory) -> emlite::Val {
        match s {
            AudioContextRenderSizeCategory::DEFAULT => emlite::Val::from("default"),
            AudioContextRenderSizeCategory::HARDWARE => emlite::Val::from("hardware"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AudioContextLatencyCategory {
    BALANCED,
    INTERACTIVE,
    PLAYBACK,
}
impl FromVal for AudioContextLatencyCategory {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "balanced" => Self::BALANCED,
            "interactive" => Self::INTERACTIVE,
            "playback" => Self::PLAYBACK,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AudioContextLatencyCategory> for emlite::Val {
    fn from(s: AudioContextLatencyCategory) -> emlite::Val {
        match s {
            AudioContextLatencyCategory::BALANCED => emlite::Val::from("balanced"),
            AudioContextLatencyCategory::INTERACTIVE => emlite::Val::from("interactive"),
            AudioContextLatencyCategory::PLAYBACK => emlite::Val::from("playback"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AudioSinkType {
    NONE,
}
impl FromVal for AudioSinkType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AudioSinkType> for emlite::Val {
    fn from(s: AudioSinkType) -> emlite::Val {
        match s {
            AudioSinkType::NONE => emlite::Val::from("none"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ChannelCountMode {
    MAX,
    CLAMPED_MAX,
    EXPLICIT,
}
impl FromVal for ChannelCountMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "max" => Self::MAX,
            "clamped-max" => Self::CLAMPED_MAX,
            "explicit" => Self::EXPLICIT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ChannelCountMode> for emlite::Val {
    fn from(s: ChannelCountMode) -> emlite::Val {
        match s {
            ChannelCountMode::MAX => emlite::Val::from("max"),
            ChannelCountMode::CLAMPED_MAX => emlite::Val::from("clamped-max"),
            ChannelCountMode::EXPLICIT => emlite::Val::from("explicit"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ChannelInterpretation {
    SPEAKERS,
    DISCRETE,
}
impl FromVal for ChannelInterpretation {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "speakers" => Self::SPEAKERS,
            "discrete" => Self::DISCRETE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ChannelInterpretation> for emlite::Val {
    fn from(s: ChannelInterpretation) -> emlite::Val {
        match s {
            ChannelInterpretation::SPEAKERS => emlite::Val::from("speakers"),
            ChannelInterpretation::DISCRETE => emlite::Val::from("discrete"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AutomationRate {
    A_RATE,
    K_RATE,
}
impl FromVal for AutomationRate {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "a-rate" => Self::A_RATE,
            "k-rate" => Self::K_RATE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AutomationRate> for emlite::Val {
    fn from(s: AutomationRate) -> emlite::Val {
        match s {
            AutomationRate::A_RATE => emlite::Val::from("a-rate"),
            AutomationRate::K_RATE => emlite::Val::from("k-rate"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BiquadFilterType {
    LOWPASS,
    HIGHPASS,
    BANDPASS,
    LOWSHELF,
    HIGHSHELF,
    PEAKING,
    NOTCH,
    ALLPASS,
}
impl FromVal for BiquadFilterType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "lowpass" => Self::LOWPASS,
            "highpass" => Self::HIGHPASS,
            "bandpass" => Self::BANDPASS,
            "lowshelf" => Self::LOWSHELF,
            "highshelf" => Self::HIGHSHELF,
            "peaking" => Self::PEAKING,
            "notch" => Self::NOTCH,
            "allpass" => Self::ALLPASS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<BiquadFilterType> for emlite::Val {
    fn from(s: BiquadFilterType) -> emlite::Val {
        match s {
            BiquadFilterType::LOWPASS => emlite::Val::from("lowpass"),
            BiquadFilterType::HIGHPASS => emlite::Val::from("highpass"),
            BiquadFilterType::BANDPASS => emlite::Val::from("bandpass"),
            BiquadFilterType::LOWSHELF => emlite::Val::from("lowshelf"),
            BiquadFilterType::HIGHSHELF => emlite::Val::from("highshelf"),
            BiquadFilterType::PEAKING => emlite::Val::from("peaking"),
            BiquadFilterType::NOTCH => emlite::Val::from("notch"),
            BiquadFilterType::ALLPASS => emlite::Val::from("allpass"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OscillatorType {
    SINE,
    SQUARE,
    SAWTOOTH,
    TRIANGLE,
    CUSTOM,
}
impl FromVal for OscillatorType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "sine" => Self::SINE,
            "square" => Self::SQUARE,
            "sawtooth" => Self::SAWTOOTH,
            "triangle" => Self::TRIANGLE,
            "custom" => Self::CUSTOM,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OscillatorType> for emlite::Val {
    fn from(s: OscillatorType) -> emlite::Val {
        match s {
            OscillatorType::SINE => emlite::Val::from("sine"),
            OscillatorType::SQUARE => emlite::Val::from("square"),
            OscillatorType::SAWTOOTH => emlite::Val::from("sawtooth"),
            OscillatorType::TRIANGLE => emlite::Val::from("triangle"),
            OscillatorType::CUSTOM => emlite::Val::from("custom"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PanningModelType {
    EQUALPOWER,
    HRTF,
}
impl FromVal for PanningModelType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "equalpower" => Self::EQUALPOWER,
            "HRTF" => Self::HRTF,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PanningModelType> for emlite::Val {
    fn from(s: PanningModelType) -> emlite::Val {
        match s {
            PanningModelType::EQUALPOWER => emlite::Val::from("equalpower"),
            PanningModelType::HRTF => emlite::Val::from("HRTF"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DistanceModelType {
    LINEAR,
    INVERSE,
    EXPONENTIAL,
}
impl FromVal for DistanceModelType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "linear" => Self::LINEAR,
            "inverse" => Self::INVERSE,
            "exponential" => Self::EXPONENTIAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<DistanceModelType> for emlite::Val {
    fn from(s: DistanceModelType) -> emlite::Val {
        match s {
            DistanceModelType::LINEAR => emlite::Val::from("linear"),
            DistanceModelType::INVERSE => emlite::Val::from("inverse"),
            DistanceModelType::EXPONENTIAL => emlite::Val::from("exponential"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OverSampleType {
    NONE,
    _2X,
    _4X,
}
impl FromVal for OverSampleType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "2x" => Self::_2X,
            "4x" => Self::_4X,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OverSampleType> for emlite::Val {
    fn from(s: OverSampleType) -> emlite::Val {
        match s {
            OverSampleType::NONE => emlite::Val::from("none"),
            OverSampleType::_2X => emlite::Val::from("2x"),
            OverSampleType::_4X => emlite::Val::from("4x"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AuthenticatorAttachment {
    PLATFORM,
    CROSS_PLATFORM,
}
impl FromVal for AuthenticatorAttachment {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "platform" => Self::PLATFORM,
            "cross-platform" => Self::CROSS_PLATFORM,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AuthenticatorAttachment> for emlite::Val {
    fn from(s: AuthenticatorAttachment) -> emlite::Val {
        match s {
            AuthenticatorAttachment::PLATFORM => emlite::Val::from("platform"),
            AuthenticatorAttachment::CROSS_PLATFORM => emlite::Val::from("cross-platform"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ResidentKeyRequirement {
    DISCOURAGED,
    PREFERRED,
    REQUIRED,
}
impl FromVal for ResidentKeyRequirement {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "discouraged" => Self::DISCOURAGED,
            "preferred" => Self::PREFERRED,
            "required" => Self::REQUIRED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ResidentKeyRequirement> for emlite::Val {
    fn from(s: ResidentKeyRequirement) -> emlite::Val {
        match s {
            ResidentKeyRequirement::DISCOURAGED => emlite::Val::from("discouraged"),
            ResidentKeyRequirement::PREFERRED => emlite::Val::from("preferred"),
            ResidentKeyRequirement::REQUIRED => emlite::Val::from("required"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AttestationConveyancePreference {
    NONE,
    INDIRECT,
    DIRECT,
    ENTERPRISE,
}
impl FromVal for AttestationConveyancePreference {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "indirect" => Self::INDIRECT,
            "direct" => Self::DIRECT,
            "enterprise" => Self::ENTERPRISE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AttestationConveyancePreference> for emlite::Val {
    fn from(s: AttestationConveyancePreference) -> emlite::Val {
        match s {
            AttestationConveyancePreference::NONE => emlite::Val::from("none"),
            AttestationConveyancePreference::INDIRECT => emlite::Val::from("indirect"),
            AttestationConveyancePreference::DIRECT => emlite::Val::from("direct"),
            AttestationConveyancePreference::ENTERPRISE => emlite::Val::from("enterprise"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TokenBindingStatus {
    PRESENT,
    SUPPORTED,
}
impl FromVal for TokenBindingStatus {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "present" => Self::PRESENT,
            "supported" => Self::SUPPORTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<TokenBindingStatus> for emlite::Val {
    fn from(s: TokenBindingStatus) -> emlite::Val {
        match s {
            TokenBindingStatus::PRESENT => emlite::Val::from("present"),
            TokenBindingStatus::SUPPORTED => emlite::Val::from("supported"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PublicKeyCredentialType {
    PUBLIC_KEY,
}
impl FromVal for PublicKeyCredentialType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "public-key" => Self::PUBLIC_KEY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PublicKeyCredentialType> for emlite::Val {
    fn from(s: PublicKeyCredentialType) -> emlite::Val {
        match s {
            PublicKeyCredentialType::PUBLIC_KEY => emlite::Val::from("public-key"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AuthenticatorTransport {
    USB,
    NFC,
    BLE,
    SMART_CARD,
    HYBRID,
    INTERNAL,
}
impl FromVal for AuthenticatorTransport {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "usb" => Self::USB,
            "nfc" => Self::NFC,
            "ble" => Self::BLE,
            "smart-card" => Self::SMART_CARD,
            "hybrid" => Self::HYBRID,
            "internal" => Self::INTERNAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AuthenticatorTransport> for emlite::Val {
    fn from(s: AuthenticatorTransport) -> emlite::Val {
        match s {
            AuthenticatorTransport::USB => emlite::Val::from("usb"),
            AuthenticatorTransport::NFC => emlite::Val::from("nfc"),
            AuthenticatorTransport::BLE => emlite::Val::from("ble"),
            AuthenticatorTransport::SMART_CARD => emlite::Val::from("smart-card"),
            AuthenticatorTransport::HYBRID => emlite::Val::from("hybrid"),
            AuthenticatorTransport::INTERNAL => emlite::Val::from("internal"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum UserVerificationRequirement {
    REQUIRED,
    PREFERRED,
    DISCOURAGED,
}
impl FromVal for UserVerificationRequirement {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "required" => Self::REQUIRED,
            "preferred" => Self::PREFERRED,
            "discouraged" => Self::DISCOURAGED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<UserVerificationRequirement> for emlite::Val {
    fn from(s: UserVerificationRequirement) -> emlite::Val {
        match s {
            UserVerificationRequirement::REQUIRED => emlite::Val::from("required"),
            UserVerificationRequirement::PREFERRED => emlite::Val::from("preferred"),
            UserVerificationRequirement::DISCOURAGED => emlite::Val::from("discouraged"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ClientCapability {
    CONDITIONAL_CREATE,
    CONDITIONAL_GET,
    HYBRID_TRANSPORT,
    PASSKEY_PLATFORM_AUTHENTICATOR,
    USER_VERIFYING_PLATFORM_AUTHENTICATOR,
    RELATED_ORIGINS,
    SIGNAL_ALL_ACCEPTED_CREDENTIALS,
    SIGNAL_CURRENT_USER_DETAILS,
    SIGNAL_UNKNOWN_CREDENTIAL,
}
impl FromVal for ClientCapability {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "conditionalCreate" => Self::CONDITIONAL_CREATE,
            "conditionalGet" => Self::CONDITIONAL_GET,
            "hybridTransport" => Self::HYBRID_TRANSPORT,
            "passkeyPlatformAuthenticator" => Self::PASSKEY_PLATFORM_AUTHENTICATOR,
            "userVerifyingPlatformAuthenticator" => Self::USER_VERIFYING_PLATFORM_AUTHENTICATOR,
            "relatedOrigins" => Self::RELATED_ORIGINS,
            "signalAllAcceptedCredentials" => Self::SIGNAL_ALL_ACCEPTED_CREDENTIALS,
            "signalCurrentUserDetails" => Self::SIGNAL_CURRENT_USER_DETAILS,
            "signalUnknownCredential" => Self::SIGNAL_UNKNOWN_CREDENTIAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ClientCapability> for emlite::Val {
    fn from(s: ClientCapability) -> emlite::Val {
        match s {
            ClientCapability::CONDITIONAL_CREATE => emlite::Val::from("conditionalCreate"),
            ClientCapability::CONDITIONAL_GET => emlite::Val::from("conditionalGet"),
            ClientCapability::HYBRID_TRANSPORT => emlite::Val::from("hybridTransport"),
            ClientCapability::PASSKEY_PLATFORM_AUTHENTICATOR => {
                emlite::Val::from("passkeyPlatformAuthenticator")
            }
            ClientCapability::USER_VERIFYING_PLATFORM_AUTHENTICATOR => {
                emlite::Val::from("userVerifyingPlatformAuthenticator")
            }
            ClientCapability::RELATED_ORIGINS => emlite::Val::from("relatedOrigins"),
            ClientCapability::SIGNAL_ALL_ACCEPTED_CREDENTIALS => {
                emlite::Val::from("signalAllAcceptedCredentials")
            }
            ClientCapability::SIGNAL_CURRENT_USER_DETAILS => {
                emlite::Val::from("signalCurrentUserDetails")
            }
            ClientCapability::SIGNAL_UNKNOWN_CREDENTIAL => {
                emlite::Val::from("signalUnknownCredential")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PublicKeyCredentialHint {
    SECURITY_KEY,
    CLIENT_DEVICE,
    HYBRID,
}
impl FromVal for PublicKeyCredentialHint {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "security-key" => Self::SECURITY_KEY,
            "client-device" => Self::CLIENT_DEVICE,
            "hybrid" => Self::HYBRID,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PublicKeyCredentialHint> for emlite::Val {
    fn from(s: PublicKeyCredentialHint) -> emlite::Val {
        match s {
            PublicKeyCredentialHint::SECURITY_KEY => emlite::Val::from("security-key"),
            PublicKeyCredentialHint::CLIENT_DEVICE => emlite::Val::from("client-device"),
            PublicKeyCredentialHint::HYBRID => emlite::Val::from("hybrid"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LargeBlobSupport {
    REQUIRED,
    PREFERRED,
}
impl FromVal for LargeBlobSupport {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "required" => Self::REQUIRED,
            "preferred" => Self::PREFERRED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<LargeBlobSupport> for emlite::Val {
    fn from(s: LargeBlobSupport) -> emlite::Val {
        match s {
            LargeBlobSupport::REQUIRED => emlite::Val::from("required"),
            LargeBlobSupport::PREFERRED => emlite::Val::from("preferred"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AacBitstreamFormat {
    AAC,
    ADTS,
}
impl FromVal for AacBitstreamFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "aac" => Self::AAC,
            "adts" => Self::ADTS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AacBitstreamFormat> for emlite::Val {
    fn from(s: AacBitstreamFormat) -> emlite::Val {
        match s {
            AacBitstreamFormat::AAC => emlite::Val::from("aac"),
            AacBitstreamFormat::ADTS => emlite::Val::from("adts"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AvcBitstreamFormat {
    ANNEXB,
    AVC,
}
impl FromVal for AvcBitstreamFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "annexb" => Self::ANNEXB,
            "avc" => Self::AVC,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AvcBitstreamFormat> for emlite::Val {
    fn from(s: AvcBitstreamFormat) -> emlite::Val {
        match s {
            AvcBitstreamFormat::ANNEXB => emlite::Val::from("annexb"),
            AvcBitstreamFormat::AVC => emlite::Val::from("avc"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum HevcBitstreamFormat {
    ANNEXB,
    HEVC,
}
impl FromVal for HevcBitstreamFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "annexb" => Self::ANNEXB,
            "hevc" => Self::HEVC,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<HevcBitstreamFormat> for emlite::Val {
    fn from(s: HevcBitstreamFormat) -> emlite::Val {
        match s {
            HevcBitstreamFormat::ANNEXB => emlite::Val::from("annexb"),
            HevcBitstreamFormat::HEVC => emlite::Val::from("hevc"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OpusBitstreamFormat {
    OPUS,
    OGG,
}
impl FromVal for OpusBitstreamFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "opus" => Self::OPUS,
            "ogg" => Self::OGG,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OpusBitstreamFormat> for emlite::Val {
    fn from(s: OpusBitstreamFormat) -> emlite::Val {
        match s {
            OpusBitstreamFormat::OPUS => emlite::Val::from("opus"),
            OpusBitstreamFormat::OGG => emlite::Val::from("ogg"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OpusSignal {
    AUTO,
    MUSIC,
    VOICE,
}
impl FromVal for OpusSignal {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            "music" => Self::MUSIC,
            "voice" => Self::VOICE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OpusSignal> for emlite::Val {
    fn from(s: OpusSignal) -> emlite::Val {
        match s {
            OpusSignal::AUTO => emlite::Val::from("auto"),
            OpusSignal::MUSIC => emlite::Val::from("music"),
            OpusSignal::VOICE => emlite::Val::from("voice"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OpusApplication {
    VOIP,
    AUDIO,
    LOWDELAY,
}
impl FromVal for OpusApplication {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "voip" => Self::VOIP,
            "audio" => Self::AUDIO,
            "lowdelay" => Self::LOWDELAY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<OpusApplication> for emlite::Val {
    fn from(s: OpusApplication) -> emlite::Val {
        match s {
            OpusApplication::VOIP => emlite::Val::from("voip"),
            OpusApplication::AUDIO => emlite::Val::from("audio"),
            OpusApplication::LOWDELAY => emlite::Val::from("lowdelay"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum HardwareAcceleration {
    NO_PREFERENCE,
    PREFER_HARDWARE,
    PREFER_SOFTWARE,
}
impl FromVal for HardwareAcceleration {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "no-preference" => Self::NO_PREFERENCE,
            "prefer-hardware" => Self::PREFER_HARDWARE,
            "prefer-software" => Self::PREFER_SOFTWARE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<HardwareAcceleration> for emlite::Val {
    fn from(s: HardwareAcceleration) -> emlite::Val {
        match s {
            HardwareAcceleration::NO_PREFERENCE => emlite::Val::from("no-preference"),
            HardwareAcceleration::PREFER_HARDWARE => emlite::Val::from("prefer-hardware"),
            HardwareAcceleration::PREFER_SOFTWARE => emlite::Val::from("prefer-software"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AlphaOption {
    KEEP,
    DISCARD,
}
impl FromVal for AlphaOption {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "keep" => Self::KEEP,
            "discard" => Self::DISCARD,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AlphaOption> for emlite::Val {
    fn from(s: AlphaOption) -> emlite::Val {
        match s {
            AlphaOption::KEEP => emlite::Val::from("keep"),
            AlphaOption::DISCARD => emlite::Val::from("discard"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LatencyMode {
    QUALITY,
    REALTIME,
}
impl FromVal for LatencyMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "quality" => Self::QUALITY,
            "realtime" => Self::REALTIME,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<LatencyMode> for emlite::Val {
    fn from(s: LatencyMode) -> emlite::Val {
        match s {
            LatencyMode::QUALITY => emlite::Val::from("quality"),
            LatencyMode::REALTIME => emlite::Val::from("realtime"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum VideoEncoderBitrateMode {
    CONSTANT,
    VARIABLE,
    QUANTIZER,
}
impl FromVal for VideoEncoderBitrateMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "constant" => Self::CONSTANT,
            "variable" => Self::VARIABLE,
            "quantizer" => Self::QUANTIZER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<VideoEncoderBitrateMode> for emlite::Val {
    fn from(s: VideoEncoderBitrateMode) -> emlite::Val {
        match s {
            VideoEncoderBitrateMode::CONSTANT => emlite::Val::from("constant"),
            VideoEncoderBitrateMode::VARIABLE => emlite::Val::from("variable"),
            VideoEncoderBitrateMode::QUANTIZER => emlite::Val::from("quantizer"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CodecState {
    UNCONFIGURED,
    CONFIGURED,
    CLOSED,
}
impl FromVal for CodecState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "unconfigured" => Self::UNCONFIGURED,
            "configured" => Self::CONFIGURED,
            "closed" => Self::CLOSED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<CodecState> for emlite::Val {
    fn from(s: CodecState) -> emlite::Val {
        match s {
            CodecState::UNCONFIGURED => emlite::Val::from("unconfigured"),
            CodecState::CONFIGURED => emlite::Val::from("configured"),
            CodecState::CLOSED => emlite::Val::from("closed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EncodedAudioChunkType {
    KEY,
    DELTA,
}
impl FromVal for EncodedAudioChunkType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "key" => Self::KEY,
            "delta" => Self::DELTA,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<EncodedAudioChunkType> for emlite::Val {
    fn from(s: EncodedAudioChunkType) -> emlite::Val {
        match s {
            EncodedAudioChunkType::KEY => emlite::Val::from("key"),
            EncodedAudioChunkType::DELTA => emlite::Val::from("delta"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EncodedVideoChunkType {
    KEY,
    DELTA,
}
impl FromVal for EncodedVideoChunkType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "key" => Self::KEY,
            "delta" => Self::DELTA,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<EncodedVideoChunkType> for emlite::Val {
    fn from(s: EncodedVideoChunkType) -> emlite::Val {
        match s {
            EncodedVideoChunkType::KEY => emlite::Val::from("key"),
            EncodedVideoChunkType::DELTA => emlite::Val::from("delta"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AudioSampleFormat {
    U8_,
    S16,
    S32,
    F32_,
    U8_PLANAR,
    S16_PLANAR,
    S32_PLANAR,
    F32_PLANAR,
}
impl FromVal for AudioSampleFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "u8" => Self::U8_,
            "s16" => Self::S16,
            "s32" => Self::S32,
            "f32" => Self::F32_,
            "u8-planar" => Self::U8_PLANAR,
            "s16-planar" => Self::S16_PLANAR,
            "s32-planar" => Self::S32_PLANAR,
            "f32-planar" => Self::F32_PLANAR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AudioSampleFormat> for emlite::Val {
    fn from(s: AudioSampleFormat) -> emlite::Val {
        match s {
            AudioSampleFormat::U8_ => emlite::Val::from("u8"),
            AudioSampleFormat::S16 => emlite::Val::from("s16"),
            AudioSampleFormat::S32 => emlite::Val::from("s32"),
            AudioSampleFormat::F32_ => emlite::Val::from("f32"),
            AudioSampleFormat::U8_PLANAR => emlite::Val::from("u8-planar"),
            AudioSampleFormat::S16_PLANAR => emlite::Val::from("s16-planar"),
            AudioSampleFormat::S32_PLANAR => emlite::Val::from("s32-planar"),
            AudioSampleFormat::F32_PLANAR => emlite::Val::from("f32-planar"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum VideoPixelFormat {
    I420,
    I420_P10,
    I420_P12,
    I420_A,
    I420_AP10,
    I420_AP12,
    I422,
    I422_P10,
    I422_P12,
    I422_A,
    I422_AP10,
    I422_AP12,
    I444,
    I444_P10,
    I444_P12,
    I444_A,
    I444_AP10,
    I444_AP12,
    NV12,
    RGBA,
    RGBX,
    BGRA,
    BGRX,
}
impl FromVal for VideoPixelFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "I420" => Self::I420,
            "I420P10" => Self::I420_P10,
            "I420P12" => Self::I420_P12,
            "I420A" => Self::I420_A,
            "I420AP10" => Self::I420_AP10,
            "I420AP12" => Self::I420_AP12,
            "I422" => Self::I422,
            "I422P10" => Self::I422_P10,
            "I422P12" => Self::I422_P12,
            "I422A" => Self::I422_A,
            "I422AP10" => Self::I422_AP10,
            "I422AP12" => Self::I422_AP12,
            "I444" => Self::I444,
            "I444P10" => Self::I444_P10,
            "I444P12" => Self::I444_P12,
            "I444A" => Self::I444_A,
            "I444AP10" => Self::I444_AP10,
            "I444AP12" => Self::I444_AP12,
            "NV12" => Self::NV12,
            "RGBA" => Self::RGBA,
            "RGBX" => Self::RGBX,
            "BGRA" => Self::BGRA,
            "BGRX" => Self::BGRX,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<VideoPixelFormat> for emlite::Val {
    fn from(s: VideoPixelFormat) -> emlite::Val {
        match s {
            VideoPixelFormat::I420 => emlite::Val::from("I420"),
            VideoPixelFormat::I420_P10 => emlite::Val::from("I420P10"),
            VideoPixelFormat::I420_P12 => emlite::Val::from("I420P12"),
            VideoPixelFormat::I420_A => emlite::Val::from("I420A"),
            VideoPixelFormat::I420_AP10 => emlite::Val::from("I420AP10"),
            VideoPixelFormat::I420_AP12 => emlite::Val::from("I420AP12"),
            VideoPixelFormat::I422 => emlite::Val::from("I422"),
            VideoPixelFormat::I422_P10 => emlite::Val::from("I422P10"),
            VideoPixelFormat::I422_P12 => emlite::Val::from("I422P12"),
            VideoPixelFormat::I422_A => emlite::Val::from("I422A"),
            VideoPixelFormat::I422_AP10 => emlite::Val::from("I422AP10"),
            VideoPixelFormat::I422_AP12 => emlite::Val::from("I422AP12"),
            VideoPixelFormat::I444 => emlite::Val::from("I444"),
            VideoPixelFormat::I444_P10 => emlite::Val::from("I444P10"),
            VideoPixelFormat::I444_P12 => emlite::Val::from("I444P12"),
            VideoPixelFormat::I444_A => emlite::Val::from("I444A"),
            VideoPixelFormat::I444_AP10 => emlite::Val::from("I444AP10"),
            VideoPixelFormat::I444_AP12 => emlite::Val::from("I444AP12"),
            VideoPixelFormat::NV12 => emlite::Val::from("NV12"),
            VideoPixelFormat::RGBA => emlite::Val::from("RGBA"),
            VideoPixelFormat::RGBX => emlite::Val::from("RGBX"),
            VideoPixelFormat::BGRA => emlite::Val::from("BGRA"),
            VideoPixelFormat::BGRX => emlite::Val::from("BGRX"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum VideoColorPrimaries {
    BT709,
    BT470BG,
    SMPTE170M,
    BT2020,
    SMPTE432,
}
impl FromVal for VideoColorPrimaries {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "bt709" => Self::BT709,
            "bt470bg" => Self::BT470BG,
            "smpte170m" => Self::SMPTE170M,
            "bt2020" => Self::BT2020,
            "smpte432" => Self::SMPTE432,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<VideoColorPrimaries> for emlite::Val {
    fn from(s: VideoColorPrimaries) -> emlite::Val {
        match s {
            VideoColorPrimaries::BT709 => emlite::Val::from("bt709"),
            VideoColorPrimaries::BT470BG => emlite::Val::from("bt470bg"),
            VideoColorPrimaries::SMPTE170M => emlite::Val::from("smpte170m"),
            VideoColorPrimaries::BT2020 => emlite::Val::from("bt2020"),
            VideoColorPrimaries::SMPTE432 => emlite::Val::from("smpte432"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum VideoTransferCharacteristics {
    BT709,
    SMPTE170M,
    IEC61966_2_1,
    LINEAR,
    PQ,
    HLG,
}
impl FromVal for VideoTransferCharacteristics {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "bt709" => Self::BT709,
            "smpte170m" => Self::SMPTE170M,
            "iec61966-2-1" => Self::IEC61966_2_1,
            "linear" => Self::LINEAR,
            "pq" => Self::PQ,
            "hlg" => Self::HLG,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<VideoTransferCharacteristics> for emlite::Val {
    fn from(s: VideoTransferCharacteristics) -> emlite::Val {
        match s {
            VideoTransferCharacteristics::BT709 => emlite::Val::from("bt709"),
            VideoTransferCharacteristics::SMPTE170M => emlite::Val::from("smpte170m"),
            VideoTransferCharacteristics::IEC61966_2_1 => emlite::Val::from("iec61966-2-1"),
            VideoTransferCharacteristics::LINEAR => emlite::Val::from("linear"),
            VideoTransferCharacteristics::PQ => emlite::Val::from("pq"),
            VideoTransferCharacteristics::HLG => emlite::Val::from("hlg"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum VideoMatrixCoefficients {
    RGB,
    BT709,
    BT470BG,
    SMPTE170M,
    BT2020_NCL,
}
impl FromVal for VideoMatrixCoefficients {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "rgb" => Self::RGB,
            "bt709" => Self::BT709,
            "bt470bg" => Self::BT470BG,
            "smpte170m" => Self::SMPTE170M,
            "bt2020-ncl" => Self::BT2020_NCL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<VideoMatrixCoefficients> for emlite::Val {
    fn from(s: VideoMatrixCoefficients) -> emlite::Val {
        match s {
            VideoMatrixCoefficients::RGB => emlite::Val::from("rgb"),
            VideoMatrixCoefficients::BT709 => emlite::Val::from("bt709"),
            VideoMatrixCoefficients::BT470BG => emlite::Val::from("bt470bg"),
            VideoMatrixCoefficients::SMPTE170M => emlite::Val::from("smpte170m"),
            VideoMatrixCoefficients::BT2020_NCL => emlite::Val::from("bt2020-ncl"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum KeyType {
    PUBLIC,
    PRIVATE,
    SECRET,
}
impl FromVal for KeyType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "public" => Self::PUBLIC,
            "private" => Self::PRIVATE,
            "secret" => Self::SECRET,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<KeyType> for emlite::Val {
    fn from(s: KeyType) -> emlite::Val {
        match s {
            KeyType::PUBLIC => emlite::Val::from("public"),
            KeyType::PRIVATE => emlite::Val::from("private"),
            KeyType::SECRET => emlite::Val::from("secret"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum KeyUsage {
    ENCRYPT,
    DECRYPT,
    SIGN,
    VERIFY,
    DERIVE_KEY,
    DERIVE_BITS,
    WRAP_KEY,
    UNWRAP_KEY,
}
impl FromVal for KeyUsage {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "encrypt" => Self::ENCRYPT,
            "decrypt" => Self::DECRYPT,
            "sign" => Self::SIGN,
            "verify" => Self::VERIFY,
            "deriveKey" => Self::DERIVE_KEY,
            "deriveBits" => Self::DERIVE_BITS,
            "wrapKey" => Self::WRAP_KEY,
            "unwrapKey" => Self::UNWRAP_KEY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<KeyUsage> for emlite::Val {
    fn from(s: KeyUsage) -> emlite::Val {
        match s {
            KeyUsage::ENCRYPT => emlite::Val::from("encrypt"),
            KeyUsage::DECRYPT => emlite::Val::from("decrypt"),
            KeyUsage::SIGN => emlite::Val::from("sign"),
            KeyUsage::VERIFY => emlite::Val::from("verify"),
            KeyUsage::DERIVE_KEY => emlite::Val::from("deriveKey"),
            KeyUsage::DERIVE_BITS => emlite::Val::from("deriveBits"),
            KeyUsage::WRAP_KEY => emlite::Val::from("wrapKey"),
            KeyUsage::UNWRAP_KEY => emlite::Val::from("unwrapKey"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum KeyFormat {
    RAW_,
    SPKI,
    PKCS8,
    JWK,
}
impl FromVal for KeyFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "raw" => Self::RAW_,
            "spki" => Self::SPKI,
            "pkcs8" => Self::PKCS8,
            "jwk" => Self::JWK,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<KeyFormat> for emlite::Val {
    fn from(s: KeyFormat) -> emlite::Val {
        match s {
            KeyFormat::RAW_ => emlite::Val::from("raw"),
            KeyFormat::SPKI => emlite::Val::from("spki"),
            KeyFormat::PKCS8 => emlite::Val::from("pkcs8"),
            KeyFormat::JWK => emlite::Val::from("jwk"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WebGLPowerPreference {
    DEFAULT,
    LOW_POWER,
    HIGH_PERFORMANCE,
}
impl FromVal for WebGLPowerPreference {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "default" => Self::DEFAULT,
            "low-power" => Self::LOW_POWER,
            "high-performance" => Self::HIGH_PERFORMANCE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WebGLPowerPreference> for emlite::Val {
    fn from(s: WebGLPowerPreference) -> emlite::Val {
        match s {
            WebGLPowerPreference::DEFAULT => emlite::Val::from("default"),
            WebGLPowerPreference::LOW_POWER => emlite::Val::from("low-power"),
            WebGLPowerPreference::HIGH_PERFORMANCE => emlite::Val::from("high-performance"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUPowerPreference {
    LOW_POWER,
    HIGH_PERFORMANCE,
}
impl FromVal for GPUPowerPreference {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "low-power" => Self::LOW_POWER,
            "high-performance" => Self::HIGH_PERFORMANCE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUPowerPreference> for emlite::Val {
    fn from(s: GPUPowerPreference) -> emlite::Val {
        match s {
            GPUPowerPreference::LOW_POWER => emlite::Val::from("low-power"),
            GPUPowerPreference::HIGH_PERFORMANCE => emlite::Val::from("high-performance"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUFeatureName {
    CORE_FEATURES_AND_LIMITS,
    DEPTH_CLIP_CONTROL,
    DEPTH32FLOAT_STENCIL8,
    TEXTURE_COMPRESSION_BC,
    TEXTURE_COMPRESSION_BC_SLICED_3D,
    TEXTURE_COMPRESSION_ETC2,
    TEXTURE_COMPRESSION_ASTC,
    TEXTURE_COMPRESSION_ASTC_SLICED_3D,
    TIMESTAMP_QUERY,
    INDIRECT_FIRST_INSTANCE,
    SHADER_F16,
    RG11B10UFLOAT_RENDERABLE,
    BGRA8UNORM_STORAGE,
    FLOAT32_FILTERABLE,
    FLOAT32_BLENDABLE,
    CLIP_DISTANCES,
    DUAL_SOURCE_BLENDING,
    SUBGROUPS,
    TEXTURE_FORMATS_TIER1,
    TEXTURE_FORMATS_TIER2,
}
impl FromVal for GPUFeatureName {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "core-features-and-limits" => Self::CORE_FEATURES_AND_LIMITS,
            "depth-clip-control" => Self::DEPTH_CLIP_CONTROL,
            "depth32float-stencil8" => Self::DEPTH32FLOAT_STENCIL8,
            "texture-compression-bc" => Self::TEXTURE_COMPRESSION_BC,
            "texture-compression-bc-sliced-3d" => Self::TEXTURE_COMPRESSION_BC_SLICED_3D,
            "texture-compression-etc2" => Self::TEXTURE_COMPRESSION_ETC2,
            "texture-compression-astc" => Self::TEXTURE_COMPRESSION_ASTC,
            "texture-compression-astc-sliced-3d" => Self::TEXTURE_COMPRESSION_ASTC_SLICED_3D,
            "timestamp-query" => Self::TIMESTAMP_QUERY,
            "indirect-first-instance" => Self::INDIRECT_FIRST_INSTANCE,
            "shader-f16" => Self::SHADER_F16,
            "rg11b10ufloat-renderable" => Self::RG11B10UFLOAT_RENDERABLE,
            "bgra8unorm-storage" => Self::BGRA8UNORM_STORAGE,
            "float32-filterable" => Self::FLOAT32_FILTERABLE,
            "float32-blendable" => Self::FLOAT32_BLENDABLE,
            "clip-distances" => Self::CLIP_DISTANCES,
            "dual-source-blending" => Self::DUAL_SOURCE_BLENDING,
            "subgroups" => Self::SUBGROUPS,
            "texture-formats-tier1" => Self::TEXTURE_FORMATS_TIER1,
            "texture-formats-tier2" => Self::TEXTURE_FORMATS_TIER2,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUFeatureName> for emlite::Val {
    fn from(s: GPUFeatureName) -> emlite::Val {
        match s {
            GPUFeatureName::CORE_FEATURES_AND_LIMITS => {
                emlite::Val::from("core-features-and-limits")
            }
            GPUFeatureName::DEPTH_CLIP_CONTROL => emlite::Val::from("depth-clip-control"),
            GPUFeatureName::DEPTH32FLOAT_STENCIL8 => emlite::Val::from("depth32float-stencil8"),
            GPUFeatureName::TEXTURE_COMPRESSION_BC => emlite::Val::from("texture-compression-bc"),
            GPUFeatureName::TEXTURE_COMPRESSION_BC_SLICED_3D => {
                emlite::Val::from("texture-compression-bc-sliced-3d")
            }
            GPUFeatureName::TEXTURE_COMPRESSION_ETC2 => {
                emlite::Val::from("texture-compression-etc2")
            }
            GPUFeatureName::TEXTURE_COMPRESSION_ASTC => {
                emlite::Val::from("texture-compression-astc")
            }
            GPUFeatureName::TEXTURE_COMPRESSION_ASTC_SLICED_3D => {
                emlite::Val::from("texture-compression-astc-sliced-3d")
            }
            GPUFeatureName::TIMESTAMP_QUERY => emlite::Val::from("timestamp-query"),
            GPUFeatureName::INDIRECT_FIRST_INSTANCE => emlite::Val::from("indirect-first-instance"),
            GPUFeatureName::SHADER_F16 => emlite::Val::from("shader-f16"),
            GPUFeatureName::RG11B10UFLOAT_RENDERABLE => {
                emlite::Val::from("rg11b10ufloat-renderable")
            }
            GPUFeatureName::BGRA8UNORM_STORAGE => emlite::Val::from("bgra8unorm-storage"),
            GPUFeatureName::FLOAT32_FILTERABLE => emlite::Val::from("float32-filterable"),
            GPUFeatureName::FLOAT32_BLENDABLE => emlite::Val::from("float32-blendable"),
            GPUFeatureName::CLIP_DISTANCES => emlite::Val::from("clip-distances"),
            GPUFeatureName::DUAL_SOURCE_BLENDING => emlite::Val::from("dual-source-blending"),
            GPUFeatureName::SUBGROUPS => emlite::Val::from("subgroups"),
            GPUFeatureName::TEXTURE_FORMATS_TIER1 => emlite::Val::from("texture-formats-tier1"),
            GPUFeatureName::TEXTURE_FORMATS_TIER2 => emlite::Val::from("texture-formats-tier2"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUBufferMapState {
    UNMAPPED,
    PENDING,
    MAPPED,
}
impl FromVal for GPUBufferMapState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "unmapped" => Self::UNMAPPED,
            "pending" => Self::PENDING,
            "mapped" => Self::MAPPED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUBufferMapState> for emlite::Val {
    fn from(s: GPUBufferMapState) -> emlite::Val {
        match s {
            GPUBufferMapState::UNMAPPED => emlite::Val::from("unmapped"),
            GPUBufferMapState::PENDING => emlite::Val::from("pending"),
            GPUBufferMapState::MAPPED => emlite::Val::from("mapped"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUTextureDimension {
    _1D,
    _2D,
    _3D,
}
impl FromVal for GPUTextureDimension {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "1d" => Self::_1D,
            "2d" => Self::_2D,
            "3d" => Self::_3D,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUTextureDimension> for emlite::Val {
    fn from(s: GPUTextureDimension) -> emlite::Val {
        match s {
            GPUTextureDimension::_1D => emlite::Val::from("1d"),
            GPUTextureDimension::_2D => emlite::Val::from("2d"),
            GPUTextureDimension::_3D => emlite::Val::from("3d"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUTextureViewDimension {
    _1D,
    _2D,
    _2D_ARRAY,
    CUBE,
    CUBE_ARRAY,
    _3D,
}
impl FromVal for GPUTextureViewDimension {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "1d" => Self::_1D,
            "2d" => Self::_2D,
            "2d-array" => Self::_2D_ARRAY,
            "cube" => Self::CUBE,
            "cube-array" => Self::CUBE_ARRAY,
            "3d" => Self::_3D,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUTextureViewDimension> for emlite::Val {
    fn from(s: GPUTextureViewDimension) -> emlite::Val {
        match s {
            GPUTextureViewDimension::_1D => emlite::Val::from("1d"),
            GPUTextureViewDimension::_2D => emlite::Val::from("2d"),
            GPUTextureViewDimension::_2D_ARRAY => emlite::Val::from("2d-array"),
            GPUTextureViewDimension::CUBE => emlite::Val::from("cube"),
            GPUTextureViewDimension::CUBE_ARRAY => emlite::Val::from("cube-array"),
            GPUTextureViewDimension::_3D => emlite::Val::from("3d"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUTextureAspect {
    ALL,
    STENCIL_ONLY,
    DEPTH_ONLY,
}
impl FromVal for GPUTextureAspect {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "all" => Self::ALL,
            "stencil-only" => Self::STENCIL_ONLY,
            "depth-only" => Self::DEPTH_ONLY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUTextureAspect> for emlite::Val {
    fn from(s: GPUTextureAspect) -> emlite::Val {
        match s {
            GPUTextureAspect::ALL => emlite::Val::from("all"),
            GPUTextureAspect::STENCIL_ONLY => emlite::Val::from("stencil-only"),
            GPUTextureAspect::DEPTH_ONLY => emlite::Val::from("depth-only"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUTextureFormat {
    R8UNORM,
    R8SNORM,
    R8UINT,
    R8SINT,
    R16UNORM,
    R16SNORM,
    R16UINT,
    R16SINT,
    R16FLOAT,
    RG8UNORM,
    RG8SNORM,
    RG8UINT,
    RG8SINT,
    R32UINT,
    R32SINT,
    R32FLOAT,
    RG16UNORM,
    RG16SNORM,
    RG16UINT,
    RG16SINT,
    RG16FLOAT,
    RGBA8UNORM,
    RGBA8UNORM_SRGB,
    RGBA8SNORM,
    RGBA8UINT,
    RGBA8SINT,
    BGRA8UNORM,
    BGRA8UNORM_SRGB,
    RGB9E5UFLOAT,
    RGB10A2UINT,
    RGB10A2UNORM,
    RG11B10UFLOAT,
    RG32UINT,
    RG32SINT,
    RG32FLOAT,
    RGBA16UNORM,
    RGBA16SNORM,
    RGBA16UINT,
    RGBA16SINT,
    RGBA16FLOAT,
    RGBA32UINT,
    RGBA32SINT,
    RGBA32FLOAT,
    STENCIL8,
    DEPTH16UNORM,
    DEPTH24PLUS,
    DEPTH24PLUS_STENCIL8,
    DEPTH32FLOAT,
    DEPTH32FLOAT_STENCIL8,
    BC1_RGBA_UNORM,
    BC1_RGBA_UNORM_SRGB,
    BC2_RGBA_UNORM,
    BC2_RGBA_UNORM_SRGB,
    BC3_RGBA_UNORM,
    BC3_RGBA_UNORM_SRGB,
    BC4_R_UNORM,
    BC4_R_SNORM,
    BC5_RG_UNORM,
    BC5_RG_SNORM,
    BC6H_RGB_UFLOAT,
    BC6H_RGB_FLOAT,
    BC7_RGBA_UNORM,
    BC7_RGBA_UNORM_SRGB,
    ETC2_RGB8UNORM,
    ETC2_RGB8UNORM_SRGB,
    ETC2_RGB8A1UNORM,
    ETC2_RGB8A1UNORM_SRGB,
    ETC2_RGBA8UNORM,
    ETC2_RGBA8UNORM_SRGB,
    EAC_R11UNORM,
    EAC_R11SNORM,
    EAC_RG11UNORM,
    EAC_RG11SNORM,
    ASTC_4X4_UNORM,
    ASTC_4X4_UNORM_SRGB,
    ASTC_5X4_UNORM,
    ASTC_5X4_UNORM_SRGB,
    ASTC_5X5_UNORM,
    ASTC_5X5_UNORM_SRGB,
    ASTC_6X5_UNORM,
    ASTC_6X5_UNORM_SRGB,
    ASTC_6X6_UNORM,
    ASTC_6X6_UNORM_SRGB,
    ASTC_8X5_UNORM,
    ASTC_8X5_UNORM_SRGB,
    ASTC_8X6_UNORM,
    ASTC_8X6_UNORM_SRGB,
    ASTC_8X8_UNORM,
    ASTC_8X8_UNORM_SRGB,
    ASTC_10X5_UNORM,
    ASTC_10X5_UNORM_SRGB,
    ASTC_10X6_UNORM,
    ASTC_10X6_UNORM_SRGB,
    ASTC_10X8_UNORM,
    ASTC_10X8_UNORM_SRGB,
    ASTC_10X10_UNORM,
    ASTC_10X10_UNORM_SRGB,
    ASTC_12X10_UNORM,
    ASTC_12X10_UNORM_SRGB,
    ASTC_12X12_UNORM,
    ASTC_12X12_UNORM_SRGB,
}
impl FromVal for GPUTextureFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "r8unorm" => Self::R8UNORM,
            "r8snorm" => Self::R8SNORM,
            "r8uint" => Self::R8UINT,
            "r8sint" => Self::R8SINT,
            "r16unorm" => Self::R16UNORM,
            "r16snorm" => Self::R16SNORM,
            "r16uint" => Self::R16UINT,
            "r16sint" => Self::R16SINT,
            "r16float" => Self::R16FLOAT,
            "rg8unorm" => Self::RG8UNORM,
            "rg8snorm" => Self::RG8SNORM,
            "rg8uint" => Self::RG8UINT,
            "rg8sint" => Self::RG8SINT,
            "r32uint" => Self::R32UINT,
            "r32sint" => Self::R32SINT,
            "r32float" => Self::R32FLOAT,
            "rg16unorm" => Self::RG16UNORM,
            "rg16snorm" => Self::RG16SNORM,
            "rg16uint" => Self::RG16UINT,
            "rg16sint" => Self::RG16SINT,
            "rg16float" => Self::RG16FLOAT,
            "rgba8unorm" => Self::RGBA8UNORM,
            "rgba8unorm-srgb" => Self::RGBA8UNORM_SRGB,
            "rgba8snorm" => Self::RGBA8SNORM,
            "rgba8uint" => Self::RGBA8UINT,
            "rgba8sint" => Self::RGBA8SINT,
            "bgra8unorm" => Self::BGRA8UNORM,
            "bgra8unorm-srgb" => Self::BGRA8UNORM_SRGB,
            "rgb9e5ufloat" => Self::RGB9E5UFLOAT,
            "rgb10a2uint" => Self::RGB10A2UINT,
            "rgb10a2unorm" => Self::RGB10A2UNORM,
            "rg11b10ufloat" => Self::RG11B10UFLOAT,
            "rg32uint" => Self::RG32UINT,
            "rg32sint" => Self::RG32SINT,
            "rg32float" => Self::RG32FLOAT,
            "rgba16unorm" => Self::RGBA16UNORM,
            "rgba16snorm" => Self::RGBA16SNORM,
            "rgba16uint" => Self::RGBA16UINT,
            "rgba16sint" => Self::RGBA16SINT,
            "rgba16float" => Self::RGBA16FLOAT,
            "rgba32uint" => Self::RGBA32UINT,
            "rgba32sint" => Self::RGBA32SINT,
            "rgba32float" => Self::RGBA32FLOAT,
            "stencil8" => Self::STENCIL8,
            "depth16unorm" => Self::DEPTH16UNORM,
            "depth24plus" => Self::DEPTH24PLUS,
            "depth24plus-stencil8" => Self::DEPTH24PLUS_STENCIL8,
            "depth32float" => Self::DEPTH32FLOAT,
            "depth32float-stencil8" => Self::DEPTH32FLOAT_STENCIL8,
            "bc1-rgba-unorm" => Self::BC1_RGBA_UNORM,
            "bc1-rgba-unorm-srgb" => Self::BC1_RGBA_UNORM_SRGB,
            "bc2-rgba-unorm" => Self::BC2_RGBA_UNORM,
            "bc2-rgba-unorm-srgb" => Self::BC2_RGBA_UNORM_SRGB,
            "bc3-rgba-unorm" => Self::BC3_RGBA_UNORM,
            "bc3-rgba-unorm-srgb" => Self::BC3_RGBA_UNORM_SRGB,
            "bc4-r-unorm" => Self::BC4_R_UNORM,
            "bc4-r-snorm" => Self::BC4_R_SNORM,
            "bc5-rg-unorm" => Self::BC5_RG_UNORM,
            "bc5-rg-snorm" => Self::BC5_RG_SNORM,
            "bc6h-rgb-ufloat" => Self::BC6H_RGB_UFLOAT,
            "bc6h-rgb-float" => Self::BC6H_RGB_FLOAT,
            "bc7-rgba-unorm" => Self::BC7_RGBA_UNORM,
            "bc7-rgba-unorm-srgb" => Self::BC7_RGBA_UNORM_SRGB,
            "etc2-rgb8unorm" => Self::ETC2_RGB8UNORM,
            "etc2-rgb8unorm-srgb" => Self::ETC2_RGB8UNORM_SRGB,
            "etc2-rgb8a1unorm" => Self::ETC2_RGB8A1UNORM,
            "etc2-rgb8a1unorm-srgb" => Self::ETC2_RGB8A1UNORM_SRGB,
            "etc2-rgba8unorm" => Self::ETC2_RGBA8UNORM,
            "etc2-rgba8unorm-srgb" => Self::ETC2_RGBA8UNORM_SRGB,
            "eac-r11unorm" => Self::EAC_R11UNORM,
            "eac-r11snorm" => Self::EAC_R11SNORM,
            "eac-rg11unorm" => Self::EAC_RG11UNORM,
            "eac-rg11snorm" => Self::EAC_RG11SNORM,
            "astc-4x4-unorm" => Self::ASTC_4X4_UNORM,
            "astc-4x4-unorm-srgb" => Self::ASTC_4X4_UNORM_SRGB,
            "astc-5x4-unorm" => Self::ASTC_5X4_UNORM,
            "astc-5x4-unorm-srgb" => Self::ASTC_5X4_UNORM_SRGB,
            "astc-5x5-unorm" => Self::ASTC_5X5_UNORM,
            "astc-5x5-unorm-srgb" => Self::ASTC_5X5_UNORM_SRGB,
            "astc-6x5-unorm" => Self::ASTC_6X5_UNORM,
            "astc-6x5-unorm-srgb" => Self::ASTC_6X5_UNORM_SRGB,
            "astc-6x6-unorm" => Self::ASTC_6X6_UNORM,
            "astc-6x6-unorm-srgb" => Self::ASTC_6X6_UNORM_SRGB,
            "astc-8x5-unorm" => Self::ASTC_8X5_UNORM,
            "astc-8x5-unorm-srgb" => Self::ASTC_8X5_UNORM_SRGB,
            "astc-8x6-unorm" => Self::ASTC_8X6_UNORM,
            "astc-8x6-unorm-srgb" => Self::ASTC_8X6_UNORM_SRGB,
            "astc-8x8-unorm" => Self::ASTC_8X8_UNORM,
            "astc-8x8-unorm-srgb" => Self::ASTC_8X8_UNORM_SRGB,
            "astc-10x5-unorm" => Self::ASTC_10X5_UNORM,
            "astc-10x5-unorm-srgb" => Self::ASTC_10X5_UNORM_SRGB,
            "astc-10x6-unorm" => Self::ASTC_10X6_UNORM,
            "astc-10x6-unorm-srgb" => Self::ASTC_10X6_UNORM_SRGB,
            "astc-10x8-unorm" => Self::ASTC_10X8_UNORM,
            "astc-10x8-unorm-srgb" => Self::ASTC_10X8_UNORM_SRGB,
            "astc-10x10-unorm" => Self::ASTC_10X10_UNORM,
            "astc-10x10-unorm-srgb" => Self::ASTC_10X10_UNORM_SRGB,
            "astc-12x10-unorm" => Self::ASTC_12X10_UNORM,
            "astc-12x10-unorm-srgb" => Self::ASTC_12X10_UNORM_SRGB,
            "astc-12x12-unorm" => Self::ASTC_12X12_UNORM,
            "astc-12x12-unorm-srgb" => Self::ASTC_12X12_UNORM_SRGB,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUTextureFormat> for emlite::Val {
    fn from(s: GPUTextureFormat) -> emlite::Val {
        match s {
            GPUTextureFormat::R8UNORM => emlite::Val::from("r8unorm"),
            GPUTextureFormat::R8SNORM => emlite::Val::from("r8snorm"),
            GPUTextureFormat::R8UINT => emlite::Val::from("r8uint"),
            GPUTextureFormat::R8SINT => emlite::Val::from("r8sint"),
            GPUTextureFormat::R16UNORM => emlite::Val::from("r16unorm"),
            GPUTextureFormat::R16SNORM => emlite::Val::from("r16snorm"),
            GPUTextureFormat::R16UINT => emlite::Val::from("r16uint"),
            GPUTextureFormat::R16SINT => emlite::Val::from("r16sint"),
            GPUTextureFormat::R16FLOAT => emlite::Val::from("r16float"),
            GPUTextureFormat::RG8UNORM => emlite::Val::from("rg8unorm"),
            GPUTextureFormat::RG8SNORM => emlite::Val::from("rg8snorm"),
            GPUTextureFormat::RG8UINT => emlite::Val::from("rg8uint"),
            GPUTextureFormat::RG8SINT => emlite::Val::from("rg8sint"),
            GPUTextureFormat::R32UINT => emlite::Val::from("r32uint"),
            GPUTextureFormat::R32SINT => emlite::Val::from("r32sint"),
            GPUTextureFormat::R32FLOAT => emlite::Val::from("r32float"),
            GPUTextureFormat::RG16UNORM => emlite::Val::from("rg16unorm"),
            GPUTextureFormat::RG16SNORM => emlite::Val::from("rg16snorm"),
            GPUTextureFormat::RG16UINT => emlite::Val::from("rg16uint"),
            GPUTextureFormat::RG16SINT => emlite::Val::from("rg16sint"),
            GPUTextureFormat::RG16FLOAT => emlite::Val::from("rg16float"),
            GPUTextureFormat::RGBA8UNORM => emlite::Val::from("rgba8unorm"),
            GPUTextureFormat::RGBA8UNORM_SRGB => emlite::Val::from("rgba8unorm-srgb"),
            GPUTextureFormat::RGBA8SNORM => emlite::Val::from("rgba8snorm"),
            GPUTextureFormat::RGBA8UINT => emlite::Val::from("rgba8uint"),
            GPUTextureFormat::RGBA8SINT => emlite::Val::from("rgba8sint"),
            GPUTextureFormat::BGRA8UNORM => emlite::Val::from("bgra8unorm"),
            GPUTextureFormat::BGRA8UNORM_SRGB => emlite::Val::from("bgra8unorm-srgb"),
            GPUTextureFormat::RGB9E5UFLOAT => emlite::Val::from("rgb9e5ufloat"),
            GPUTextureFormat::RGB10A2UINT => emlite::Val::from("rgb10a2uint"),
            GPUTextureFormat::RGB10A2UNORM => emlite::Val::from("rgb10a2unorm"),
            GPUTextureFormat::RG11B10UFLOAT => emlite::Val::from("rg11b10ufloat"),
            GPUTextureFormat::RG32UINT => emlite::Val::from("rg32uint"),
            GPUTextureFormat::RG32SINT => emlite::Val::from("rg32sint"),
            GPUTextureFormat::RG32FLOAT => emlite::Val::from("rg32float"),
            GPUTextureFormat::RGBA16UNORM => emlite::Val::from("rgba16unorm"),
            GPUTextureFormat::RGBA16SNORM => emlite::Val::from("rgba16snorm"),
            GPUTextureFormat::RGBA16UINT => emlite::Val::from("rgba16uint"),
            GPUTextureFormat::RGBA16SINT => emlite::Val::from("rgba16sint"),
            GPUTextureFormat::RGBA16FLOAT => emlite::Val::from("rgba16float"),
            GPUTextureFormat::RGBA32UINT => emlite::Val::from("rgba32uint"),
            GPUTextureFormat::RGBA32SINT => emlite::Val::from("rgba32sint"),
            GPUTextureFormat::RGBA32FLOAT => emlite::Val::from("rgba32float"),
            GPUTextureFormat::STENCIL8 => emlite::Val::from("stencil8"),
            GPUTextureFormat::DEPTH16UNORM => emlite::Val::from("depth16unorm"),
            GPUTextureFormat::DEPTH24PLUS => emlite::Val::from("depth24plus"),
            GPUTextureFormat::DEPTH24PLUS_STENCIL8 => emlite::Val::from("depth24plus-stencil8"),
            GPUTextureFormat::DEPTH32FLOAT => emlite::Val::from("depth32float"),
            GPUTextureFormat::DEPTH32FLOAT_STENCIL8 => emlite::Val::from("depth32float-stencil8"),
            GPUTextureFormat::BC1_RGBA_UNORM => emlite::Val::from("bc1-rgba-unorm"),
            GPUTextureFormat::BC1_RGBA_UNORM_SRGB => emlite::Val::from("bc1-rgba-unorm-srgb"),
            GPUTextureFormat::BC2_RGBA_UNORM => emlite::Val::from("bc2-rgba-unorm"),
            GPUTextureFormat::BC2_RGBA_UNORM_SRGB => emlite::Val::from("bc2-rgba-unorm-srgb"),
            GPUTextureFormat::BC3_RGBA_UNORM => emlite::Val::from("bc3-rgba-unorm"),
            GPUTextureFormat::BC3_RGBA_UNORM_SRGB => emlite::Val::from("bc3-rgba-unorm-srgb"),
            GPUTextureFormat::BC4_R_UNORM => emlite::Val::from("bc4-r-unorm"),
            GPUTextureFormat::BC4_R_SNORM => emlite::Val::from("bc4-r-snorm"),
            GPUTextureFormat::BC5_RG_UNORM => emlite::Val::from("bc5-rg-unorm"),
            GPUTextureFormat::BC5_RG_SNORM => emlite::Val::from("bc5-rg-snorm"),
            GPUTextureFormat::BC6H_RGB_UFLOAT => emlite::Val::from("bc6h-rgb-ufloat"),
            GPUTextureFormat::BC6H_RGB_FLOAT => emlite::Val::from("bc6h-rgb-float"),
            GPUTextureFormat::BC7_RGBA_UNORM => emlite::Val::from("bc7-rgba-unorm"),
            GPUTextureFormat::BC7_RGBA_UNORM_SRGB => emlite::Val::from("bc7-rgba-unorm-srgb"),
            GPUTextureFormat::ETC2_RGB8UNORM => emlite::Val::from("etc2-rgb8unorm"),
            GPUTextureFormat::ETC2_RGB8UNORM_SRGB => emlite::Val::from("etc2-rgb8unorm-srgb"),
            GPUTextureFormat::ETC2_RGB8A1UNORM => emlite::Val::from("etc2-rgb8a1unorm"),
            GPUTextureFormat::ETC2_RGB8A1UNORM_SRGB => emlite::Val::from("etc2-rgb8a1unorm-srgb"),
            GPUTextureFormat::ETC2_RGBA8UNORM => emlite::Val::from("etc2-rgba8unorm"),
            GPUTextureFormat::ETC2_RGBA8UNORM_SRGB => emlite::Val::from("etc2-rgba8unorm-srgb"),
            GPUTextureFormat::EAC_R11UNORM => emlite::Val::from("eac-r11unorm"),
            GPUTextureFormat::EAC_R11SNORM => emlite::Val::from("eac-r11snorm"),
            GPUTextureFormat::EAC_RG11UNORM => emlite::Val::from("eac-rg11unorm"),
            GPUTextureFormat::EAC_RG11SNORM => emlite::Val::from("eac-rg11snorm"),
            GPUTextureFormat::ASTC_4X4_UNORM => emlite::Val::from("astc-4x4-unorm"),
            GPUTextureFormat::ASTC_4X4_UNORM_SRGB => emlite::Val::from("astc-4x4-unorm-srgb"),
            GPUTextureFormat::ASTC_5X4_UNORM => emlite::Val::from("astc-5x4-unorm"),
            GPUTextureFormat::ASTC_5X4_UNORM_SRGB => emlite::Val::from("astc-5x4-unorm-srgb"),
            GPUTextureFormat::ASTC_5X5_UNORM => emlite::Val::from("astc-5x5-unorm"),
            GPUTextureFormat::ASTC_5X5_UNORM_SRGB => emlite::Val::from("astc-5x5-unorm-srgb"),
            GPUTextureFormat::ASTC_6X5_UNORM => emlite::Val::from("astc-6x5-unorm"),
            GPUTextureFormat::ASTC_6X5_UNORM_SRGB => emlite::Val::from("astc-6x5-unorm-srgb"),
            GPUTextureFormat::ASTC_6X6_UNORM => emlite::Val::from("astc-6x6-unorm"),
            GPUTextureFormat::ASTC_6X6_UNORM_SRGB => emlite::Val::from("astc-6x6-unorm-srgb"),
            GPUTextureFormat::ASTC_8X5_UNORM => emlite::Val::from("astc-8x5-unorm"),
            GPUTextureFormat::ASTC_8X5_UNORM_SRGB => emlite::Val::from("astc-8x5-unorm-srgb"),
            GPUTextureFormat::ASTC_8X6_UNORM => emlite::Val::from("astc-8x6-unorm"),
            GPUTextureFormat::ASTC_8X6_UNORM_SRGB => emlite::Val::from("astc-8x6-unorm-srgb"),
            GPUTextureFormat::ASTC_8X8_UNORM => emlite::Val::from("astc-8x8-unorm"),
            GPUTextureFormat::ASTC_8X8_UNORM_SRGB => emlite::Val::from("astc-8x8-unorm-srgb"),
            GPUTextureFormat::ASTC_10X5_UNORM => emlite::Val::from("astc-10x5-unorm"),
            GPUTextureFormat::ASTC_10X5_UNORM_SRGB => emlite::Val::from("astc-10x5-unorm-srgb"),
            GPUTextureFormat::ASTC_10X6_UNORM => emlite::Val::from("astc-10x6-unorm"),
            GPUTextureFormat::ASTC_10X6_UNORM_SRGB => emlite::Val::from("astc-10x6-unorm-srgb"),
            GPUTextureFormat::ASTC_10X8_UNORM => emlite::Val::from("astc-10x8-unorm"),
            GPUTextureFormat::ASTC_10X8_UNORM_SRGB => emlite::Val::from("astc-10x8-unorm-srgb"),
            GPUTextureFormat::ASTC_10X10_UNORM => emlite::Val::from("astc-10x10-unorm"),
            GPUTextureFormat::ASTC_10X10_UNORM_SRGB => emlite::Val::from("astc-10x10-unorm-srgb"),
            GPUTextureFormat::ASTC_12X10_UNORM => emlite::Val::from("astc-12x10-unorm"),
            GPUTextureFormat::ASTC_12X10_UNORM_SRGB => emlite::Val::from("astc-12x10-unorm-srgb"),
            GPUTextureFormat::ASTC_12X12_UNORM => emlite::Val::from("astc-12x12-unorm"),
            GPUTextureFormat::ASTC_12X12_UNORM_SRGB => emlite::Val::from("astc-12x12-unorm-srgb"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUAddressMode {
    CLAMP_TO_EDGE,
    REPEAT,
    MIRROR_REPEAT,
}
impl FromVal for GPUAddressMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "clamp-to-edge" => Self::CLAMP_TO_EDGE,
            "repeat" => Self::REPEAT,
            "mirror-repeat" => Self::MIRROR_REPEAT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUAddressMode> for emlite::Val {
    fn from(s: GPUAddressMode) -> emlite::Val {
        match s {
            GPUAddressMode::CLAMP_TO_EDGE => emlite::Val::from("clamp-to-edge"),
            GPUAddressMode::REPEAT => emlite::Val::from("repeat"),
            GPUAddressMode::MIRROR_REPEAT => emlite::Val::from("mirror-repeat"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUFilterMode {
    NEAREST,
    LINEAR,
}
impl FromVal for GPUFilterMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "nearest" => Self::NEAREST,
            "linear" => Self::LINEAR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUFilterMode> for emlite::Val {
    fn from(s: GPUFilterMode) -> emlite::Val {
        match s {
            GPUFilterMode::NEAREST => emlite::Val::from("nearest"),
            GPUFilterMode::LINEAR => emlite::Val::from("linear"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUMipmapFilterMode {
    NEAREST,
    LINEAR,
}
impl FromVal for GPUMipmapFilterMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "nearest" => Self::NEAREST,
            "linear" => Self::LINEAR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUMipmapFilterMode> for emlite::Val {
    fn from(s: GPUMipmapFilterMode) -> emlite::Val {
        match s {
            GPUMipmapFilterMode::NEAREST => emlite::Val::from("nearest"),
            GPUMipmapFilterMode::LINEAR => emlite::Val::from("linear"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUCompareFunction {
    NEVER,
    LESS,
    EQUAL,
    LESS_EQUAL,
    GREATER,
    NOT_EQUAL,
    GREATER_EQUAL,
    ALWAYS,
}
impl FromVal for GPUCompareFunction {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "never" => Self::NEVER,
            "less" => Self::LESS,
            "equal" => Self::EQUAL,
            "less-equal" => Self::LESS_EQUAL,
            "greater" => Self::GREATER,
            "not-equal" => Self::NOT_EQUAL,
            "greater-equal" => Self::GREATER_EQUAL,
            "always" => Self::ALWAYS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUCompareFunction> for emlite::Val {
    fn from(s: GPUCompareFunction) -> emlite::Val {
        match s {
            GPUCompareFunction::NEVER => emlite::Val::from("never"),
            GPUCompareFunction::LESS => emlite::Val::from("less"),
            GPUCompareFunction::EQUAL => emlite::Val::from("equal"),
            GPUCompareFunction::LESS_EQUAL => emlite::Val::from("less-equal"),
            GPUCompareFunction::GREATER => emlite::Val::from("greater"),
            GPUCompareFunction::NOT_EQUAL => emlite::Val::from("not-equal"),
            GPUCompareFunction::GREATER_EQUAL => emlite::Val::from("greater-equal"),
            GPUCompareFunction::ALWAYS => emlite::Val::from("always"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUBufferBindingType {
    UNIFORM,
    STORAGE,
    READ_ONLY_STORAGE,
}
impl FromVal for GPUBufferBindingType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "uniform" => Self::UNIFORM,
            "storage" => Self::STORAGE,
            "read-only-storage" => Self::READ_ONLY_STORAGE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUBufferBindingType> for emlite::Val {
    fn from(s: GPUBufferBindingType) -> emlite::Val {
        match s {
            GPUBufferBindingType::UNIFORM => emlite::Val::from("uniform"),
            GPUBufferBindingType::STORAGE => emlite::Val::from("storage"),
            GPUBufferBindingType::READ_ONLY_STORAGE => emlite::Val::from("read-only-storage"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUSamplerBindingType {
    FILTERING,
    NON_FILTERING,
    COMPARISON,
}
impl FromVal for GPUSamplerBindingType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "filtering" => Self::FILTERING,
            "non-filtering" => Self::NON_FILTERING,
            "comparison" => Self::COMPARISON,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUSamplerBindingType> for emlite::Val {
    fn from(s: GPUSamplerBindingType) -> emlite::Val {
        match s {
            GPUSamplerBindingType::FILTERING => emlite::Val::from("filtering"),
            GPUSamplerBindingType::NON_FILTERING => emlite::Val::from("non-filtering"),
            GPUSamplerBindingType::COMPARISON => emlite::Val::from("comparison"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUTextureSampleType {
    FLOAT,
    UNFILTERABLE_FLOAT,
    DEPTH,
    SINT,
    UINT,
}
impl FromVal for GPUTextureSampleType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "float" => Self::FLOAT,
            "unfilterable-float" => Self::UNFILTERABLE_FLOAT,
            "depth" => Self::DEPTH,
            "sint" => Self::SINT,
            "uint" => Self::UINT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUTextureSampleType> for emlite::Val {
    fn from(s: GPUTextureSampleType) -> emlite::Val {
        match s {
            GPUTextureSampleType::FLOAT => emlite::Val::from("float"),
            GPUTextureSampleType::UNFILTERABLE_FLOAT => emlite::Val::from("unfilterable-float"),
            GPUTextureSampleType::DEPTH => emlite::Val::from("depth"),
            GPUTextureSampleType::SINT => emlite::Val::from("sint"),
            GPUTextureSampleType::UINT => emlite::Val::from("uint"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUStorageTextureAccess {
    WRITE_ONLY,
    READ_ONLY,
    READ_WRITE,
}
impl FromVal for GPUStorageTextureAccess {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "write-only" => Self::WRITE_ONLY,
            "read-only" => Self::READ_ONLY,
            "read-write" => Self::READ_WRITE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUStorageTextureAccess> for emlite::Val {
    fn from(s: GPUStorageTextureAccess) -> emlite::Val {
        match s {
            GPUStorageTextureAccess::WRITE_ONLY => emlite::Val::from("write-only"),
            GPUStorageTextureAccess::READ_ONLY => emlite::Val::from("read-only"),
            GPUStorageTextureAccess::READ_WRITE => emlite::Val::from("read-write"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUCompilationMessageType {
    ERROR,
    WARNING,
    INFO,
}
impl FromVal for GPUCompilationMessageType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "error" => Self::ERROR,
            "warning" => Self::WARNING,
            "info" => Self::INFO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUCompilationMessageType> for emlite::Val {
    fn from(s: GPUCompilationMessageType) -> emlite::Val {
        match s {
            GPUCompilationMessageType::ERROR => emlite::Val::from("error"),
            GPUCompilationMessageType::WARNING => emlite::Val::from("warning"),
            GPUCompilationMessageType::INFO => emlite::Val::from("info"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUPipelineErrorReason {
    VALIDATION,
    INTERNAL,
}
impl FromVal for GPUPipelineErrorReason {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "validation" => Self::VALIDATION,
            "internal" => Self::INTERNAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUPipelineErrorReason> for emlite::Val {
    fn from(s: GPUPipelineErrorReason) -> emlite::Val {
        match s {
            GPUPipelineErrorReason::VALIDATION => emlite::Val::from("validation"),
            GPUPipelineErrorReason::INTERNAL => emlite::Val::from("internal"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUAutoLayoutMode {
    AUTO,
}
impl FromVal for GPUAutoLayoutMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUAutoLayoutMode> for emlite::Val {
    fn from(s: GPUAutoLayoutMode) -> emlite::Val {
        match s {
            GPUAutoLayoutMode::AUTO => emlite::Val::from("auto"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUPrimitiveTopology {
    POINT_LIST,
    LINE_LIST,
    LINE_STRIP,
    TRIANGLE_LIST,
    TRIANGLE_STRIP,
}
impl FromVal for GPUPrimitiveTopology {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "point-list" => Self::POINT_LIST,
            "line-list" => Self::LINE_LIST,
            "line-strip" => Self::LINE_STRIP,
            "triangle-list" => Self::TRIANGLE_LIST,
            "triangle-strip" => Self::TRIANGLE_STRIP,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUPrimitiveTopology> for emlite::Val {
    fn from(s: GPUPrimitiveTopology) -> emlite::Val {
        match s {
            GPUPrimitiveTopology::POINT_LIST => emlite::Val::from("point-list"),
            GPUPrimitiveTopology::LINE_LIST => emlite::Val::from("line-list"),
            GPUPrimitiveTopology::LINE_STRIP => emlite::Val::from("line-strip"),
            GPUPrimitiveTopology::TRIANGLE_LIST => emlite::Val::from("triangle-list"),
            GPUPrimitiveTopology::TRIANGLE_STRIP => emlite::Val::from("triangle-strip"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUFrontFace {
    CCW,
    CW,
}
impl FromVal for GPUFrontFace {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "ccw" => Self::CCW,
            "cw" => Self::CW,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUFrontFace> for emlite::Val {
    fn from(s: GPUFrontFace) -> emlite::Val {
        match s {
            GPUFrontFace::CCW => emlite::Val::from("ccw"),
            GPUFrontFace::CW => emlite::Val::from("cw"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUCullMode {
    NONE,
    FRONT,
    BACK,
}
impl FromVal for GPUCullMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "front" => Self::FRONT,
            "back" => Self::BACK,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUCullMode> for emlite::Val {
    fn from(s: GPUCullMode) -> emlite::Val {
        match s {
            GPUCullMode::NONE => emlite::Val::from("none"),
            GPUCullMode::FRONT => emlite::Val::from("front"),
            GPUCullMode::BACK => emlite::Val::from("back"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUBlendFactor {
    ZERO,
    ONE,
    SRC,
    ONE_MINUS_SRC,
    SRC_ALPHA,
    ONE_MINUS_SRC_ALPHA,
    DST,
    ONE_MINUS_DST,
    DST_ALPHA,
    ONE_MINUS_DST_ALPHA,
    SRC_ALPHA_SATURATED,
    CONSTANT,
    ONE_MINUS_CONSTANT,
    SRC1,
    ONE_MINUS_SRC1,
    SRC1_ALPHA,
    ONE_MINUS_SRC1_ALPHA,
}
impl FromVal for GPUBlendFactor {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "zero" => Self::ZERO,
            "one" => Self::ONE,
            "src" => Self::SRC,
            "one-minus-src" => Self::ONE_MINUS_SRC,
            "src-alpha" => Self::SRC_ALPHA,
            "one-minus-src-alpha" => Self::ONE_MINUS_SRC_ALPHA,
            "dst" => Self::DST,
            "one-minus-dst" => Self::ONE_MINUS_DST,
            "dst-alpha" => Self::DST_ALPHA,
            "one-minus-dst-alpha" => Self::ONE_MINUS_DST_ALPHA,
            "src-alpha-saturated" => Self::SRC_ALPHA_SATURATED,
            "constant" => Self::CONSTANT,
            "one-minus-constant" => Self::ONE_MINUS_CONSTANT,
            "src1" => Self::SRC1,
            "one-minus-src1" => Self::ONE_MINUS_SRC1,
            "src1-alpha" => Self::SRC1_ALPHA,
            "one-minus-src1-alpha" => Self::ONE_MINUS_SRC1_ALPHA,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUBlendFactor> for emlite::Val {
    fn from(s: GPUBlendFactor) -> emlite::Val {
        match s {
            GPUBlendFactor::ZERO => emlite::Val::from("zero"),
            GPUBlendFactor::ONE => emlite::Val::from("one"),
            GPUBlendFactor::SRC => emlite::Val::from("src"),
            GPUBlendFactor::ONE_MINUS_SRC => emlite::Val::from("one-minus-src"),
            GPUBlendFactor::SRC_ALPHA => emlite::Val::from("src-alpha"),
            GPUBlendFactor::ONE_MINUS_SRC_ALPHA => emlite::Val::from("one-minus-src-alpha"),
            GPUBlendFactor::DST => emlite::Val::from("dst"),
            GPUBlendFactor::ONE_MINUS_DST => emlite::Val::from("one-minus-dst"),
            GPUBlendFactor::DST_ALPHA => emlite::Val::from("dst-alpha"),
            GPUBlendFactor::ONE_MINUS_DST_ALPHA => emlite::Val::from("one-minus-dst-alpha"),
            GPUBlendFactor::SRC_ALPHA_SATURATED => emlite::Val::from("src-alpha-saturated"),
            GPUBlendFactor::CONSTANT => emlite::Val::from("constant"),
            GPUBlendFactor::ONE_MINUS_CONSTANT => emlite::Val::from("one-minus-constant"),
            GPUBlendFactor::SRC1 => emlite::Val::from("src1"),
            GPUBlendFactor::ONE_MINUS_SRC1 => emlite::Val::from("one-minus-src1"),
            GPUBlendFactor::SRC1_ALPHA => emlite::Val::from("src1-alpha"),
            GPUBlendFactor::ONE_MINUS_SRC1_ALPHA => emlite::Val::from("one-minus-src1-alpha"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUBlendOperation {
    ADD,
    SUBTRACT,
    REVERSE_SUBTRACT,
    MIN,
    MAX,
}
impl FromVal for GPUBlendOperation {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "add" => Self::ADD,
            "subtract" => Self::SUBTRACT,
            "reverse-subtract" => Self::REVERSE_SUBTRACT,
            "min" => Self::MIN,
            "max" => Self::MAX,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUBlendOperation> for emlite::Val {
    fn from(s: GPUBlendOperation) -> emlite::Val {
        match s {
            GPUBlendOperation::ADD => emlite::Val::from("add"),
            GPUBlendOperation::SUBTRACT => emlite::Val::from("subtract"),
            GPUBlendOperation::REVERSE_SUBTRACT => emlite::Val::from("reverse-subtract"),
            GPUBlendOperation::MIN => emlite::Val::from("min"),
            GPUBlendOperation::MAX => emlite::Val::from("max"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUStencilOperation {
    KEEP,
    ZERO,
    REPLACE,
    INVERT,
    INCREMENT_CLAMP,
    DECREMENT_CLAMP,
    INCREMENT_WRAP,
    DECREMENT_WRAP,
}
impl FromVal for GPUStencilOperation {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "keep" => Self::KEEP,
            "zero" => Self::ZERO,
            "replace" => Self::REPLACE,
            "invert" => Self::INVERT,
            "increment-clamp" => Self::INCREMENT_CLAMP,
            "decrement-clamp" => Self::DECREMENT_CLAMP,
            "increment-wrap" => Self::INCREMENT_WRAP,
            "decrement-wrap" => Self::DECREMENT_WRAP,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUStencilOperation> for emlite::Val {
    fn from(s: GPUStencilOperation) -> emlite::Val {
        match s {
            GPUStencilOperation::KEEP => emlite::Val::from("keep"),
            GPUStencilOperation::ZERO => emlite::Val::from("zero"),
            GPUStencilOperation::REPLACE => emlite::Val::from("replace"),
            GPUStencilOperation::INVERT => emlite::Val::from("invert"),
            GPUStencilOperation::INCREMENT_CLAMP => emlite::Val::from("increment-clamp"),
            GPUStencilOperation::DECREMENT_CLAMP => emlite::Val::from("decrement-clamp"),
            GPUStencilOperation::INCREMENT_WRAP => emlite::Val::from("increment-wrap"),
            GPUStencilOperation::DECREMENT_WRAP => emlite::Val::from("decrement-wrap"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUIndexFormat {
    UINT16,
    UINT32,
}
impl FromVal for GPUIndexFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "uint16" => Self::UINT16,
            "uint32" => Self::UINT32,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUIndexFormat> for emlite::Val {
    fn from(s: GPUIndexFormat) -> emlite::Val {
        match s {
            GPUIndexFormat::UINT16 => emlite::Val::from("uint16"),
            GPUIndexFormat::UINT32 => emlite::Val::from("uint32"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUVertexFormat {
    UINT8,
    UINT8X2,
    UINT8X4,
    SINT8,
    SINT8X2,
    SINT8X4,
    UNORM8,
    UNORM8X2,
    UNORM8X4,
    SNORM8,
    SNORM8X2,
    SNORM8X4,
    UINT16,
    UINT16X2,
    UINT16X4,
    SINT16,
    SINT16X2,
    SINT16X4,
    UNORM16,
    UNORM16X2,
    UNORM16X4,
    SNORM16,
    SNORM16X2,
    SNORM16X4,
    FLOAT16,
    FLOAT16X2,
    FLOAT16X4,
    FLOAT32,
    FLOAT32X2,
    FLOAT32X3,
    FLOAT32X4,
    UINT32,
    UINT32X2,
    UINT32X3,
    UINT32X4,
    SINT32,
    SINT32X2,
    SINT32X3,
    SINT32X4,
    UNORM10_10_10_2,
    UNORM8X4_BGRA,
}
impl FromVal for GPUVertexFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "uint8" => Self::UINT8,
            "uint8x2" => Self::UINT8X2,
            "uint8x4" => Self::UINT8X4,
            "sint8" => Self::SINT8,
            "sint8x2" => Self::SINT8X2,
            "sint8x4" => Self::SINT8X4,
            "unorm8" => Self::UNORM8,
            "unorm8x2" => Self::UNORM8X2,
            "unorm8x4" => Self::UNORM8X4,
            "snorm8" => Self::SNORM8,
            "snorm8x2" => Self::SNORM8X2,
            "snorm8x4" => Self::SNORM8X4,
            "uint16" => Self::UINT16,
            "uint16x2" => Self::UINT16X2,
            "uint16x4" => Self::UINT16X4,
            "sint16" => Self::SINT16,
            "sint16x2" => Self::SINT16X2,
            "sint16x4" => Self::SINT16X4,
            "unorm16" => Self::UNORM16,
            "unorm16x2" => Self::UNORM16X2,
            "unorm16x4" => Self::UNORM16X4,
            "snorm16" => Self::SNORM16,
            "snorm16x2" => Self::SNORM16X2,
            "snorm16x4" => Self::SNORM16X4,
            "float16" => Self::FLOAT16,
            "float16x2" => Self::FLOAT16X2,
            "float16x4" => Self::FLOAT16X4,
            "float32" => Self::FLOAT32,
            "float32x2" => Self::FLOAT32X2,
            "float32x3" => Self::FLOAT32X3,
            "float32x4" => Self::FLOAT32X4,
            "uint32" => Self::UINT32,
            "uint32x2" => Self::UINT32X2,
            "uint32x3" => Self::UINT32X3,
            "uint32x4" => Self::UINT32X4,
            "sint32" => Self::SINT32,
            "sint32x2" => Self::SINT32X2,
            "sint32x3" => Self::SINT32X3,
            "sint32x4" => Self::SINT32X4,
            "unorm10-10-10-2" => Self::UNORM10_10_10_2,
            "unorm8x4-bgra" => Self::UNORM8X4_BGRA,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUVertexFormat> for emlite::Val {
    fn from(s: GPUVertexFormat) -> emlite::Val {
        match s {
            GPUVertexFormat::UINT8 => emlite::Val::from("uint8"),
            GPUVertexFormat::UINT8X2 => emlite::Val::from("uint8x2"),
            GPUVertexFormat::UINT8X4 => emlite::Val::from("uint8x4"),
            GPUVertexFormat::SINT8 => emlite::Val::from("sint8"),
            GPUVertexFormat::SINT8X2 => emlite::Val::from("sint8x2"),
            GPUVertexFormat::SINT8X4 => emlite::Val::from("sint8x4"),
            GPUVertexFormat::UNORM8 => emlite::Val::from("unorm8"),
            GPUVertexFormat::UNORM8X2 => emlite::Val::from("unorm8x2"),
            GPUVertexFormat::UNORM8X4 => emlite::Val::from("unorm8x4"),
            GPUVertexFormat::SNORM8 => emlite::Val::from("snorm8"),
            GPUVertexFormat::SNORM8X2 => emlite::Val::from("snorm8x2"),
            GPUVertexFormat::SNORM8X4 => emlite::Val::from("snorm8x4"),
            GPUVertexFormat::UINT16 => emlite::Val::from("uint16"),
            GPUVertexFormat::UINT16X2 => emlite::Val::from("uint16x2"),
            GPUVertexFormat::UINT16X4 => emlite::Val::from("uint16x4"),
            GPUVertexFormat::SINT16 => emlite::Val::from("sint16"),
            GPUVertexFormat::SINT16X2 => emlite::Val::from("sint16x2"),
            GPUVertexFormat::SINT16X4 => emlite::Val::from("sint16x4"),
            GPUVertexFormat::UNORM16 => emlite::Val::from("unorm16"),
            GPUVertexFormat::UNORM16X2 => emlite::Val::from("unorm16x2"),
            GPUVertexFormat::UNORM16X4 => emlite::Val::from("unorm16x4"),
            GPUVertexFormat::SNORM16 => emlite::Val::from("snorm16"),
            GPUVertexFormat::SNORM16X2 => emlite::Val::from("snorm16x2"),
            GPUVertexFormat::SNORM16X4 => emlite::Val::from("snorm16x4"),
            GPUVertexFormat::FLOAT16 => emlite::Val::from("float16"),
            GPUVertexFormat::FLOAT16X2 => emlite::Val::from("float16x2"),
            GPUVertexFormat::FLOAT16X4 => emlite::Val::from("float16x4"),
            GPUVertexFormat::FLOAT32 => emlite::Val::from("float32"),
            GPUVertexFormat::FLOAT32X2 => emlite::Val::from("float32x2"),
            GPUVertexFormat::FLOAT32X3 => emlite::Val::from("float32x3"),
            GPUVertexFormat::FLOAT32X4 => emlite::Val::from("float32x4"),
            GPUVertexFormat::UINT32 => emlite::Val::from("uint32"),
            GPUVertexFormat::UINT32X2 => emlite::Val::from("uint32x2"),
            GPUVertexFormat::UINT32X3 => emlite::Val::from("uint32x3"),
            GPUVertexFormat::UINT32X4 => emlite::Val::from("uint32x4"),
            GPUVertexFormat::SINT32 => emlite::Val::from("sint32"),
            GPUVertexFormat::SINT32X2 => emlite::Val::from("sint32x2"),
            GPUVertexFormat::SINT32X3 => emlite::Val::from("sint32x3"),
            GPUVertexFormat::SINT32X4 => emlite::Val::from("sint32x4"),
            GPUVertexFormat::UNORM10_10_10_2 => emlite::Val::from("unorm10-10-10-2"),
            GPUVertexFormat::UNORM8X4_BGRA => emlite::Val::from("unorm8x4-bgra"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUVertexStepMode {
    VERTEX,
    INSTANCE,
}
impl FromVal for GPUVertexStepMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "vertex" => Self::VERTEX,
            "instance" => Self::INSTANCE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUVertexStepMode> for emlite::Val {
    fn from(s: GPUVertexStepMode) -> emlite::Val {
        match s {
            GPUVertexStepMode::VERTEX => emlite::Val::from("vertex"),
            GPUVertexStepMode::INSTANCE => emlite::Val::from("instance"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPULoadOp {
    LOAD,
    CLEAR,
}
impl FromVal for GPULoadOp {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "load" => Self::LOAD,
            "clear" => Self::CLEAR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPULoadOp> for emlite::Val {
    fn from(s: GPULoadOp) -> emlite::Val {
        match s {
            GPULoadOp::LOAD => emlite::Val::from("load"),
            GPULoadOp::CLEAR => emlite::Val::from("clear"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUStoreOp {
    STORE,
    DISCARD,
}
impl FromVal for GPUStoreOp {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "store" => Self::STORE,
            "discard" => Self::DISCARD,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUStoreOp> for emlite::Val {
    fn from(s: GPUStoreOp) -> emlite::Val {
        match s {
            GPUStoreOp::STORE => emlite::Val::from("store"),
            GPUStoreOp::DISCARD => emlite::Val::from("discard"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUQueryType {
    OCCLUSION,
    TIMESTAMP,
}
impl FromVal for GPUQueryType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "occlusion" => Self::OCCLUSION,
            "timestamp" => Self::TIMESTAMP,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUQueryType> for emlite::Val {
    fn from(s: GPUQueryType) -> emlite::Val {
        match s {
            GPUQueryType::OCCLUSION => emlite::Val::from("occlusion"),
            GPUQueryType::TIMESTAMP => emlite::Val::from("timestamp"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUCanvasAlphaMode {
    OPAQUE,
    PREMULTIPLIED,
}
impl FromVal for GPUCanvasAlphaMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "opaque" => Self::OPAQUE,
            "premultiplied" => Self::PREMULTIPLIED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUCanvasAlphaMode> for emlite::Val {
    fn from(s: GPUCanvasAlphaMode) -> emlite::Val {
        match s {
            GPUCanvasAlphaMode::OPAQUE => emlite::Val::from("opaque"),
            GPUCanvasAlphaMode::PREMULTIPLIED => emlite::Val::from("premultiplied"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUCanvasToneMappingMode {
    STANDARD,
    EXTENDED,
}
impl FromVal for GPUCanvasToneMappingMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "standard" => Self::STANDARD,
            "extended" => Self::EXTENDED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUCanvasToneMappingMode> for emlite::Val {
    fn from(s: GPUCanvasToneMappingMode) -> emlite::Val {
        match s {
            GPUCanvasToneMappingMode::STANDARD => emlite::Val::from("standard"),
            GPUCanvasToneMappingMode::EXTENDED => emlite::Val::from("extended"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUDeviceLostReason {
    UNKNOWN,
    DESTROYED,
}
impl FromVal for GPUDeviceLostReason {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "unknown" => Self::UNKNOWN,
            "destroyed" => Self::DESTROYED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUDeviceLostReason> for emlite::Val {
    fn from(s: GPUDeviceLostReason) -> emlite::Val {
        match s {
            GPUDeviceLostReason::UNKNOWN => emlite::Val::from("unknown"),
            GPUDeviceLostReason::DESTROYED => emlite::Val::from("destroyed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUErrorFilter {
    VALIDATION,
    OUT_OF_MEMORY,
    INTERNAL,
}
impl FromVal for GPUErrorFilter {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "validation" => Self::VALIDATION,
            "out-of-memory" => Self::OUT_OF_MEMORY,
            "internal" => Self::INTERNAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<GPUErrorFilter> for emlite::Val {
    fn from(s: GPUErrorFilter) -> emlite::Val {
        match s {
            GPUErrorFilter::VALIDATION => emlite::Val::from("validation"),
            GPUErrorFilter::OUT_OF_MEMORY => emlite::Val::from("out-of-memory"),
            GPUErrorFilter::INTERNAL => emlite::Val::from("internal"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum HIDUnitSystem {
    NONE,
    SI_LINEAR,
    SI_ROTATION,
    ENGLISH_LINEAR,
    ENGLISH_ROTATION,
    VENDOR_DEFINED,
    RESERVED,
}
impl FromVal for HIDUnitSystem {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "si-linear" => Self::SI_LINEAR,
            "si-rotation" => Self::SI_ROTATION,
            "english-linear" => Self::ENGLISH_LINEAR,
            "english-rotation" => Self::ENGLISH_ROTATION,
            "vendor-defined" => Self::VENDOR_DEFINED,
            "reserved" => Self::RESERVED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<HIDUnitSystem> for emlite::Val {
    fn from(s: HIDUnitSystem) -> emlite::Val {
        match s {
            HIDUnitSystem::NONE => emlite::Val::from("none"),
            HIDUnitSystem::SI_LINEAR => emlite::Val::from("si-linear"),
            HIDUnitSystem::SI_ROTATION => emlite::Val::from("si-rotation"),
            HIDUnitSystem::ENGLISH_LINEAR => emlite::Val::from("english-linear"),
            HIDUnitSystem::ENGLISH_ROTATION => emlite::Val::from("english-rotation"),
            HIDUnitSystem::VENDOR_DEFINED => emlite::Val::from("vendor-defined"),
            HIDUnitSystem::RESERVED => emlite::Val::from("reserved"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MIDIPortType {
    INPUT,
    OUTPUT,
}
impl FromVal for MIDIPortType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "input" => Self::INPUT,
            "output" => Self::OUTPUT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MIDIPortType> for emlite::Val {
    fn from(s: MIDIPortType) -> emlite::Val {
        match s {
            MIDIPortType::INPUT => emlite::Val::from("input"),
            MIDIPortType::OUTPUT => emlite::Val::from("output"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MIDIPortDeviceState {
    DISCONNECTED,
    CONNECTED,
}
impl FromVal for MIDIPortDeviceState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "disconnected" => Self::DISCONNECTED,
            "connected" => Self::CONNECTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MIDIPortDeviceState> for emlite::Val {
    fn from(s: MIDIPortDeviceState) -> emlite::Val {
        match s {
            MIDIPortDeviceState::DISCONNECTED => emlite::Val::from("disconnected"),
            MIDIPortDeviceState::CONNECTED => emlite::Val::from("connected"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MIDIPortConnectionState {
    OPEN,
    CLOSED,
    PENDING,
}
impl FromVal for MIDIPortConnectionState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "open" => Self::OPEN,
            "closed" => Self::CLOSED,
            "pending" => Self::PENDING,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MIDIPortConnectionState> for emlite::Val {
    fn from(s: MIDIPortConnectionState) -> emlite::Val {
        match s {
            MIDIPortConnectionState::OPEN => emlite::Val::from("open"),
            MIDIPortConnectionState::CLOSED => emlite::Val::from("closed"),
            MIDIPortConnectionState::PENDING => emlite::Val::from("pending"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLPowerPreference {
    DEFAULT,
    HIGH_PERFORMANCE,
    LOW_POWER,
}
impl FromVal for MLPowerPreference {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "default" => Self::DEFAULT,
            "high-performance" => Self::HIGH_PERFORMANCE,
            "low-power" => Self::LOW_POWER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLPowerPreference> for emlite::Val {
    fn from(s: MLPowerPreference) -> emlite::Val {
        match s {
            MLPowerPreference::DEFAULT => emlite::Val::from("default"),
            MLPowerPreference::HIGH_PERFORMANCE => emlite::Val::from("high-performance"),
            MLPowerPreference::LOW_POWER => emlite::Val::from("low-power"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLInputOperandLayout {
    NCHW,
    NHWC,
}
impl FromVal for MLInputOperandLayout {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "nchw" => Self::NCHW,
            "nhwc" => Self::NHWC,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLInputOperandLayout> for emlite::Val {
    fn from(s: MLInputOperandLayout) -> emlite::Val {
        match s {
            MLInputOperandLayout::NCHW => emlite::Val::from("nchw"),
            MLInputOperandLayout::NHWC => emlite::Val::from("nhwc"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLOperandDataType {
    FLOAT32,
    FLOAT16,
    INT32,
    UINT32,
    INT64,
    UINT64,
    INT8,
    UINT8,
}
impl FromVal for MLOperandDataType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "float32" => Self::FLOAT32,
            "float16" => Self::FLOAT16,
            "int32" => Self::INT32,
            "uint32" => Self::UINT32,
            "int64" => Self::INT64,
            "uint64" => Self::UINT64,
            "int8" => Self::INT8,
            "uint8" => Self::UINT8,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLOperandDataType> for emlite::Val {
    fn from(s: MLOperandDataType) -> emlite::Val {
        match s {
            MLOperandDataType::FLOAT32 => emlite::Val::from("float32"),
            MLOperandDataType::FLOAT16 => emlite::Val::from("float16"),
            MLOperandDataType::INT32 => emlite::Val::from("int32"),
            MLOperandDataType::UINT32 => emlite::Val::from("uint32"),
            MLOperandDataType::INT64 => emlite::Val::from("int64"),
            MLOperandDataType::UINT64 => emlite::Val::from("uint64"),
            MLOperandDataType::INT8 => emlite::Val::from("int8"),
            MLOperandDataType::UINT8 => emlite::Val::from("uint8"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLConv2dFilterOperandLayout {
    OIHW,
    HWIO,
    OHWI,
    IHWO,
}
impl FromVal for MLConv2dFilterOperandLayout {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "oihw" => Self::OIHW,
            "hwio" => Self::HWIO,
            "ohwi" => Self::OHWI,
            "ihwo" => Self::IHWO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLConv2dFilterOperandLayout> for emlite::Val {
    fn from(s: MLConv2dFilterOperandLayout) -> emlite::Val {
        match s {
            MLConv2dFilterOperandLayout::OIHW => emlite::Val::from("oihw"),
            MLConv2dFilterOperandLayout::HWIO => emlite::Val::from("hwio"),
            MLConv2dFilterOperandLayout::OHWI => emlite::Val::from("ohwi"),
            MLConv2dFilterOperandLayout::IHWO => emlite::Val::from("ihwo"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLConvTranspose2dFilterOperandLayout {
    IOHW,
    HWOI,
    OHWI,
}
impl FromVal for MLConvTranspose2dFilterOperandLayout {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "iohw" => Self::IOHW,
            "hwoi" => Self::HWOI,
            "ohwi" => Self::OHWI,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLConvTranspose2dFilterOperandLayout> for emlite::Val {
    fn from(s: MLConvTranspose2dFilterOperandLayout) -> emlite::Val {
        match s {
            MLConvTranspose2dFilterOperandLayout::IOHW => emlite::Val::from("iohw"),
            MLConvTranspose2dFilterOperandLayout::HWOI => emlite::Val::from("hwoi"),
            MLConvTranspose2dFilterOperandLayout::OHWI => emlite::Val::from("ohwi"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLGruWeightLayout {
    ZRN,
    RZN,
}
impl FromVal for MLGruWeightLayout {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "zrn" => Self::ZRN,
            "rzn" => Self::RZN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLGruWeightLayout> for emlite::Val {
    fn from(s: MLGruWeightLayout) -> emlite::Val {
        match s {
            MLGruWeightLayout::ZRN => emlite::Val::from("zrn"),
            MLGruWeightLayout::RZN => emlite::Val::from("rzn"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLRecurrentNetworkActivation {
    RELU,
    SIGMOID,
    TANH,
}
impl FromVal for MLRecurrentNetworkActivation {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "relu" => Self::RELU,
            "sigmoid" => Self::SIGMOID,
            "tanh" => Self::TANH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLRecurrentNetworkActivation> for emlite::Val {
    fn from(s: MLRecurrentNetworkActivation) -> emlite::Val {
        match s {
            MLRecurrentNetworkActivation::RELU => emlite::Val::from("relu"),
            MLRecurrentNetworkActivation::SIGMOID => emlite::Val::from("sigmoid"),
            MLRecurrentNetworkActivation::TANH => emlite::Val::from("tanh"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLRecurrentNetworkDirection {
    FORWARD,
    BACKWARD,
    BOTH,
}
impl FromVal for MLRecurrentNetworkDirection {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "forward" => Self::FORWARD,
            "backward" => Self::BACKWARD,
            "both" => Self::BOTH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLRecurrentNetworkDirection> for emlite::Val {
    fn from(s: MLRecurrentNetworkDirection) -> emlite::Val {
        match s {
            MLRecurrentNetworkDirection::FORWARD => emlite::Val::from("forward"),
            MLRecurrentNetworkDirection::BACKWARD => emlite::Val::from("backward"),
            MLRecurrentNetworkDirection::BOTH => emlite::Val::from("both"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLLstmWeightLayout {
    IOFG,
    IFGO,
}
impl FromVal for MLLstmWeightLayout {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "iofg" => Self::IOFG,
            "ifgo" => Self::IFGO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLLstmWeightLayout> for emlite::Val {
    fn from(s: MLLstmWeightLayout) -> emlite::Val {
        match s {
            MLLstmWeightLayout::IOFG => emlite::Val::from("iofg"),
            MLLstmWeightLayout::IFGO => emlite::Val::from("ifgo"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLPaddingMode {
    CONSTANT,
    EDGE,
    REFLECTION,
}
impl FromVal for MLPaddingMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "constant" => Self::CONSTANT,
            "edge" => Self::EDGE,
            "reflection" => Self::REFLECTION,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLPaddingMode> for emlite::Val {
    fn from(s: MLPaddingMode) -> emlite::Val {
        match s {
            MLPaddingMode::CONSTANT => emlite::Val::from("constant"),
            MLPaddingMode::EDGE => emlite::Val::from("edge"),
            MLPaddingMode::REFLECTION => emlite::Val::from("reflection"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLRoundingType {
    FLOOR,
    CEIL,
}
impl FromVal for MLRoundingType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "floor" => Self::FLOOR,
            "ceil" => Self::CEIL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLRoundingType> for emlite::Val {
    fn from(s: MLRoundingType) -> emlite::Val {
        match s {
            MLRoundingType::FLOOR => emlite::Val::from("floor"),
            MLRoundingType::CEIL => emlite::Val::from("ceil"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLInterpolationMode {
    NEAREST_NEIGHBOR,
    LINEAR,
}
impl FromVal for MLInterpolationMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "nearest-neighbor" => Self::NEAREST_NEIGHBOR,
            "linear" => Self::LINEAR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<MLInterpolationMode> for emlite::Val {
    fn from(s: MLInterpolationMode) -> emlite::Val {
        match s {
            MLInterpolationMode::NEAREST_NEIGHBOR => emlite::Val::from("nearest-neighbor"),
            MLInterpolationMode::LINEAR => emlite::Val::from("linear"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SFrameTransformRole {
    ENCRYPT,
    DECRYPT,
}
impl FromVal for SFrameTransformRole {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "encrypt" => Self::ENCRYPT,
            "decrypt" => Self::DECRYPT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SFrameTransformRole> for emlite::Val {
    fn from(s: SFrameTransformRole) -> emlite::Val {
        match s {
            SFrameTransformRole::ENCRYPT => emlite::Val::from("encrypt"),
            SFrameTransformRole::DECRYPT => emlite::Val::from("decrypt"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SFrameTransformErrorEventType {
    AUTHENTICATION,
    KEY_ID,
    SYNTAX,
}
impl FromVal for SFrameTransformErrorEventType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "authentication" => Self::AUTHENTICATION,
            "keyID" => Self::KEY_ID,
            "syntax" => Self::SYNTAX,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SFrameTransformErrorEventType> for emlite::Val {
    fn from(s: SFrameTransformErrorEventType) -> emlite::Val {
        match s {
            SFrameTransformErrorEventType::AUTHENTICATION => emlite::Val::from("authentication"),
            SFrameTransformErrorEventType::KEY_ID => emlite::Val::from("keyID"),
            SFrameTransformErrorEventType::SYNTAX => emlite::Val::from("syntax"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCEncodedVideoFrameType {
    EMPTY,
    KEY,
    DELTA,
}
impl FromVal for RTCEncodedVideoFrameType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "empty" => Self::EMPTY,
            "key" => Self::KEY,
            "delta" => Self::DELTA,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCEncodedVideoFrameType> for emlite::Val {
    fn from(s: RTCEncodedVideoFrameType) -> emlite::Val {
        match s {
            RTCEncodedVideoFrameType::EMPTY => emlite::Val::from("empty"),
            RTCEncodedVideoFrameType::KEY => emlite::Val::from("key"),
            RTCEncodedVideoFrameType::DELTA => emlite::Val::from("delta"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCErrorDetailTypeIdp {
    IDP_BAD_SCRIPT_FAILURE,
    IDP_EXECUTION_FAILURE,
    IDP_LOAD_FAILURE,
    IDP_NEED_LOGIN,
    IDP_TIMEOUT,
    IDP_TLS_FAILURE,
    IDP_TOKEN_EXPIRED,
    IDP_TOKEN_INVALID,
}
impl FromVal for RTCErrorDetailTypeIdp {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "idp-bad-script-failure" => Self::IDP_BAD_SCRIPT_FAILURE,
            "idp-execution-failure" => Self::IDP_EXECUTION_FAILURE,
            "idp-load-failure" => Self::IDP_LOAD_FAILURE,
            "idp-need-login" => Self::IDP_NEED_LOGIN,
            "idp-timeout" => Self::IDP_TIMEOUT,
            "idp-tls-failure" => Self::IDP_TLS_FAILURE,
            "idp-token-expired" => Self::IDP_TOKEN_EXPIRED,
            "idp-token-invalid" => Self::IDP_TOKEN_INVALID,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCErrorDetailTypeIdp> for emlite::Val {
    fn from(s: RTCErrorDetailTypeIdp) -> emlite::Val {
        match s {
            RTCErrorDetailTypeIdp::IDP_BAD_SCRIPT_FAILURE => {
                emlite::Val::from("idp-bad-script-failure")
            }
            RTCErrorDetailTypeIdp::IDP_EXECUTION_FAILURE => {
                emlite::Val::from("idp-execution-failure")
            }
            RTCErrorDetailTypeIdp::IDP_LOAD_FAILURE => emlite::Val::from("idp-load-failure"),
            RTCErrorDetailTypeIdp::IDP_NEED_LOGIN => emlite::Val::from("idp-need-login"),
            RTCErrorDetailTypeIdp::IDP_TIMEOUT => emlite::Val::from("idp-timeout"),
            RTCErrorDetailTypeIdp::IDP_TLS_FAILURE => emlite::Val::from("idp-tls-failure"),
            RTCErrorDetailTypeIdp::IDP_TOKEN_EXPIRED => emlite::Val::from("idp-token-expired"),
            RTCErrorDetailTypeIdp::IDP_TOKEN_INVALID => emlite::Val::from("idp-token-invalid"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCPriorityType {
    VERY_LOW,
    LOW,
    MEDIUM,
    HIGH,
}
impl FromVal for RTCPriorityType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "very-low" => Self::VERY_LOW,
            "low" => Self::LOW,
            "medium" => Self::MEDIUM,
            "high" => Self::HIGH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCPriorityType> for emlite::Val {
    fn from(s: RTCPriorityType) -> emlite::Val {
        match s {
            RTCPriorityType::VERY_LOW => emlite::Val::from("very-low"),
            RTCPriorityType::LOW => emlite::Val::from("low"),
            RTCPriorityType::MEDIUM => emlite::Val::from("medium"),
            RTCPriorityType::HIGH => emlite::Val::from("high"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCStatsType {
    CODEC,
    INBOUND_RTP,
    OUTBOUND_RTP,
    REMOTE_INBOUND_RTP,
    REMOTE_OUTBOUND_RTP,
    MEDIA_SOURCE,
    MEDIA_PLAYOUT,
    PEER_CONNECTION,
    DATA_CHANNEL,
    TRANSPORT,
    CANDIDATE_PAIR,
    LOCAL_CANDIDATE,
    REMOTE_CANDIDATE,
    CERTIFICATE,
}
impl FromVal for RTCStatsType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "codec" => Self::CODEC,
            "inbound-rtp" => Self::INBOUND_RTP,
            "outbound-rtp" => Self::OUTBOUND_RTP,
            "remote-inbound-rtp" => Self::REMOTE_INBOUND_RTP,
            "remote-outbound-rtp" => Self::REMOTE_OUTBOUND_RTP,
            "media-source" => Self::MEDIA_SOURCE,
            "media-playout" => Self::MEDIA_PLAYOUT,
            "peer-connection" => Self::PEER_CONNECTION,
            "data-channel" => Self::DATA_CHANNEL,
            "transport" => Self::TRANSPORT,
            "candidate-pair" => Self::CANDIDATE_PAIR,
            "local-candidate" => Self::LOCAL_CANDIDATE,
            "remote-candidate" => Self::REMOTE_CANDIDATE,
            "certificate" => Self::CERTIFICATE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCStatsType> for emlite::Val {
    fn from(s: RTCStatsType) -> emlite::Val {
        match s {
            RTCStatsType::CODEC => emlite::Val::from("codec"),
            RTCStatsType::INBOUND_RTP => emlite::Val::from("inbound-rtp"),
            RTCStatsType::OUTBOUND_RTP => emlite::Val::from("outbound-rtp"),
            RTCStatsType::REMOTE_INBOUND_RTP => emlite::Val::from("remote-inbound-rtp"),
            RTCStatsType::REMOTE_OUTBOUND_RTP => emlite::Val::from("remote-outbound-rtp"),
            RTCStatsType::MEDIA_SOURCE => emlite::Val::from("media-source"),
            RTCStatsType::MEDIA_PLAYOUT => emlite::Val::from("media-playout"),
            RTCStatsType::PEER_CONNECTION => emlite::Val::from("peer-connection"),
            RTCStatsType::DATA_CHANNEL => emlite::Val::from("data-channel"),
            RTCStatsType::TRANSPORT => emlite::Val::from("transport"),
            RTCStatsType::CANDIDATE_PAIR => emlite::Val::from("candidate-pair"),
            RTCStatsType::LOCAL_CANDIDATE => emlite::Val::from("local-candidate"),
            RTCStatsType::REMOTE_CANDIDATE => emlite::Val::from("remote-candidate"),
            RTCStatsType::CERTIFICATE => emlite::Val::from("certificate"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCQualityLimitationReason {
    NONE,
    CPU,
    BANDWIDTH,
    OTHER,
}
impl FromVal for RTCQualityLimitationReason {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "cpu" => Self::CPU,
            "bandwidth" => Self::BANDWIDTH,
            "other" => Self::OTHER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCQualityLimitationReason> for emlite::Val {
    fn from(s: RTCQualityLimitationReason) -> emlite::Val {
        match s {
            RTCQualityLimitationReason::NONE => emlite::Val::from("none"),
            RTCQualityLimitationReason::CPU => emlite::Val::from("cpu"),
            RTCQualityLimitationReason::BANDWIDTH => emlite::Val::from("bandwidth"),
            RTCQualityLimitationReason::OTHER => emlite::Val::from("other"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCDtlsRole {
    CLIENT,
    SERVER,
    UNKNOWN,
}
impl FromVal for RTCDtlsRole {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "client" => Self::CLIENT,
            "server" => Self::SERVER,
            "unknown" => Self::UNKNOWN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCDtlsRole> for emlite::Val {
    fn from(s: RTCDtlsRole) -> emlite::Val {
        match s {
            RTCDtlsRole::CLIENT => emlite::Val::from("client"),
            RTCDtlsRole::SERVER => emlite::Val::from("server"),
            RTCDtlsRole::UNKNOWN => emlite::Val::from("unknown"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCStatsIceCandidatePairState {
    FROZEN,
    WAITING,
    IN_PROGRESS,
    FAILED,
    SUCCEEDED,
}
impl FromVal for RTCStatsIceCandidatePairState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "frozen" => Self::FROZEN,
            "waiting" => Self::WAITING,
            "in-progress" => Self::IN_PROGRESS,
            "failed" => Self::FAILED,
            "succeeded" => Self::SUCCEEDED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCStatsIceCandidatePairState> for emlite::Val {
    fn from(s: RTCStatsIceCandidatePairState) -> emlite::Val {
        match s {
            RTCStatsIceCandidatePairState::FROZEN => emlite::Val::from("frozen"),
            RTCStatsIceCandidatePairState::WAITING => emlite::Val::from("waiting"),
            RTCStatsIceCandidatePairState::IN_PROGRESS => emlite::Val::from("in-progress"),
            RTCStatsIceCandidatePairState::FAILED => emlite::Val::from("failed"),
            RTCStatsIceCandidatePairState::SUCCEEDED => emlite::Val::from("succeeded"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceTransportPolicy {
    RELAY,
    ALL,
}
impl FromVal for RTCIceTransportPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "relay" => Self::RELAY,
            "all" => Self::ALL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceTransportPolicy> for emlite::Val {
    fn from(s: RTCIceTransportPolicy) -> emlite::Val {
        match s {
            RTCIceTransportPolicy::RELAY => emlite::Val::from("relay"),
            RTCIceTransportPolicy::ALL => emlite::Val::from("all"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCBundlePolicy {
    BALANCED,
    MAX_COMPAT,
    MAX_BUNDLE,
}
impl FromVal for RTCBundlePolicy {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "balanced" => Self::BALANCED,
            "max-compat" => Self::MAX_COMPAT,
            "max-bundle" => Self::MAX_BUNDLE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCBundlePolicy> for emlite::Val {
    fn from(s: RTCBundlePolicy) -> emlite::Val {
        match s {
            RTCBundlePolicy::BALANCED => emlite::Val::from("balanced"),
            RTCBundlePolicy::MAX_COMPAT => emlite::Val::from("max-compat"),
            RTCBundlePolicy::MAX_BUNDLE => emlite::Val::from("max-bundle"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCRtcpMuxPolicy {
    REQUIRE,
}
impl FromVal for RTCRtcpMuxPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "require" => Self::REQUIRE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCRtcpMuxPolicy> for emlite::Val {
    fn from(s: RTCRtcpMuxPolicy) -> emlite::Val {
        match s {
            RTCRtcpMuxPolicy::REQUIRE => emlite::Val::from("require"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCSignalingState {
    STABLE,
    HAVE_LOCAL_OFFER,
    HAVE_REMOTE_OFFER,
    HAVE_LOCAL_PRANSWER,
    HAVE_REMOTE_PRANSWER,
    CLOSED,
}
impl FromVal for RTCSignalingState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "stable" => Self::STABLE,
            "have-local-offer" => Self::HAVE_LOCAL_OFFER,
            "have-remote-offer" => Self::HAVE_REMOTE_OFFER,
            "have-local-pranswer" => Self::HAVE_LOCAL_PRANSWER,
            "have-remote-pranswer" => Self::HAVE_REMOTE_PRANSWER,
            "closed" => Self::CLOSED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCSignalingState> for emlite::Val {
    fn from(s: RTCSignalingState) -> emlite::Val {
        match s {
            RTCSignalingState::STABLE => emlite::Val::from("stable"),
            RTCSignalingState::HAVE_LOCAL_OFFER => emlite::Val::from("have-local-offer"),
            RTCSignalingState::HAVE_REMOTE_OFFER => emlite::Val::from("have-remote-offer"),
            RTCSignalingState::HAVE_LOCAL_PRANSWER => emlite::Val::from("have-local-pranswer"),
            RTCSignalingState::HAVE_REMOTE_PRANSWER => emlite::Val::from("have-remote-pranswer"),
            RTCSignalingState::CLOSED => emlite::Val::from("closed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceGatheringState {
    NEW,
    GATHERING,
    COMPLETE,
}
impl FromVal for RTCIceGatheringState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "new" => Self::NEW,
            "gathering" => Self::GATHERING,
            "complete" => Self::COMPLETE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceGatheringState> for emlite::Val {
    fn from(s: RTCIceGatheringState) -> emlite::Val {
        match s {
            RTCIceGatheringState::NEW => emlite::Val::from("new"),
            RTCIceGatheringState::GATHERING => emlite::Val::from("gathering"),
            RTCIceGatheringState::COMPLETE => emlite::Val::from("complete"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCPeerConnectionState {
    CLOSED,
    FAILED,
    DISCONNECTED,
    NEW,
    CONNECTING,
    CONNECTED,
}
impl FromVal for RTCPeerConnectionState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "closed" => Self::CLOSED,
            "failed" => Self::FAILED,
            "disconnected" => Self::DISCONNECTED,
            "new" => Self::NEW,
            "connecting" => Self::CONNECTING,
            "connected" => Self::CONNECTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCPeerConnectionState> for emlite::Val {
    fn from(s: RTCPeerConnectionState) -> emlite::Val {
        match s {
            RTCPeerConnectionState::CLOSED => emlite::Val::from("closed"),
            RTCPeerConnectionState::FAILED => emlite::Val::from("failed"),
            RTCPeerConnectionState::DISCONNECTED => emlite::Val::from("disconnected"),
            RTCPeerConnectionState::NEW => emlite::Val::from("new"),
            RTCPeerConnectionState::CONNECTING => emlite::Val::from("connecting"),
            RTCPeerConnectionState::CONNECTED => emlite::Val::from("connected"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceConnectionState {
    CLOSED,
    FAILED,
    DISCONNECTED,
    NEW,
    CHECKING,
    COMPLETED,
    CONNECTED,
}
impl FromVal for RTCIceConnectionState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "closed" => Self::CLOSED,
            "failed" => Self::FAILED,
            "disconnected" => Self::DISCONNECTED,
            "new" => Self::NEW,
            "checking" => Self::CHECKING,
            "completed" => Self::COMPLETED,
            "connected" => Self::CONNECTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceConnectionState> for emlite::Val {
    fn from(s: RTCIceConnectionState) -> emlite::Val {
        match s {
            RTCIceConnectionState::CLOSED => emlite::Val::from("closed"),
            RTCIceConnectionState::FAILED => emlite::Val::from("failed"),
            RTCIceConnectionState::DISCONNECTED => emlite::Val::from("disconnected"),
            RTCIceConnectionState::NEW => emlite::Val::from("new"),
            RTCIceConnectionState::CHECKING => emlite::Val::from("checking"),
            RTCIceConnectionState::COMPLETED => emlite::Val::from("completed"),
            RTCIceConnectionState::CONNECTED => emlite::Val::from("connected"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCSdpType {
    OFFER,
    PRANSWER,
    ANSWER,
    ROLLBACK,
}
impl FromVal for RTCSdpType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "offer" => Self::OFFER,
            "pranswer" => Self::PRANSWER,
            "answer" => Self::ANSWER,
            "rollback" => Self::ROLLBACK,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCSdpType> for emlite::Val {
    fn from(s: RTCSdpType) -> emlite::Val {
        match s {
            RTCSdpType::OFFER => emlite::Val::from("offer"),
            RTCSdpType::PRANSWER => emlite::Val::from("pranswer"),
            RTCSdpType::ANSWER => emlite::Val::from("answer"),
            RTCSdpType::ROLLBACK => emlite::Val::from("rollback"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceProtocol {
    UDP,
    TCP,
}
impl FromVal for RTCIceProtocol {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "udp" => Self::UDP,
            "tcp" => Self::TCP,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceProtocol> for emlite::Val {
    fn from(s: RTCIceProtocol) -> emlite::Val {
        match s {
            RTCIceProtocol::UDP => emlite::Val::from("udp"),
            RTCIceProtocol::TCP => emlite::Val::from("tcp"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceTcpCandidateType {
    ACTIVE,
    PASSIVE,
    SO,
}
impl FromVal for RTCIceTcpCandidateType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "active" => Self::ACTIVE,
            "passive" => Self::PASSIVE,
            "so" => Self::SO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceTcpCandidateType> for emlite::Val {
    fn from(s: RTCIceTcpCandidateType) -> emlite::Val {
        match s {
            RTCIceTcpCandidateType::ACTIVE => emlite::Val::from("active"),
            RTCIceTcpCandidateType::PASSIVE => emlite::Val::from("passive"),
            RTCIceTcpCandidateType::SO => emlite::Val::from("so"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceCandidateType {
    HOST,
    SRFLX,
    PRFLX,
    RELAY,
}
impl FromVal for RTCIceCandidateType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "host" => Self::HOST,
            "srflx" => Self::SRFLX,
            "prflx" => Self::PRFLX,
            "relay" => Self::RELAY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceCandidateType> for emlite::Val {
    fn from(s: RTCIceCandidateType) -> emlite::Val {
        match s {
            RTCIceCandidateType::HOST => emlite::Val::from("host"),
            RTCIceCandidateType::SRFLX => emlite::Val::from("srflx"),
            RTCIceCandidateType::PRFLX => emlite::Val::from("prflx"),
            RTCIceCandidateType::RELAY => emlite::Val::from("relay"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceServerTransportProtocol {
    UDP,
    TCP,
    TLS,
}
impl FromVal for RTCIceServerTransportProtocol {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "udp" => Self::UDP,
            "tcp" => Self::TCP,
            "tls" => Self::TLS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceServerTransportProtocol> for emlite::Val {
    fn from(s: RTCIceServerTransportProtocol) -> emlite::Val {
        match s {
            RTCIceServerTransportProtocol::UDP => emlite::Val::from("udp"),
            RTCIceServerTransportProtocol::TCP => emlite::Val::from("tcp"),
            RTCIceServerTransportProtocol::TLS => emlite::Val::from("tls"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCRtpTransceiverDirection {
    SENDRECV,
    SENDONLY,
    RECVONLY,
    INACTIVE,
    STOPPED,
}
impl FromVal for RTCRtpTransceiverDirection {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "sendrecv" => Self::SENDRECV,
            "sendonly" => Self::SENDONLY,
            "recvonly" => Self::RECVONLY,
            "inactive" => Self::INACTIVE,
            "stopped" => Self::STOPPED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCRtpTransceiverDirection> for emlite::Val {
    fn from(s: RTCRtpTransceiverDirection) -> emlite::Val {
        match s {
            RTCRtpTransceiverDirection::SENDRECV => emlite::Val::from("sendrecv"),
            RTCRtpTransceiverDirection::SENDONLY => emlite::Val::from("sendonly"),
            RTCRtpTransceiverDirection::RECVONLY => emlite::Val::from("recvonly"),
            RTCRtpTransceiverDirection::INACTIVE => emlite::Val::from("inactive"),
            RTCRtpTransceiverDirection::STOPPED => emlite::Val::from("stopped"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCDtlsTransportState {
    NEW,
    CONNECTING,
    CONNECTED,
    CLOSED,
    FAILED,
}
impl FromVal for RTCDtlsTransportState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "new" => Self::NEW,
            "connecting" => Self::CONNECTING,
            "connected" => Self::CONNECTED,
            "closed" => Self::CLOSED,
            "failed" => Self::FAILED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCDtlsTransportState> for emlite::Val {
    fn from(s: RTCDtlsTransportState) -> emlite::Val {
        match s {
            RTCDtlsTransportState::NEW => emlite::Val::from("new"),
            RTCDtlsTransportState::CONNECTING => emlite::Val::from("connecting"),
            RTCDtlsTransportState::CONNECTED => emlite::Val::from("connected"),
            RTCDtlsTransportState::CLOSED => emlite::Val::from("closed"),
            RTCDtlsTransportState::FAILED => emlite::Val::from("failed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceGathererState {
    NEW,
    GATHERING,
    COMPLETE,
}
impl FromVal for RTCIceGathererState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "new" => Self::NEW,
            "gathering" => Self::GATHERING,
            "complete" => Self::COMPLETE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceGathererState> for emlite::Val {
    fn from(s: RTCIceGathererState) -> emlite::Val {
        match s {
            RTCIceGathererState::NEW => emlite::Val::from("new"),
            RTCIceGathererState::GATHERING => emlite::Val::from("gathering"),
            RTCIceGathererState::COMPLETE => emlite::Val::from("complete"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceTransportState {
    CLOSED,
    FAILED,
    DISCONNECTED,
    NEW,
    CHECKING,
    COMPLETED,
    CONNECTED,
}
impl FromVal for RTCIceTransportState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "closed" => Self::CLOSED,
            "failed" => Self::FAILED,
            "disconnected" => Self::DISCONNECTED,
            "new" => Self::NEW,
            "checking" => Self::CHECKING,
            "completed" => Self::COMPLETED,
            "connected" => Self::CONNECTED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceTransportState> for emlite::Val {
    fn from(s: RTCIceTransportState) -> emlite::Val {
        match s {
            RTCIceTransportState::CLOSED => emlite::Val::from("closed"),
            RTCIceTransportState::FAILED => emlite::Val::from("failed"),
            RTCIceTransportState::DISCONNECTED => emlite::Val::from("disconnected"),
            RTCIceTransportState::NEW => emlite::Val::from("new"),
            RTCIceTransportState::CHECKING => emlite::Val::from("checking"),
            RTCIceTransportState::COMPLETED => emlite::Val::from("completed"),
            RTCIceTransportState::CONNECTED => emlite::Val::from("connected"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceRole {
    UNKNOWN,
    CONTROLLING,
    CONTROLLED,
}
impl FromVal for RTCIceRole {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "unknown" => Self::UNKNOWN,
            "controlling" => Self::CONTROLLING,
            "controlled" => Self::CONTROLLED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceRole> for emlite::Val {
    fn from(s: RTCIceRole) -> emlite::Val {
        match s {
            RTCIceRole::UNKNOWN => emlite::Val::from("unknown"),
            RTCIceRole::CONTROLLING => emlite::Val::from("controlling"),
            RTCIceRole::CONTROLLED => emlite::Val::from("controlled"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceComponent {
    RTP,
    RTCP,
}
impl FromVal for RTCIceComponent {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "rtp" => Self::RTP,
            "rtcp" => Self::RTCP,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCIceComponent> for emlite::Val {
    fn from(s: RTCIceComponent) -> emlite::Val {
        match s {
            RTCIceComponent::RTP => emlite::Val::from("rtp"),
            RTCIceComponent::RTCP => emlite::Val::from("rtcp"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCSctpTransportState {
    CONNECTING,
    CONNECTED,
    CLOSED,
}
impl FromVal for RTCSctpTransportState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "connecting" => Self::CONNECTING,
            "connected" => Self::CONNECTED,
            "closed" => Self::CLOSED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCSctpTransportState> for emlite::Val {
    fn from(s: RTCSctpTransportState) -> emlite::Val {
        match s {
            RTCSctpTransportState::CONNECTING => emlite::Val::from("connecting"),
            RTCSctpTransportState::CONNECTED => emlite::Val::from("connected"),
            RTCSctpTransportState::CLOSED => emlite::Val::from("closed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCDataChannelState {
    CONNECTING,
    OPEN,
    CLOSING,
    CLOSED,
}
impl FromVal for RTCDataChannelState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "connecting" => Self::CONNECTING,
            "open" => Self::OPEN,
            "closing" => Self::CLOSING,
            "closed" => Self::CLOSED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCDataChannelState> for emlite::Val {
    fn from(s: RTCDataChannelState) -> emlite::Val {
        match s {
            RTCDataChannelState::CONNECTING => emlite::Val::from("connecting"),
            RTCDataChannelState::OPEN => emlite::Val::from("open"),
            RTCDataChannelState::CLOSING => emlite::Val::from("closing"),
            RTCDataChannelState::CLOSED => emlite::Val::from("closed"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCErrorDetailType {
    DATA_CHANNEL_FAILURE,
    DTLS_FAILURE,
    FINGERPRINT_FAILURE,
    SCTP_FAILURE,
    SDP_SYNTAX_ERROR,
    HARDWARE_ENCODER_NOT_AVAILABLE,
    HARDWARE_ENCODER_ERROR,
}
impl FromVal for RTCErrorDetailType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "data-channel-failure" => Self::DATA_CHANNEL_FAILURE,
            "dtls-failure" => Self::DTLS_FAILURE,
            "fingerprint-failure" => Self::FINGERPRINT_FAILURE,
            "sctp-failure" => Self::SCTP_FAILURE,
            "sdp-syntax-error" => Self::SDP_SYNTAX_ERROR,
            "hardware-encoder-not-available" => Self::HARDWARE_ENCODER_NOT_AVAILABLE,
            "hardware-encoder-error" => Self::HARDWARE_ENCODER_ERROR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RTCErrorDetailType> for emlite::Val {
    fn from(s: RTCErrorDetailType) -> emlite::Val {
        match s {
            RTCErrorDetailType::DATA_CHANNEL_FAILURE => emlite::Val::from("data-channel-failure"),
            RTCErrorDetailType::DTLS_FAILURE => emlite::Val::from("dtls-failure"),
            RTCErrorDetailType::FINGERPRINT_FAILURE => emlite::Val::from("fingerprint-failure"),
            RTCErrorDetailType::SCTP_FAILURE => emlite::Val::from("sctp-failure"),
            RTCErrorDetailType::SDP_SYNTAX_ERROR => emlite::Val::from("sdp-syntax-error"),
            RTCErrorDetailType::HARDWARE_ENCODER_NOT_AVAILABLE => {
                emlite::Val::from("hardware-encoder-not-available")
            }
            RTCErrorDetailType::HARDWARE_ENCODER_ERROR => {
                emlite::Val::from("hardware-encoder-error")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BinaryType {
    BLOB,
    ARRAYBUFFER,
}
impl FromVal for BinaryType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "blob" => Self::BLOB,
            "arraybuffer" => Self::ARRAYBUFFER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<BinaryType> for emlite::Val {
    fn from(s: BinaryType) -> emlite::Val {
        match s {
            BinaryType::BLOB => emlite::Val::from("blob"),
            BinaryType::ARRAYBUFFER => emlite::Val::from("arraybuffer"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WebTransportReliabilityMode {
    PENDING,
    RELIABLE_ONLY,
    SUPPORTS_UNRELIABLE,
}
impl FromVal for WebTransportReliabilityMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "pending" => Self::PENDING,
            "reliable-only" => Self::RELIABLE_ONLY,
            "supports-unreliable" => Self::SUPPORTS_UNRELIABLE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WebTransportReliabilityMode> for emlite::Val {
    fn from(s: WebTransportReliabilityMode) -> emlite::Val {
        match s {
            WebTransportReliabilityMode::PENDING => emlite::Val::from("pending"),
            WebTransportReliabilityMode::RELIABLE_ONLY => emlite::Val::from("reliable-only"),
            WebTransportReliabilityMode::SUPPORTS_UNRELIABLE => {
                emlite::Val::from("supports-unreliable")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WebTransportCongestionControl {
    DEFAULT,
    THROUGHPUT,
    LOW_LATENCY,
}
impl FromVal for WebTransportCongestionControl {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "default" => Self::DEFAULT,
            "throughput" => Self::THROUGHPUT,
            "low-latency" => Self::LOW_LATENCY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WebTransportCongestionControl> for emlite::Val {
    fn from(s: WebTransportCongestionControl) -> emlite::Val {
        match s {
            WebTransportCongestionControl::DEFAULT => emlite::Val::from("default"),
            WebTransportCongestionControl::THROUGHPUT => emlite::Val::from("throughput"),
            WebTransportCongestionControl::LOW_LATENCY => emlite::Val::from("low-latency"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WebTransportErrorSource {
    STREAM,
    SESSION,
}
impl FromVal for WebTransportErrorSource {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "stream" => Self::STREAM,
            "session" => Self::SESSION,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WebTransportErrorSource> for emlite::Val {
    fn from(s: WebTransportErrorSource) -> emlite::Val {
        match s {
            WebTransportErrorSource::STREAM => emlite::Val::from("stream"),
            WebTransportErrorSource::SESSION => emlite::Val::from("session"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum USBTransferStatus {
    OK,
    STALL,
    BABBLE,
}
impl FromVal for USBTransferStatus {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "ok" => Self::OK,
            "stall" => Self::STALL,
            "babble" => Self::BABBLE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<USBTransferStatus> for emlite::Val {
    fn from(s: USBTransferStatus) -> emlite::Val {
        match s {
            USBTransferStatus::OK => emlite::Val::from("ok"),
            USBTransferStatus::STALL => emlite::Val::from("stall"),
            USBTransferStatus::BABBLE => emlite::Val::from("babble"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum USBRequestType {
    STANDARD,
    CLASS,
    VENDOR,
}
impl FromVal for USBRequestType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "standard" => Self::STANDARD,
            "class" => Self::CLASS,
            "vendor" => Self::VENDOR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<USBRequestType> for emlite::Val {
    fn from(s: USBRequestType) -> emlite::Val {
        match s {
            USBRequestType::STANDARD => emlite::Val::from("standard"),
            USBRequestType::CLASS => emlite::Val::from("class"),
            USBRequestType::VENDOR => emlite::Val::from("vendor"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum USBRecipient {
    DEVICE,
    INTERFACE,
    ENDPOINT,
    OTHER,
}
impl FromVal for USBRecipient {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "device" => Self::DEVICE,
            "interface" => Self::INTERFACE,
            "endpoint" => Self::ENDPOINT,
            "other" => Self::OTHER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<USBRecipient> for emlite::Val {
    fn from(s: USBRecipient) -> emlite::Val {
        match s {
            USBRecipient::DEVICE => emlite::Val::from("device"),
            USBRecipient::INTERFACE => emlite::Val::from("interface"),
            USBRecipient::ENDPOINT => emlite::Val::from("endpoint"),
            USBRecipient::OTHER => emlite::Val::from("other"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum USBDirection {
    IN_,
    OUT,
}
impl FromVal for USBDirection {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "in" => Self::IN_,
            "out" => Self::OUT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<USBDirection> for emlite::Val {
    fn from(s: USBDirection) -> emlite::Val {
        match s {
            USBDirection::IN_ => emlite::Val::from("in"),
            USBDirection::OUT => emlite::Val::from("out"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum USBEndpointType {
    BULK,
    INTERRUPT,
    ISOCHRONOUS,
}
impl FromVal for USBEndpointType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "bulk" => Self::BULK,
            "interrupt" => Self::INTERRUPT,
            "isochronous" => Self::ISOCHRONOUS,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<USBEndpointType> for emlite::Val {
    fn from(s: USBEndpointType) -> emlite::Val {
        match s {
            USBEndpointType::BULK => emlite::Val::from("bulk"),
            USBEndpointType::INTERRUPT => emlite::Val::from("interrupt"),
            USBEndpointType::ISOCHRONOUS => emlite::Val::from("isochronous"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AutoKeyword {
    AUTO,
}
impl FromVal for AutoKeyword {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "auto" => Self::AUTO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AutoKeyword> for emlite::Val {
    fn from(s: AutoKeyword) -> emlite::Val {
        match s {
            AutoKeyword::AUTO => emlite::Val::from("auto"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DirectionSetting {
    NONE,
    RL,
    LR,
}
impl FromVal for DirectionSetting {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "rl" => Self::RL,
            "lr" => Self::LR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<DirectionSetting> for emlite::Val {
    fn from(s: DirectionSetting) -> emlite::Val {
        match s {
            DirectionSetting::NONE => emlite::Val::from(""),
            DirectionSetting::RL => emlite::Val::from("rl"),
            DirectionSetting::LR => emlite::Val::from("lr"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LineAlignSetting {
    START,
    CENTER,
    END,
}
impl FromVal for LineAlignSetting {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "start" => Self::START,
            "center" => Self::CENTER,
            "end" => Self::END,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<LineAlignSetting> for emlite::Val {
    fn from(s: LineAlignSetting) -> emlite::Val {
        match s {
            LineAlignSetting::START => emlite::Val::from("start"),
            LineAlignSetting::CENTER => emlite::Val::from("center"),
            LineAlignSetting::END => emlite::Val::from("end"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PositionAlignSetting {
    LINE_LEFT,
    CENTER,
    LINE_RIGHT,
    AUTO,
}
impl FromVal for PositionAlignSetting {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "line-left" => Self::LINE_LEFT,
            "center" => Self::CENTER,
            "line-right" => Self::LINE_RIGHT,
            "auto" => Self::AUTO,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<PositionAlignSetting> for emlite::Val {
    fn from(s: PositionAlignSetting) -> emlite::Val {
        match s {
            PositionAlignSetting::LINE_LEFT => emlite::Val::from("line-left"),
            PositionAlignSetting::CENTER => emlite::Val::from("center"),
            PositionAlignSetting::LINE_RIGHT => emlite::Val::from("line-right"),
            PositionAlignSetting::AUTO => emlite::Val::from("auto"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AlignSetting {
    START,
    CENTER,
    END,
    LEFT,
    RIGHT,
}
impl FromVal for AlignSetting {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "start" => Self::START,
            "center" => Self::CENTER,
            "end" => Self::END,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<AlignSetting> for emlite::Val {
    fn from(s: AlignSetting) -> emlite::Val {
        match s {
            AlignSetting::START => emlite::Val::from("start"),
            AlignSetting::CENTER => emlite::Val::from("center"),
            AlignSetting::END => emlite::Val::from("end"),
            AlignSetting::LEFT => emlite::Val::from("left"),
            AlignSetting::RIGHT => emlite::Val::from("right"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScrollSetting {
    NONE,
    UP,
}
impl FromVal for ScrollSetting {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "up" => Self::UP,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<ScrollSetting> for emlite::Val {
    fn from(s: ScrollSetting) -> emlite::Val {
        match s {
            ScrollSetting::NONE => emlite::Val::from(""),
            ScrollSetting::UP => emlite::Val::from("up"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XREnvironmentBlendMode {
    OPAQUE,
    ALPHA_BLEND,
    ADDITIVE,
}
impl FromVal for XREnvironmentBlendMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "opaque" => Self::OPAQUE,
            "alpha-blend" => Self::ALPHA_BLEND,
            "additive" => Self::ADDITIVE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XREnvironmentBlendMode> for emlite::Val {
    fn from(s: XREnvironmentBlendMode) -> emlite::Val {
        match s {
            XREnvironmentBlendMode::OPAQUE => emlite::Val::from("opaque"),
            XREnvironmentBlendMode::ALPHA_BLEND => emlite::Val::from("alpha-blend"),
            XREnvironmentBlendMode::ADDITIVE => emlite::Val::from("additive"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRInteractionMode {
    SCREEN_SPACE,
    WORLD_SPACE,
}
impl FromVal for XRInteractionMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "screen-space" => Self::SCREEN_SPACE,
            "world-space" => Self::WORLD_SPACE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRInteractionMode> for emlite::Val {
    fn from(s: XRInteractionMode) -> emlite::Val {
        match s {
            XRInteractionMode::SCREEN_SPACE => emlite::Val::from("screen-space"),
            XRInteractionMode::WORLD_SPACE => emlite::Val::from("world-space"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRDepthType {
    RAW_,
    SMOOTH,
}
impl FromVal for XRDepthType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "raw" => Self::RAW_,
            "smooth" => Self::SMOOTH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRDepthType> for emlite::Val {
    fn from(s: XRDepthType) -> emlite::Val {
        match s {
            XRDepthType::RAW_ => emlite::Val::from("raw"),
            XRDepthType::SMOOTH => emlite::Val::from("smooth"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRDepthUsage {
    CPU_OPTIMIZED,
    GPU_OPTIMIZED,
}
impl FromVal for XRDepthUsage {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "cpu-optimized" => Self::CPU_OPTIMIZED,
            "gpu-optimized" => Self::GPU_OPTIMIZED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRDepthUsage> for emlite::Val {
    fn from(s: XRDepthUsage) -> emlite::Val {
        match s {
            XRDepthUsage::CPU_OPTIMIZED => emlite::Val::from("cpu-optimized"),
            XRDepthUsage::GPU_OPTIMIZED => emlite::Val::from("gpu-optimized"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRDepthDataFormat {
    LUMINANCE_ALPHA,
    FLOAT32,
    UNSIGNED_SHORT,
}
impl FromVal for XRDepthDataFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "luminance-alpha" => Self::LUMINANCE_ALPHA,
            "float32" => Self::FLOAT32,
            "unsigned-short" => Self::UNSIGNED_SHORT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRDepthDataFormat> for emlite::Val {
    fn from(s: XRDepthDataFormat) -> emlite::Val {
        match s {
            XRDepthDataFormat::LUMINANCE_ALPHA => emlite::Val::from("luminance-alpha"),
            XRDepthDataFormat::FLOAT32 => emlite::Val::from("float32"),
            XRDepthDataFormat::UNSIGNED_SHORT => emlite::Val::from("unsigned-short"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRDOMOverlayType {
    SCREEN,
    FLOATING,
    HEAD_LOCKED,
}
impl FromVal for XRDOMOverlayType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "screen" => Self::SCREEN,
            "floating" => Self::FLOATING,
            "head-locked" => Self::HEAD_LOCKED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRDOMOverlayType> for emlite::Val {
    fn from(s: XRDOMOverlayType) -> emlite::Val {
        match s {
            XRDOMOverlayType::SCREEN => emlite::Val::from("screen"),
            XRDOMOverlayType::FLOATING => emlite::Val::from("floating"),
            XRDOMOverlayType::HEAD_LOCKED => emlite::Val::from("head-locked"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRHandJoint {
    WRIST,
    THUMB_METACARPAL,
    THUMB_PHALANX_PROXIMAL,
    THUMB_PHALANX_DISTAL,
    THUMB_TIP,
    INDEX_FINGER_METACARPAL,
    INDEX_FINGER_PHALANX_PROXIMAL,
    INDEX_FINGER_PHALANX_INTERMEDIATE,
    INDEX_FINGER_PHALANX_DISTAL,
    INDEX_FINGER_TIP,
    MIDDLE_FINGER_METACARPAL,
    MIDDLE_FINGER_PHALANX_PROXIMAL,
    MIDDLE_FINGER_PHALANX_INTERMEDIATE,
    MIDDLE_FINGER_PHALANX_DISTAL,
    MIDDLE_FINGER_TIP,
    RING_FINGER_METACARPAL,
    RING_FINGER_PHALANX_PROXIMAL,
    RING_FINGER_PHALANX_INTERMEDIATE,
    RING_FINGER_PHALANX_DISTAL,
    RING_FINGER_TIP,
    PINKY_FINGER_METACARPAL,
    PINKY_FINGER_PHALANX_PROXIMAL,
    PINKY_FINGER_PHALANX_INTERMEDIATE,
    PINKY_FINGER_PHALANX_DISTAL,
    PINKY_FINGER_TIP,
}
impl FromVal for XRHandJoint {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "wrist" => Self::WRIST,
            "thumb-metacarpal" => Self::THUMB_METACARPAL,
            "thumb-phalanx-proximal" => Self::THUMB_PHALANX_PROXIMAL,
            "thumb-phalanx-distal" => Self::THUMB_PHALANX_DISTAL,
            "thumb-tip" => Self::THUMB_TIP,
            "index-finger-metacarpal" => Self::INDEX_FINGER_METACARPAL,
            "index-finger-phalanx-proximal" => Self::INDEX_FINGER_PHALANX_PROXIMAL,
            "index-finger-phalanx-intermediate" => Self::INDEX_FINGER_PHALANX_INTERMEDIATE,
            "index-finger-phalanx-distal" => Self::INDEX_FINGER_PHALANX_DISTAL,
            "index-finger-tip" => Self::INDEX_FINGER_TIP,
            "middle-finger-metacarpal" => Self::MIDDLE_FINGER_METACARPAL,
            "middle-finger-phalanx-proximal" => Self::MIDDLE_FINGER_PHALANX_PROXIMAL,
            "middle-finger-phalanx-intermediate" => Self::MIDDLE_FINGER_PHALANX_INTERMEDIATE,
            "middle-finger-phalanx-distal" => Self::MIDDLE_FINGER_PHALANX_DISTAL,
            "middle-finger-tip" => Self::MIDDLE_FINGER_TIP,
            "ring-finger-metacarpal" => Self::RING_FINGER_METACARPAL,
            "ring-finger-phalanx-proximal" => Self::RING_FINGER_PHALANX_PROXIMAL,
            "ring-finger-phalanx-intermediate" => Self::RING_FINGER_PHALANX_INTERMEDIATE,
            "ring-finger-phalanx-distal" => Self::RING_FINGER_PHALANX_DISTAL,
            "ring-finger-tip" => Self::RING_FINGER_TIP,
            "pinky-finger-metacarpal" => Self::PINKY_FINGER_METACARPAL,
            "pinky-finger-phalanx-proximal" => Self::PINKY_FINGER_PHALANX_PROXIMAL,
            "pinky-finger-phalanx-intermediate" => Self::PINKY_FINGER_PHALANX_INTERMEDIATE,
            "pinky-finger-phalanx-distal" => Self::PINKY_FINGER_PHALANX_DISTAL,
            "pinky-finger-tip" => Self::PINKY_FINGER_TIP,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRHandJoint> for emlite::Val {
    fn from(s: XRHandJoint) -> emlite::Val {
        match s {
            XRHandJoint::WRIST => emlite::Val::from("wrist"),
            XRHandJoint::THUMB_METACARPAL => emlite::Val::from("thumb-metacarpal"),
            XRHandJoint::THUMB_PHALANX_PROXIMAL => emlite::Val::from("thumb-phalanx-proximal"),
            XRHandJoint::THUMB_PHALANX_DISTAL => emlite::Val::from("thumb-phalanx-distal"),
            XRHandJoint::THUMB_TIP => emlite::Val::from("thumb-tip"),
            XRHandJoint::INDEX_FINGER_METACARPAL => emlite::Val::from("index-finger-metacarpal"),
            XRHandJoint::INDEX_FINGER_PHALANX_PROXIMAL => {
                emlite::Val::from("index-finger-phalanx-proximal")
            }
            XRHandJoint::INDEX_FINGER_PHALANX_INTERMEDIATE => {
                emlite::Val::from("index-finger-phalanx-intermediate")
            }
            XRHandJoint::INDEX_FINGER_PHALANX_DISTAL => {
                emlite::Val::from("index-finger-phalanx-distal")
            }
            XRHandJoint::INDEX_FINGER_TIP => emlite::Val::from("index-finger-tip"),
            XRHandJoint::MIDDLE_FINGER_METACARPAL => emlite::Val::from("middle-finger-metacarpal"),
            XRHandJoint::MIDDLE_FINGER_PHALANX_PROXIMAL => {
                emlite::Val::from("middle-finger-phalanx-proximal")
            }
            XRHandJoint::MIDDLE_FINGER_PHALANX_INTERMEDIATE => {
                emlite::Val::from("middle-finger-phalanx-intermediate")
            }
            XRHandJoint::MIDDLE_FINGER_PHALANX_DISTAL => {
                emlite::Val::from("middle-finger-phalanx-distal")
            }
            XRHandJoint::MIDDLE_FINGER_TIP => emlite::Val::from("middle-finger-tip"),
            XRHandJoint::RING_FINGER_METACARPAL => emlite::Val::from("ring-finger-metacarpal"),
            XRHandJoint::RING_FINGER_PHALANX_PROXIMAL => {
                emlite::Val::from("ring-finger-phalanx-proximal")
            }
            XRHandJoint::RING_FINGER_PHALANX_INTERMEDIATE => {
                emlite::Val::from("ring-finger-phalanx-intermediate")
            }
            XRHandJoint::RING_FINGER_PHALANX_DISTAL => {
                emlite::Val::from("ring-finger-phalanx-distal")
            }
            XRHandJoint::RING_FINGER_TIP => emlite::Val::from("ring-finger-tip"),
            XRHandJoint::PINKY_FINGER_METACARPAL => emlite::Val::from("pinky-finger-metacarpal"),
            XRHandJoint::PINKY_FINGER_PHALANX_PROXIMAL => {
                emlite::Val::from("pinky-finger-phalanx-proximal")
            }
            XRHandJoint::PINKY_FINGER_PHALANX_INTERMEDIATE => {
                emlite::Val::from("pinky-finger-phalanx-intermediate")
            }
            XRHandJoint::PINKY_FINGER_PHALANX_DISTAL => {
                emlite::Val::from("pinky-finger-phalanx-distal")
            }
            XRHandJoint::PINKY_FINGER_TIP => emlite::Val::from("pinky-finger-tip"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRHitTestTrackableType {
    POINT,
    PLANE,
    MESH,
}
impl FromVal for XRHitTestTrackableType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "point" => Self::POINT,
            "plane" => Self::PLANE,
            "mesh" => Self::MESH,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRHitTestTrackableType> for emlite::Val {
    fn from(s: XRHitTestTrackableType) -> emlite::Val {
        match s {
            XRHitTestTrackableType::POINT => emlite::Val::from("point"),
            XRHitTestTrackableType::PLANE => emlite::Val::from("plane"),
            XRHitTestTrackableType::MESH => emlite::Val::from("mesh"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRReflectionFormat {
    SRGBA8,
    RGBA16F,
}
impl FromVal for XRReflectionFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "srgba8" => Self::SRGBA8,
            "rgba16f" => Self::RGBA16F,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRReflectionFormat> for emlite::Val {
    fn from(s: XRReflectionFormat) -> emlite::Val {
        match s {
            XRReflectionFormat::SRGBA8 => emlite::Val::from("srgba8"),
            XRReflectionFormat::RGBA16F => emlite::Val::from("rgba16f"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRPlaneOrientation {
    HORIZONTAL,
    VERTICAL,
}
impl FromVal for XRPlaneOrientation {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "horizontal" => Self::HORIZONTAL,
            "vertical" => Self::VERTICAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRPlaneOrientation> for emlite::Val {
    fn from(s: XRPlaneOrientation) -> emlite::Val {
        match s {
            XRPlaneOrientation::HORIZONTAL => emlite::Val::from("horizontal"),
            XRPlaneOrientation::VERTICAL => emlite::Val::from("vertical"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRSessionMode {
    INLINE,
    IMMERSIVE_VR,
    IMMERSIVE_AR,
}
impl FromVal for XRSessionMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "inline" => Self::INLINE,
            "immersive-vr" => Self::IMMERSIVE_VR,
            "immersive-ar" => Self::IMMERSIVE_AR,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRSessionMode> for emlite::Val {
    fn from(s: XRSessionMode) -> emlite::Val {
        match s {
            XRSessionMode::INLINE => emlite::Val::from("inline"),
            XRSessionMode::IMMERSIVE_VR => emlite::Val::from("immersive-vr"),
            XRSessionMode::IMMERSIVE_AR => emlite::Val::from("immersive-ar"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRVisibilityState {
    VISIBLE,
    VISIBLE_BLURRED,
    HIDDEN,
}
impl FromVal for XRVisibilityState {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "visible" => Self::VISIBLE,
            "visible-blurred" => Self::VISIBLE_BLURRED,
            "hidden" => Self::HIDDEN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRVisibilityState> for emlite::Val {
    fn from(s: XRVisibilityState) -> emlite::Val {
        match s {
            XRVisibilityState::VISIBLE => emlite::Val::from("visible"),
            XRVisibilityState::VISIBLE_BLURRED => emlite::Val::from("visible-blurred"),
            XRVisibilityState::HIDDEN => emlite::Val::from("hidden"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRReferenceSpaceType {
    VIEWER,
    LOCAL,
    LOCAL_FLOOR,
    BOUNDED_FLOOR,
    UNBOUNDED,
}
impl FromVal for XRReferenceSpaceType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "viewer" => Self::VIEWER,
            "local" => Self::LOCAL,
            "local-floor" => Self::LOCAL_FLOOR,
            "bounded-floor" => Self::BOUNDED_FLOOR,
            "unbounded" => Self::UNBOUNDED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRReferenceSpaceType> for emlite::Val {
    fn from(s: XRReferenceSpaceType) -> emlite::Val {
        match s {
            XRReferenceSpaceType::VIEWER => emlite::Val::from("viewer"),
            XRReferenceSpaceType::LOCAL => emlite::Val::from("local"),
            XRReferenceSpaceType::LOCAL_FLOOR => emlite::Val::from("local-floor"),
            XRReferenceSpaceType::BOUNDED_FLOOR => emlite::Val::from("bounded-floor"),
            XRReferenceSpaceType::UNBOUNDED => emlite::Val::from("unbounded"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XREye {
    NONE,
    LEFT,
    RIGHT,
}
impl FromVal for XREye {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XREye> for emlite::Val {
    fn from(s: XREye) -> emlite::Val {
        match s {
            XREye::NONE => emlite::Val::from("none"),
            XREye::LEFT => emlite::Val::from("left"),
            XREye::RIGHT => emlite::Val::from("right"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRHandedness {
    NONE,
    LEFT,
    RIGHT,
}
impl FromVal for XRHandedness {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "none" => Self::NONE,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRHandedness> for emlite::Val {
    fn from(s: XRHandedness) -> emlite::Val {
        match s {
            XRHandedness::NONE => emlite::Val::from("none"),
            XRHandedness::LEFT => emlite::Val::from("left"),
            XRHandedness::RIGHT => emlite::Val::from("right"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRTargetRayMode {
    GAZE,
    TRACKED_POINTER,
    SCREEN,
    TRANSIENT_POINTER,
}
impl FromVal for XRTargetRayMode {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "gaze" => Self::GAZE,
            "tracked-pointer" => Self::TRACKED_POINTER,
            "screen" => Self::SCREEN,
            "transient-pointer" => Self::TRANSIENT_POINTER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRTargetRayMode> for emlite::Val {
    fn from(s: XRTargetRayMode) -> emlite::Val {
        match s {
            XRTargetRayMode::GAZE => emlite::Val::from("gaze"),
            XRTargetRayMode::TRACKED_POINTER => emlite::Val::from("tracked-pointer"),
            XRTargetRayMode::SCREEN => emlite::Val::from("screen"),
            XRTargetRayMode::TRANSIENT_POINTER => emlite::Val::from("transient-pointer"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRLayerLayout {
    DEFAULT,
    MONO,
    STEREO,
    STEREO_LEFT_RIGHT,
    STEREO_TOP_BOTTOM,
}
impl FromVal for XRLayerLayout {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "default" => Self::DEFAULT,
            "mono" => Self::MONO,
            "stereo" => Self::STEREO,
            "stereo-left-right" => Self::STEREO_LEFT_RIGHT,
            "stereo-top-bottom" => Self::STEREO_TOP_BOTTOM,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRLayerLayout> for emlite::Val {
    fn from(s: XRLayerLayout) -> emlite::Val {
        match s {
            XRLayerLayout::DEFAULT => emlite::Val::from("default"),
            XRLayerLayout::MONO => emlite::Val::from("mono"),
            XRLayerLayout::STEREO => emlite::Val::from("stereo"),
            XRLayerLayout::STEREO_LEFT_RIGHT => emlite::Val::from("stereo-left-right"),
            XRLayerLayout::STEREO_TOP_BOTTOM => emlite::Val::from("stereo-top-bottom"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRLayerQuality {
    DEFAULT,
    TEXT_OPTIMIZED,
    GRAPHICS_OPTIMIZED,
}
impl FromVal for XRLayerQuality {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "default" => Self::DEFAULT,
            "text-optimized" => Self::TEXT_OPTIMIZED,
            "graphics-optimized" => Self::GRAPHICS_OPTIMIZED,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRLayerQuality> for emlite::Val {
    fn from(s: XRLayerQuality) -> emlite::Val {
        match s {
            XRLayerQuality::DEFAULT => emlite::Val::from("default"),
            XRLayerQuality::TEXT_OPTIMIZED => emlite::Val::from("text-optimized"),
            XRLayerQuality::GRAPHICS_OPTIMIZED => emlite::Val::from("graphics-optimized"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRTextureType {
    TEXTURE,
    TEXTURE_ARRAY,
}
impl FromVal for XRTextureType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "texture" => Self::TEXTURE,
            "texture-array" => Self::TEXTURE_ARRAY,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XRTextureType> for emlite::Val {
    fn from(s: XRTextureType) -> emlite::Val {
        match s {
            XRTextureType::TEXTURE => emlite::Val::from("texture"),
            XRTextureType::TEXTURE_ARRAY => emlite::Val::from("texture-array"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SummarizerType {
    TLDR,
    TEASER,
    KEY_POINTS,
    HEADLINE,
}
impl FromVal for SummarizerType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "tldr" => Self::TLDR,
            "teaser" => Self::TEASER,
            "key-points" => Self::KEY_POINTS,
            "headline" => Self::HEADLINE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SummarizerType> for emlite::Val {
    fn from(s: SummarizerType) -> emlite::Val {
        match s {
            SummarizerType::TLDR => emlite::Val::from("tldr"),
            SummarizerType::TEASER => emlite::Val::from("teaser"),
            SummarizerType::KEY_POINTS => emlite::Val::from("key-points"),
            SummarizerType::HEADLINE => emlite::Val::from("headline"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SummarizerFormat {
    PLAIN_TEXT,
    MARKDOWN,
}
impl FromVal for SummarizerFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "plain-text" => Self::PLAIN_TEXT,
            "markdown" => Self::MARKDOWN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SummarizerFormat> for emlite::Val {
    fn from(s: SummarizerFormat) -> emlite::Val {
        match s {
            SummarizerFormat::PLAIN_TEXT => emlite::Val::from("plain-text"),
            SummarizerFormat::MARKDOWN => emlite::Val::from("markdown"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SummarizerLength {
    SHORT,
    MEDIUM,
    LONG,
}
impl FromVal for SummarizerLength {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "short" => Self::SHORT,
            "medium" => Self::MEDIUM,
            "long" => Self::LONG,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<SummarizerLength> for emlite::Val {
    fn from(s: SummarizerLength) -> emlite::Val {
        match s {
            SummarizerLength::SHORT => emlite::Val::from("short"),
            SummarizerLength::MEDIUM => emlite::Val::from("medium"),
            SummarizerLength::LONG => emlite::Val::from("long"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WriterTone {
    FORMAL,
    NEUTRAL,
    CASUAL,
}
impl FromVal for WriterTone {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "formal" => Self::FORMAL,
            "neutral" => Self::NEUTRAL,
            "casual" => Self::CASUAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WriterTone> for emlite::Val {
    fn from(s: WriterTone) -> emlite::Val {
        match s {
            WriterTone::FORMAL => emlite::Val::from("formal"),
            WriterTone::NEUTRAL => emlite::Val::from("neutral"),
            WriterTone::CASUAL => emlite::Val::from("casual"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WriterFormat {
    PLAIN_TEXT,
    MARKDOWN,
}
impl FromVal for WriterFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "plain-text" => Self::PLAIN_TEXT,
            "markdown" => Self::MARKDOWN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WriterFormat> for emlite::Val {
    fn from(s: WriterFormat) -> emlite::Val {
        match s {
            WriterFormat::PLAIN_TEXT => emlite::Val::from("plain-text"),
            WriterFormat::MARKDOWN => emlite::Val::from("markdown"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WriterLength {
    SHORT,
    MEDIUM,
    LONG,
}
impl FromVal for WriterLength {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "short" => Self::SHORT,
            "medium" => Self::MEDIUM,
            "long" => Self::LONG,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<WriterLength> for emlite::Val {
    fn from(s: WriterLength) -> emlite::Val {
        match s {
            WriterLength::SHORT => emlite::Val::from("short"),
            WriterLength::MEDIUM => emlite::Val::from("medium"),
            WriterLength::LONG => emlite::Val::from("long"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RewriterTone {
    AS_IS,
    MORE_FORMAL,
    MORE_CASUAL,
}
impl FromVal for RewriterTone {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "as-is" => Self::AS_IS,
            "more-formal" => Self::MORE_FORMAL,
            "more-casual" => Self::MORE_CASUAL,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RewriterTone> for emlite::Val {
    fn from(s: RewriterTone) -> emlite::Val {
        match s {
            RewriterTone::AS_IS => emlite::Val::from("as-is"),
            RewriterTone::MORE_FORMAL => emlite::Val::from("more-formal"),
            RewriterTone::MORE_CASUAL => emlite::Val::from("more-casual"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RewriterFormat {
    AS_IS,
    PLAIN_TEXT,
    MARKDOWN,
}
impl FromVal for RewriterFormat {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "as-is" => Self::AS_IS,
            "plain-text" => Self::PLAIN_TEXT,
            "markdown" => Self::MARKDOWN,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RewriterFormat> for emlite::Val {
    fn from(s: RewriterFormat) -> emlite::Val {
        match s {
            RewriterFormat::AS_IS => emlite::Val::from("as-is"),
            RewriterFormat::PLAIN_TEXT => emlite::Val::from("plain-text"),
            RewriterFormat::MARKDOWN => emlite::Val::from("markdown"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RewriterLength {
    AS_IS,
    SHORTER,
    LONGER,
}
impl FromVal for RewriterLength {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "as-is" => Self::AS_IS,
            "shorter" => Self::SHORTER,
            "longer" => Self::LONGER,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<RewriterLength> for emlite::Val {
    fn from(s: RewriterLength) -> emlite::Val {
        match s {
            RewriterLength::AS_IS => emlite::Val::from("as-is"),
            RewriterLength::SHORTER => emlite::Val::from("shorter"),
            RewriterLength::LONGER => emlite::Val::from("longer"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Availability {
    UNAVAILABLE,
    DOWNLOADABLE,
    DOWNLOADING,
    AVAILABLE,
}
impl FromVal for Availability {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "unavailable" => Self::UNAVAILABLE,
            "downloadable" => Self::DOWNLOADABLE,
            "downloading" => Self::DOWNLOADING,
            "available" => Self::AVAILABLE,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<Availability> for emlite::Val {
    fn from(s: Availability) -> emlite::Val {
        match s {
            Availability::UNAVAILABLE => emlite::Val::from("unavailable"),
            Availability::DOWNLOADABLE => emlite::Val::from("downloadable"),
            Availability::DOWNLOADING => emlite::Val::from("downloading"),
            Availability::AVAILABLE => emlite::Val::from("available"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XMLHttpRequestResponseType {
    NONE,
    ARRAYBUFFER,
    BLOB,
    DOCUMENT,
    JSON,
    TEXT,
}
impl FromVal for XMLHttpRequestResponseType {
    fn from_val(v: &emlite::Val) -> Self {
        match v.as_::<&str>() {
            "" => Self::NONE,
            "arraybuffer" => Self::ARRAYBUFFER,
            "blob" => Self::BLOB,
            "document" => Self::DOCUMENT,
            "json" => Self::JSON,
            "text" => Self::TEXT,
            _ => unreachable!(),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        emlite::Val::from(*self).as_handle()
    }
}
impl From<XMLHttpRequestResponseType> for emlite::Val {
    fn from(s: XMLHttpRequestResponseType) -> emlite::Val {
        match s {
            XMLHttpRequestResponseType::NONE => emlite::Val::from(""),
            XMLHttpRequestResponseType::ARRAYBUFFER => emlite::Val::from("arraybuffer"),
            XMLHttpRequestResponseType::BLOB => emlite::Val::from("blob"),
            XMLHttpRequestResponseType::DOCUMENT => emlite::Val::from("document"),
            XMLHttpRequestResponseType::JSON => emlite::Val::from("json"),
            XMLHttpRequestResponseType::TEXT => emlite::Val::from("text"),
        }
    }
}
