use super::*;




#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SecurityPolicyViolationEventDisposition {
    ENFORCE,
    REPORT,
}
impl FromVal for SecurityPolicyViolationEventDisposition {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "enforce" => Self::ENFORCE,
            "report" => Self::REPORT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SecurityPolicyViolationEventDisposition> for Any {
    fn from(s: SecurityPolicyViolationEventDisposition) -> Any {
         match s {
            SecurityPolicyViolationEventDisposition::ENFORCE => Any::from("enforce"),
            SecurityPolicyViolationEventDisposition::REPORT => Any::from("report"),
         }
    }
}
impl From<&SecurityPolicyViolationEventDisposition> for Any {
    fn from(s: &SecurityPolicyViolationEventDisposition) -> Any {
         match *s {
            SecurityPolicyViolationEventDisposition::ENFORCE => Any::from("enforce"),
            SecurityPolicyViolationEventDisposition::REPORT => Any::from("report"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EndingType {
    TRANSPARENT,
    NATIVE,
}
impl FromVal for EndingType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "transparent" => Self::TRANSPARENT,
            "native" => Self::NATIVE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<EndingType> for Any {
    fn from(s: EndingType) -> Any {
         match s {
            EndingType::TRANSPARENT => Any::from("transparent"),
            EndingType::NATIVE => Any::from("native"),
         }
    }
}
impl From<&EndingType> for Any {
    fn from(s: &EndingType) -> Any {
         match *s {
            EndingType::TRANSPARENT => Any::from("transparent"),
            EndingType::NATIVE => Any::from("native"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IDBRequestReadyState {
    PENDING,
    DONE,
}
impl FromVal for IDBRequestReadyState {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "pending" => Self::PENDING,
            "done" => Self::DONE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<IDBRequestReadyState> for Any {
    fn from(s: IDBRequestReadyState) -> Any {
         match s {
            IDBRequestReadyState::PENDING => Any::from("pending"),
            IDBRequestReadyState::DONE => Any::from("done"),
         }
    }
}
impl From<&IDBRequestReadyState> for Any {
    fn from(s: &IDBRequestReadyState) -> Any {
         match *s {
            IDBRequestReadyState::PENDING => Any::from("pending"),
            IDBRequestReadyState::DONE => Any::from("done"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "default" => Self::DEFAULT,
            "strict" => Self::STRICT,
            "relaxed" => Self::RELAXED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<IDBTransactionDurability> for Any {
    fn from(s: IDBTransactionDurability) -> Any {
         match s {
            IDBTransactionDurability::DEFAULT => Any::from("default"),
            IDBTransactionDurability::STRICT => Any::from("strict"),
            IDBTransactionDurability::RELAXED => Any::from("relaxed"),
         }
    }
}
impl From<&IDBTransactionDurability> for Any {
    fn from(s: &IDBTransactionDurability) -> Any {
         match *s {
            IDBTransactionDurability::DEFAULT => Any::from("default"),
            IDBTransactionDurability::STRICT => Any::from("strict"),
            IDBTransactionDurability::RELAXED => Any::from("relaxed"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "next" => Self::NEXT,
            "nextunique" => Self::NEXTUNIQUE,
            "prev" => Self::PREV,
            "prevunique" => Self::PREVUNIQUE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<IDBCursorDirection> for Any {
    fn from(s: IDBCursorDirection) -> Any {
         match s {
            IDBCursorDirection::NEXT => Any::from("next"),
            IDBCursorDirection::NEXTUNIQUE => Any::from("nextunique"),
            IDBCursorDirection::PREV => Any::from("prev"),
            IDBCursorDirection::PREVUNIQUE => Any::from("prevunique"),
         }
    }
}
impl From<&IDBCursorDirection> for Any {
    fn from(s: &IDBCursorDirection) -> Any {
         match *s {
            IDBCursorDirection::NEXT => Any::from("next"),
            IDBCursorDirection::NEXTUNIQUE => Any::from("nextunique"),
            IDBCursorDirection::PREV => Any::from("prev"),
            IDBCursorDirection::PREVUNIQUE => Any::from("prevunique"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "readonly" => Self::READONLY,
            "readwrite" => Self::READWRITE,
            "versionchange" => Self::VERSIONCHANGE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<IDBTransactionMode> for Any {
    fn from(s: IDBTransactionMode) -> Any {
         match s {
            IDBTransactionMode::READONLY => Any::from("readonly"),
            IDBTransactionMode::READWRITE => Any::from("readwrite"),
            IDBTransactionMode::VERSIONCHANGE => Any::from("versionchange"),
         }
    }
}
impl From<&IDBTransactionMode> for Any {
    fn from(s: &IDBTransactionMode) -> Any {
         match *s {
            IDBTransactionMode::READONLY => Any::from("readonly"),
            IDBTransactionMode::READWRITE => Any::from("readwrite"),
            IDBTransactionMode::VERSIONCHANGE => Any::from("versionchange"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AccelerometerLocalCoordinateSystem {
    DEVICE,
    SCREEN,
}
impl FromVal for AccelerometerLocalCoordinateSystem {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "device" => Self::DEVICE,
            "screen" => Self::SCREEN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AccelerometerLocalCoordinateSystem> for Any {
    fn from(s: AccelerometerLocalCoordinateSystem) -> Any {
         match s {
            AccelerometerLocalCoordinateSystem::DEVICE => Any::from("device"),
            AccelerometerLocalCoordinateSystem::SCREEN => Any::from("screen"),
         }
    }
}
impl From<&AccelerometerLocalCoordinateSystem> for Any {
    fn from(s: &AccelerometerLocalCoordinateSystem) -> Any {
         match *s {
            AccelerometerLocalCoordinateSystem::DEVICE => Any::from("device"),
            AccelerometerLocalCoordinateSystem::SCREEN => Any::from("screen"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
            "playback" => Self::PLAYBACK,
            "transient" => Self::TRANSIENT,
            "transient-solo" => Self::TRANSIENT_SOLO,
            "ambient" => Self::AMBIENT,
            "play-and-record" => Self::PLAY_AND_RECORD,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AudioSessionType> for Any {
    fn from(s: AudioSessionType) -> Any {
         match s {
            AudioSessionType::AUTO => Any::from("auto"),
            AudioSessionType::PLAYBACK => Any::from("playback"),
            AudioSessionType::TRANSIENT => Any::from("transient"),
            AudioSessionType::TRANSIENT_SOLO => Any::from("transient-solo"),
            AudioSessionType::AMBIENT => Any::from("ambient"),
            AudioSessionType::PLAY_AND_RECORD => Any::from("play-and-record"),
         }
    }
}
impl From<&AudioSessionType> for Any {
    fn from(s: &AudioSessionType) -> Any {
         match *s {
            AudioSessionType::AUTO => Any::from("auto"),
            AudioSessionType::PLAYBACK => Any::from("playback"),
            AudioSessionType::TRANSIENT => Any::from("transient"),
            AudioSessionType::TRANSIENT_SOLO => Any::from("transient-solo"),
            AudioSessionType::AMBIENT => Any::from("ambient"),
            AudioSessionType::PLAY_AND_RECORD => Any::from("play-and-record"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "inactive" => Self::INACTIVE,
            "active" => Self::ACTIVE,
            "interrupted" => Self::INTERRUPTED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AudioSessionState> for Any {
    fn from(s: AudioSessionState) -> Any {
         match s {
            AudioSessionState::INACTIVE => Any::from("inactive"),
            AudioSessionState::ACTIVE => Any::from("active"),
            AudioSessionState::INTERRUPTED => Any::from("interrupted"),
         }
    }
}
impl From<&AudioSessionState> for Any {
    fn from(s: &AudioSessionState) -> Any {
         match *s {
            AudioSessionState::INACTIVE => Any::from("inactive"),
            AudioSessionState::ACTIVE => Any::from("active"),
            AudioSessionState::INTERRUPTED => Any::from("interrupted"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "allowed" => Self::ALLOWED,
            "allowed-muted" => Self::ALLOWED_MUTED,
            "disallowed" => Self::DISALLOWED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AutoplayPolicy> for Any {
    fn from(s: AutoplayPolicy) -> Any {
         match s {
            AutoplayPolicy::ALLOWED => Any::from("allowed"),
            AutoplayPolicy::ALLOWED_MUTED => Any::from("allowed-muted"),
            AutoplayPolicy::DISALLOWED => Any::from("disallowed"),
         }
    }
}
impl From<&AutoplayPolicy> for Any {
    fn from(s: &AutoplayPolicy) -> Any {
         match *s {
            AutoplayPolicy::ALLOWED => Any::from("allowed"),
            AutoplayPolicy::ALLOWED_MUTED => Any::from("allowed-muted"),
            AutoplayPolicy::DISALLOWED => Any::from("disallowed"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AutoplayPolicyMediaType {
    MEDIAELEMENT,
    AUDIOCONTEXT,
}
impl FromVal for AutoplayPolicyMediaType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "mediaelement" => Self::MEDIAELEMENT,
            "audiocontext" => Self::AUDIOCONTEXT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AutoplayPolicyMediaType> for Any {
    fn from(s: AutoplayPolicyMediaType) -> Any {
         match s {
            AutoplayPolicyMediaType::MEDIAELEMENT => Any::from("mediaelement"),
            AutoplayPolicyMediaType::AUDIOCONTEXT => Any::from("audiocontext"),
         }
    }
}
impl From<&AutoplayPolicyMediaType> for Any {
    fn from(s: &AutoplayPolicyMediaType) -> Any {
         match *s {
            AutoplayPolicyMediaType::MEDIAELEMENT => Any::from("mediaelement"),
            AutoplayPolicyMediaType::AUDIOCONTEXT => Any::from("audiocontext"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "" => Self::NONE,
            "success" => Self::SUCCESS,
            "failure" => Self::FAILURE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<BackgroundFetchResult> for Any {
    fn from(s: BackgroundFetchResult) -> Any {
         match s {
            BackgroundFetchResult::NONE => Any::from(""),
            BackgroundFetchResult::SUCCESS => Any::from("success"),
            BackgroundFetchResult::FAILURE => Any::from("failure"),
         }
    }
}
impl From<&BackgroundFetchResult> for Any {
    fn from(s: &BackgroundFetchResult) -> Any {
         match *s {
            BackgroundFetchResult::NONE => Any::from(""),
            BackgroundFetchResult::SUCCESS => Any::from("success"),
            BackgroundFetchResult::FAILURE => Any::from("failure"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "" => Self::NONE,
            "aborted" => Self::ABORTED,
            "bad-status" => Self::BAD_STATUS,
            "fetch-error" => Self::FETCH_ERROR,
            "quota-exceeded" => Self::QUOTA_EXCEEDED,
            "download-total-exceeded" => Self::DOWNLOAD_TOTAL_EXCEEDED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<BackgroundFetchFailureReason> for Any {
    fn from(s: BackgroundFetchFailureReason) -> Any {
         match s {
            BackgroundFetchFailureReason::NONE => Any::from(""),
            BackgroundFetchFailureReason::ABORTED => Any::from("aborted"),
            BackgroundFetchFailureReason::BAD_STATUS => Any::from("bad-status"),
            BackgroundFetchFailureReason::FETCH_ERROR => Any::from("fetch-error"),
            BackgroundFetchFailureReason::QUOTA_EXCEEDED => Any::from("quota-exceeded"),
            BackgroundFetchFailureReason::DOWNLOAD_TOTAL_EXCEEDED => Any::from("download-total-exceeded"),
         }
    }
}
impl From<&BackgroundFetchFailureReason> for Any {
    fn from(s: &BackgroundFetchFailureReason) -> Any {
         match *s {
            BackgroundFetchFailureReason::NONE => Any::from(""),
            BackgroundFetchFailureReason::ABORTED => Any::from("aborted"),
            BackgroundFetchFailureReason::BAD_STATUS => Any::from("bad-status"),
            BackgroundFetchFailureReason::FETCH_ERROR => Any::from("fetch-error"),
            BackgroundFetchFailureReason::QUOTA_EXCEEDED => Any::from("quota-exceeded"),
            BackgroundFetchFailureReason::DOWNLOAD_TOTAL_EXCEEDED => Any::from("download-total-exceeded"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "unspecified" => Self::UNSPECIFIED,
            "inline" => Self::INLINE,
            "attachment" => Self::ATTACHMENT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PresentationStyle> for Any {
    fn from(s: PresentationStyle) -> Any {
         match s {
            PresentationStyle::UNSPECIFIED => Any::from("unspecified"),
            PresentationStyle::INLINE => Any::from("inline"),
            PresentationStyle::ATTACHMENT => Any::from("attachment"),
         }
    }
}
impl From<&PresentationStyle> for Any {
    fn from(s: &PresentationStyle) -> Any {
         match *s {
            PresentationStyle::UNSPECIFIED => Any::from("unspecified"),
            PresentationStyle::INLINE => Any::from("inline"),
            PresentationStyle::ATTACHMENT => Any::from("attachment"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "deflate" => Self::DEFLATE,
            "deflate-raw" => Self::DEFLATE_RAW,
            "gzip" => Self::GZIP,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CompressionFormat> for Any {
    fn from(s: CompressionFormat) -> Any {
         match s {
            CompressionFormat::DEFLATE => Any::from("deflate"),
            CompressionFormat::DEFLATE_RAW => Any::from("deflate-raw"),
            CompressionFormat::GZIP => Any::from("gzip"),
         }
    }
}
impl From<&CompressionFormat> for Any {
    fn from(s: &CompressionFormat) -> Any {
         match *s {
            CompressionFormat::DEFLATE => Any::from("deflate"),
            CompressionFormat::DEFLATE_RAW => Any::from("deflate-raw"),
            CompressionFormat::GZIP => Any::from("gzip"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PressureSource {
    CPU,
}
impl FromVal for PressureSource {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "cpu" => Self::CPU,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PressureSource> for Any {
    fn from(s: PressureSource) -> Any {
         match s {
            PressureSource::CPU => Any::from("cpu"),
         }
    }
}
impl From<&PressureSource> for Any {
    fn from(s: &PressureSource) -> Any {
         match *s {
            PressureSource::CPU => Any::from("cpu"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "nominal" => Self::NOMINAL,
            "fair" => Self::FAIR,
            "serious" => Self::SERIOUS,
            "critical" => Self::CRITICAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PressureState> for Any {
    fn from(s: PressureState) -> Any {
         match s {
            PressureState::NOMINAL => Any::from("nominal"),
            PressureState::FAIR => Any::from("fair"),
            PressureState::SERIOUS => Any::from("serious"),
            PressureState::CRITICAL => Any::from("critical"),
         }
    }
}
impl From<&PressureState> for Any {
    fn from(s: &PressureState) -> Any {
         match *s {
            PressureState::NOMINAL => Any::from("nominal"),
            PressureState::FAIR => Any::from("fair"),
            PressureState::SERIOUS => Any::from("serious"),
            PressureState::CRITICAL => Any::from("critical"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "address" => Self::ADDRESS,
            "email" => Self::EMAIL,
            "icon" => Self::ICON,
            "name" => Self::NAME,
            "tel" => Self::TEL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ContactProperty> for Any {
    fn from(s: ContactProperty) -> Any {
         match s {
            ContactProperty::ADDRESS => Any::from("address"),
            ContactProperty::EMAIL => Any::from("email"),
            ContactProperty::ICON => Any::from("icon"),
            ContactProperty::NAME => Any::from("name"),
            ContactProperty::TEL => Any::from("tel"),
         }
    }
}
impl From<&ContactProperty> for Any {
    fn from(s: &ContactProperty) -> Any {
         match *s {
            ContactProperty::ADDRESS => Any::from("address"),
            ContactProperty::EMAIL => Any::from("email"),
            ContactProperty::ICON => Any::from("icon"),
            ContactProperty::NAME => Any::from("name"),
            ContactProperty::TEL => Any::from("tel"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "" => Self::NONE,
            "homepage" => Self::HOMEPAGE,
            "article" => Self::ARTICLE,
            "video" => Self::VIDEO,
            "audio" => Self::AUDIO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ContentCategory> for Any {
    fn from(s: ContentCategory) -> Any {
         match s {
            ContentCategory::NONE => Any::from(""),
            ContentCategory::HOMEPAGE => Any::from("homepage"),
            ContentCategory::ARTICLE => Any::from("article"),
            ContentCategory::VIDEO => Any::from("video"),
            ContentCategory::AUDIO => Any::from("audio"),
         }
    }
}
impl From<&ContentCategory> for Any {
    fn from(s: &ContentCategory) -> Any {
         match *s {
            ContentCategory::NONE => Any::from(""),
            ContentCategory::HOMEPAGE => Any::from("homepage"),
            ContentCategory::ARTICLE => Any::from("article"),
            ContentCategory::VIDEO => Any::from("video"),
            ContentCategory::AUDIO => Any::from("audio"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "strict" => Self::STRICT,
            "lax" => Self::LAX,
            "none" => Self::NONE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CookieSameSite> for Any {
    fn from(s: CookieSameSite) -> Any {
         match s {
            CookieSameSite::STRICT => Any::from("strict"),
            CookieSameSite::LAX => Any::from("lax"),
            CookieSameSite::NONE => Any::from("none"),
         }
    }
}
impl From<&CookieSameSite> for Any {
    fn from(s: &CookieSameSite) -> Any {
         match *s {
            CookieSameSite::STRICT => Any::from("strict"),
            CookieSameSite::LAX => Any::from("lax"),
            CookieSameSite::NONE => Any::from("none"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "silent" => Self::SILENT,
            "optional" => Self::OPTIONAL,
            "conditional" => Self::CONDITIONAL,
            "required" => Self::REQUIRED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CredentialMediationRequirement> for Any {
    fn from(s: CredentialMediationRequirement) -> Any {
         match s {
            CredentialMediationRequirement::SILENT => Any::from("silent"),
            CredentialMediationRequirement::OPTIONAL => Any::from("optional"),
            CredentialMediationRequirement::CONDITIONAL => Any::from("conditional"),
            CredentialMediationRequirement::REQUIRED => Any::from("required"),
         }
    }
}
impl From<&CredentialMediationRequirement> for Any {
    fn from(s: &CredentialMediationRequirement) -> Any {
         match *s {
            CredentialMediationRequirement::SILENT => Any::from("silent"),
            CredentialMediationRequirement::OPTIONAL => Any::from("optional"),
            CredentialMediationRequirement::CONDITIONAL => Any::from("conditional"),
            CredentialMediationRequirement::REQUIRED => Any::from("required"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "externalScript" => Self::EXTERNAL_SCRIPT,
            "inlineScript" => Self::INLINE_SCRIPT,
            "inlineEventHandler" => Self::INLINE_EVENT_HANDLER,
            "eval" => Self::EVAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ScriptingPolicyViolationType> for Any {
    fn from(s: ScriptingPolicyViolationType) -> Any {
         match s {
            ScriptingPolicyViolationType::EXTERNAL_SCRIPT => Any::from("externalScript"),
            ScriptingPolicyViolationType::INLINE_SCRIPT => Any::from("inlineScript"),
            ScriptingPolicyViolationType::INLINE_EVENT_HANDLER => Any::from("inlineEventHandler"),
            ScriptingPolicyViolationType::EVAL => Any::from("eval"),
         }
    }
}
impl From<&ScriptingPolicyViolationType> for Any {
    fn from(s: &ScriptingPolicyViolationType) -> Any {
         match *s {
            ScriptingPolicyViolationType::EXTERNAL_SCRIPT => Any::from("externalScript"),
            ScriptingPolicyViolationType::INLINE_SCRIPT => Any::from("inlineScript"),
            ScriptingPolicyViolationType::INLINE_EVENT_HANDLER => Any::from("inlineEventHandler"),
            ScriptingPolicyViolationType::EVAL => Any::from("eval"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "unloaded" => Self::UNLOADED,
            "loading" => Self::LOADING,
            "loaded" => Self::LOADED,
            "error" => Self::ERROR,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FontFaceLoadStatus> for Any {
    fn from(s: FontFaceLoadStatus) -> Any {
         match s {
            FontFaceLoadStatus::UNLOADED => Any::from("unloaded"),
            FontFaceLoadStatus::LOADING => Any::from("loading"),
            FontFaceLoadStatus::LOADED => Any::from("loaded"),
            FontFaceLoadStatus::ERROR => Any::from("error"),
         }
    }
}
impl From<&FontFaceLoadStatus> for Any {
    fn from(s: &FontFaceLoadStatus) -> Any {
         match *s {
            FontFaceLoadStatus::UNLOADED => Any::from("unloaded"),
            FontFaceLoadStatus::LOADING => Any::from("loading"),
            FontFaceLoadStatus::LOADED => Any::from("loaded"),
            FontFaceLoadStatus::ERROR => Any::from("error"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FontFaceSetLoadStatus {
    LOADING,
    LOADED,
}
impl FromVal for FontFaceSetLoadStatus {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "loading" => Self::LOADING,
            "loaded" => Self::LOADED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FontFaceSetLoadStatus> for Any {
    fn from(s: FontFaceSetLoadStatus) -> Any {
         match s {
            FontFaceSetLoadStatus::LOADING => Any::from("loading"),
            FontFaceSetLoadStatus::LOADED => Any::from("loaded"),
         }
    }
}
impl From<&FontFaceSetLoadStatus> for Any {
    fn from(s: &FontFaceSetLoadStatus) -> Any {
         match *s {
            FontFaceSetLoadStatus::LOADING => Any::from("loading"),
            FontFaceSetLoadStatus::LOADED => Any::from("loaded"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "highlight" => Self::HIGHLIGHT,
            "spelling-error" => Self::SPELLING_ERROR,
            "grammar-error" => Self::GRAMMAR_ERROR,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<HighlightType> for Any {
    fn from(s: HighlightType) -> Any {
         match s {
            HighlightType::HIGHLIGHT => Any::from("highlight"),
            HighlightType::SPELLING_ERROR => Any::from("spelling-error"),
            HighlightType::GRAMMAR_ERROR => Any::from("grammar-error"),
         }
    }
}
impl From<&HighlightType> for Any {
    fn from(s: &HighlightType) -> Any {
         match *s {
            HighlightType::HIGHLIGHT => Any::from("highlight"),
            HighlightType::SPELLING_ERROR => Any::from("spelling-error"),
            HighlightType::GRAMMAR_ERROR => Any::from("grammar-error"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ChildDisplayType {
    BLOCK,
    NORMAL,
}
impl FromVal for ChildDisplayType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "block" => Self::BLOCK,
            "normal" => Self::NORMAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ChildDisplayType> for Any {
    fn from(s: ChildDisplayType) -> Any {
         match s {
            ChildDisplayType::BLOCK => Any::from("block"),
            ChildDisplayType::NORMAL => Any::from("normal"),
         }
    }
}
impl From<&ChildDisplayType> for Any {
    fn from(s: &ChildDisplayType) -> Any {
         match *s {
            ChildDisplayType::BLOCK => Any::from("block"),
            ChildDisplayType::NORMAL => Any::from("normal"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LayoutSizingMode {
    BLOCK_LIKE,
    MANUAL,
}
impl FromVal for LayoutSizingMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "block-like" => Self::BLOCK_LIKE,
            "manual" => Self::MANUAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<LayoutSizingMode> for Any {
    fn from(s: LayoutSizingMode) -> Any {
         match s {
            LayoutSizingMode::BLOCK_LIKE => Any::from("block-like"),
            LayoutSizingMode::MANUAL => Any::from("manual"),
         }
    }
}
impl From<&LayoutSizingMode> for Any {
    fn from(s: &LayoutSizingMode) -> Any {
         match *s {
            LayoutSizingMode::BLOCK_LIKE => Any::from("block-like"),
            LayoutSizingMode::MANUAL => Any::from("manual"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "page" => Self::PAGE,
            "column" => Self::COLUMN,
            "region" => Self::REGION,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<BlockFragmentationType> for Any {
    fn from(s: BlockFragmentationType) -> Any {
         match s {
            BlockFragmentationType::NONE => Any::from("none"),
            BlockFragmentationType::PAGE => Any::from("page"),
            BlockFragmentationType::COLUMN => Any::from("column"),
            BlockFragmentationType::REGION => Any::from("region"),
         }
    }
}
impl From<&BlockFragmentationType> for Any {
    fn from(s: &BlockFragmentationType) -> Any {
         match *s {
            BlockFragmentationType::NONE => Any::from("none"),
            BlockFragmentationType::PAGE => Any::from("page"),
            BlockFragmentationType::COLUMN => Any::from("column"),
            BlockFragmentationType::REGION => Any::from("region"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "line" => Self::LINE,
            "column" => Self::COLUMN,
            "page" => Self::PAGE,
            "region" => Self::REGION,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<BreakType> for Any {
    fn from(s: BreakType) -> Any {
         match s {
            BreakType::NONE => Any::from("none"),
            BreakType::LINE => Any::from("line"),
            BreakType::COLUMN => Any::from("column"),
            BreakType::PAGE => Any::from("page"),
            BreakType::REGION => Any::from("region"),
         }
    }
}
impl From<&BreakType> for Any {
    fn from(s: &BreakType) -> Any {
         match *s {
            BreakType::NONE => Any::from("none"),
            BreakType::LINE => Any::from("line"),
            BreakType::COLUMN => Any::from("column"),
            BreakType::PAGE => Any::from("page"),
            BreakType::REGION => Any::from("region"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "up" => Self::UP,
            "down" => Self::DOWN,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SpatialNavigationDirection> for Any {
    fn from(s: SpatialNavigationDirection) -> Any {
         match s {
            SpatialNavigationDirection::UP => Any::from("up"),
            SpatialNavigationDirection::DOWN => Any::from("down"),
            SpatialNavigationDirection::LEFT => Any::from("left"),
            SpatialNavigationDirection::RIGHT => Any::from("right"),
         }
    }
}
impl From<&SpatialNavigationDirection> for Any {
    fn from(s: &SpatialNavigationDirection) -> Any {
         match *s {
            SpatialNavigationDirection::UP => Any::from("up"),
            SpatialNavigationDirection::DOWN => Any::from("down"),
            SpatialNavigationDirection::LEFT => Any::from("left"),
            SpatialNavigationDirection::RIGHT => Any::from("right"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FocusableAreaSearchMode {
    VISIBLE,
    ALL,
}
impl FromVal for FocusableAreaSearchMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "visible" => Self::VISIBLE,
            "all" => Self::ALL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FocusableAreaSearchMode> for Any {
    fn from(s: FocusableAreaSearchMode) -> Any {
         match s {
            FocusableAreaSearchMode::VISIBLE => Any::from("visible"),
            FocusableAreaSearchMode::ALL => Any::from("all"),
         }
    }
}
impl From<&FocusableAreaSearchMode> for Any {
    fn from(s: &FocusableAreaSearchMode) -> Any {
         match *s {
            FocusableAreaSearchMode::VISIBLE => Any::from("visible"),
            FocusableAreaSearchMode::ALL => Any::from("all"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CSSNumericBaseType> for Any {
    fn from(s: CSSNumericBaseType) -> Any {
         match s {
            CSSNumericBaseType::LENGTH => Any::from("length"),
            CSSNumericBaseType::ANGLE => Any::from("angle"),
            CSSNumericBaseType::TIME => Any::from("time"),
            CSSNumericBaseType::FREQUENCY => Any::from("frequency"),
            CSSNumericBaseType::RESOLUTION => Any::from("resolution"),
            CSSNumericBaseType::FLEX => Any::from("flex"),
            CSSNumericBaseType::PERCENT => Any::from("percent"),
         }
    }
}
impl From<&CSSNumericBaseType> for Any {
    fn from(s: &CSSNumericBaseType) -> Any {
         match *s {
            CSSNumericBaseType::LENGTH => Any::from("length"),
            CSSNumericBaseType::ANGLE => Any::from("angle"),
            CSSNumericBaseType::TIME => Any::from("time"),
            CSSNumericBaseType::FREQUENCY => Any::from("frequency"),
            CSSNumericBaseType::RESOLUTION => Any::from("resolution"),
            CSSNumericBaseType::FLEX => Any::from("flex"),
            CSSNumericBaseType::PERCENT => Any::from("percent"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CSSMathOperator> for Any {
    fn from(s: CSSMathOperator) -> Any {
         match s {
            CSSMathOperator::SUM => Any::from("sum"),
            CSSMathOperator::PRODUCT => Any::from("product"),
            CSSMathOperator::NEGATE => Any::from("negate"),
            CSSMathOperator::INVERT => Any::from("invert"),
            CSSMathOperator::MIN => Any::from("min"),
            CSSMathOperator::MAX => Any::from("max"),
            CSSMathOperator::CLAMP => Any::from("clamp"),
         }
    }
}
impl From<&CSSMathOperator> for Any {
    fn from(s: &CSSMathOperator) -> Any {
         match *s {
            CSSMathOperator::SUM => Any::from("sum"),
            CSSMathOperator::PRODUCT => Any::from("product"),
            CSSMathOperator::NEGATE => Any::from("negate"),
            CSSMathOperator::INVERT => Any::from("invert"),
            CSSMathOperator::MIN => Any::from("min"),
            CSSMathOperator::MAX => Any::from("max"),
            CSSMathOperator::CLAMP => Any::from("clamp"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
            "instant" => Self::INSTANT,
            "smooth" => Self::SMOOTH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ScrollBehavior> for Any {
    fn from(s: ScrollBehavior) -> Any {
         match s {
            ScrollBehavior::AUTO => Any::from("auto"),
            ScrollBehavior::INSTANT => Any::from("instant"),
            ScrollBehavior::SMOOTH => Any::from("smooth"),
         }
    }
}
impl From<&ScrollBehavior> for Any {
    fn from(s: &ScrollBehavior) -> Any {
         match *s {
            ScrollBehavior::AUTO => Any::from("auto"),
            ScrollBehavior::INSTANT => Any::from("instant"),
            ScrollBehavior::SMOOTH => Any::from("smooth"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "start" => Self::START,
            "center" => Self::CENTER,
            "end" => Self::END,
            "nearest" => Self::NEAREST,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ScrollLogicalPosition> for Any {
    fn from(s: ScrollLogicalPosition) -> Any {
         match s {
            ScrollLogicalPosition::START => Any::from("start"),
            ScrollLogicalPosition::CENTER => Any::from("center"),
            ScrollLogicalPosition::END => Any::from("end"),
            ScrollLogicalPosition::NEAREST => Any::from("nearest"),
         }
    }
}
impl From<&ScrollLogicalPosition> for Any {
    fn from(s: &ScrollLogicalPosition) -> Any {
         match *s {
            ScrollLogicalPosition::START => Any::from("start"),
            ScrollLogicalPosition::CENTER => Any::from("center"),
            ScrollLogicalPosition::END => Any::from("end"),
            ScrollLogicalPosition::NEAREST => Any::from("nearest"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScrollIntoViewContainer {
    ALL,
    NEAREST,
}
impl FromVal for ScrollIntoViewContainer {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "all" => Self::ALL,
            "nearest" => Self::NEAREST,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ScrollIntoViewContainer> for Any {
    fn from(s: ScrollIntoViewContainer) -> Any {
         match s {
            ScrollIntoViewContainer::ALL => Any::from("all"),
            ScrollIntoViewContainer::NEAREST => Any::from("nearest"),
         }
    }
}
impl From<&ScrollIntoViewContainer> for Any {
    fn from(s: &ScrollIntoViewContainer) -> Any {
         match *s {
            ScrollIntoViewContainer::ALL => Any::from("all"),
            ScrollIntoViewContainer::NEAREST => Any::from("nearest"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "margin" => Self::MARGIN,
            "border" => Self::BORDER,
            "padding" => Self::PADDING,
            "content" => Self::CONTENT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CSSBoxType> for Any {
    fn from(s: CSSBoxType) -> Any {
         match s {
            CSSBoxType::MARGIN => Any::from("margin"),
            CSSBoxType::BORDER => Any::from("border"),
            CSSBoxType::PADDING => Any::from("padding"),
            CSSBoxType::CONTENT => Any::from("content"),
         }
    }
}
impl From<&CSSBoxType> for Any {
    fn from(s: &CSSBoxType) -> Any {
         match *s {
            CSSBoxType::MARGIN => Any::from("margin"),
            CSSBoxType::BORDER => Any::from("border"),
            CSSBoxType::PADDING => Any::from("padding"),
            CSSBoxType::CONTENT => Any::from("content"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DevicePostureType {
    CONTINUOUS,
    FOLDED,
}
impl FromVal for DevicePostureType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "continuous" => Self::CONTINUOUS,
            "folded" => Self::FOLDED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<DevicePostureType> for Any {
    fn from(s: DevicePostureType) -> Any {
         match s {
            DevicePostureType::CONTINUOUS => Any::from("continuous"),
            DevicePostureType::FOLDED => Any::from("folded"),
         }
    }
}
impl From<&DevicePostureType> for Any {
    fn from(s: &DevicePostureType) -> Any {
         match *s {
            DevicePostureType::CONTINUOUS => Any::from("continuous"),
            DevicePostureType::FOLDED => Any::from("folded"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ItemType {
    PRODUCT,
    SUBSCRIPTION,
}
impl FromVal for ItemType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "product" => Self::PRODUCT,
            "subscription" => Self::SUBSCRIPTION,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ItemType> for Any {
    fn from(s: ItemType) -> Any {
         match s {
            ItemType::PRODUCT => Any::from("product"),
            ItemType::SUBSCRIPTION => Any::from("subscription"),
         }
    }
}
impl From<&ItemType> for Any {
    fn from(s: &ItemType) -> Any {
         match *s {
            ItemType::PRODUCT => Any::from("product"),
            ItemType::SUBSCRIPTION => Any::from("subscription"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ShadowRootMode {
    OPEN,
    CLOSED,
}
impl FromVal for ShadowRootMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "open" => Self::OPEN,
            "closed" => Self::CLOSED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ShadowRootMode> for Any {
    fn from(s: ShadowRootMode) -> Any {
         match s {
            ShadowRootMode::OPEN => Any::from("open"),
            ShadowRootMode::CLOSED => Any::from("closed"),
         }
    }
}
impl From<&ShadowRootMode> for Any {
    fn from(s: &ShadowRootMode) -> Any {
         match *s {
            ShadowRootMode::OPEN => Any::from("open"),
            ShadowRootMode::CLOSED => Any::from("closed"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SlotAssignmentMode {
    MANUAL,
    NAMED,
}
impl FromVal for SlotAssignmentMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "manual" => Self::MANUAL,
            "named" => Self::NAMED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SlotAssignmentMode> for Any {
    fn from(s: SlotAssignmentMode) -> Any {
         match s {
            SlotAssignmentMode::MANUAL => Any::from("manual"),
            SlotAssignmentMode::NAMED => Any::from("named"),
         }
    }
}
impl From<&SlotAssignmentMode> for Any {
    fn from(s: &SlotAssignmentMode) -> Any {
         match *s {
            SlotAssignmentMode::MANUAL => Any::from("manual"),
            SlotAssignmentMode::NAMED => Any::from("named"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "solid" => Self::SOLID,
            "dotted" => Self::DOTTED,
            "dashed" => Self::DASHED,
            "wavy" => Self::WAVY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<UnderlineStyle> for Any {
    fn from(s: UnderlineStyle) -> Any {
         match s {
            UnderlineStyle::NONE => Any::from("none"),
            UnderlineStyle::SOLID => Any::from("solid"),
            UnderlineStyle::DOTTED => Any::from("dotted"),
            UnderlineStyle::DASHED => Any::from("dashed"),
            UnderlineStyle::WAVY => Any::from("wavy"),
         }
    }
}
impl From<&UnderlineStyle> for Any {
    fn from(s: &UnderlineStyle) -> Any {
         match *s {
            UnderlineStyle::NONE => Any::from("none"),
            UnderlineStyle::SOLID => Any::from("solid"),
            UnderlineStyle::DOTTED => Any::from("dotted"),
            UnderlineStyle::DASHED => Any::from("dashed"),
            UnderlineStyle::WAVY => Any::from("wavy"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "thin" => Self::THIN,
            "thick" => Self::THICK,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<UnderlineThickness> for Any {
    fn from(s: UnderlineThickness) -> Any {
         match s {
            UnderlineThickness::NONE => Any::from("none"),
            UnderlineThickness::THIN => Any::from("thin"),
            UnderlineThickness::THICK => Any::from("thick"),
         }
    }
}
impl From<&UnderlineThickness> for Any {
    fn from(s: &UnderlineThickness) -> Any {
         match *s {
            UnderlineThickness::NONE => Any::from("none"),
            UnderlineThickness::THIN => Any::from("thin"),
            UnderlineThickness::THICK => Any::from("thick"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "required" => Self::REQUIRED,
            "optional" => Self::OPTIONAL,
            "not-allowed" => Self::NOT_ALLOWED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaKeysRequirement> for Any {
    fn from(s: MediaKeysRequirement) -> Any {
         match s {
            MediaKeysRequirement::REQUIRED => Any::from("required"),
            MediaKeysRequirement::OPTIONAL => Any::from("optional"),
            MediaKeysRequirement::NOT_ALLOWED => Any::from("not-allowed"),
         }
    }
}
impl From<&MediaKeysRequirement> for Any {
    fn from(s: &MediaKeysRequirement) -> Any {
         match *s {
            MediaKeysRequirement::REQUIRED => Any::from("required"),
            MediaKeysRequirement::OPTIONAL => Any::from("optional"),
            MediaKeysRequirement::NOT_ALLOWED => Any::from("not-allowed"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaKeySessionType {
    TEMPORARY,
    PERSISTENT_LICENSE,
}
impl FromVal for MediaKeySessionType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "temporary" => Self::TEMPORARY,
            "persistent-license" => Self::PERSISTENT_LICENSE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaKeySessionType> for Any {
    fn from(s: MediaKeySessionType) -> Any {
         match s {
            MediaKeySessionType::TEMPORARY => Any::from("temporary"),
            MediaKeySessionType::PERSISTENT_LICENSE => Any::from("persistent-license"),
         }
    }
}
impl From<&MediaKeySessionType> for Any {
    fn from(s: &MediaKeySessionType) -> Any {
         match *s {
            MediaKeySessionType::TEMPORARY => Any::from("temporary"),
            MediaKeySessionType::PERSISTENT_LICENSE => Any::from("persistent-license"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "internal-error" => Self::INTERNAL_ERROR,
            "closed-by-application" => Self::CLOSED_BY_APPLICATION,
            "release-acknowledged" => Self::RELEASE_ACKNOWLEDGED,
            "hardware-context-reset" => Self::HARDWARE_CONTEXT_RESET,
            "resource-evicted" => Self::RESOURCE_EVICTED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaKeySessionClosedReason> for Any {
    fn from(s: MediaKeySessionClosedReason) -> Any {
         match s {
            MediaKeySessionClosedReason::INTERNAL_ERROR => Any::from("internal-error"),
            MediaKeySessionClosedReason::CLOSED_BY_APPLICATION => Any::from("closed-by-application"),
            MediaKeySessionClosedReason::RELEASE_ACKNOWLEDGED => Any::from("release-acknowledged"),
            MediaKeySessionClosedReason::HARDWARE_CONTEXT_RESET => Any::from("hardware-context-reset"),
            MediaKeySessionClosedReason::RESOURCE_EVICTED => Any::from("resource-evicted"),
         }
    }
}
impl From<&MediaKeySessionClosedReason> for Any {
    fn from(s: &MediaKeySessionClosedReason) -> Any {
         match *s {
            MediaKeySessionClosedReason::INTERNAL_ERROR => Any::from("internal-error"),
            MediaKeySessionClosedReason::CLOSED_BY_APPLICATION => Any::from("closed-by-application"),
            MediaKeySessionClosedReason::RELEASE_ACKNOWLEDGED => Any::from("release-acknowledged"),
            MediaKeySessionClosedReason::HARDWARE_CONTEXT_RESET => Any::from("hardware-context-reset"),
            MediaKeySessionClosedReason::RESOURCE_EVICTED => Any::from("resource-evicted"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaKeyStatus> for Any {
    fn from(s: MediaKeyStatus) -> Any {
         match s {
            MediaKeyStatus::USABLE => Any::from("usable"),
            MediaKeyStatus::EXPIRED => Any::from("expired"),
            MediaKeyStatus::RELEASED => Any::from("released"),
            MediaKeyStatus::OUTPUT_RESTRICTED => Any::from("output-restricted"),
            MediaKeyStatus::OUTPUT_DOWNSCALED => Any::from("output-downscaled"),
            MediaKeyStatus::USABLE_IN_FUTURE => Any::from("usable-in-future"),
            MediaKeyStatus::STATUS_PENDING => Any::from("status-pending"),
            MediaKeyStatus::INTERNAL_ERROR => Any::from("internal-error"),
         }
    }
}
impl From<&MediaKeyStatus> for Any {
    fn from(s: &MediaKeyStatus) -> Any {
         match *s {
            MediaKeyStatus::USABLE => Any::from("usable"),
            MediaKeyStatus::EXPIRED => Any::from("expired"),
            MediaKeyStatus::RELEASED => Any::from("released"),
            MediaKeyStatus::OUTPUT_RESTRICTED => Any::from("output-restricted"),
            MediaKeyStatus::OUTPUT_DOWNSCALED => Any::from("output-downscaled"),
            MediaKeyStatus::USABLE_IN_FUTURE => Any::from("usable-in-future"),
            MediaKeyStatus::STATUS_PENDING => Any::from("status-pending"),
            MediaKeyStatus::INTERNAL_ERROR => Any::from("internal-error"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "license-request" => Self::LICENSE_REQUEST,
            "license-renewal" => Self::LICENSE_RENEWAL,
            "license-release" => Self::LICENSE_RELEASE,
            "individualization-request" => Self::INDIVIDUALIZATION_REQUEST,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaKeyMessageType> for Any {
    fn from(s: MediaKeyMessageType) -> Any {
         match s {
            MediaKeyMessageType::LICENSE_REQUEST => Any::from("license-request"),
            MediaKeyMessageType::LICENSE_RENEWAL => Any::from("license-renewal"),
            MediaKeyMessageType::LICENSE_RELEASE => Any::from("license-release"),
            MediaKeyMessageType::INDIVIDUALIZATION_REQUEST => Any::from("individualization-request"),
         }
    }
}
impl From<&MediaKeyMessageType> for Any {
    fn from(s: &MediaKeyMessageType) -> Any {
         match *s {
            MediaKeyMessageType::LICENSE_REQUEST => Any::from("license-request"),
            MediaKeyMessageType::LICENSE_RENEWAL => Any::from("license-renewal"),
            MediaKeyMessageType::LICENSE_RELEASE => Any::from("license-release"),
            MediaKeyMessageType::INDIVIDUALIZATION_REQUEST => Any::from("individualization-request"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "signin" => Self::SIGNIN,
            "signup" => Self::SIGNUP,
            "use" => Self::USE_,
            "continue" => Self::CONTINUE_,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<IdentityCredentialRequestOptionsContext> for Any {
    fn from(s: IdentityCredentialRequestOptionsContext) -> Any {
         match s {
            IdentityCredentialRequestOptionsContext::SIGNIN => Any::from("signin"),
            IdentityCredentialRequestOptionsContext::SIGNUP => Any::from("signup"),
            IdentityCredentialRequestOptionsContext::USE_ => Any::from("use"),
            IdentityCredentialRequestOptionsContext::CONTINUE_ => Any::from("continue"),
         }
    }
}
impl From<&IdentityCredentialRequestOptionsContext> for Any {
    fn from(s: &IdentityCredentialRequestOptionsContext) -> Any {
         match *s {
            IdentityCredentialRequestOptionsContext::SIGNIN => Any::from("signin"),
            IdentityCredentialRequestOptionsContext::SIGNUP => Any::from("signup"),
            IdentityCredentialRequestOptionsContext::USE_ => Any::from("use"),
            IdentityCredentialRequestOptionsContext::CONTINUE_ => Any::from("continue"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IdentityCredentialRequestOptionsMode {
    ACTIVE,
    PASSIVE,
}
impl FromVal for IdentityCredentialRequestOptionsMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "active" => Self::ACTIVE,
            "passive" => Self::PASSIVE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<IdentityCredentialRequestOptionsMode> for Any {
    fn from(s: IdentityCredentialRequestOptionsMode) -> Any {
         match s {
            IdentityCredentialRequestOptionsMode::ACTIVE => Any::from("active"),
            IdentityCredentialRequestOptionsMode::PASSIVE => Any::from("passive"),
         }
    }
}
impl From<&IdentityCredentialRequestOptionsMode> for Any {
    fn from(s: &IdentityCredentialRequestOptionsMode) -> Any {
         match *s {
            IdentityCredentialRequestOptionsMode::ACTIVE => Any::from("active"),
            IdentityCredentialRequestOptionsMode::PASSIVE => Any::from("passive"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OpaqueProperty {
    OPAQUE,
}
impl FromVal for OpaqueProperty {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "opaque" => Self::OPAQUE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OpaqueProperty> for Any {
    fn from(s: OpaqueProperty) -> Any {
         match s {
            OpaqueProperty::OPAQUE => Any::from("opaque"),
         }
    }
}
impl From<&OpaqueProperty> for Any {
    fn from(s: &OpaqueProperty) -> Any {
         match *s {
            OpaqueProperty::OPAQUE => Any::from("opaque"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "buyer" => Self::BUYER,
            "seller" => Self::SELLER,
            "component-seller" => Self::COMPONENT_SELLER,
            "direct-seller" => Self::DIRECT_SELLER,
            "shared-storage-select-url" => Self::SHARED_STORAGE_SELECT_URL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FenceReportingDestination> for Any {
    fn from(s: FenceReportingDestination) -> Any {
         match s {
            FenceReportingDestination::BUYER => Any::from("buyer"),
            FenceReportingDestination::SELLER => Any::from("seller"),
            FenceReportingDestination::COMPONENT_SELLER => Any::from("component-seller"),
            FenceReportingDestination::DIRECT_SELLER => Any::from("direct-seller"),
            FenceReportingDestination::SHARED_STORAGE_SELECT_URL => Any::from("shared-storage-select-url"),
         }
    }
}
impl From<&FenceReportingDestination> for Any {
    fn from(s: &FenceReportingDestination) -> Any {
         match *s {
            FenceReportingDestination::BUYER => Any::from("buyer"),
            FenceReportingDestination::SELLER => Any::from("seller"),
            FenceReportingDestination::COMPONENT_SELLER => Any::from("component-seller"),
            FenceReportingDestination::DIRECT_SELLER => Any::from("direct-seller"),
            FenceReportingDestination::SHARED_STORAGE_SELECT_URL => Any::from("shared-storage-select-url"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RequestDestination> for Any {
    fn from(s: RequestDestination) -> Any {
         match s {
            RequestDestination::NONE => Any::from(""),
            RequestDestination::AUDIO => Any::from("audio"),
            RequestDestination::AUDIOWORKLET => Any::from("audioworklet"),
            RequestDestination::DOCUMENT => Any::from("document"),
            RequestDestination::EMBED => Any::from("embed"),
            RequestDestination::FONT => Any::from("font"),
            RequestDestination::FRAME => Any::from("frame"),
            RequestDestination::IFRAME => Any::from("iframe"),
            RequestDestination::IMAGE => Any::from("image"),
            RequestDestination::JSON => Any::from("json"),
            RequestDestination::MANIFEST => Any::from("manifest"),
            RequestDestination::OBJECT => Any::from("object"),
            RequestDestination::PAINTWORKLET => Any::from("paintworklet"),
            RequestDestination::REPORT => Any::from("report"),
            RequestDestination::SCRIPT => Any::from("script"),
            RequestDestination::SHAREDWORKER => Any::from("sharedworker"),
            RequestDestination::STYLE => Any::from("style"),
            RequestDestination::TRACK => Any::from("track"),
            RequestDestination::VIDEO => Any::from("video"),
            RequestDestination::WORKER => Any::from("worker"),
            RequestDestination::XSLT => Any::from("xslt"),
         }
    }
}
impl From<&RequestDestination> for Any {
    fn from(s: &RequestDestination) -> Any {
         match *s {
            RequestDestination::NONE => Any::from(""),
            RequestDestination::AUDIO => Any::from("audio"),
            RequestDestination::AUDIOWORKLET => Any::from("audioworklet"),
            RequestDestination::DOCUMENT => Any::from("document"),
            RequestDestination::EMBED => Any::from("embed"),
            RequestDestination::FONT => Any::from("font"),
            RequestDestination::FRAME => Any::from("frame"),
            RequestDestination::IFRAME => Any::from("iframe"),
            RequestDestination::IMAGE => Any::from("image"),
            RequestDestination::JSON => Any::from("json"),
            RequestDestination::MANIFEST => Any::from("manifest"),
            RequestDestination::OBJECT => Any::from("object"),
            RequestDestination::PAINTWORKLET => Any::from("paintworklet"),
            RequestDestination::REPORT => Any::from("report"),
            RequestDestination::SCRIPT => Any::from("script"),
            RequestDestination::SHAREDWORKER => Any::from("sharedworker"),
            RequestDestination::STYLE => Any::from("style"),
            RequestDestination::TRACK => Any::from("track"),
            RequestDestination::VIDEO => Any::from("video"),
            RequestDestination::WORKER => Any::from("worker"),
            RequestDestination::XSLT => Any::from("xslt"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "navigate" => Self::NAVIGATE,
            "same-origin" => Self::SAME_ORIGIN,
            "no-cors" => Self::NO_CORS,
            "cors" => Self::CORS,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RequestMode> for Any {
    fn from(s: RequestMode) -> Any {
         match s {
            RequestMode::NAVIGATE => Any::from("navigate"),
            RequestMode::SAME_ORIGIN => Any::from("same-origin"),
            RequestMode::NO_CORS => Any::from("no-cors"),
            RequestMode::CORS => Any::from("cors"),
         }
    }
}
impl From<&RequestMode> for Any {
    fn from(s: &RequestMode) -> Any {
         match *s {
            RequestMode::NAVIGATE => Any::from("navigate"),
            RequestMode::SAME_ORIGIN => Any::from("same-origin"),
            RequestMode::NO_CORS => Any::from("no-cors"),
            RequestMode::CORS => Any::from("cors"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "omit" => Self::OMIT,
            "same-origin" => Self::SAME_ORIGIN,
            "include" => Self::INCLUDE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RequestCredentials> for Any {
    fn from(s: RequestCredentials) -> Any {
         match s {
            RequestCredentials::OMIT => Any::from("omit"),
            RequestCredentials::SAME_ORIGIN => Any::from("same-origin"),
            RequestCredentials::INCLUDE => Any::from("include"),
         }
    }
}
impl From<&RequestCredentials> for Any {
    fn from(s: &RequestCredentials) -> Any {
         match *s {
            RequestCredentials::OMIT => Any::from("omit"),
            RequestCredentials::SAME_ORIGIN => Any::from("same-origin"),
            RequestCredentials::INCLUDE => Any::from("include"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "default" => Self::DEFAULT,
            "no-store" => Self::NO_STORE,
            "reload" => Self::RELOAD,
            "no-cache" => Self::NO_CACHE,
            "force-cache" => Self::FORCE_CACHE,
            "only-if-cached" => Self::ONLY_IF_CACHED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RequestCache> for Any {
    fn from(s: RequestCache) -> Any {
         match s {
            RequestCache::DEFAULT => Any::from("default"),
            RequestCache::NO_STORE => Any::from("no-store"),
            RequestCache::RELOAD => Any::from("reload"),
            RequestCache::NO_CACHE => Any::from("no-cache"),
            RequestCache::FORCE_CACHE => Any::from("force-cache"),
            RequestCache::ONLY_IF_CACHED => Any::from("only-if-cached"),
         }
    }
}
impl From<&RequestCache> for Any {
    fn from(s: &RequestCache) -> Any {
         match *s {
            RequestCache::DEFAULT => Any::from("default"),
            RequestCache::NO_STORE => Any::from("no-store"),
            RequestCache::RELOAD => Any::from("reload"),
            RequestCache::NO_CACHE => Any::from("no-cache"),
            RequestCache::FORCE_CACHE => Any::from("force-cache"),
            RequestCache::ONLY_IF_CACHED => Any::from("only-if-cached"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "follow" => Self::FOLLOW,
            "error" => Self::ERROR,
            "manual" => Self::MANUAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RequestRedirect> for Any {
    fn from(s: RequestRedirect) -> Any {
         match s {
            RequestRedirect::FOLLOW => Any::from("follow"),
            RequestRedirect::ERROR => Any::from("error"),
            RequestRedirect::MANUAL => Any::from("manual"),
         }
    }
}
impl From<&RequestRedirect> for Any {
    fn from(s: &RequestRedirect) -> Any {
         match *s {
            RequestRedirect::FOLLOW => Any::from("follow"),
            RequestRedirect::ERROR => Any::from("error"),
            RequestRedirect::MANUAL => Any::from("manual"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RequestDuplex {
    HALF,
}
impl FromVal for RequestDuplex {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "half" => Self::HALF,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RequestDuplex> for Any {
    fn from(s: RequestDuplex) -> Any {
         match s {
            RequestDuplex::HALF => Any::from("half"),
         }
    }
}
impl From<&RequestDuplex> for Any {
    fn from(s: &RequestDuplex) -> Any {
         match *s {
            RequestDuplex::HALF => Any::from("half"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "high" => Self::HIGH,
            "low" => Self::LOW,
            "auto" => Self::AUTO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RequestPriority> for Any {
    fn from(s: RequestPriority) -> Any {
         match s {
            RequestPriority::HIGH => Any::from("high"),
            RequestPriority::LOW => Any::from("low"),
            RequestPriority::AUTO => Any::from("auto"),
         }
    }
}
impl From<&RequestPriority> for Any {
    fn from(s: &RequestPriority) -> Any {
         match *s {
            RequestPriority::HIGH => Any::from("high"),
            RequestPriority::LOW => Any::from("low"),
            RequestPriority::AUTO => Any::from("auto"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "basic" => Self::BASIC,
            "cors" => Self::CORS,
            "default" => Self::DEFAULT,
            "error" => Self::ERROR,
            "opaque" => Self::OPAQUE,
            "opaqueredirect" => Self::OPAQUEREDIRECT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ResponseType> for Any {
    fn from(s: ResponseType) -> Any {
         match s {
            ResponseType::BASIC => Any::from("basic"),
            ResponseType::CORS => Any::from("cors"),
            ResponseType::DEFAULT => Any::from("default"),
            ResponseType::ERROR => Any::from("error"),
            ResponseType::OPAQUE => Any::from("opaque"),
            ResponseType::OPAQUEREDIRECT => Any::from("opaqueredirect"),
         }
    }
}
impl From<&ResponseType> for Any {
    fn from(s: &ResponseType) -> Any {
         match *s {
            ResponseType::BASIC => Any::from("basic"),
            ResponseType::CORS => Any::from("cors"),
            ResponseType::DEFAULT => Any::from("default"),
            ResponseType::ERROR => Any::from("error"),
            ResponseType::OPAQUE => Any::from("opaque"),
            ResponseType::OPAQUEREDIRECT => Any::from("opaqueredirect"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FileSystemPermissionMode {
    READ,
    READWRITE,
}
impl FromVal for FileSystemPermissionMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "read" => Self::READ,
            "readwrite" => Self::READWRITE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FileSystemPermissionMode> for Any {
    fn from(s: FileSystemPermissionMode) -> Any {
         match s {
            FileSystemPermissionMode::READ => Any::from("read"),
            FileSystemPermissionMode::READWRITE => Any::from("readwrite"),
         }
    }
}
impl From<&FileSystemPermissionMode> for Any {
    fn from(s: &FileSystemPermissionMode) -> Any {
         match *s {
            FileSystemPermissionMode::READ => Any::from("read"),
            FileSystemPermissionMode::READWRITE => Any::from("readwrite"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "desktop" => Self::DESKTOP,
            "documents" => Self::DOCUMENTS,
            "downloads" => Self::DOWNLOADS,
            "music" => Self::MUSIC,
            "pictures" => Self::PICTURES,
            "videos" => Self::VIDEOS,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WellKnownDirectory> for Any {
    fn from(s: WellKnownDirectory) -> Any {
         match s {
            WellKnownDirectory::DESKTOP => Any::from("desktop"),
            WellKnownDirectory::DOCUMENTS => Any::from("documents"),
            WellKnownDirectory::DOWNLOADS => Any::from("downloads"),
            WellKnownDirectory::MUSIC => Any::from("music"),
            WellKnownDirectory::PICTURES => Any::from("pictures"),
            WellKnownDirectory::VIDEOS => Any::from("videos"),
         }
    }
}
impl From<&WellKnownDirectory> for Any {
    fn from(s: &WellKnownDirectory) -> Any {
         match *s {
            WellKnownDirectory::DESKTOP => Any::from("desktop"),
            WellKnownDirectory::DOCUMENTS => Any::from("documents"),
            WellKnownDirectory::DOWNLOADS => Any::from("downloads"),
            WellKnownDirectory::MUSIC => Any::from("music"),
            WellKnownDirectory::PICTURES => Any::from("pictures"),
            WellKnownDirectory::VIDEOS => Any::from("videos"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FileSystemHandleKind {
    FILE,
    DIRECTORY,
}
impl FromVal for FileSystemHandleKind {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "file" => Self::FILE,
            "directory" => Self::DIRECTORY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FileSystemHandleKind> for Any {
    fn from(s: FileSystemHandleKind) -> Any {
         match s {
            FileSystemHandleKind::FILE => Any::from("file"),
            FileSystemHandleKind::DIRECTORY => Any::from("directory"),
         }
    }
}
impl From<&FileSystemHandleKind> for Any {
    fn from(s: &FileSystemHandleKind) -> Any {
         match *s {
            FileSystemHandleKind::FILE => Any::from("file"),
            FileSystemHandleKind::DIRECTORY => Any::from("directory"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "write" => Self::WRITE,
            "seek" => Self::SEEK,
            "truncate" => Self::TRUNCATE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WriteCommandType> for Any {
    fn from(s: WriteCommandType) -> Any {
         match s {
            WriteCommandType::WRITE => Any::from("write"),
            WriteCommandType::SEEK => Any::from("seek"),
            WriteCommandType::TRUNCATE => Any::from("truncate"),
         }
    }
}
impl From<&WriteCommandType> for Any {
    fn from(s: &WriteCommandType) -> Any {
         match *s {
            WriteCommandType::WRITE => Any::from("write"),
            WriteCommandType::SEEK => Any::from("seek"),
            WriteCommandType::TRUNCATE => Any::from("truncate"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
            "show" => Self::SHOW,
            "hide" => Self::HIDE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FullscreenNavigationUI> for Any {
    fn from(s: FullscreenNavigationUI) -> Any {
         match s {
            FullscreenNavigationUI::AUTO => Any::from("auto"),
            FullscreenNavigationUI::SHOW => Any::from("show"),
            FullscreenNavigationUI::HIDE => Any::from("hide"),
         }
    }
}
impl From<&FullscreenNavigationUI> for Any {
    fn from(s: &FullscreenNavigationUI) -> Any {
         match *s {
            FullscreenNavigationUI::AUTO => Any::from("auto"),
            FullscreenNavigationUI::SHOW => Any::from("show"),
            FullscreenNavigationUI::HIDE => Any::from("hide"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "" => Self::NONE,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GamepadHand> for Any {
    fn from(s: GamepadHand) -> Any {
         match s {
            GamepadHand::NONE => Any::from(""),
            GamepadHand::LEFT => Any::from("left"),
            GamepadHand::RIGHT => Any::from("right"),
         }
    }
}
impl From<&GamepadHand> for Any {
    fn from(s: &GamepadHand) -> Any {
         match *s {
            GamepadHand::NONE => Any::from(""),
            GamepadHand::LEFT => Any::from("left"),
            GamepadHand::RIGHT => Any::from("right"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "" => Self::NONE,
            "standard" => Self::STANDARD,
            "xr-standard" => Self::XR_STANDARD,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GamepadMappingType> for Any {
    fn from(s: GamepadMappingType) -> Any {
         match s {
            GamepadMappingType::NONE => Any::from(""),
            GamepadMappingType::STANDARD => Any::from("standard"),
            GamepadMappingType::XR_STANDARD => Any::from("xr-standard"),
         }
    }
}
impl From<&GamepadMappingType> for Any {
    fn from(s: &GamepadMappingType) -> Any {
         match *s {
            GamepadMappingType::NONE => Any::from(""),
            GamepadMappingType::STANDARD => Any::from("standard"),
            GamepadMappingType::XR_STANDARD => Any::from("xr-standard"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GamepadHapticsResult {
    COMPLETE,
    PREEMPTED,
}
impl FromVal for GamepadHapticsResult {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "complete" => Self::COMPLETE,
            "preempted" => Self::PREEMPTED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GamepadHapticsResult> for Any {
    fn from(s: GamepadHapticsResult) -> Any {
         match s {
            GamepadHapticsResult::COMPLETE => Any::from("complete"),
            GamepadHapticsResult::PREEMPTED => Any::from("preempted"),
         }
    }
}
impl From<&GamepadHapticsResult> for Any {
    fn from(s: &GamepadHapticsResult) -> Any {
         match *s {
            GamepadHapticsResult::COMPLETE => Any::from("complete"),
            GamepadHapticsResult::PREEMPTED => Any::from("preempted"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GamepadHapticEffectType {
    DUAL_RUMBLE,
    TRIGGER_RUMBLE,
}
impl FromVal for GamepadHapticEffectType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "dual-rumble" => Self::DUAL_RUMBLE,
            "trigger-rumble" => Self::TRIGGER_RUMBLE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GamepadHapticEffectType> for Any {
    fn from(s: GamepadHapticEffectType) -> Any {
         match s {
            GamepadHapticEffectType::DUAL_RUMBLE => Any::from("dual-rumble"),
            GamepadHapticEffectType::TRIGGER_RUMBLE => Any::from("trigger-rumble"),
         }
    }
}
impl From<&GamepadHapticEffectType> for Any {
    fn from(s: &GamepadHapticEffectType) -> Any {
         match *s {
            GamepadHapticEffectType::DUAL_RUMBLE => Any::from("dual-rumble"),
            GamepadHapticEffectType::TRIGGER_RUMBLE => Any::from("trigger-rumble"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GyroscopeLocalCoordinateSystem {
    DEVICE,
    SCREEN,
}
impl FromVal for GyroscopeLocalCoordinateSystem {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "device" => Self::DEVICE,
            "screen" => Self::SCREEN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GyroscopeLocalCoordinateSystem> for Any {
    fn from(s: GyroscopeLocalCoordinateSystem) -> Any {
         match s {
            GyroscopeLocalCoordinateSystem::DEVICE => Any::from("device"),
            GyroscopeLocalCoordinateSystem::SCREEN => Any::from("screen"),
         }
    }
}
impl From<&GyroscopeLocalCoordinateSystem> for Any {
    fn from(s: &GyroscopeLocalCoordinateSystem) -> Any {
         match *s {
            GyroscopeLocalCoordinateSystem::DEVICE => Any::from("device"),
            GyroscopeLocalCoordinateSystem::SCREEN => Any::from("screen"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum HandwritingRecognitionType {
    TEXT,
    PER_CHARACTER,
}
impl FromVal for HandwritingRecognitionType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "text" => Self::TEXT,
            "per-character" => Self::PER_CHARACTER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<HandwritingRecognitionType> for Any {
    fn from(s: HandwritingRecognitionType) -> Any {
         match s {
            HandwritingRecognitionType::TEXT => Any::from("text"),
            HandwritingRecognitionType::PER_CHARACTER => Any::from("per-character"),
         }
    }
}
impl From<&HandwritingRecognitionType> for Any {
    fn from(s: &HandwritingRecognitionType) -> Any {
         match *s {
            HandwritingRecognitionType::TEXT => Any::from("text"),
            HandwritingRecognitionType::PER_CHARACTER => Any::from("per-character"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "mouse" => Self::MOUSE,
            "stylus" => Self::STYLUS,
            "touch" => Self::TOUCH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<HandwritingInputType> for Any {
    fn from(s: HandwritingInputType) -> Any {
         match s {
            HandwritingInputType::MOUSE => Any::from("mouse"),
            HandwritingInputType::STYLUS => Any::from("stylus"),
            HandwritingInputType::TOUCH => Any::from("touch"),
         }
    }
}
impl From<&HandwritingInputType> for Any {
    fn from(s: &HandwritingInputType) -> Any {
         match *s {
            HandwritingInputType::MOUSE => Any::from("mouse"),
            HandwritingInputType::STYLUS => Any::from("stylus"),
            HandwritingInputType::TOUCH => Any::from("touch"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "loading" => Self::LOADING,
            "interactive" => Self::INTERACTIVE,
            "complete" => Self::COMPLETE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<DocumentReadyState> for Any {
    fn from(s: DocumentReadyState) -> Any {
         match s {
            DocumentReadyState::LOADING => Any::from("loading"),
            DocumentReadyState::INTERACTIVE => Any::from("interactive"),
            DocumentReadyState::COMPLETE => Any::from("complete"),
         }
    }
}
impl From<&DocumentReadyState> for Any {
    fn from(s: &DocumentReadyState) -> Any {
         match *s {
            DocumentReadyState::LOADING => Any::from("loading"),
            DocumentReadyState::INTERACTIVE => Any::from("interactive"),
            DocumentReadyState::COMPLETE => Any::from("complete"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DocumentVisibilityState {
    VISIBLE,
    HIDDEN,
}
impl FromVal for DocumentVisibilityState {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "visible" => Self::VISIBLE,
            "hidden" => Self::HIDDEN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<DocumentVisibilityState> for Any {
    fn from(s: DocumentVisibilityState) -> Any {
         match s {
            DocumentVisibilityState::VISIBLE => Any::from("visible"),
            DocumentVisibilityState::HIDDEN => Any::from("hidden"),
         }
    }
}
impl From<&DocumentVisibilityState> for Any {
    fn from(s: &DocumentVisibilityState) -> Any {
         match *s {
            DocumentVisibilityState::VISIBLE => Any::from("visible"),
            DocumentVisibilityState::HIDDEN => Any::from("hidden"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "" => Self::NONE,
            "maybe" => Self::MAYBE,
            "probably" => Self::PROBABLY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanPlayTypeResult> for Any {
    fn from(s: CanPlayTypeResult) -> Any {
         match s {
            CanPlayTypeResult::NONE => Any::from(""),
            CanPlayTypeResult::MAYBE => Any::from("maybe"),
            CanPlayTypeResult::PROBABLY => Any::from("probably"),
         }
    }
}
impl From<&CanPlayTypeResult> for Any {
    fn from(s: &CanPlayTypeResult) -> Any {
         match *s {
            CanPlayTypeResult::NONE => Any::from(""),
            CanPlayTypeResult::MAYBE => Any::from("maybe"),
            CanPlayTypeResult::PROBABLY => Any::from("probably"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "disabled" => Self::DISABLED,
            "hidden" => Self::HIDDEN,
            "showing" => Self::SHOWING,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<TextTrackMode> for Any {
    fn from(s: TextTrackMode) -> Any {
         match s {
            TextTrackMode::DISABLED => Any::from("disabled"),
            TextTrackMode::HIDDEN => Any::from("hidden"),
            TextTrackMode::SHOWING => Any::from("showing"),
         }
    }
}
impl From<&TextTrackMode> for Any {
    fn from(s: &TextTrackMode) -> Any {
         match *s {
            TextTrackMode::DISABLED => Any::from("disabled"),
            TextTrackMode::HIDDEN => Any::from("hidden"),
            TextTrackMode::SHOWING => Any::from("showing"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "subtitles" => Self::SUBTITLES,
            "captions" => Self::CAPTIONS,
            "descriptions" => Self::DESCRIPTIONS,
            "chapters" => Self::CHAPTERS,
            "metadata" => Self::METADATA,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<TextTrackKind> for Any {
    fn from(s: TextTrackKind) -> Any {
         match s {
            TextTrackKind::SUBTITLES => Any::from("subtitles"),
            TextTrackKind::CAPTIONS => Any::from("captions"),
            TextTrackKind::DESCRIPTIONS => Any::from("descriptions"),
            TextTrackKind::CHAPTERS => Any::from("chapters"),
            TextTrackKind::METADATA => Any::from("metadata"),
         }
    }
}
impl From<&TextTrackKind> for Any {
    fn from(s: &TextTrackKind) -> Any {
         match *s {
            TextTrackKind::SUBTITLES => Any::from("subtitles"),
            TextTrackKind::CAPTIONS => Any::from("captions"),
            TextTrackKind::DESCRIPTIONS => Any::from("descriptions"),
            TextTrackKind::CHAPTERS => Any::from("chapters"),
            TextTrackKind::METADATA => Any::from("metadata"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "select" => Self::SELECT,
            "start" => Self::START,
            "end" => Self::END,
            "preserve" => Self::PRESERVE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SelectionMode> for Any {
    fn from(s: SelectionMode) -> Any {
         match s {
            SelectionMode::SELECT => Any::from("select"),
            SelectionMode::START => Any::from("start"),
            SelectionMode::END => Any::from("end"),
            SelectionMode::PRESERVE => Any::from("preserve"),
         }
    }
}
impl From<&SelectionMode> for Any {
    fn from(s: &SelectionMode) -> Any {
         match *s {
            SelectionMode::SELECT => Any::from("select"),
            SelectionMode::START => Any::from("start"),
            SelectionMode::END => Any::from("end"),
            SelectionMode::PRESERVE => Any::from("preserve"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PredefinedColorSpace {
    SRGB,
    DISPLAY_P3,
}
impl FromVal for PredefinedColorSpace {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "srgb" => Self::SRGB,
            "display-p3" => Self::DISPLAY_P3,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PredefinedColorSpace> for Any {
    fn from(s: PredefinedColorSpace) -> Any {
         match s {
            PredefinedColorSpace::SRGB => Any::from("srgb"),
            PredefinedColorSpace::DISPLAY_P3 => Any::from("display-p3"),
         }
    }
}
impl From<&PredefinedColorSpace> for Any {
    fn from(s: &PredefinedColorSpace) -> Any {
         match *s {
            PredefinedColorSpace::SRGB => Any::from("srgb"),
            PredefinedColorSpace::DISPLAY_P3 => Any::from("display-p3"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasColorType {
    UNORM8,
    FLOAT16,
}
impl FromVal for CanvasColorType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "unorm8" => Self::UNORM8,
            "float16" => Self::FLOAT16,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasColorType> for Any {
    fn from(s: CanvasColorType) -> Any {
         match s {
            CanvasColorType::UNORM8 => Any::from("unorm8"),
            CanvasColorType::FLOAT16 => Any::from("float16"),
         }
    }
}
impl From<&CanvasColorType> for Any {
    fn from(s: &CanvasColorType) -> Any {
         match *s {
            CanvasColorType::UNORM8 => Any::from("unorm8"),
            CanvasColorType::FLOAT16 => Any::from("float16"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum CanvasFillRule {
    NONZERO,
    EVENODD,
}
impl FromVal for CanvasFillRule {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "nonzero" => Self::NONZERO,
            "evenodd" => Self::EVENODD,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasFillRule> for Any {
    fn from(s: CanvasFillRule) -> Any {
         match s {
            CanvasFillRule::NONZERO => Any::from("nonzero"),
            CanvasFillRule::EVENODD => Any::from("evenodd"),
         }
    }
}
impl From<&CanvasFillRule> for Any {
    fn from(s: &CanvasFillRule) -> Any {
         match *s {
            CanvasFillRule::NONZERO => Any::from("nonzero"),
            CanvasFillRule::EVENODD => Any::from("evenodd"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "low" => Self::LOW,
            "medium" => Self::MEDIUM,
            "high" => Self::HIGH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ImageSmoothingQuality> for Any {
    fn from(s: ImageSmoothingQuality) -> Any {
         match s {
            ImageSmoothingQuality::LOW => Any::from("low"),
            ImageSmoothingQuality::MEDIUM => Any::from("medium"),
            ImageSmoothingQuality::HIGH => Any::from("high"),
         }
    }
}
impl From<&ImageSmoothingQuality> for Any {
    fn from(s: &ImageSmoothingQuality) -> Any {
         match *s {
            ImageSmoothingQuality::LOW => Any::from("low"),
            ImageSmoothingQuality::MEDIUM => Any::from("medium"),
            ImageSmoothingQuality::HIGH => Any::from("high"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "butt" => Self::BUTT,
            "round" => Self::ROUND,
            "square" => Self::SQUARE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasLineCap> for Any {
    fn from(s: CanvasLineCap) -> Any {
         match s {
            CanvasLineCap::BUTT => Any::from("butt"),
            CanvasLineCap::ROUND => Any::from("round"),
            CanvasLineCap::SQUARE => Any::from("square"),
         }
    }
}
impl From<&CanvasLineCap> for Any {
    fn from(s: &CanvasLineCap) -> Any {
         match *s {
            CanvasLineCap::BUTT => Any::from("butt"),
            CanvasLineCap::ROUND => Any::from("round"),
            CanvasLineCap::SQUARE => Any::from("square"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "round" => Self::ROUND,
            "bevel" => Self::BEVEL,
            "miter" => Self::MITER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasLineJoin> for Any {
    fn from(s: CanvasLineJoin) -> Any {
         match s {
            CanvasLineJoin::ROUND => Any::from("round"),
            CanvasLineJoin::BEVEL => Any::from("bevel"),
            CanvasLineJoin::MITER => Any::from("miter"),
         }
    }
}
impl From<&CanvasLineJoin> for Any {
    fn from(s: &CanvasLineJoin) -> Any {
         match *s {
            CanvasLineJoin::ROUND => Any::from("round"),
            CanvasLineJoin::BEVEL => Any::from("bevel"),
            CanvasLineJoin::MITER => Any::from("miter"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "start" => Self::START,
            "end" => Self::END,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
            "center" => Self::CENTER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasTextAlign> for Any {
    fn from(s: CanvasTextAlign) -> Any {
         match s {
            CanvasTextAlign::START => Any::from("start"),
            CanvasTextAlign::END => Any::from("end"),
            CanvasTextAlign::LEFT => Any::from("left"),
            CanvasTextAlign::RIGHT => Any::from("right"),
            CanvasTextAlign::CENTER => Any::from("center"),
         }
    }
}
impl From<&CanvasTextAlign> for Any {
    fn from(s: &CanvasTextAlign) -> Any {
         match *s {
            CanvasTextAlign::START => Any::from("start"),
            CanvasTextAlign::END => Any::from("end"),
            CanvasTextAlign::LEFT => Any::from("left"),
            CanvasTextAlign::RIGHT => Any::from("right"),
            CanvasTextAlign::CENTER => Any::from("center"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "top" => Self::TOP,
            "hanging" => Self::HANGING,
            "middle" => Self::MIDDLE,
            "alphabetic" => Self::ALPHABETIC,
            "ideographic" => Self::IDEOGRAPHIC,
            "bottom" => Self::BOTTOM,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasTextBaseline> for Any {
    fn from(s: CanvasTextBaseline) -> Any {
         match s {
            CanvasTextBaseline::TOP => Any::from("top"),
            CanvasTextBaseline::HANGING => Any::from("hanging"),
            CanvasTextBaseline::MIDDLE => Any::from("middle"),
            CanvasTextBaseline::ALPHABETIC => Any::from("alphabetic"),
            CanvasTextBaseline::IDEOGRAPHIC => Any::from("ideographic"),
            CanvasTextBaseline::BOTTOM => Any::from("bottom"),
         }
    }
}
impl From<&CanvasTextBaseline> for Any {
    fn from(s: &CanvasTextBaseline) -> Any {
         match *s {
            CanvasTextBaseline::TOP => Any::from("top"),
            CanvasTextBaseline::HANGING => Any::from("hanging"),
            CanvasTextBaseline::MIDDLE => Any::from("middle"),
            CanvasTextBaseline::ALPHABETIC => Any::from("alphabetic"),
            CanvasTextBaseline::IDEOGRAPHIC => Any::from("ideographic"),
            CanvasTextBaseline::BOTTOM => Any::from("bottom"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "ltr" => Self::LTR,
            "rtl" => Self::RTL,
            "inherit" => Self::INHERIT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasDirection> for Any {
    fn from(s: CanvasDirection) -> Any {
         match s {
            CanvasDirection::LTR => Any::from("ltr"),
            CanvasDirection::RTL => Any::from("rtl"),
            CanvasDirection::INHERIT => Any::from("inherit"),
         }
    }
}
impl From<&CanvasDirection> for Any {
    fn from(s: &CanvasDirection) -> Any {
         match *s {
            CanvasDirection::LTR => Any::from("ltr"),
            CanvasDirection::RTL => Any::from("rtl"),
            CanvasDirection::INHERIT => Any::from("inherit"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
            "normal" => Self::NORMAL,
            "none" => Self::NONE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasFontKerning> for Any {
    fn from(s: CanvasFontKerning) -> Any {
         match s {
            CanvasFontKerning::AUTO => Any::from("auto"),
            CanvasFontKerning::NORMAL => Any::from("normal"),
            CanvasFontKerning::NONE => Any::from("none"),
         }
    }
}
impl From<&CanvasFontKerning> for Any {
    fn from(s: &CanvasFontKerning) -> Any {
         match *s {
            CanvasFontKerning::AUTO => Any::from("auto"),
            CanvasFontKerning::NORMAL => Any::from("normal"),
            CanvasFontKerning::NONE => Any::from("none"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasFontStretch> for Any {
    fn from(s: CanvasFontStretch) -> Any {
         match s {
            CanvasFontStretch::ULTRA_CONDENSED => Any::from("ultra-condensed"),
            CanvasFontStretch::EXTRA_CONDENSED => Any::from("extra-condensed"),
            CanvasFontStretch::CONDENSED => Any::from("condensed"),
            CanvasFontStretch::SEMI_CONDENSED => Any::from("semi-condensed"),
            CanvasFontStretch::NORMAL => Any::from("normal"),
            CanvasFontStretch::SEMI_EXPANDED => Any::from("semi-expanded"),
            CanvasFontStretch::EXPANDED => Any::from("expanded"),
            CanvasFontStretch::EXTRA_EXPANDED => Any::from("extra-expanded"),
            CanvasFontStretch::ULTRA_EXPANDED => Any::from("ultra-expanded"),
         }
    }
}
impl From<&CanvasFontStretch> for Any {
    fn from(s: &CanvasFontStretch) -> Any {
         match *s {
            CanvasFontStretch::ULTRA_CONDENSED => Any::from("ultra-condensed"),
            CanvasFontStretch::EXTRA_CONDENSED => Any::from("extra-condensed"),
            CanvasFontStretch::CONDENSED => Any::from("condensed"),
            CanvasFontStretch::SEMI_CONDENSED => Any::from("semi-condensed"),
            CanvasFontStretch::NORMAL => Any::from("normal"),
            CanvasFontStretch::SEMI_EXPANDED => Any::from("semi-expanded"),
            CanvasFontStretch::EXPANDED => Any::from("expanded"),
            CanvasFontStretch::EXTRA_EXPANDED => Any::from("extra-expanded"),
            CanvasFontStretch::ULTRA_EXPANDED => Any::from("ultra-expanded"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasFontVariantCaps> for Any {
    fn from(s: CanvasFontVariantCaps) -> Any {
         match s {
            CanvasFontVariantCaps::NORMAL => Any::from("normal"),
            CanvasFontVariantCaps::SMALL_CAPS => Any::from("small-caps"),
            CanvasFontVariantCaps::ALL_SMALL_CAPS => Any::from("all-small-caps"),
            CanvasFontVariantCaps::PETITE_CAPS => Any::from("petite-caps"),
            CanvasFontVariantCaps::ALL_PETITE_CAPS => Any::from("all-petite-caps"),
            CanvasFontVariantCaps::UNICASE => Any::from("unicase"),
            CanvasFontVariantCaps::TITLING_CAPS => Any::from("titling-caps"),
         }
    }
}
impl From<&CanvasFontVariantCaps> for Any {
    fn from(s: &CanvasFontVariantCaps) -> Any {
         match *s {
            CanvasFontVariantCaps::NORMAL => Any::from("normal"),
            CanvasFontVariantCaps::SMALL_CAPS => Any::from("small-caps"),
            CanvasFontVariantCaps::ALL_SMALL_CAPS => Any::from("all-small-caps"),
            CanvasFontVariantCaps::PETITE_CAPS => Any::from("petite-caps"),
            CanvasFontVariantCaps::ALL_PETITE_CAPS => Any::from("all-petite-caps"),
            CanvasFontVariantCaps::UNICASE => Any::from("unicase"),
            CanvasFontVariantCaps::TITLING_CAPS => Any::from("titling-caps"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
            "optimizeSpeed" => Self::OPTIMIZE_SPEED,
            "optimizeLegibility" => Self::OPTIMIZE_LEGIBILITY,
            "geometricPrecision" => Self::GEOMETRIC_PRECISION,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CanvasTextRendering> for Any {
    fn from(s: CanvasTextRendering) -> Any {
         match s {
            CanvasTextRendering::AUTO => Any::from("auto"),
            CanvasTextRendering::OPTIMIZE_SPEED => Any::from("optimizeSpeed"),
            CanvasTextRendering::OPTIMIZE_LEGIBILITY => Any::from("optimizeLegibility"),
            CanvasTextRendering::GEOMETRIC_PRECISION => Any::from("geometricPrecision"),
         }
    }
}
impl From<&CanvasTextRendering> for Any {
    fn from(s: &CanvasTextRendering) -> Any {
         match *s {
            CanvasTextRendering::AUTO => Any::from("auto"),
            CanvasTextRendering::OPTIMIZE_SPEED => Any::from("optimizeSpeed"),
            CanvasTextRendering::OPTIMIZE_LEGIBILITY => Any::from("optimizeLegibility"),
            CanvasTextRendering::GEOMETRIC_PRECISION => Any::from("geometricPrecision"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "2d" => Self::_2D,
            "bitmaprenderer" => Self::BITMAPRENDERER,
            "webgl" => Self::WEBGL,
            "webgl2" => Self::WEBGL2,
            "webgpu" => Self::WEBGPU,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OffscreenRenderingContextId> for Any {
    fn from(s: OffscreenRenderingContextId) -> Any {
         match s {
            OffscreenRenderingContextId::_2D => Any::from("2d"),
            OffscreenRenderingContextId::BITMAPRENDERER => Any::from("bitmaprenderer"),
            OffscreenRenderingContextId::WEBGL => Any::from("webgl"),
            OffscreenRenderingContextId::WEBGL2 => Any::from("webgl2"),
            OffscreenRenderingContextId::WEBGPU => Any::from("webgpu"),
         }
    }
}
impl From<&OffscreenRenderingContextId> for Any {
    fn from(s: &OffscreenRenderingContextId) -> Any {
         match *s {
            OffscreenRenderingContextId::_2D => Any::from("2d"),
            OffscreenRenderingContextId::BITMAPRENDERER => Any::from("bitmaprenderer"),
            OffscreenRenderingContextId::WEBGL => Any::from("webgl"),
            OffscreenRenderingContextId::WEBGL2 => Any::from("webgl2"),
            OffscreenRenderingContextId::WEBGPU => Any::from("webgpu"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScrollRestoration {
    AUTO,
    MANUAL,
}
impl FromVal for ScrollRestoration {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
            "manual" => Self::MANUAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ScrollRestoration> for Any {
    fn from(s: ScrollRestoration) -> Any {
         match s {
            ScrollRestoration::AUTO => Any::from("auto"),
            ScrollRestoration::MANUAL => Any::from("manual"),
         }
    }
}
impl From<&ScrollRestoration> for Any {
    fn from(s: &ScrollRestoration) -> Any {
         match *s {
            ScrollRestoration::AUTO => Any::from("auto"),
            ScrollRestoration::MANUAL => Any::from("manual"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
            "push" => Self::PUSH,
            "replace" => Self::REPLACE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<NavigationHistoryBehavior> for Any {
    fn from(s: NavigationHistoryBehavior) -> Any {
         match s {
            NavigationHistoryBehavior::AUTO => Any::from("auto"),
            NavigationHistoryBehavior::PUSH => Any::from("push"),
            NavigationHistoryBehavior::REPLACE => Any::from("replace"),
         }
    }
}
impl From<&NavigationHistoryBehavior> for Any {
    fn from(s: &NavigationHistoryBehavior) -> Any {
         match *s {
            NavigationHistoryBehavior::AUTO => Any::from("auto"),
            NavigationHistoryBehavior::PUSH => Any::from("push"),
            NavigationHistoryBehavior::REPLACE => Any::from("replace"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "push" => Self::PUSH,
            "replace" => Self::REPLACE,
            "reload" => Self::RELOAD,
            "traverse" => Self::TRAVERSE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<NavigationType> for Any {
    fn from(s: NavigationType) -> Any {
         match s {
            NavigationType::PUSH => Any::from("push"),
            NavigationType::REPLACE => Any::from("replace"),
            NavigationType::RELOAD => Any::from("reload"),
            NavigationType::TRAVERSE => Any::from("traverse"),
         }
    }
}
impl From<&NavigationType> for Any {
    fn from(s: &NavigationType) -> Any {
         match *s {
            NavigationType::PUSH => Any::from("push"),
            NavigationType::REPLACE => Any::from("replace"),
            NavigationType::RELOAD => Any::from("reload"),
            NavigationType::TRAVERSE => Any::from("traverse"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum NavigationFocusReset {
    AFTER_TRANSITION,
    MANUAL,
}
impl FromVal for NavigationFocusReset {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "after-transition" => Self::AFTER_TRANSITION,
            "manual" => Self::MANUAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<NavigationFocusReset> for Any {
    fn from(s: NavigationFocusReset) -> Any {
         match s {
            NavigationFocusReset::AFTER_TRANSITION => Any::from("after-transition"),
            NavigationFocusReset::MANUAL => Any::from("manual"),
         }
    }
}
impl From<&NavigationFocusReset> for Any {
    fn from(s: &NavigationFocusReset) -> Any {
         match *s {
            NavigationFocusReset::AFTER_TRANSITION => Any::from("after-transition"),
            NavigationFocusReset::MANUAL => Any::from("manual"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum NavigationScrollBehavior {
    AFTER_TRANSITION,
    MANUAL,
}
impl FromVal for NavigationScrollBehavior {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "after-transition" => Self::AFTER_TRANSITION,
            "manual" => Self::MANUAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<NavigationScrollBehavior> for Any {
    fn from(s: NavigationScrollBehavior) -> Any {
         match s {
            NavigationScrollBehavior::AFTER_TRANSITION => Any::from("after-transition"),
            NavigationScrollBehavior::MANUAL => Any::from("manual"),
         }
    }
}
impl From<&NavigationScrollBehavior> for Any {
    fn from(s: &NavigationScrollBehavior) -> Any {
         match *s {
            NavigationScrollBehavior::AFTER_TRANSITION => Any::from("after-transition"),
            NavigationScrollBehavior::MANUAL => Any::from("manual"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "text/html" => Self::TEXT_HTML,
            "text/xml" => Self::TEXT_XML,
            "application/xml" => Self::APPLICATION_XML,
            "application/xhtml+xml" => Self::APPLICATION_XHTML_XML,
            "image/svg+xml" => Self::IMAGE_SVG_XML,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<DOMParserSupportedType> for Any {
    fn from(s: DOMParserSupportedType) -> Any {
         match s {
            DOMParserSupportedType::TEXT_HTML => Any::from("text/html"),
            DOMParserSupportedType::TEXT_XML => Any::from("text/xml"),
            DOMParserSupportedType::APPLICATION_XML => Any::from("application/xml"),
            DOMParserSupportedType::APPLICATION_XHTML_XML => Any::from("application/xhtml+xml"),
            DOMParserSupportedType::IMAGE_SVG_XML => Any::from("image/svg+xml"),
         }
    }
}
impl From<&DOMParserSupportedType> for Any {
    fn from(s: &DOMParserSupportedType) -> Any {
         match *s {
            DOMParserSupportedType::TEXT_HTML => Any::from("text/html"),
            DOMParserSupportedType::TEXT_XML => Any::from("text/xml"),
            DOMParserSupportedType::APPLICATION_XML => Any::from("application/xml"),
            DOMParserSupportedType::APPLICATION_XHTML_XML => Any::from("application/xhtml+xml"),
            DOMParserSupportedType::IMAGE_SVG_XML => Any::from("image/svg+xml"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ImageDataPixelFormat {
    RGBA_UNORM8,
    RGBA_FLOAT16,
}
impl FromVal for ImageDataPixelFormat {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "rgba-unorm8" => Self::RGBA_UNORM8,
            "rgba-float16" => Self::RGBA_FLOAT16,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ImageDataPixelFormat> for Any {
    fn from(s: ImageDataPixelFormat) -> Any {
         match s {
            ImageDataPixelFormat::RGBA_UNORM8 => Any::from("rgba-unorm8"),
            ImageDataPixelFormat::RGBA_FLOAT16 => Any::from("rgba-float16"),
         }
    }
}
impl From<&ImageDataPixelFormat> for Any {
    fn from(s: &ImageDataPixelFormat) -> Any {
         match *s {
            ImageDataPixelFormat::RGBA_UNORM8 => Any::from("rgba-unorm8"),
            ImageDataPixelFormat::RGBA_FLOAT16 => Any::from("rgba-float16"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ImageOrientation {
    FROM_IMAGE,
    FLIP_Y,
}
impl FromVal for ImageOrientation {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "from-image" => Self::FROM_IMAGE,
            "flipY" => Self::FLIP_Y,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ImageOrientation> for Any {
    fn from(s: ImageOrientation) -> Any {
         match s {
            ImageOrientation::FROM_IMAGE => Any::from("from-image"),
            ImageOrientation::FLIP_Y => Any::from("flipY"),
         }
    }
}
impl From<&ImageOrientation> for Any {
    fn from(s: &ImageOrientation) -> Any {
         match *s {
            ImageOrientation::FROM_IMAGE => Any::from("from-image"),
            ImageOrientation::FLIP_Y => Any::from("flipY"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "premultiply" => Self::PREMULTIPLY,
            "default" => Self::DEFAULT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PremultiplyAlpha> for Any {
    fn from(s: PremultiplyAlpha) -> Any {
         match s {
            PremultiplyAlpha::NONE => Any::from("none"),
            PremultiplyAlpha::PREMULTIPLY => Any::from("premultiply"),
            PremultiplyAlpha::DEFAULT => Any::from("default"),
         }
    }
}
impl From<&PremultiplyAlpha> for Any {
    fn from(s: &PremultiplyAlpha) -> Any {
         match *s {
            PremultiplyAlpha::NONE => Any::from("none"),
            PremultiplyAlpha::PREMULTIPLY => Any::from("premultiply"),
            PremultiplyAlpha::DEFAULT => Any::from("default"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ColorSpaceConversion {
    NONE,
    DEFAULT,
}
impl FromVal for ColorSpaceConversion {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "default" => Self::DEFAULT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ColorSpaceConversion> for Any {
    fn from(s: ColorSpaceConversion) -> Any {
         match s {
            ColorSpaceConversion::NONE => Any::from("none"),
            ColorSpaceConversion::DEFAULT => Any::from("default"),
         }
    }
}
impl From<&ColorSpaceConversion> for Any {
    fn from(s: &ColorSpaceConversion) -> Any {
         match *s {
            ColorSpaceConversion::NONE => Any::from("none"),
            ColorSpaceConversion::DEFAULT => Any::from("default"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "pixelated" => Self::PIXELATED,
            "low" => Self::LOW,
            "medium" => Self::MEDIUM,
            "high" => Self::HIGH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ResizeQuality> for Any {
    fn from(s: ResizeQuality) -> Any {
         match s {
            ResizeQuality::PIXELATED => Any::from("pixelated"),
            ResizeQuality::LOW => Any::from("low"),
            ResizeQuality::MEDIUM => Any::from("medium"),
            ResizeQuality::HIGH => Any::from("high"),
         }
    }
}
impl From<&ResizeQuality> for Any {
    fn from(s: &ResizeQuality) -> Any {
         match *s {
            ResizeQuality::PIXELATED => Any::from("pixelated"),
            ResizeQuality::LOW => Any::from("low"),
            ResizeQuality::MEDIUM => Any::from("medium"),
            ResizeQuality::HIGH => Any::from("high"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WorkerType {
    CLASSIC,
    MODULE,
}
impl FromVal for WorkerType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "classic" => Self::CLASSIC,
            "module" => Self::MODULE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WorkerType> for Any {
    fn from(s: WorkerType) -> Any {
         match s {
            WorkerType::CLASSIC => Any::from("classic"),
            WorkerType::MODULE => Any::from("module"),
         }
    }
}
impl From<&WorkerType> for Any {
    fn from(s: &WorkerType) -> Any {
         match *s {
            WorkerType::CLASSIC => Any::from("classic"),
            WorkerType::MODULE => Any::from("module"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum UserIdleState {
    ACTIVE,
    IDLE,
}
impl FromVal for UserIdleState {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "active" => Self::ACTIVE,
            "idle" => Self::IDLE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<UserIdleState> for Any {
    fn from(s: UserIdleState) -> Any {
         match s {
            UserIdleState::ACTIVE => Any::from("active"),
            UserIdleState::IDLE => Any::from("idle"),
         }
    }
}
impl From<&UserIdleState> for Any {
    fn from(s: &UserIdleState) -> Any {
         match *s {
            UserIdleState::ACTIVE => Any::from("active"),
            UserIdleState::IDLE => Any::from("idle"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScreenIdleState {
    LOCKED,
    UNLOCKED,
}
impl FromVal for ScreenIdleState {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "locked" => Self::LOCKED,
            "unlocked" => Self::UNLOCKED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ScreenIdleState> for Any {
    fn from(s: ScreenIdleState) -> Any {
         match s {
            ScreenIdleState::LOCKED => Any::from("locked"),
            ScreenIdleState::UNLOCKED => Any::from("unlocked"),
         }
    }
}
impl From<&ScreenIdleState> for Any {
    fn from(s: &ScreenIdleState) -> Any {
         match *s {
            ScreenIdleState::LOCKED => Any::from("locked"),
            ScreenIdleState::UNLOCKED => Any::from("unlocked"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "never" => Self::NEVER,
            "always" => Self::ALWAYS,
            "controllable" => Self::CONTROLLABLE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RedEyeReduction> for Any {
    fn from(s: RedEyeReduction) -> Any {
         match s {
            RedEyeReduction::NEVER => Any::from("never"),
            RedEyeReduction::ALWAYS => Any::from("always"),
            RedEyeReduction::CONTROLLABLE => Any::from("controllable"),
         }
    }
}
impl From<&RedEyeReduction> for Any {
    fn from(s: &RedEyeReduction) -> Any {
         match *s {
            RedEyeReduction::NEVER => Any::from("never"),
            RedEyeReduction::ALWAYS => Any::from("always"),
            RedEyeReduction::CONTROLLABLE => Any::from("controllable"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
            "off" => Self::OFF,
            "flash" => Self::FLASH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FillLightMode> for Any {
    fn from(s: FillLightMode) -> Any {
         match s {
            FillLightMode::AUTO => Any::from("auto"),
            FillLightMode::OFF => Any::from("off"),
            FillLightMode::FLASH => Any::from("flash"),
         }
    }
}
impl From<&FillLightMode> for Any {
    fn from(s: &FillLightMode) -> Any {
         match *s {
            FillLightMode::AUTO => Any::from("auto"),
            FillLightMode::OFF => Any::from("off"),
            FillLightMode::FLASH => Any::from("flash"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "manual" => Self::MANUAL,
            "single-shot" => Self::SINGLE_SHOT,
            "continuous" => Self::CONTINUOUS,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MeteringMode> for Any {
    fn from(s: MeteringMode) -> Any {
         match s {
            MeteringMode::NONE => Any::from("none"),
            MeteringMode::MANUAL => Any::from("manual"),
            MeteringMode::SINGLE_SHOT => Any::from("single-shot"),
            MeteringMode::CONTINUOUS => Any::from("continuous"),
         }
    }
}
impl From<&MeteringMode> for Any {
    fn from(s: &MeteringMode) -> Any {
         match *s {
            MeteringMode::NONE => Any::from("none"),
            MeteringMode::MANUAL => Any::from("manual"),
            MeteringMode::SINGLE_SHOT => Any::from("single-shot"),
            MeteringMode::CONTINUOUS => Any::from("continuous"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LoginStatus {
    LOGGED_IN,
    LOGGED_OUT,
}
impl FromVal for LoginStatus {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "logged-in" => Self::LOGGED_IN,
            "logged-out" => Self::LOGGED_OUT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<LoginStatus> for Any {
    fn from(s: LoginStatus) -> Any {
         match s {
            LoginStatus::LOGGED_IN => Any::from("logged-in"),
            LoginStatus::LOGGED_OUT => Any::from("logged-out"),
         }
    }
}
impl From<&LoginStatus> for Any {
    fn from(s: &LoginStatus) -> Any {
         match *s {
            LoginStatus::LOGGED_IN => Any::from("logged-in"),
            LoginStatus::LOGGED_OUT => Any::from("logged-out"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "classic-script" => Self::CLASSIC_SCRIPT,
            "module-script" => Self::MODULE_SCRIPT,
            "event-listener" => Self::EVENT_LISTENER,
            "user-callback" => Self::USER_CALLBACK,
            "resolve-promise" => Self::RESOLVE_PROMISE,
            "reject-promise" => Self::REJECT_PROMISE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ScriptInvokerType> for Any {
    fn from(s: ScriptInvokerType) -> Any {
         match s {
            ScriptInvokerType::CLASSIC_SCRIPT => Any::from("classic-script"),
            ScriptInvokerType::MODULE_SCRIPT => Any::from("module-script"),
            ScriptInvokerType::EVENT_LISTENER => Any::from("event-listener"),
            ScriptInvokerType::USER_CALLBACK => Any::from("user-callback"),
            ScriptInvokerType::RESOLVE_PROMISE => Any::from("resolve-promise"),
            ScriptInvokerType::REJECT_PROMISE => Any::from("reject-promise"),
         }
    }
}
impl From<&ScriptInvokerType> for Any {
    fn from(s: &ScriptInvokerType) -> Any {
         match *s {
            ScriptInvokerType::CLASSIC_SCRIPT => Any::from("classic-script"),
            ScriptInvokerType::MODULE_SCRIPT => Any::from("module-script"),
            ScriptInvokerType::EVENT_LISTENER => Any::from("event-listener"),
            ScriptInvokerType::USER_CALLBACK => Any::from("user-callback"),
            ScriptInvokerType::RESOLVE_PROMISE => Any::from("resolve-promise"),
            ScriptInvokerType::REJECT_PROMISE => Any::from("reject-promise"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "self" => Self::SELF_,
            "descendant" => Self::DESCENDANT,
            "ancestor" => Self::ANCESTOR,
            "same-page" => Self::SAME_PAGE,
            "other" => Self::OTHER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ScriptWindowAttribution> for Any {
    fn from(s: ScriptWindowAttribution) -> Any {
         match s {
            ScriptWindowAttribution::SELF_ => Any::from("self"),
            ScriptWindowAttribution::DESCENDANT => Any::from("descendant"),
            ScriptWindowAttribution::ANCESTOR => Any::from("ancestor"),
            ScriptWindowAttribution::SAME_PAGE => Any::from("same-page"),
            ScriptWindowAttribution::OTHER => Any::from("other"),
         }
    }
}
impl From<&ScriptWindowAttribution> for Any {
    fn from(s: &ScriptWindowAttribution) -> Any {
         match *s {
            ScriptWindowAttribution::SELF_ => Any::from("self"),
            ScriptWindowAttribution::DESCENDANT => Any::from("descendant"),
            ScriptWindowAttribution::ANCESTOR => Any::from("ancestor"),
            ScriptWindowAttribution::SAME_PAGE => Any::from("same-page"),
            ScriptWindowAttribution::OTHER => Any::from("other"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MagnetometerLocalCoordinateSystem {
    DEVICE,
    SCREEN,
}
impl FromVal for MagnetometerLocalCoordinateSystem {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "device" => Self::DEVICE,
            "screen" => Self::SCREEN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MagnetometerLocalCoordinateSystem> for Any {
    fn from(s: MagnetometerLocalCoordinateSystem) -> Any {
         match s {
            MagnetometerLocalCoordinateSystem::DEVICE => Any::from("device"),
            MagnetometerLocalCoordinateSystem::SCREEN => Any::from("screen"),
         }
    }
}
impl From<&MagnetometerLocalCoordinateSystem> for Any {
    fn from(s: &MagnetometerLocalCoordinateSystem) -> Any {
         match *s {
            MagnetometerLocalCoordinateSystem::DEVICE => Any::from("device"),
            MagnetometerLocalCoordinateSystem::SCREEN => Any::from("screen"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AppBannerPromptOutcome {
    ACCEPTED,
    DISMISSED,
}
impl FromVal for AppBannerPromptOutcome {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "accepted" => Self::ACCEPTED,
            "dismissed" => Self::DISMISSED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AppBannerPromptOutcome> for Any {
    fn from(s: AppBannerPromptOutcome) -> Any {
         match s {
            AppBannerPromptOutcome::ACCEPTED => Any::from("accepted"),
            AppBannerPromptOutcome::DISMISSED => Any::from("dismissed"),
         }
    }
}
impl From<&AppBannerPromptOutcome> for Any {
    fn from(s: &AppBannerPromptOutcome) -> Any {
         match *s {
            AppBannerPromptOutcome::ACCEPTED => Any::from("accepted"),
            AppBannerPromptOutcome::DISMISSED => Any::from("dismissed"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "file" => Self::FILE,
            "media-source" => Self::MEDIA_SOURCE,
            "webrtc" => Self::WEBRTC,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaDecodingType> for Any {
    fn from(s: MediaDecodingType) -> Any {
         match s {
            MediaDecodingType::FILE => Any::from("file"),
            MediaDecodingType::MEDIA_SOURCE => Any::from("media-source"),
            MediaDecodingType::WEBRTC => Any::from("webrtc"),
         }
    }
}
impl From<&MediaDecodingType> for Any {
    fn from(s: &MediaDecodingType) -> Any {
         match *s {
            MediaDecodingType::FILE => Any::from("file"),
            MediaDecodingType::MEDIA_SOURCE => Any::from("media-source"),
            MediaDecodingType::WEBRTC => Any::from("webrtc"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaEncodingType {
    RECORD,
    WEBRTC,
}
impl FromVal for MediaEncodingType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "record" => Self::RECORD,
            "webrtc" => Self::WEBRTC,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaEncodingType> for Any {
    fn from(s: MediaEncodingType) -> Any {
         match s {
            MediaEncodingType::RECORD => Any::from("record"),
            MediaEncodingType::WEBRTC => Any::from("webrtc"),
         }
    }
}
impl From<&MediaEncodingType> for Any {
    fn from(s: &MediaEncodingType) -> Any {
         match *s {
            MediaEncodingType::RECORD => Any::from("record"),
            MediaEncodingType::WEBRTC => Any::from("webrtc"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "smpteSt2086" => Self::SMPTE_ST2086,
            "smpteSt2094-10" => Self::SMPTE_ST2094_10,
            "smpteSt2094-40" => Self::SMPTE_ST2094_40,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<HdrMetadataType> for Any {
    fn from(s: HdrMetadataType) -> Any {
         match s {
            HdrMetadataType::SMPTE_ST2086 => Any::from("smpteSt2086"),
            HdrMetadataType::SMPTE_ST2094_10 => Any::from("smpteSt2094-10"),
            HdrMetadataType::SMPTE_ST2094_40 => Any::from("smpteSt2094-40"),
         }
    }
}
impl From<&HdrMetadataType> for Any {
    fn from(s: &HdrMetadataType) -> Any {
         match *s {
            HdrMetadataType::SMPTE_ST2086 => Any::from("smpteSt2086"),
            HdrMetadataType::SMPTE_ST2094_10 => Any::from("smpteSt2094-10"),
            HdrMetadataType::SMPTE_ST2094_40 => Any::from("smpteSt2094-40"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "srgb" => Self::SRGB,
            "p3" => Self::P3,
            "rec2020" => Self::REC2020,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ColorGamut> for Any {
    fn from(s: ColorGamut) -> Any {
         match s {
            ColorGamut::SRGB => Any::from("srgb"),
            ColorGamut::P3 => Any::from("p3"),
            ColorGamut::REC2020 => Any::from("rec2020"),
         }
    }
}
impl From<&ColorGamut> for Any {
    fn from(s: &ColorGamut) -> Any {
         match *s {
            ColorGamut::SRGB => Any::from("srgb"),
            ColorGamut::P3 => Any::from("p3"),
            ColorGamut::REC2020 => Any::from("rec2020"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "srgb" => Self::SRGB,
            "pq" => Self::PQ,
            "hlg" => Self::HLG,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<TransferFunction> for Any {
    fn from(s: TransferFunction) -> Any {
         match s {
            TransferFunction::SRGB => Any::from("srgb"),
            TransferFunction::PQ => Any::from("pq"),
            TransferFunction::HLG => Any::from("hlg"),
         }
    }
}
impl From<&TransferFunction> for Any {
    fn from(s: &TransferFunction) -> Any {
         match *s {
            TransferFunction::SRGB => Any::from("srgb"),
            TransferFunction::PQ => Any::from("pq"),
            TransferFunction::HLG => Any::from("hlg"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "closed" => Self::CLOSED,
            "open" => Self::OPEN,
            "ended" => Self::ENDED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ReadyState> for Any {
    fn from(s: ReadyState) -> Any {
         match s {
            ReadyState::CLOSED => Any::from("closed"),
            ReadyState::OPEN => Any::from("open"),
            ReadyState::ENDED => Any::from("ended"),
         }
    }
}
impl From<&ReadyState> for Any {
    fn from(s: &ReadyState) -> Any {
         match *s {
            ReadyState::CLOSED => Any::from("closed"),
            ReadyState::OPEN => Any::from("open"),
            ReadyState::ENDED => Any::from("ended"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EndOfStreamError {
    NETWORK,
    DECODE,
}
impl FromVal for EndOfStreamError {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "network" => Self::NETWORK,
            "decode" => Self::DECODE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<EndOfStreamError> for Any {
    fn from(s: EndOfStreamError) -> Any {
         match s {
            EndOfStreamError::NETWORK => Any::from("network"),
            EndOfStreamError::DECODE => Any::from("decode"),
         }
    }
}
impl From<&EndOfStreamError> for Any {
    fn from(s: &EndOfStreamError) -> Any {
         match *s {
            EndOfStreamError::NETWORK => Any::from("network"),
            EndOfStreamError::DECODE => Any::from("decode"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AppendMode {
    SEGMENTS,
    SEQUENCE,
}
impl FromVal for AppendMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "segments" => Self::SEGMENTS,
            "sequence" => Self::SEQUENCE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AppendMode> for Any {
    fn from(s: AppendMode) -> Any {
         match s {
            AppendMode::SEGMENTS => Any::from("segments"),
            AppendMode::SEQUENCE => Any::from("sequence"),
         }
    }
}
impl From<&AppendMode> for Any {
    fn from(s: &AppendMode) -> Any {
         match *s {
            AppendMode::SEGMENTS => Any::from("segments"),
            AppendMode::SEQUENCE => Any::from("sequence"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MockCapturePromptResult {
    GRANTED,
    DENIED,
}
impl FromVal for MockCapturePromptResult {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "granted" => Self::GRANTED,
            "denied" => Self::DENIED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MockCapturePromptResult> for Any {
    fn from(s: MockCapturePromptResult) -> Any {
         match s {
            MockCapturePromptResult::GRANTED => Any::from("granted"),
            MockCapturePromptResult::DENIED => Any::from("denied"),
         }
    }
}
impl From<&MockCapturePromptResult> for Any {
    fn from(s: &MockCapturePromptResult) -> Any {
         match *s {
            MockCapturePromptResult::GRANTED => Any::from("granted"),
            MockCapturePromptResult::DENIED => Any::from("denied"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "next" => Self::NEXT,
            "previous" => Self::PREVIOUS,
            "first" => Self::FIRST,
            "last" => Self::LAST,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CaptureAction> for Any {
    fn from(s: CaptureAction) -> Any {
         match s {
            CaptureAction::NEXT => Any::from("next"),
            CaptureAction::PREVIOUS => Any::from("previous"),
            CaptureAction::FIRST => Any::from("first"),
            CaptureAction::LAST => Any::from("last"),
         }
    }
}
impl From<&CaptureAction> for Any {
    fn from(s: &CaptureAction) -> Any {
         match *s {
            CaptureAction::NEXT => Any::from("next"),
            CaptureAction::PREVIOUS => Any::from("previous"),
            CaptureAction::FIRST => Any::from("first"),
            CaptureAction::LAST => Any::from("last"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MediaStreamTrackState {
    LIVE,
    ENDED,
}
impl FromVal for MediaStreamTrackState {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "live" => Self::LIVE,
            "ended" => Self::ENDED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaStreamTrackState> for Any {
    fn from(s: MediaStreamTrackState) -> Any {
         match s {
            MediaStreamTrackState::LIVE => Any::from("live"),
            MediaStreamTrackState::ENDED => Any::from("ended"),
         }
    }
}
impl From<&MediaStreamTrackState> for Any {
    fn from(s: &MediaStreamTrackState) -> Any {
         match *s {
            MediaStreamTrackState::LIVE => Any::from("live"),
            MediaStreamTrackState::ENDED => Any::from("ended"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "user" => Self::USER,
            "environment" => Self::ENVIRONMENT,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<VideoFacingModeEnum> for Any {
    fn from(s: VideoFacingModeEnum) -> Any {
         match s {
            VideoFacingModeEnum::USER => Any::from("user"),
            VideoFacingModeEnum::ENVIRONMENT => Any::from("environment"),
            VideoFacingModeEnum::LEFT => Any::from("left"),
            VideoFacingModeEnum::RIGHT => Any::from("right"),
         }
    }
}
impl From<&VideoFacingModeEnum> for Any {
    fn from(s: &VideoFacingModeEnum) -> Any {
         match *s {
            VideoFacingModeEnum::USER => Any::from("user"),
            VideoFacingModeEnum::ENVIRONMENT => Any::from("environment"),
            VideoFacingModeEnum::LEFT => Any::from("left"),
            VideoFacingModeEnum::RIGHT => Any::from("right"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum VideoResizeModeEnum {
    NONE,
    CROP_AND_SCALE,
}
impl FromVal for VideoResizeModeEnum {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "crop-and-scale" => Self::CROP_AND_SCALE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<VideoResizeModeEnum> for Any {
    fn from(s: VideoResizeModeEnum) -> Any {
         match s {
            VideoResizeModeEnum::NONE => Any::from("none"),
            VideoResizeModeEnum::CROP_AND_SCALE => Any::from("crop-and-scale"),
         }
    }
}
impl From<&VideoResizeModeEnum> for Any {
    fn from(s: &VideoResizeModeEnum) -> Any {
         match *s {
            VideoResizeModeEnum::NONE => Any::from("none"),
            VideoResizeModeEnum::CROP_AND_SCALE => Any::from("crop-and-scale"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EchoCancellationModeEnum {
    ALL,
    REMOTE_ONLY,
}
impl FromVal for EchoCancellationModeEnum {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "all" => Self::ALL,
            "remote-only" => Self::REMOTE_ONLY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<EchoCancellationModeEnum> for Any {
    fn from(s: EchoCancellationModeEnum) -> Any {
         match s {
            EchoCancellationModeEnum::ALL => Any::from("all"),
            EchoCancellationModeEnum::REMOTE_ONLY => Any::from("remote-only"),
         }
    }
}
impl From<&EchoCancellationModeEnum> for Any {
    fn from(s: &EchoCancellationModeEnum) -> Any {
         match *s {
            EchoCancellationModeEnum::ALL => Any::from("all"),
            EchoCancellationModeEnum::REMOTE_ONLY => Any::from("remote-only"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "audioinput" => Self::AUDIOINPUT,
            "audiooutput" => Self::AUDIOOUTPUT,
            "videoinput" => Self::VIDEOINPUT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaDeviceKind> for Any {
    fn from(s: MediaDeviceKind) -> Any {
         match s {
            MediaDeviceKind::AUDIOINPUT => Any::from("audioinput"),
            MediaDeviceKind::AUDIOOUTPUT => Any::from("audiooutput"),
            MediaDeviceKind::VIDEOINPUT => Any::from("videoinput"),
         }
    }
}
impl From<&MediaDeviceKind> for Any {
    fn from(s: &MediaDeviceKind) -> Any {
         match *s {
            MediaDeviceKind::AUDIOINPUT => Any::from("audioinput"),
            MediaDeviceKind::AUDIOOUTPUT => Any::from("audiooutput"),
            MediaDeviceKind::VIDEOINPUT => Any::from("videoinput"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "paused" => Self::PAUSED,
            "playing" => Self::PLAYING,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaSessionPlaybackState> for Any {
    fn from(s: MediaSessionPlaybackState) -> Any {
         match s {
            MediaSessionPlaybackState::NONE => Any::from("none"),
            MediaSessionPlaybackState::PAUSED => Any::from("paused"),
            MediaSessionPlaybackState::PLAYING => Any::from("playing"),
         }
    }
}
impl From<&MediaSessionPlaybackState> for Any {
    fn from(s: &MediaSessionPlaybackState) -> Any {
         match *s {
            MediaSessionPlaybackState::NONE => Any::from("none"),
            MediaSessionPlaybackState::PAUSED => Any::from("paused"),
            MediaSessionPlaybackState::PLAYING => Any::from("playing"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MediaSessionAction> for Any {
    fn from(s: MediaSessionAction) -> Any {
         match s {
            MediaSessionAction::PLAY => Any::from("play"),
            MediaSessionAction::PAUSE => Any::from("pause"),
            MediaSessionAction::SEEKBACKWARD => Any::from("seekbackward"),
            MediaSessionAction::SEEKFORWARD => Any::from("seekforward"),
            MediaSessionAction::PREVIOUSTRACK => Any::from("previoustrack"),
            MediaSessionAction::NEXTTRACK => Any::from("nexttrack"),
            MediaSessionAction::SKIPAD => Any::from("skipad"),
            MediaSessionAction::STOP => Any::from("stop"),
            MediaSessionAction::SEEKTO => Any::from("seekto"),
            MediaSessionAction::TOGGLEMICROPHONE => Any::from("togglemicrophone"),
            MediaSessionAction::TOGGLECAMERA => Any::from("togglecamera"),
            MediaSessionAction::TOGGLESCREENSHARE => Any::from("togglescreenshare"),
            MediaSessionAction::HANGUP => Any::from("hangup"),
            MediaSessionAction::PREVIOUSSLIDE => Any::from("previousslide"),
            MediaSessionAction::NEXTSLIDE => Any::from("nextslide"),
            MediaSessionAction::ENTERPICTUREINPICTURE => Any::from("enterpictureinpicture"),
            MediaSessionAction::VOICEACTIVITY => Any::from("voiceactivity"),
         }
    }
}
impl From<&MediaSessionAction> for Any {
    fn from(s: &MediaSessionAction) -> Any {
         match *s {
            MediaSessionAction::PLAY => Any::from("play"),
            MediaSessionAction::PAUSE => Any::from("pause"),
            MediaSessionAction::SEEKBACKWARD => Any::from("seekbackward"),
            MediaSessionAction::SEEKFORWARD => Any::from("seekforward"),
            MediaSessionAction::PREVIOUSTRACK => Any::from("previoustrack"),
            MediaSessionAction::NEXTTRACK => Any::from("nexttrack"),
            MediaSessionAction::SKIPAD => Any::from("skipad"),
            MediaSessionAction::STOP => Any::from("stop"),
            MediaSessionAction::SEEKTO => Any::from("seekto"),
            MediaSessionAction::TOGGLEMICROPHONE => Any::from("togglemicrophone"),
            MediaSessionAction::TOGGLECAMERA => Any::from("togglecamera"),
            MediaSessionAction::TOGGLESCREENSHARE => Any::from("togglescreenshare"),
            MediaSessionAction::HANGUP => Any::from("hangup"),
            MediaSessionAction::PREVIOUSSLIDE => Any::from("previousslide"),
            MediaSessionAction::NEXTSLIDE => Any::from("nextslide"),
            MediaSessionAction::ENTERPICTUREINPICTURE => Any::from("enterpictureinpicture"),
            MediaSessionAction::VOICEACTIVITY => Any::from("voiceactivity"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BitrateMode {
    CONSTANT,
    VARIABLE,
}
impl FromVal for BitrateMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "constant" => Self::CONSTANT,
            "variable" => Self::VARIABLE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<BitrateMode> for Any {
    fn from(s: BitrateMode) -> Any {
         match s {
            BitrateMode::CONSTANT => Any::from("constant"),
            BitrateMode::VARIABLE => Any::from("variable"),
         }
    }
}
impl From<&BitrateMode> for Any {
    fn from(s: &BitrateMode) -> Any {
         match *s {
            BitrateMode::CONSTANT => Any::from("constant"),
            BitrateMode::VARIABLE => Any::from("variable"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "inactive" => Self::INACTIVE,
            "recording" => Self::RECORDING,
            "paused" => Self::PAUSED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RecordingState> for Any {
    fn from(s: RecordingState) -> Any {
         match s {
            RecordingState::INACTIVE => Any::from("inactive"),
            RecordingState::RECORDING => Any::from("recording"),
            RecordingState::PAUSED => Any::from("paused"),
         }
    }
}
impl From<&RecordingState> for Any {
    fn from(s: &RecordingState) -> Any {
         match *s {
            RecordingState::INACTIVE => Any::from("inactive"),
            RecordingState::RECORDING => Any::from("recording"),
            RecordingState::PAUSED => Any::from("paused"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "maintain-framerate" => Self::MAINTAIN_FRAMERATE,
            "maintain-resolution" => Self::MAINTAIN_RESOLUTION,
            "balanced" => Self::BALANCED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCDegradationPreference> for Any {
    fn from(s: RTCDegradationPreference) -> Any {
         match s {
            RTCDegradationPreference::MAINTAIN_FRAMERATE => Any::from("maintain-framerate"),
            RTCDegradationPreference::MAINTAIN_RESOLUTION => Any::from("maintain-resolution"),
            RTCDegradationPreference::BALANCED => Any::from("balanced"),
         }
    }
}
impl From<&RTCDegradationPreference> for Any {
    fn from(s: &RTCDegradationPreference) -> Any {
         match *s {
            RTCDegradationPreference::MAINTAIN_FRAMERATE => Any::from("maintain-framerate"),
            RTCDegradationPreference::MAINTAIN_RESOLUTION => Any::from("maintain-resolution"),
            RTCDegradationPreference::BALANCED => Any::from("balanced"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "navigate" => Self::NAVIGATE,
            "reload" => Self::RELOAD,
            "back_forward" => Self::BACK_FORWARD,
            "prerender" => Self::PRERENDER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<NavigationTimingType> for Any {
    fn from(s: NavigationTimingType) -> Any {
         match s {
            NavigationTimingType::NAVIGATE => Any::from("navigate"),
            NavigationTimingType::RELOAD => Any::from("reload"),
            NavigationTimingType::BACK_FORWARD => Any::from("back_forward"),
            NavigationTimingType::PRERENDER => Any::from("prerender"),
         }
    }
}
impl From<&NavigationTimingType> for Any {
    fn from(s: &NavigationTimingType) -> Any {
         match *s {
            NavigationTimingType::NAVIGATE => Any::from("navigate"),
            NavigationTimingType::RELOAD => Any::from("reload"),
            NavigationTimingType::BACK_FORWARD => Any::from("back_forward"),
            NavigationTimingType::PRERENDER => Any::from("prerender"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ConnectionType> for Any {
    fn from(s: ConnectionType) -> Any {
         match s {
            ConnectionType::BLUETOOTH => Any::from("bluetooth"),
            ConnectionType::CELLULAR => Any::from("cellular"),
            ConnectionType::ETHERNET => Any::from("ethernet"),
            ConnectionType::MIXED => Any::from("mixed"),
            ConnectionType::NONE => Any::from("none"),
            ConnectionType::OTHER => Any::from("other"),
            ConnectionType::UNKNOWN => Any::from("unknown"),
            ConnectionType::WIFI => Any::from("wifi"),
            ConnectionType::WIMAX => Any::from("wimax"),
         }
    }
}
impl From<&ConnectionType> for Any {
    fn from(s: &ConnectionType) -> Any {
         match *s {
            ConnectionType::BLUETOOTH => Any::from("bluetooth"),
            ConnectionType::CELLULAR => Any::from("cellular"),
            ConnectionType::ETHERNET => Any::from("ethernet"),
            ConnectionType::MIXED => Any::from("mixed"),
            ConnectionType::NONE => Any::from("none"),
            ConnectionType::OTHER => Any::from("other"),
            ConnectionType::UNKNOWN => Any::from("unknown"),
            ConnectionType::WIFI => Any::from("wifi"),
            ConnectionType::WIMAX => Any::from("wimax"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "2g" => Self::_2G,
            "3g" => Self::_3G,
            "4g" => Self::_4G,
            "slow-2g" => Self::SLOW_2G,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<EffectiveConnectionType> for Any {
    fn from(s: EffectiveConnectionType) -> Any {
         match s {
            EffectiveConnectionType::_2G => Any::from("2g"),
            EffectiveConnectionType::_3G => Any::from("3g"),
            EffectiveConnectionType::_4G => Any::from("4g"),
            EffectiveConnectionType::SLOW_2G => Any::from("slow-2g"),
         }
    }
}
impl From<&EffectiveConnectionType> for Any {
    fn from(s: &EffectiveConnectionType) -> Any {
         match *s {
            EffectiveConnectionType::_2G => Any::from("2g"),
            EffectiveConnectionType::_3G => Any::from("3g"),
            EffectiveConnectionType::_4G => Any::from("4g"),
            EffectiveConnectionType::SLOW_2G => Any::from("slow-2g"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "default" => Self::DEFAULT,
            "denied" => Self::DENIED,
            "granted" => Self::GRANTED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<NotificationPermission> for Any {
    fn from(s: NotificationPermission) -> Any {
         match s {
            NotificationPermission::DEFAULT => Any::from("default"),
            NotificationPermission::DENIED => Any::from("denied"),
            NotificationPermission::GRANTED => Any::from("granted"),
         }
    }
}
impl From<&NotificationPermission> for Any {
    fn from(s: &NotificationPermission) -> Any {
         match *s {
            NotificationPermission::DEFAULT => Any::from("default"),
            NotificationPermission::DENIED => Any::from("denied"),
            NotificationPermission::GRANTED => Any::from("granted"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
            "ltr" => Self::LTR,
            "rtl" => Self::RTL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<NotificationDirection> for Any {
    fn from(s: NotificationDirection) -> Any {
         match s {
            NotificationDirection::AUTO => Any::from("auto"),
            NotificationDirection::LTR => Any::from("ltr"),
            NotificationDirection::RTL => Any::from("rtl"),
         }
    }
}
impl From<&NotificationDirection> for Any {
    fn from(s: &NotificationDirection) -> Any {
         match *s {
            NotificationDirection::AUTO => Any::from("auto"),
            NotificationDirection::LTR => Any::from("ltr"),
            NotificationDirection::RTL => Any::from("rtl"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OrientationSensorLocalCoordinateSystem {
    DEVICE,
    SCREEN,
}
impl FromVal for OrientationSensorLocalCoordinateSystem {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "device" => Self::DEVICE,
            "screen" => Self::SCREEN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OrientationSensorLocalCoordinateSystem> for Any {
    fn from(s: OrientationSensorLocalCoordinateSystem) -> Any {
         match s {
            OrientationSensorLocalCoordinateSystem::DEVICE => Any::from("device"),
            OrientationSensorLocalCoordinateSystem::SCREEN => Any::from("screen"),
         }
    }
}
impl From<&OrientationSensorLocalCoordinateSystem> for Any {
    fn from(s: &OrientationSensorLocalCoordinateSystem) -> Any {
         match *s {
            OrientationSensorLocalCoordinateSystem::DEVICE => Any::from("device"),
            OrientationSensorLocalCoordinateSystem::SCREEN => Any::from("screen"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ClientLifecycleState {
    ACTIVE,
    FROZEN,
}
impl FromVal for ClientLifecycleState {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "active" => Self::ACTIVE,
            "frozen" => Self::FROZEN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ClientLifecycleState> for Any {
    fn from(s: ClientLifecycleState) -> Any {
         match s {
            ClientLifecycleState::ACTIVE => Any::from("active"),
            ClientLifecycleState::FROZEN => Any::from("frozen"),
         }
    }
}
impl From<&ClientLifecycleState> for Any {
    fn from(s: &ClientLifecycleState) -> Any {
         match *s {
            ClientLifecycleState::ACTIVE => Any::from("active"),
            ClientLifecycleState::FROZEN => Any::from("frozen"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "shippingAddress" => Self::SHIPPING_ADDRESS,
            "payerName" => Self::PAYER_NAME,
            "payerPhone" => Self::PAYER_PHONE,
            "payerEmail" => Self::PAYER_EMAIL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PaymentDelegation> for Any {
    fn from(s: PaymentDelegation) -> Any {
         match s {
            PaymentDelegation::SHIPPING_ADDRESS => Any::from("shippingAddress"),
            PaymentDelegation::PAYER_NAME => Any::from("payerName"),
            PaymentDelegation::PAYER_PHONE => Any::from("payerPhone"),
            PaymentDelegation::PAYER_EMAIL => Any::from("payerEmail"),
         }
    }
}
impl From<&PaymentDelegation> for Any {
    fn from(s: &PaymentDelegation) -> Any {
         match *s {
            PaymentDelegation::SHIPPING_ADDRESS => Any::from("shippingAddress"),
            PaymentDelegation::PAYER_NAME => Any::from("payerName"),
            PaymentDelegation::PAYER_PHONE => Any::from("payerPhone"),
            PaymentDelegation::PAYER_EMAIL => Any::from("payerEmail"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "shipping" => Self::SHIPPING,
            "delivery" => Self::DELIVERY,
            "pickup" => Self::PICKUP,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PaymentShippingType> for Any {
    fn from(s: PaymentShippingType) -> Any {
         match s {
            PaymentShippingType::SHIPPING => Any::from("shipping"),
            PaymentShippingType::DELIVERY => Any::from("delivery"),
            PaymentShippingType::PICKUP => Any::from("pickup"),
         }
    }
}
impl From<&PaymentShippingType> for Any {
    fn from(s: &PaymentShippingType) -> Any {
         match *s {
            PaymentShippingType::SHIPPING => Any::from("shipping"),
            PaymentShippingType::DELIVERY => Any::from("delivery"),
            PaymentShippingType::PICKUP => Any::from("pickup"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "fail" => Self::FAIL,
            "success" => Self::SUCCESS,
            "unknown" => Self::UNKNOWN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PaymentComplete> for Any {
    fn from(s: PaymentComplete) -> Any {
         match s {
            PaymentComplete::FAIL => Any::from("fail"),
            PaymentComplete::SUCCESS => Any::from("success"),
            PaymentComplete::UNKNOWN => Any::from("unknown"),
         }
    }
}
impl From<&PaymentComplete> for Any {
    fn from(s: &PaymentComplete) -> Any {
         match *s {
            PaymentComplete::FAIL => Any::from("fail"),
            PaymentComplete::SUCCESS => Any::from("success"),
            PaymentComplete::UNKNOWN => Any::from("unknown"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "granted" => Self::GRANTED,
            "denied" => Self::DENIED,
            "prompt" => Self::PROMPT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PermissionState> for Any {
    fn from(s: PermissionState) -> Any {
         match s {
            PermissionState::GRANTED => Any::from("granted"),
            PermissionState::DENIED => Any::from("denied"),
            PermissionState::PROMPT => Any::from("prompt"),
         }
    }
}
impl From<&PermissionState> for Any {
    fn from(s: &PermissionState) -> Any {
         match *s {
            PermissionState::GRANTED => Any::from("granted"),
            PermissionState::DENIED => Any::from("denied"),
            PermissionState::PROMPT => Any::from("prompt"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "block" => Self::BLOCK,
            "inline" => Self::INLINE,
            "x" => Self::X,
            "y" => Self::Y,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PointerAxis> for Any {
    fn from(s: PointerAxis) -> Any {
         match s {
            PointerAxis::BLOCK => Any::from("block"),
            PointerAxis::INLINE => Any::from("inline"),
            PointerAxis::X => Any::from("x"),
            PointerAxis::Y => Any::from("y"),
         }
    }
}
impl From<&PointerAxis> for Any {
    fn from(s: &PointerAxis) -> Any {
         match *s {
            PointerAxis::BLOCK => Any::from("block"),
            PointerAxis::INLINE => Any::from("inline"),
            PointerAxis::X => Any::from("x"),
            PointerAxis::Y => Any::from("y"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "connecting" => Self::CONNECTING,
            "connected" => Self::CONNECTED,
            "closed" => Self::CLOSED,
            "terminated" => Self::TERMINATED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PresentationConnectionState> for Any {
    fn from(s: PresentationConnectionState) -> Any {
         match s {
            PresentationConnectionState::CONNECTING => Any::from("connecting"),
            PresentationConnectionState::CONNECTED => Any::from("connected"),
            PresentationConnectionState::CLOSED => Any::from("closed"),
            PresentationConnectionState::TERMINATED => Any::from("terminated"),
         }
    }
}
impl From<&PresentationConnectionState> for Any {
    fn from(s: &PresentationConnectionState) -> Any {
         match *s {
            PresentationConnectionState::CONNECTING => Any::from("connecting"),
            PresentationConnectionState::CONNECTED => Any::from("connected"),
            PresentationConnectionState::CLOSED => Any::from("closed"),
            PresentationConnectionState::TERMINATED => Any::from("terminated"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "error" => Self::ERROR,
            "closed" => Self::CLOSED,
            "wentaway" => Self::WENTAWAY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PresentationConnectionCloseReason> for Any {
    fn from(s: PresentationConnectionCloseReason) -> Any {
         match s {
            PresentationConnectionCloseReason::ERROR => Any::from("error"),
            PresentationConnectionCloseReason::CLOSED => Any::from("closed"),
            PresentationConnectionCloseReason::WENTAWAY => Any::from("wentaway"),
         }
    }
}
impl From<&PresentationConnectionCloseReason> for Any {
    fn from(s: &PresentationConnectionCloseReason) -> Any {
         match *s {
            PresentationConnectionCloseReason::ERROR => Any::from("error"),
            PresentationConnectionCloseReason::CLOSED => Any::from("closed"),
            PresentationConnectionCloseReason::WENTAWAY => Any::from("wentaway"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PrivateAttributionAggregationProtocol {
    DAP_12_HISTOGRAM,
    TEE_00,
}
impl FromVal for PrivateAttributionAggregationProtocol {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "dap-12-histogram" => Self::DAP_12_HISTOGRAM,
            "tee-00" => Self::TEE_00,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PrivateAttributionAggregationProtocol> for Any {
    fn from(s: PrivateAttributionAggregationProtocol) -> Any {
         match s {
            PrivateAttributionAggregationProtocol::DAP_12_HISTOGRAM => Any::from("dap-12-histogram"),
            PrivateAttributionAggregationProtocol::TEE_00 => Any::from("tee-00"),
         }
    }
}
impl From<&PrivateAttributionAggregationProtocol> for Any {
    fn from(s: &PrivateAttributionAggregationProtocol) -> Any {
         match *s {
            PrivateAttributionAggregationProtocol::DAP_12_HISTOGRAM => Any::from("dap-12-histogram"),
            PrivateAttributionAggregationProtocol::TEE_00 => Any::from("tee-00"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AttributionLogic {
    LAST_TOUCH,
}
impl FromVal for AttributionLogic {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "last-touch" => Self::LAST_TOUCH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AttributionLogic> for Any {
    fn from(s: AttributionLogic) -> Any {
         match s {
            AttributionLogic::LAST_TOUCH => Any::from("last-touch"),
         }
    }
}
impl From<&AttributionLogic> for Any {
    fn from(s: &AttributionLogic) -> Any {
         match *s {
            AttributionLogic::LAST_TOUCH => Any::from("last-touch"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "public" => Self::PUBLIC,
            "private" => Self::PRIVATE,
            "local" => Self::LOCAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<IPAddressSpace> for Any {
    fn from(s: IPAddressSpace) -> Any {
         match s {
            IPAddressSpace::PUBLIC => Any::from("public"),
            IPAddressSpace::PRIVATE => Any::from("private"),
            IPAddressSpace::LOCAL => Any::from("local"),
         }
    }
}
impl From<&IPAddressSpace> for Any {
    fn from(s: &IPAddressSpace) -> Any {
         match *s {
            IPAddressSpace::PUBLIC => Any::from("public"),
            IPAddressSpace::PRIVATE => Any::from("private"),
            IPAddressSpace::LOCAL => Any::from("local"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PushEncryptionKeyName {
    P256DH,
    AUTH,
}
impl FromVal for PushEncryptionKeyName {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "p256dh" => Self::P256DH,
            "auth" => Self::AUTH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PushEncryptionKeyName> for Any {
    fn from(s: PushEncryptionKeyName) -> Any {
         match s {
            PushEncryptionKeyName::P256DH => Any::from("p256dh"),
            PushEncryptionKeyName::AUTH => Any::from("auth"),
         }
    }
}
impl From<&PushEncryptionKeyName> for Any {
    fn from(s: &PushEncryptionKeyName) -> Any {
         match *s {
            PushEncryptionKeyName::P256DH => Any::from("p256dh"),
            PushEncryptionKeyName::AUTH => Any::from("auth"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ReferrerPolicy> for Any {
    fn from(s: ReferrerPolicy) -> Any {
         match s {
            ReferrerPolicy::NONE => Any::from(""),
            ReferrerPolicy::NO_REFERRER => Any::from("no-referrer"),
            ReferrerPolicy::NO_REFERRER_WHEN_DOWNGRADE => Any::from("no-referrer-when-downgrade"),
            ReferrerPolicy::SAME_ORIGIN => Any::from("same-origin"),
            ReferrerPolicy::ORIGIN => Any::from("origin"),
            ReferrerPolicy::STRICT_ORIGIN => Any::from("strict-origin"),
            ReferrerPolicy::ORIGIN_WHEN_CROSS_ORIGIN => Any::from("origin-when-cross-origin"),
            ReferrerPolicy::STRICT_ORIGIN_WHEN_CROSS_ORIGIN => Any::from("strict-origin-when-cross-origin"),
            ReferrerPolicy::UNSAFE_URL => Any::from("unsafe-url"),
         }
    }
}
impl From<&ReferrerPolicy> for Any {
    fn from(s: &ReferrerPolicy) -> Any {
         match *s {
            ReferrerPolicy::NONE => Any::from(""),
            ReferrerPolicy::NO_REFERRER => Any::from("no-referrer"),
            ReferrerPolicy::NO_REFERRER_WHEN_DOWNGRADE => Any::from("no-referrer-when-downgrade"),
            ReferrerPolicy::SAME_ORIGIN => Any::from("same-origin"),
            ReferrerPolicy::ORIGIN => Any::from("origin"),
            ReferrerPolicy::STRICT_ORIGIN => Any::from("strict-origin"),
            ReferrerPolicy::ORIGIN_WHEN_CROSS_ORIGIN => Any::from("origin-when-cross-origin"),
            ReferrerPolicy::STRICT_ORIGIN_WHEN_CROSS_ORIGIN => Any::from("strict-origin-when-cross-origin"),
            ReferrerPolicy::UNSAFE_URL => Any::from("unsafe-url"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "connecting" => Self::CONNECTING,
            "connected" => Self::CONNECTED,
            "disconnected" => Self::DISCONNECTED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RemotePlaybackState> for Any {
    fn from(s: RemotePlaybackState) -> Any {
         match s {
            RemotePlaybackState::CONNECTING => Any::from("connecting"),
            RemotePlaybackState::CONNECTED => Any::from("connected"),
            RemotePlaybackState::DISCONNECTED => Any::from("disconnected"),
         }
    }
}
impl From<&RemotePlaybackState> for Any {
    fn from(s: &RemotePlaybackState) -> Any {
         match *s {
            RemotePlaybackState::CONNECTING => Any::from("connecting"),
            RemotePlaybackState::CONNECTED => Any::from("connected"),
            RemotePlaybackState::DISCONNECTED => Any::from("disconnected"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "border-box" => Self::BORDER_BOX,
            "content-box" => Self::CONTENT_BOX,
            "device-pixel-content-box" => Self::DEVICE_PIXEL_CONTENT_BOX,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ResizeObserverBoxOptions> for Any {
    fn from(s: ResizeObserverBoxOptions) -> Any {
         match s {
            ResizeObserverBoxOptions::BORDER_BOX => Any::from("border-box"),
            ResizeObserverBoxOptions::CONTENT_BOX => Any::from("content-box"),
            ResizeObserverBoxOptions::DEVICE_PIXEL_CONTENT_BOX => Any::from("device-pixel-content-box"),
         }
    }
}
impl From<&ResizeObserverBoxOptions> for Any {
    fn from(s: &ResizeObserverBoxOptions) -> Any {
         match *s {
            ResizeObserverBoxOptions::BORDER_BOX => Any::from("border-box"),
            ResizeObserverBoxOptions::CONTENT_BOX => Any::from("content-box"),
            ResizeObserverBoxOptions::DEVICE_PIXEL_CONTENT_BOX => Any::from("device-pixel-content-box"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RenderBlockingStatusType {
    BLOCKING,
    NON_BLOCKING,
}
impl FromVal for RenderBlockingStatusType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "blocking" => Self::BLOCKING,
            "non-blocking" => Self::NON_BLOCKING,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RenderBlockingStatusType> for Any {
    fn from(s: RenderBlockingStatusType) -> Any {
         match s {
            RenderBlockingStatusType::BLOCKING => Any::from("blocking"),
            RenderBlockingStatusType::NON_BLOCKING => Any::from("non-blocking"),
         }
    }
}
impl From<&RenderBlockingStatusType> for Any {
    fn from(s: &RenderBlockingStatusType) -> Any {
         match *s {
            RenderBlockingStatusType::BLOCKING => Any::from("blocking"),
            RenderBlockingStatusType::NON_BLOCKING => Any::from("non-blocking"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SameSiteCookiesType {
    ALL,
    NONE,
}
impl FromVal for SameSiteCookiesType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "all" => Self::ALL,
            "none" => Self::NONE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SameSiteCookiesType> for Any {
    fn from(s: SameSiteCookiesType) -> Any {
         match s {
            SameSiteCookiesType::ALL => Any::from("all"),
            SameSiteCookiesType::NONE => Any::from("none"),
         }
    }
}
impl From<&SameSiteCookiesType> for Any {
    fn from(s: &SameSiteCookiesType) -> Any {
         match *s {
            SameSiteCookiesType::ALL => Any::from("all"),
            SameSiteCookiesType::NONE => Any::from("none"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SanitizerPresets {
    DEFAULT,
}
impl FromVal for SanitizerPresets {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "default" => Self::DEFAULT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SanitizerPresets> for Any {
    fn from(s: SanitizerPresets) -> Any {
         match s {
            SanitizerPresets::DEFAULT => Any::from("default"),
         }
    }
}
impl From<&SanitizerPresets> for Any {
    fn from(s: &SanitizerPresets) -> Any {
         match *s {
            SanitizerPresets::DEFAULT => Any::from("default"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "user-blocking" => Self::USER_BLOCKING,
            "user-visible" => Self::USER_VISIBLE,
            "background" => Self::BACKGROUND,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<TaskPriority> for Any {
    fn from(s: TaskPriority) -> Any {
         match s {
            TaskPriority::USER_BLOCKING => Any::from("user-blocking"),
            TaskPriority::USER_VISIBLE => Any::from("user-visible"),
            TaskPriority::BACKGROUND => Any::from("background"),
         }
    }
}
impl From<&TaskPriority> for Any {
    fn from(s: &TaskPriority) -> Any {
         match *s {
            TaskPriority::USER_BLOCKING => Any::from("user-blocking"),
            TaskPriority::USER_VISIBLE => Any::from("user-visible"),
            TaskPriority::BACKGROUND => Any::from("background"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "focus-capturing-application" => Self::FOCUS_CAPTURING_APPLICATION,
            "focus-captured-surface" => Self::FOCUS_CAPTURED_SURFACE,
            "no-focus-change" => Self::NO_FOCUS_CHANGE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CaptureStartFocusBehavior> for Any {
    fn from(s: CaptureStartFocusBehavior) -> Any {
         match s {
            CaptureStartFocusBehavior::FOCUS_CAPTURING_APPLICATION => Any::from("focus-capturing-application"),
            CaptureStartFocusBehavior::FOCUS_CAPTURED_SURFACE => Any::from("focus-captured-surface"),
            CaptureStartFocusBehavior::NO_FOCUS_CHANGE => Any::from("no-focus-change"),
         }
    }
}
impl From<&CaptureStartFocusBehavior> for Any {
    fn from(s: &CaptureStartFocusBehavior) -> Any {
         match *s {
            CaptureStartFocusBehavior::FOCUS_CAPTURING_APPLICATION => Any::from("focus-capturing-application"),
            CaptureStartFocusBehavior::FOCUS_CAPTURED_SURFACE => Any::from("focus-captured-surface"),
            CaptureStartFocusBehavior::NO_FOCUS_CHANGE => Any::from("no-focus-change"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SelfCapturePreferenceEnum {
    INCLUDE,
    EXCLUDE,
}
impl FromVal for SelfCapturePreferenceEnum {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "include" => Self::INCLUDE,
            "exclude" => Self::EXCLUDE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SelfCapturePreferenceEnum> for Any {
    fn from(s: SelfCapturePreferenceEnum) -> Any {
         match s {
            SelfCapturePreferenceEnum::INCLUDE => Any::from("include"),
            SelfCapturePreferenceEnum::EXCLUDE => Any::from("exclude"),
         }
    }
}
impl From<&SelfCapturePreferenceEnum> for Any {
    fn from(s: &SelfCapturePreferenceEnum) -> Any {
         match *s {
            SelfCapturePreferenceEnum::INCLUDE => Any::from("include"),
            SelfCapturePreferenceEnum::EXCLUDE => Any::from("exclude"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SystemAudioPreferenceEnum {
    INCLUDE,
    EXCLUDE,
}
impl FromVal for SystemAudioPreferenceEnum {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "include" => Self::INCLUDE,
            "exclude" => Self::EXCLUDE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SystemAudioPreferenceEnum> for Any {
    fn from(s: SystemAudioPreferenceEnum) -> Any {
         match s {
            SystemAudioPreferenceEnum::INCLUDE => Any::from("include"),
            SystemAudioPreferenceEnum::EXCLUDE => Any::from("exclude"),
         }
    }
}
impl From<&SystemAudioPreferenceEnum> for Any {
    fn from(s: &SystemAudioPreferenceEnum) -> Any {
         match *s {
            SystemAudioPreferenceEnum::INCLUDE => Any::from("include"),
            SystemAudioPreferenceEnum::EXCLUDE => Any::from("exclude"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "system" => Self::SYSTEM,
            "window" => Self::WINDOW,
            "exclude" => Self::EXCLUDE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WindowAudioPreferenceEnum> for Any {
    fn from(s: WindowAudioPreferenceEnum) -> Any {
         match s {
            WindowAudioPreferenceEnum::SYSTEM => Any::from("system"),
            WindowAudioPreferenceEnum::WINDOW => Any::from("window"),
            WindowAudioPreferenceEnum::EXCLUDE => Any::from("exclude"),
         }
    }
}
impl From<&WindowAudioPreferenceEnum> for Any {
    fn from(s: &WindowAudioPreferenceEnum) -> Any {
         match *s {
            WindowAudioPreferenceEnum::SYSTEM => Any::from("system"),
            WindowAudioPreferenceEnum::WINDOW => Any::from("window"),
            WindowAudioPreferenceEnum::EXCLUDE => Any::from("exclude"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SurfaceSwitchingPreferenceEnum {
    INCLUDE,
    EXCLUDE,
}
impl FromVal for SurfaceSwitchingPreferenceEnum {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "include" => Self::INCLUDE,
            "exclude" => Self::EXCLUDE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SurfaceSwitchingPreferenceEnum> for Any {
    fn from(s: SurfaceSwitchingPreferenceEnum) -> Any {
         match s {
            SurfaceSwitchingPreferenceEnum::INCLUDE => Any::from("include"),
            SurfaceSwitchingPreferenceEnum::EXCLUDE => Any::from("exclude"),
         }
    }
}
impl From<&SurfaceSwitchingPreferenceEnum> for Any {
    fn from(s: &SurfaceSwitchingPreferenceEnum) -> Any {
         match *s {
            SurfaceSwitchingPreferenceEnum::INCLUDE => Any::from("include"),
            SurfaceSwitchingPreferenceEnum::EXCLUDE => Any::from("exclude"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MonitorTypeSurfacesEnum {
    INCLUDE,
    EXCLUDE,
}
impl FromVal for MonitorTypeSurfacesEnum {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "include" => Self::INCLUDE,
            "exclude" => Self::EXCLUDE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MonitorTypeSurfacesEnum> for Any {
    fn from(s: MonitorTypeSurfacesEnum) -> Any {
         match s {
            MonitorTypeSurfacesEnum::INCLUDE => Any::from("include"),
            MonitorTypeSurfacesEnum::EXCLUDE => Any::from("exclude"),
         }
    }
}
impl From<&MonitorTypeSurfacesEnum> for Any {
    fn from(s: &MonitorTypeSurfacesEnum) -> Any {
         match *s {
            MonitorTypeSurfacesEnum::INCLUDE => Any::from("include"),
            MonitorTypeSurfacesEnum::EXCLUDE => Any::from("exclude"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "monitor" => Self::MONITOR,
            "window" => Self::WINDOW,
            "browser" => Self::BROWSER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<DisplayCaptureSurfaceType> for Any {
    fn from(s: DisplayCaptureSurfaceType) -> Any {
         match s {
            DisplayCaptureSurfaceType::MONITOR => Any::from("monitor"),
            DisplayCaptureSurfaceType::WINDOW => Any::from("window"),
            DisplayCaptureSurfaceType::BROWSER => Any::from("browser"),
         }
    }
}
impl From<&DisplayCaptureSurfaceType> for Any {
    fn from(s: &DisplayCaptureSurfaceType) -> Any {
         match *s {
            DisplayCaptureSurfaceType::MONITOR => Any::from("monitor"),
            DisplayCaptureSurfaceType::WINDOW => Any::from("window"),
            DisplayCaptureSurfaceType::BROWSER => Any::from("browser"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "never" => Self::NEVER,
            "always" => Self::ALWAYS,
            "motion" => Self::MOTION,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CursorCaptureConstraint> for Any {
    fn from(s: CursorCaptureConstraint) -> Any {
         match s {
            CursorCaptureConstraint::NEVER => Any::from("never"),
            CursorCaptureConstraint::ALWAYS => Any::from("always"),
            CursorCaptureConstraint::MOTION => Any::from("motion"),
         }
    }
}
impl From<&CursorCaptureConstraint> for Any {
    fn from(s: &CursorCaptureConstraint) -> Any {
         match *s {
            CursorCaptureConstraint::NEVER => Any::from("never"),
            CursorCaptureConstraint::ALWAYS => Any::from("always"),
            CursorCaptureConstraint::MOTION => Any::from("motion"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OrientationLockType> for Any {
    fn from(s: OrientationLockType) -> Any {
         match s {
            OrientationLockType::ANY => Any::from("any"),
            OrientationLockType::NATURAL => Any::from("natural"),
            OrientationLockType::LANDSCAPE => Any::from("landscape"),
            OrientationLockType::PORTRAIT => Any::from("portrait"),
            OrientationLockType::PORTRAIT_PRIMARY => Any::from("portrait-primary"),
            OrientationLockType::PORTRAIT_SECONDARY => Any::from("portrait-secondary"),
            OrientationLockType::LANDSCAPE_PRIMARY => Any::from("landscape-primary"),
            OrientationLockType::LANDSCAPE_SECONDARY => Any::from("landscape-secondary"),
         }
    }
}
impl From<&OrientationLockType> for Any {
    fn from(s: &OrientationLockType) -> Any {
         match *s {
            OrientationLockType::ANY => Any::from("any"),
            OrientationLockType::NATURAL => Any::from("natural"),
            OrientationLockType::LANDSCAPE => Any::from("landscape"),
            OrientationLockType::PORTRAIT => Any::from("portrait"),
            OrientationLockType::PORTRAIT_PRIMARY => Any::from("portrait-primary"),
            OrientationLockType::PORTRAIT_SECONDARY => Any::from("portrait-secondary"),
            OrientationLockType::LANDSCAPE_PRIMARY => Any::from("landscape-primary"),
            OrientationLockType::LANDSCAPE_SECONDARY => Any::from("landscape-secondary"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "portrait-primary" => Self::PORTRAIT_PRIMARY,
            "portrait-secondary" => Self::PORTRAIT_SECONDARY,
            "landscape-primary" => Self::LANDSCAPE_PRIMARY,
            "landscape-secondary" => Self::LANDSCAPE_SECONDARY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OrientationType> for Any {
    fn from(s: OrientationType) -> Any {
         match s {
            OrientationType::PORTRAIT_PRIMARY => Any::from("portrait-primary"),
            OrientationType::PORTRAIT_SECONDARY => Any::from("portrait-secondary"),
            OrientationType::LANDSCAPE_PRIMARY => Any::from("landscape-primary"),
            OrientationType::LANDSCAPE_SECONDARY => Any::from("landscape-secondary"),
         }
    }
}
impl From<&OrientationType> for Any {
    fn from(s: &OrientationType) -> Any {
         match *s {
            OrientationType::PORTRAIT_PRIMARY => Any::from("portrait-primary"),
            OrientationType::PORTRAIT_SECONDARY => Any::from("portrait-secondary"),
            OrientationType::LANDSCAPE_PRIMARY => Any::from("landscape-primary"),
            OrientationType::LANDSCAPE_SECONDARY => Any::from("landscape-secondary"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WakeLockType {
    SCREEN,
}
impl FromVal for WakeLockType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "screen" => Self::SCREEN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WakeLockType> for Any {
    fn from(s: WakeLockType) -> Any {
         match s {
            WakeLockType::SCREEN => Any::from("screen"),
         }
    }
}
impl From<&WakeLockType> for Any {
    fn from(s: &WakeLockType) -> Any {
         match *s {
            WakeLockType::SCREEN => Any::from("screen"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "block" => Self::BLOCK,
            "inline" => Self::INLINE,
            "x" => Self::X,
            "y" => Self::Y,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ScrollAxis> for Any {
    fn from(s: ScrollAxis) -> Any {
         match s {
            ScrollAxis::BLOCK => Any::from("block"),
            ScrollAxis::INLINE => Any::from("inline"),
            ScrollAxis::X => Any::from("x"),
            ScrollAxis::Y => Any::from("y"),
         }
    }
}
impl From<&ScrollAxis> for Any {
    fn from(s: &ScrollAxis) -> Any {
         match *s {
            ScrollAxis::BLOCK => Any::from("block"),
            ScrollAxis::INLINE => Any::from("inline"),
            ScrollAxis::X => Any::from("x"),
            ScrollAxis::Y => Any::from("y"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "available" => Self::AVAILABLE,
            "unavailable-unknown-reason" => Self::UNAVAILABLE_UNKNOWN_REASON,
            "unavailable-feature-not-enabled" => Self::UNAVAILABLE_FEATURE_NOT_ENABLED,
            "unavailable-no-permission-policy" => Self::UNAVAILABLE_NO_PERMISSION_POLICY,
            "unavailable-no-user-verifying-platform-authenticator" => Self::UNAVAILABLE_NO_USER_VERIFYING_PLATFORM_AUTHENTICATOR,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SecurePaymentConfirmationAvailability> for Any {
    fn from(s: SecurePaymentConfirmationAvailability) -> Any {
         match s {
            SecurePaymentConfirmationAvailability::AVAILABLE => Any::from("available"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_UNKNOWN_REASON => Any::from("unavailable-unknown-reason"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_FEATURE_NOT_ENABLED => Any::from("unavailable-feature-not-enabled"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_NO_PERMISSION_POLICY => Any::from("unavailable-no-permission-policy"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_NO_USER_VERIFYING_PLATFORM_AUTHENTICATOR => Any::from("unavailable-no-user-verifying-platform-authenticator"),
         }
    }
}
impl From<&SecurePaymentConfirmationAvailability> for Any {
    fn from(s: &SecurePaymentConfirmationAvailability) -> Any {
         match *s {
            SecurePaymentConfirmationAvailability::AVAILABLE => Any::from("available"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_UNKNOWN_REASON => Any::from("unavailable-unknown-reason"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_FEATURE_NOT_ENABLED => Any::from("unavailable-feature-not-enabled"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_NO_PERMISSION_POLICY => Any::from("unavailable-no-permission-policy"),
            SecurePaymentConfirmationAvailability::UNAVAILABLE_NO_USER_VERIFYING_PLATFORM_AUTHENTICATOR => Any::from("unavailable-no-user-verifying-platform-authenticator"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "even" => Self::EVEN,
            "odd" => Self::ODD,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ParityType> for Any {
    fn from(s: ParityType) -> Any {
         match s {
            ParityType::NONE => Any::from("none"),
            ParityType::EVEN => Any::from("even"),
            ParityType::ODD => Any::from("odd"),
         }
    }
}
impl From<&ParityType> for Any {
    fn from(s: &ParityType) -> Any {
         match *s {
            ParityType::NONE => Any::from("none"),
            ParityType::EVEN => Any::from("even"),
            ParityType::ODD => Any::from("odd"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum FlowControlType {
    NONE,
    HARDWARE,
}
impl FromVal for FlowControlType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "hardware" => Self::HARDWARE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FlowControlType> for Any {
    fn from(s: FlowControlType) -> Any {
         match s {
            FlowControlType::NONE => Any::from("none"),
            FlowControlType::HARDWARE => Any::from("hardware"),
         }
    }
}
impl From<&FlowControlType> for Any {
    fn from(s: &FlowControlType) -> Any {
         match *s {
            FlowControlType::NONE => Any::from("none"),
            FlowControlType::HARDWARE => Any::from("hardware"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "parsed" => Self::PARSED,
            "installing" => Self::INSTALLING,
            "installed" => Self::INSTALLED,
            "activating" => Self::ACTIVATING,
            "activated" => Self::ACTIVATED,
            "redundant" => Self::REDUNDANT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ServiceWorkerState> for Any {
    fn from(s: ServiceWorkerState) -> Any {
         match s {
            ServiceWorkerState::PARSED => Any::from("parsed"),
            ServiceWorkerState::INSTALLING => Any::from("installing"),
            ServiceWorkerState::INSTALLED => Any::from("installed"),
            ServiceWorkerState::ACTIVATING => Any::from("activating"),
            ServiceWorkerState::ACTIVATED => Any::from("activated"),
            ServiceWorkerState::REDUNDANT => Any::from("redundant"),
         }
    }
}
impl From<&ServiceWorkerState> for Any {
    fn from(s: &ServiceWorkerState) -> Any {
         match *s {
            ServiceWorkerState::PARSED => Any::from("parsed"),
            ServiceWorkerState::INSTALLING => Any::from("installing"),
            ServiceWorkerState::INSTALLED => Any::from("installed"),
            ServiceWorkerState::ACTIVATING => Any::from("activating"),
            ServiceWorkerState::ACTIVATED => Any::from("activated"),
            ServiceWorkerState::REDUNDANT => Any::from("redundant"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "imports" => Self::IMPORTS,
            "all" => Self::ALL,
            "none" => Self::NONE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ServiceWorkerUpdateViaCache> for Any {
    fn from(s: ServiceWorkerUpdateViaCache) -> Any {
         match s {
            ServiceWorkerUpdateViaCache::IMPORTS => Any::from("imports"),
            ServiceWorkerUpdateViaCache::ALL => Any::from("all"),
            ServiceWorkerUpdateViaCache::NONE => Any::from("none"),
         }
    }
}
impl From<&ServiceWorkerUpdateViaCache> for Any {
    fn from(s: &ServiceWorkerUpdateViaCache) -> Any {
         match *s {
            ServiceWorkerUpdateViaCache::IMPORTS => Any::from("imports"),
            ServiceWorkerUpdateViaCache::ALL => Any::from("all"),
            ServiceWorkerUpdateViaCache::NONE => Any::from("none"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auxiliary" => Self::AUXILIARY,
            "top-level" => Self::TOP_LEVEL,
            "nested" => Self::NESTED,
            "none" => Self::NONE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FrameType> for Any {
    fn from(s: FrameType) -> Any {
         match s {
            FrameType::AUXILIARY => Any::from("auxiliary"),
            FrameType::TOP_LEVEL => Any::from("top-level"),
            FrameType::NESTED => Any::from("nested"),
            FrameType::NONE => Any::from("none"),
         }
    }
}
impl From<&FrameType> for Any {
    fn from(s: &FrameType) -> Any {
         match *s {
            FrameType::AUXILIARY => Any::from("auxiliary"),
            FrameType::TOP_LEVEL => Any::from("top-level"),
            FrameType::NESTED => Any::from("nested"),
            FrameType::NONE => Any::from("none"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "window" => Self::WINDOW,
            "worker" => Self::WORKER,
            "sharedworker" => Self::SHAREDWORKER,
            "all" => Self::ALL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ClientType> for Any {
    fn from(s: ClientType) -> Any {
         match s {
            ClientType::WINDOW => Any::from("window"),
            ClientType::WORKER => Any::from("worker"),
            ClientType::SHAREDWORKER => Any::from("sharedworker"),
            ClientType::ALL => Any::from("all"),
         }
    }
}
impl From<&ClientType> for Any {
    fn from(s: &ClientType) -> Any {
         match *s {
            ClientType::WINDOW => Any::from("window"),
            ClientType::WORKER => Any::from("worker"),
            ClientType::SHAREDWORKER => Any::from("sharedworker"),
            ClientType::ALL => Any::from("all"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RunningStatus {
    RUNNING,
    NOT_RUNNING,
}
impl FromVal for RunningStatus {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "running" => Self::RUNNING,
            "not-running" => Self::NOT_RUNNING,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RunningStatus> for Any {
    fn from(s: RunningStatus) -> Any {
         match s {
            RunningStatus::RUNNING => Any::from("running"),
            RunningStatus::NOT_RUNNING => Any::from("not-running"),
         }
    }
}
impl From<&RunningStatus> for Any {
    fn from(s: &RunningStatus) -> Any {
         match *s {
            RunningStatus::RUNNING => Any::from("running"),
            RunningStatus::NOT_RUNNING => Any::from("not-running"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "cache" => Self::CACHE,
            "fetch-event" => Self::FETCH_EVENT,
            "network" => Self::NETWORK,
            "race-network-and-fetch-handler" => Self::RACE_NETWORK_AND_FETCH_HANDLER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RouterSourceEnum> for Any {
    fn from(s: RouterSourceEnum) -> Any {
         match s {
            RouterSourceEnum::CACHE => Any::from("cache"),
            RouterSourceEnum::FETCH_EVENT => Any::from("fetch-event"),
            RouterSourceEnum::NETWORK => Any::from("network"),
            RouterSourceEnum::RACE_NETWORK_AND_FETCH_HANDLER => Any::from("race-network-and-fetch-handler"),
         }
    }
}
impl From<&RouterSourceEnum> for Any {
    fn from(s: &RouterSourceEnum) -> Any {
         match *s {
            RouterSourceEnum::CACHE => Any::from("cache"),
            RouterSourceEnum::FETCH_EVENT => Any::from("fetch-event"),
            RouterSourceEnum::NETWORK => Any::from("network"),
            RouterSourceEnum::RACE_NETWORK_AND_FETCH_HANDLER => Any::from("race-network-and-fetch-handler"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "mouth" => Self::MOUTH,
            "eye" => Self::EYE,
            "nose" => Self::NOSE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<LandmarkType> for Any {
    fn from(s: LandmarkType) -> Any {
         match s {
            LandmarkType::MOUTH => Any::from("mouth"),
            LandmarkType::EYE => Any::from("eye"),
            LandmarkType::NOSE => Any::from("nose"),
         }
    }
}
impl From<&LandmarkType> for Any {
    fn from(s: &LandmarkType) -> Any {
         match *s {
            LandmarkType::MOUTH => Any::from("mouth"),
            LandmarkType::EYE => Any::from("eye"),
            LandmarkType::NOSE => Any::from("nose"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<BarcodeFormat> for Any {
    fn from(s: BarcodeFormat) -> Any {
         match s {
            BarcodeFormat::AZTEC => Any::from("aztec"),
            BarcodeFormat::CODE_128 => Any::from("code_128"),
            BarcodeFormat::CODE_39 => Any::from("code_39"),
            BarcodeFormat::CODE_93 => Any::from("code_93"),
            BarcodeFormat::CODABAR => Any::from("codabar"),
            BarcodeFormat::DATA_MATRIX => Any::from("data_matrix"),
            BarcodeFormat::EAN_13 => Any::from("ean_13"),
            BarcodeFormat::EAN_8 => Any::from("ean_8"),
            BarcodeFormat::ITF => Any::from("itf"),
            BarcodeFormat::PDF417 => Any::from("pdf417"),
            BarcodeFormat::QR_CODE => Any::from("qr_code"),
            BarcodeFormat::UNKNOWN => Any::from("unknown"),
            BarcodeFormat::UPC_A => Any::from("upc_a"),
            BarcodeFormat::UPC_E => Any::from("upc_e"),
         }
    }
}
impl From<&BarcodeFormat> for Any {
    fn from(s: &BarcodeFormat) -> Any {
         match *s {
            BarcodeFormat::AZTEC => Any::from("aztec"),
            BarcodeFormat::CODE_128 => Any::from("code_128"),
            BarcodeFormat::CODE_39 => Any::from("code_39"),
            BarcodeFormat::CODE_93 => Any::from("code_93"),
            BarcodeFormat::CODABAR => Any::from("codabar"),
            BarcodeFormat::DATA_MATRIX => Any::from("data_matrix"),
            BarcodeFormat::EAN_13 => Any::from("ean_13"),
            BarcodeFormat::EAN_8 => Any::from("ean_8"),
            BarcodeFormat::ITF => Any::from("itf"),
            BarcodeFormat::PDF417 => Any::from("pdf417"),
            BarcodeFormat::QR_CODE => Any::from("qr_code"),
            BarcodeFormat::UNKNOWN => Any::from("unknown"),
            BarcodeFormat::UPC_A => Any::from("upc_a"),
            BarcodeFormat::UPC_E => Any::from("upc_e"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SpeechRecognitionErrorCode> for Any {
    fn from(s: SpeechRecognitionErrorCode) -> Any {
         match s {
            SpeechRecognitionErrorCode::NO_SPEECH => Any::from("no-speech"),
            SpeechRecognitionErrorCode::ABORTED => Any::from("aborted"),
            SpeechRecognitionErrorCode::AUDIO_CAPTURE => Any::from("audio-capture"),
            SpeechRecognitionErrorCode::NETWORK => Any::from("network"),
            SpeechRecognitionErrorCode::NOT_ALLOWED => Any::from("not-allowed"),
            SpeechRecognitionErrorCode::SERVICE_NOT_ALLOWED => Any::from("service-not-allowed"),
            SpeechRecognitionErrorCode::LANGUAGE_NOT_SUPPORTED => Any::from("language-not-supported"),
            SpeechRecognitionErrorCode::PHRASES_NOT_SUPPORTED => Any::from("phrases-not-supported"),
         }
    }
}
impl From<&SpeechRecognitionErrorCode> for Any {
    fn from(s: &SpeechRecognitionErrorCode) -> Any {
         match *s {
            SpeechRecognitionErrorCode::NO_SPEECH => Any::from("no-speech"),
            SpeechRecognitionErrorCode::ABORTED => Any::from("aborted"),
            SpeechRecognitionErrorCode::AUDIO_CAPTURE => Any::from("audio-capture"),
            SpeechRecognitionErrorCode::NETWORK => Any::from("network"),
            SpeechRecognitionErrorCode::NOT_ALLOWED => Any::from("not-allowed"),
            SpeechRecognitionErrorCode::SERVICE_NOT_ALLOWED => Any::from("service-not-allowed"),
            SpeechRecognitionErrorCode::LANGUAGE_NOT_SUPPORTED => Any::from("language-not-supported"),
            SpeechRecognitionErrorCode::PHRASES_NOT_SUPPORTED => Any::from("phrases-not-supported"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "unavailable" => Self::UNAVAILABLE,
            "downloadable" => Self::DOWNLOADABLE,
            "downloading" => Self::DOWNLOADING,
            "available" => Self::AVAILABLE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AvailabilityStatus> for Any {
    fn from(s: AvailabilityStatus) -> Any {
         match s {
            AvailabilityStatus::UNAVAILABLE => Any::from("unavailable"),
            AvailabilityStatus::DOWNLOADABLE => Any::from("downloadable"),
            AvailabilityStatus::DOWNLOADING => Any::from("downloading"),
            AvailabilityStatus::AVAILABLE => Any::from("available"),
         }
    }
}
impl From<&AvailabilityStatus> for Any {
    fn from(s: &AvailabilityStatus) -> Any {
         match *s {
            AvailabilityStatus::UNAVAILABLE => Any::from("unavailable"),
            AvailabilityStatus::DOWNLOADABLE => Any::from("downloadable"),
            AvailabilityStatus::DOWNLOADING => Any::from("downloading"),
            AvailabilityStatus::AVAILABLE => Any::from("available"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SpeechSynthesisErrorCode> for Any {
    fn from(s: SpeechSynthesisErrorCode) -> Any {
         match s {
            SpeechSynthesisErrorCode::CANCELED => Any::from("canceled"),
            SpeechSynthesisErrorCode::INTERRUPTED => Any::from("interrupted"),
            SpeechSynthesisErrorCode::AUDIO_BUSY => Any::from("audio-busy"),
            SpeechSynthesisErrorCode::AUDIO_HARDWARE => Any::from("audio-hardware"),
            SpeechSynthesisErrorCode::NETWORK => Any::from("network"),
            SpeechSynthesisErrorCode::SYNTHESIS_UNAVAILABLE => Any::from("synthesis-unavailable"),
            SpeechSynthesisErrorCode::SYNTHESIS_FAILED => Any::from("synthesis-failed"),
            SpeechSynthesisErrorCode::LANGUAGE_UNAVAILABLE => Any::from("language-unavailable"),
            SpeechSynthesisErrorCode::VOICE_UNAVAILABLE => Any::from("voice-unavailable"),
            SpeechSynthesisErrorCode::TEXT_TOO_LONG => Any::from("text-too-long"),
            SpeechSynthesisErrorCode::INVALID_ARGUMENT => Any::from("invalid-argument"),
            SpeechSynthesisErrorCode::NOT_ALLOWED => Any::from("not-allowed"),
         }
    }
}
impl From<&SpeechSynthesisErrorCode> for Any {
    fn from(s: &SpeechSynthesisErrorCode) -> Any {
         match *s {
            SpeechSynthesisErrorCode::CANCELED => Any::from("canceled"),
            SpeechSynthesisErrorCode::INTERRUPTED => Any::from("interrupted"),
            SpeechSynthesisErrorCode::AUDIO_BUSY => Any::from("audio-busy"),
            SpeechSynthesisErrorCode::AUDIO_HARDWARE => Any::from("audio-hardware"),
            SpeechSynthesisErrorCode::NETWORK => Any::from("network"),
            SpeechSynthesisErrorCode::SYNTHESIS_UNAVAILABLE => Any::from("synthesis-unavailable"),
            SpeechSynthesisErrorCode::SYNTHESIS_FAILED => Any::from("synthesis-failed"),
            SpeechSynthesisErrorCode::LANGUAGE_UNAVAILABLE => Any::from("language-unavailable"),
            SpeechSynthesisErrorCode::VOICE_UNAVAILABLE => Any::from("voice-unavailable"),
            SpeechSynthesisErrorCode::TEXT_TOO_LONG => Any::from("text-too-long"),
            SpeechSynthesisErrorCode::INVALID_ARGUMENT => Any::from("invalid-argument"),
            SpeechSynthesisErrorCode::NOT_ALLOWED => Any::from("not-allowed"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ReadableStreamReaderMode {
    BYOB,
}
impl FromVal for ReadableStreamReaderMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "byob" => Self::BYOB,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ReadableStreamReaderMode> for Any {
    fn from(s: ReadableStreamReaderMode) -> Any {
         match s {
            ReadableStreamReaderMode::BYOB => Any::from("byob"),
         }
    }
}
impl From<&ReadableStreamReaderMode> for Any {
    fn from(s: &ReadableStreamReaderMode) -> Any {
         match *s {
            ReadableStreamReaderMode::BYOB => Any::from("byob"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ReadableStreamType {
    BYTES,
}
impl FromVal for ReadableStreamType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "bytes" => Self::BYTES,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ReadableStreamType> for Any {
    fn from(s: ReadableStreamType) -> Any {
         match s {
            ReadableStreamType::BYTES => Any::from("bytes"),
         }
    }
}
impl From<&ReadableStreamType> for Any {
    fn from(s: &ReadableStreamType) -> Any {
         match *s {
            ReadableStreamType::BYTES => Any::from("bytes"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TouchType {
    DIRECT,
    STYLUS,
}
impl FromVal for TouchType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "direct" => Self::DIRECT,
            "stylus" => Self::STYLUS,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<TouchType> for Any {
    fn from(s: TouchType) -> Any {
         match s {
            TouchType::DIRECT => Any::from("direct"),
            TouchType::STYLUS => Any::from("stylus"),
         }
    }
}
impl From<&TouchType> for Any {
    fn from(s: &TouchType) -> Any {
         match *s {
            TouchType::DIRECT => Any::from("direct"),
            TouchType::STYLUS => Any::from("stylus"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RefreshPolicy {
    NONE,
    REFRESH,
}
impl FromVal for RefreshPolicy {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "refresh" => Self::REFRESH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RefreshPolicy> for Any {
    fn from(s: RefreshPolicy) -> Any {
         match s {
            RefreshPolicy::NONE => Any::from("none"),
            RefreshPolicy::REFRESH => Any::from("refresh"),
         }
    }
}
impl From<&RefreshPolicy> for Any {
    fn from(s: &RefreshPolicy) -> Any {
         match *s {
            RefreshPolicy::NONE => Any::from("none"),
            RefreshPolicy::REFRESH => Any::from("refresh"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TokenVersion {
    _1,
}
impl FromVal for TokenVersion {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "1" => Self::_1,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<TokenVersion> for Any {
    fn from(s: TokenVersion) -> Any {
         match s {
            TokenVersion::_1 => Any::from("1"),
         }
    }
}
impl From<&TokenVersion> for Any {
    fn from(s: &TokenVersion) -> Any {
         match *s {
            TokenVersion::_1 => Any::from("1"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "token-request" => Self::TOKEN_REQUEST,
            "send-redemption-record" => Self::SEND_REDEMPTION_RECORD,
            "token-redemption" => Self::TOKEN_REDEMPTION,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OperationType> for Any {
    fn from(s: OperationType) -> Any {
         match s {
            OperationType::TOKEN_REQUEST => Any::from("token-request"),
            OperationType::SEND_REDEMPTION_RECORD => Any::from("send-redemption-record"),
            OperationType::TOKEN_REDEMPTION => Any::from("token-redemption"),
         }
    }
}
impl From<&OperationType> for Any {
    fn from(s: &OperationType) -> Any {
         match *s {
            OperationType::TOKEN_REQUEST => Any::from("token-request"),
            OperationType::SEND_REDEMPTION_RECORD => Any::from("send-redemption-record"),
            OperationType::TOKEN_REDEMPTION => Any::from("token-redemption"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "passedAndEnforced" => Self::PASSED_AND_ENFORCED,
            "passedNotEnforced" => Self::PASSED_NOT_ENFORCED,
            "belowThreshold" => Self::BELOW_THRESHOLD,
            "notCalculated" => Self::NOT_CALCULATED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<KAnonStatus> for Any {
    fn from(s: KAnonStatus) -> Any {
         match s {
            KAnonStatus::PASSED_AND_ENFORCED => Any::from("passedAndEnforced"),
            KAnonStatus::PASSED_NOT_ENFORCED => Any::from("passedNotEnforced"),
            KAnonStatus::BELOW_THRESHOLD => Any::from("belowThreshold"),
            KAnonStatus::NOT_CALCULATED => Any::from("notCalculated"),
         }
    }
}
impl From<&KAnonStatus> for Any {
    fn from(s: &KAnonStatus) -> Any {
         match *s {
            KAnonStatus::PASSED_AND_ENFORCED => Any::from("passedAndEnforced"),
            KAnonStatus::PASSED_NOT_ENFORCED => Any::from("passedNotEnforced"),
            KAnonStatus::BELOW_THRESHOLD => Any::from("belowThreshold"),
            KAnonStatus::NOT_CALCULATED => Any::from("notCalculated"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "function" => Self::FUNCTION,
            "table" => Self::TABLE,
            "memory" => Self::MEMORY,
            "global" => Self::GLOBAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ImportExportKind> for Any {
    fn from(s: ImportExportKind) -> Any {
         match s {
            ImportExportKind::FUNCTION => Any::from("function"),
            ImportExportKind::TABLE => Any::from("table"),
            ImportExportKind::MEMORY => Any::from("memory"),
            ImportExportKind::GLOBAL => Any::from("global"),
         }
    }
}
impl From<&ImportExportKind> for Any {
    fn from(s: &ImportExportKind) -> Any {
         match *s {
            ImportExportKind::FUNCTION => Any::from("function"),
            ImportExportKind::TABLE => Any::from("table"),
            ImportExportKind::MEMORY => Any::from("memory"),
            ImportExportKind::GLOBAL => Any::from("global"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TableKind {
    EXTERNREF,
    ANYFUNC,
}
impl FromVal for TableKind {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "externref" => Self::EXTERNREF,
            "anyfunc" => Self::ANYFUNC,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<TableKind> for Any {
    fn from(s: TableKind) -> Any {
         match s {
            TableKind::EXTERNREF => Any::from("externref"),
            TableKind::ANYFUNC => Any::from("anyfunc"),
         }
    }
}
impl From<&TableKind> for Any {
    fn from(s: &TableKind) -> Any {
         match *s {
            TableKind::EXTERNREF => Any::from("externref"),
            TableKind::ANYFUNC => Any::from("anyfunc"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ValueType> for Any {
    fn from(s: ValueType) -> Any {
         match s {
            ValueType::I32_ => Any::from("i32"),
            ValueType::I64_ => Any::from("i64"),
            ValueType::F32_ => Any::from("f32"),
            ValueType::F64_ => Any::from("f64"),
            ValueType::V128 => Any::from("v128"),
            ValueType::EXTERNREF => Any::from("externref"),
            ValueType::ANYFUNC => Any::from("anyfunc"),
         }
    }
}
impl From<&ValueType> for Any {
    fn from(s: &ValueType) -> Any {
         match *s {
            ValueType::I32_ => Any::from("i32"),
            ValueType::I64_ => Any::from("i64"),
            ValueType::F32_ => Any::from("f32"),
            ValueType::F64_ => Any::from("f64"),
            ValueType::V128 => Any::from("v128"),
            ValueType::EXTERNREF => Any::from("externref"),
            ValueType::ANYFUNC => Any::from("anyfunc"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum IterationCompositeOperation {
    REPLACE,
    ACCUMULATE,
}
impl FromVal for IterationCompositeOperation {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "replace" => Self::REPLACE,
            "accumulate" => Self::ACCUMULATE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<IterationCompositeOperation> for Any {
    fn from(s: IterationCompositeOperation) -> Any {
         match s {
            IterationCompositeOperation::REPLACE => Any::from("replace"),
            IterationCompositeOperation::ACCUMULATE => Any::from("accumulate"),
         }
    }
}
impl From<&IterationCompositeOperation> for Any {
    fn from(s: &IterationCompositeOperation) -> Any {
         match *s {
            IterationCompositeOperation::REPLACE => Any::from("replace"),
            IterationCompositeOperation::ACCUMULATE => Any::from("accumulate"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "once" => Self::ONCE,
            "repeat" => Self::REPEAT,
            "alternate" => Self::ALTERNATE,
            "state" => Self::STATE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AnimationTriggerBehavior> for Any {
    fn from(s: AnimationTriggerBehavior) -> Any {
         match s {
            AnimationTriggerBehavior::ONCE => Any::from("once"),
            AnimationTriggerBehavior::REPEAT => Any::from("repeat"),
            AnimationTriggerBehavior::ALTERNATE => Any::from("alternate"),
            AnimationTriggerBehavior::STATE => Any::from("state"),
         }
    }
}
impl From<&AnimationTriggerBehavior> for Any {
    fn from(s: &AnimationTriggerBehavior) -> Any {
         match *s {
            AnimationTriggerBehavior::ONCE => Any::from("once"),
            AnimationTriggerBehavior::REPEAT => Any::from("repeat"),
            AnimationTriggerBehavior::ALTERNATE => Any::from("alternate"),
            AnimationTriggerBehavior::STATE => Any::from("state"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "idle" => Self::IDLE,
            "running" => Self::RUNNING,
            "paused" => Self::PAUSED,
            "finished" => Self::FINISHED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AnimationPlayState> for Any {
    fn from(s: AnimationPlayState) -> Any {
         match s {
            AnimationPlayState::IDLE => Any::from("idle"),
            AnimationPlayState::RUNNING => Any::from("running"),
            AnimationPlayState::PAUSED => Any::from("paused"),
            AnimationPlayState::FINISHED => Any::from("finished"),
         }
    }
}
impl From<&AnimationPlayState> for Any {
    fn from(s: &AnimationPlayState) -> Any {
         match *s {
            AnimationPlayState::IDLE => Any::from("idle"),
            AnimationPlayState::RUNNING => Any::from("running"),
            AnimationPlayState::PAUSED => Any::from("paused"),
            AnimationPlayState::FINISHED => Any::from("finished"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "active" => Self::ACTIVE,
            "removed" => Self::REMOVED,
            "persisted" => Self::PERSISTED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AnimationReplaceState> for Any {
    fn from(s: AnimationReplaceState) -> Any {
         match s {
            AnimationReplaceState::ACTIVE => Any::from("active"),
            AnimationReplaceState::REMOVED => Any::from("removed"),
            AnimationReplaceState::PERSISTED => Any::from("persisted"),
         }
    }
}
impl From<&AnimationReplaceState> for Any {
    fn from(s: &AnimationReplaceState) -> Any {
         match *s {
            AnimationReplaceState::ACTIVE => Any::from("active"),
            AnimationReplaceState::REMOVED => Any::from("removed"),
            AnimationReplaceState::PERSISTED => Any::from("persisted"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "forwards" => Self::FORWARDS,
            "backwards" => Self::BACKWARDS,
            "both" => Self::BOTH,
            "auto" => Self::AUTO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<FillMode> for Any {
    fn from(s: FillMode) -> Any {
         match s {
            FillMode::NONE => Any::from("none"),
            FillMode::FORWARDS => Any::from("forwards"),
            FillMode::BACKWARDS => Any::from("backwards"),
            FillMode::BOTH => Any::from("both"),
            FillMode::AUTO => Any::from("auto"),
         }
    }
}
impl From<&FillMode> for Any {
    fn from(s: &FillMode) -> Any {
         match *s {
            FillMode::NONE => Any::from("none"),
            FillMode::FORWARDS => Any::from("forwards"),
            FillMode::BACKWARDS => Any::from("backwards"),
            FillMode::BOTH => Any::from("both"),
            FillMode::AUTO => Any::from("auto"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "normal" => Self::NORMAL,
            "reverse" => Self::REVERSE,
            "alternate" => Self::ALTERNATE,
            "alternate-reverse" => Self::ALTERNATE_REVERSE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PlaybackDirection> for Any {
    fn from(s: PlaybackDirection) -> Any {
         match s {
            PlaybackDirection::NORMAL => Any::from("normal"),
            PlaybackDirection::REVERSE => Any::from("reverse"),
            PlaybackDirection::ALTERNATE => Any::from("alternate"),
            PlaybackDirection::ALTERNATE_REVERSE => Any::from("alternate-reverse"),
         }
    }
}
impl From<&PlaybackDirection> for Any {
    fn from(s: &PlaybackDirection) -> Any {
         match *s {
            PlaybackDirection::NORMAL => Any::from("normal"),
            PlaybackDirection::REVERSE => Any::from("reverse"),
            PlaybackDirection::ALTERNATE => Any::from("alternate"),
            PlaybackDirection::ALTERNATE_REVERSE => Any::from("alternate-reverse"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "replace" => Self::REPLACE,
            "add" => Self::ADD,
            "accumulate" => Self::ACCUMULATE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CompositeOperation> for Any {
    fn from(s: CompositeOperation) -> Any {
         match s {
            CompositeOperation::REPLACE => Any::from("replace"),
            CompositeOperation::ADD => Any::from("add"),
            CompositeOperation::ACCUMULATE => Any::from("accumulate"),
         }
    }
}
impl From<&CompositeOperation> for Any {
    fn from(s: &CompositeOperation) -> Any {
         match *s {
            CompositeOperation::REPLACE => Any::from("replace"),
            CompositeOperation::ADD => Any::from("add"),
            CompositeOperation::ACCUMULATE => Any::from("accumulate"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "replace" => Self::REPLACE,
            "add" => Self::ADD,
            "accumulate" => Self::ACCUMULATE,
            "auto" => Self::AUTO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CompositeOperationOrAuto> for Any {
    fn from(s: CompositeOperationOrAuto) -> Any {
         match s {
            CompositeOperationOrAuto::REPLACE => Any::from("replace"),
            CompositeOperationOrAuto::ADD => Any::from("add"),
            CompositeOperationOrAuto::ACCUMULATE => Any::from("accumulate"),
            CompositeOperationOrAuto::AUTO => Any::from("auto"),
         }
    }
}
impl From<&CompositeOperationOrAuto> for Any {
    fn from(s: &CompositeOperationOrAuto) -> Any {
         match *s {
            CompositeOperationOrAuto::REPLACE => Any::from("replace"),
            CompositeOperationOrAuto::ADD => Any::from("add"),
            CompositeOperationOrAuto::ACCUMULATE => Any::from("accumulate"),
            CompositeOperationOrAuto::AUTO => Any::from("auto"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LockMode {
    SHARED,
    EXCLUSIVE,
}
impl FromVal for LockMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "shared" => Self::SHARED,
            "exclusive" => Self::EXCLUSIVE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<LockMode> for Any {
    fn from(s: LockMode) -> Any {
         match s {
            LockMode::SHARED => Any::from("shared"),
            LockMode::EXCLUSIVE => Any::from("exclusive"),
         }
    }
}
impl From<&LockMode> for Any {
    fn from(s: &LockMode) -> Any {
         match *s {
            LockMode::SHARED => Any::from("shared"),
            LockMode::EXCLUSIVE => Any::from("exclusive"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OTPCredentialTransportType {
    SMS,
}
impl FromVal for OTPCredentialTransportType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "sms" => Self::SMS,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OTPCredentialTransportType> for Any {
    fn from(s: OTPCredentialTransportType) -> Any {
         match s {
            OTPCredentialTransportType::SMS => Any::from("sms"),
         }
    }
}
impl From<&OTPCredentialTransportType> for Any {
    fn from(s: &OTPCredentialTransportType) -> Any {
         match *s {
            OTPCredentialTransportType::SMS => Any::from("sms"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "suspended" => Self::SUSPENDED,
            "running" => Self::RUNNING,
            "closed" => Self::CLOSED,
            "interrupted" => Self::INTERRUPTED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AudioContextState> for Any {
    fn from(s: AudioContextState) -> Any {
         match s {
            AudioContextState::SUSPENDED => Any::from("suspended"),
            AudioContextState::RUNNING => Any::from("running"),
            AudioContextState::CLOSED => Any::from("closed"),
            AudioContextState::INTERRUPTED => Any::from("interrupted"),
         }
    }
}
impl From<&AudioContextState> for Any {
    fn from(s: &AudioContextState) -> Any {
         match *s {
            AudioContextState::SUSPENDED => Any::from("suspended"),
            AudioContextState::RUNNING => Any::from("running"),
            AudioContextState::CLOSED => Any::from("closed"),
            AudioContextState::INTERRUPTED => Any::from("interrupted"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AudioContextRenderSizeCategory {
    DEFAULT,
    HARDWARE,
}
impl FromVal for AudioContextRenderSizeCategory {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "default" => Self::DEFAULT,
            "hardware" => Self::HARDWARE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AudioContextRenderSizeCategory> for Any {
    fn from(s: AudioContextRenderSizeCategory) -> Any {
         match s {
            AudioContextRenderSizeCategory::DEFAULT => Any::from("default"),
            AudioContextRenderSizeCategory::HARDWARE => Any::from("hardware"),
         }
    }
}
impl From<&AudioContextRenderSizeCategory> for Any {
    fn from(s: &AudioContextRenderSizeCategory) -> Any {
         match *s {
            AudioContextRenderSizeCategory::DEFAULT => Any::from("default"),
            AudioContextRenderSizeCategory::HARDWARE => Any::from("hardware"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "balanced" => Self::BALANCED,
            "interactive" => Self::INTERACTIVE,
            "playback" => Self::PLAYBACK,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AudioContextLatencyCategory> for Any {
    fn from(s: AudioContextLatencyCategory) -> Any {
         match s {
            AudioContextLatencyCategory::BALANCED => Any::from("balanced"),
            AudioContextLatencyCategory::INTERACTIVE => Any::from("interactive"),
            AudioContextLatencyCategory::PLAYBACK => Any::from("playback"),
         }
    }
}
impl From<&AudioContextLatencyCategory> for Any {
    fn from(s: &AudioContextLatencyCategory) -> Any {
         match *s {
            AudioContextLatencyCategory::BALANCED => Any::from("balanced"),
            AudioContextLatencyCategory::INTERACTIVE => Any::from("interactive"),
            AudioContextLatencyCategory::PLAYBACK => Any::from("playback"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AudioSinkType {
    NONE,
}
impl FromVal for AudioSinkType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AudioSinkType> for Any {
    fn from(s: AudioSinkType) -> Any {
         match s {
            AudioSinkType::NONE => Any::from("none"),
         }
    }
}
impl From<&AudioSinkType> for Any {
    fn from(s: &AudioSinkType) -> Any {
         match *s {
            AudioSinkType::NONE => Any::from("none"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "max" => Self::MAX,
            "clamped-max" => Self::CLAMPED_MAX,
            "explicit" => Self::EXPLICIT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ChannelCountMode> for Any {
    fn from(s: ChannelCountMode) -> Any {
         match s {
            ChannelCountMode::MAX => Any::from("max"),
            ChannelCountMode::CLAMPED_MAX => Any::from("clamped-max"),
            ChannelCountMode::EXPLICIT => Any::from("explicit"),
         }
    }
}
impl From<&ChannelCountMode> for Any {
    fn from(s: &ChannelCountMode) -> Any {
         match *s {
            ChannelCountMode::MAX => Any::from("max"),
            ChannelCountMode::CLAMPED_MAX => Any::from("clamped-max"),
            ChannelCountMode::EXPLICIT => Any::from("explicit"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ChannelInterpretation {
    SPEAKERS,
    DISCRETE,
}
impl FromVal for ChannelInterpretation {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "speakers" => Self::SPEAKERS,
            "discrete" => Self::DISCRETE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ChannelInterpretation> for Any {
    fn from(s: ChannelInterpretation) -> Any {
         match s {
            ChannelInterpretation::SPEAKERS => Any::from("speakers"),
            ChannelInterpretation::DISCRETE => Any::from("discrete"),
         }
    }
}
impl From<&ChannelInterpretation> for Any {
    fn from(s: &ChannelInterpretation) -> Any {
         match *s {
            ChannelInterpretation::SPEAKERS => Any::from("speakers"),
            ChannelInterpretation::DISCRETE => Any::from("discrete"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AutomationRate {
    A_RATE,
    K_RATE,
}
impl FromVal for AutomationRate {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "a-rate" => Self::A_RATE,
            "k-rate" => Self::K_RATE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AutomationRate> for Any {
    fn from(s: AutomationRate) -> Any {
         match s {
            AutomationRate::A_RATE => Any::from("a-rate"),
            AutomationRate::K_RATE => Any::from("k-rate"),
         }
    }
}
impl From<&AutomationRate> for Any {
    fn from(s: &AutomationRate) -> Any {
         match *s {
            AutomationRate::A_RATE => Any::from("a-rate"),
            AutomationRate::K_RATE => Any::from("k-rate"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<BiquadFilterType> for Any {
    fn from(s: BiquadFilterType) -> Any {
         match s {
            BiquadFilterType::LOWPASS => Any::from("lowpass"),
            BiquadFilterType::HIGHPASS => Any::from("highpass"),
            BiquadFilterType::BANDPASS => Any::from("bandpass"),
            BiquadFilterType::LOWSHELF => Any::from("lowshelf"),
            BiquadFilterType::HIGHSHELF => Any::from("highshelf"),
            BiquadFilterType::PEAKING => Any::from("peaking"),
            BiquadFilterType::NOTCH => Any::from("notch"),
            BiquadFilterType::ALLPASS => Any::from("allpass"),
         }
    }
}
impl From<&BiquadFilterType> for Any {
    fn from(s: &BiquadFilterType) -> Any {
         match *s {
            BiquadFilterType::LOWPASS => Any::from("lowpass"),
            BiquadFilterType::HIGHPASS => Any::from("highpass"),
            BiquadFilterType::BANDPASS => Any::from("bandpass"),
            BiquadFilterType::LOWSHELF => Any::from("lowshelf"),
            BiquadFilterType::HIGHSHELF => Any::from("highshelf"),
            BiquadFilterType::PEAKING => Any::from("peaking"),
            BiquadFilterType::NOTCH => Any::from("notch"),
            BiquadFilterType::ALLPASS => Any::from("allpass"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "sine" => Self::SINE,
            "square" => Self::SQUARE,
            "sawtooth" => Self::SAWTOOTH,
            "triangle" => Self::TRIANGLE,
            "custom" => Self::CUSTOM,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OscillatorType> for Any {
    fn from(s: OscillatorType) -> Any {
         match s {
            OscillatorType::SINE => Any::from("sine"),
            OscillatorType::SQUARE => Any::from("square"),
            OscillatorType::SAWTOOTH => Any::from("sawtooth"),
            OscillatorType::TRIANGLE => Any::from("triangle"),
            OscillatorType::CUSTOM => Any::from("custom"),
         }
    }
}
impl From<&OscillatorType> for Any {
    fn from(s: &OscillatorType) -> Any {
         match *s {
            OscillatorType::SINE => Any::from("sine"),
            OscillatorType::SQUARE => Any::from("square"),
            OscillatorType::SAWTOOTH => Any::from("sawtooth"),
            OscillatorType::TRIANGLE => Any::from("triangle"),
            OscillatorType::CUSTOM => Any::from("custom"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PanningModelType {
    EQUALPOWER,
    HRTF,
}
impl FromVal for PanningModelType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "equalpower" => Self::EQUALPOWER,
            "HRTF" => Self::HRTF,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PanningModelType> for Any {
    fn from(s: PanningModelType) -> Any {
         match s {
            PanningModelType::EQUALPOWER => Any::from("equalpower"),
            PanningModelType::HRTF => Any::from("HRTF"),
         }
    }
}
impl From<&PanningModelType> for Any {
    fn from(s: &PanningModelType) -> Any {
         match *s {
            PanningModelType::EQUALPOWER => Any::from("equalpower"),
            PanningModelType::HRTF => Any::from("HRTF"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "linear" => Self::LINEAR,
            "inverse" => Self::INVERSE,
            "exponential" => Self::EXPONENTIAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<DistanceModelType> for Any {
    fn from(s: DistanceModelType) -> Any {
         match s {
            DistanceModelType::LINEAR => Any::from("linear"),
            DistanceModelType::INVERSE => Any::from("inverse"),
            DistanceModelType::EXPONENTIAL => Any::from("exponential"),
         }
    }
}
impl From<&DistanceModelType> for Any {
    fn from(s: &DistanceModelType) -> Any {
         match *s {
            DistanceModelType::LINEAR => Any::from("linear"),
            DistanceModelType::INVERSE => Any::from("inverse"),
            DistanceModelType::EXPONENTIAL => Any::from("exponential"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "2x" => Self::_2X,
            "4x" => Self::_4X,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OverSampleType> for Any {
    fn from(s: OverSampleType) -> Any {
         match s {
            OverSampleType::NONE => Any::from("none"),
            OverSampleType::_2X => Any::from("2x"),
            OverSampleType::_4X => Any::from("4x"),
         }
    }
}
impl From<&OverSampleType> for Any {
    fn from(s: &OverSampleType) -> Any {
         match *s {
            OverSampleType::NONE => Any::from("none"),
            OverSampleType::_2X => Any::from("2x"),
            OverSampleType::_4X => Any::from("4x"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AuthenticatorAttachment {
    PLATFORM,
    CROSS_PLATFORM,
}
impl FromVal for AuthenticatorAttachment {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "platform" => Self::PLATFORM,
            "cross-platform" => Self::CROSS_PLATFORM,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AuthenticatorAttachment> for Any {
    fn from(s: AuthenticatorAttachment) -> Any {
         match s {
            AuthenticatorAttachment::PLATFORM => Any::from("platform"),
            AuthenticatorAttachment::CROSS_PLATFORM => Any::from("cross-platform"),
         }
    }
}
impl From<&AuthenticatorAttachment> for Any {
    fn from(s: &AuthenticatorAttachment) -> Any {
         match *s {
            AuthenticatorAttachment::PLATFORM => Any::from("platform"),
            AuthenticatorAttachment::CROSS_PLATFORM => Any::from("cross-platform"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "discouraged" => Self::DISCOURAGED,
            "preferred" => Self::PREFERRED,
            "required" => Self::REQUIRED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ResidentKeyRequirement> for Any {
    fn from(s: ResidentKeyRequirement) -> Any {
         match s {
            ResidentKeyRequirement::DISCOURAGED => Any::from("discouraged"),
            ResidentKeyRequirement::PREFERRED => Any::from("preferred"),
            ResidentKeyRequirement::REQUIRED => Any::from("required"),
         }
    }
}
impl From<&ResidentKeyRequirement> for Any {
    fn from(s: &ResidentKeyRequirement) -> Any {
         match *s {
            ResidentKeyRequirement::DISCOURAGED => Any::from("discouraged"),
            ResidentKeyRequirement::PREFERRED => Any::from("preferred"),
            ResidentKeyRequirement::REQUIRED => Any::from("required"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "indirect" => Self::INDIRECT,
            "direct" => Self::DIRECT,
            "enterprise" => Self::ENTERPRISE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AttestationConveyancePreference> for Any {
    fn from(s: AttestationConveyancePreference) -> Any {
         match s {
            AttestationConveyancePreference::NONE => Any::from("none"),
            AttestationConveyancePreference::INDIRECT => Any::from("indirect"),
            AttestationConveyancePreference::DIRECT => Any::from("direct"),
            AttestationConveyancePreference::ENTERPRISE => Any::from("enterprise"),
         }
    }
}
impl From<&AttestationConveyancePreference> for Any {
    fn from(s: &AttestationConveyancePreference) -> Any {
         match *s {
            AttestationConveyancePreference::NONE => Any::from("none"),
            AttestationConveyancePreference::INDIRECT => Any::from("indirect"),
            AttestationConveyancePreference::DIRECT => Any::from("direct"),
            AttestationConveyancePreference::ENTERPRISE => Any::from("enterprise"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TokenBindingStatus {
    PRESENT,
    SUPPORTED,
}
impl FromVal for TokenBindingStatus {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "present" => Self::PRESENT,
            "supported" => Self::SUPPORTED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<TokenBindingStatus> for Any {
    fn from(s: TokenBindingStatus) -> Any {
         match s {
            TokenBindingStatus::PRESENT => Any::from("present"),
            TokenBindingStatus::SUPPORTED => Any::from("supported"),
         }
    }
}
impl From<&TokenBindingStatus> for Any {
    fn from(s: &TokenBindingStatus) -> Any {
         match *s {
            TokenBindingStatus::PRESENT => Any::from("present"),
            TokenBindingStatus::SUPPORTED => Any::from("supported"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PublicKeyCredentialType {
    PUBLIC_KEY,
}
impl FromVal for PublicKeyCredentialType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "public-key" => Self::PUBLIC_KEY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PublicKeyCredentialType> for Any {
    fn from(s: PublicKeyCredentialType) -> Any {
         match s {
            PublicKeyCredentialType::PUBLIC_KEY => Any::from("public-key"),
         }
    }
}
impl From<&PublicKeyCredentialType> for Any {
    fn from(s: &PublicKeyCredentialType) -> Any {
         match *s {
            PublicKeyCredentialType::PUBLIC_KEY => Any::from("public-key"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "usb" => Self::USB,
            "nfc" => Self::NFC,
            "ble" => Self::BLE,
            "smart-card" => Self::SMART_CARD,
            "hybrid" => Self::HYBRID,
            "internal" => Self::INTERNAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AuthenticatorTransport> for Any {
    fn from(s: AuthenticatorTransport) -> Any {
         match s {
            AuthenticatorTransport::USB => Any::from("usb"),
            AuthenticatorTransport::NFC => Any::from("nfc"),
            AuthenticatorTransport::BLE => Any::from("ble"),
            AuthenticatorTransport::SMART_CARD => Any::from("smart-card"),
            AuthenticatorTransport::HYBRID => Any::from("hybrid"),
            AuthenticatorTransport::INTERNAL => Any::from("internal"),
         }
    }
}
impl From<&AuthenticatorTransport> for Any {
    fn from(s: &AuthenticatorTransport) -> Any {
         match *s {
            AuthenticatorTransport::USB => Any::from("usb"),
            AuthenticatorTransport::NFC => Any::from("nfc"),
            AuthenticatorTransport::BLE => Any::from("ble"),
            AuthenticatorTransport::SMART_CARD => Any::from("smart-card"),
            AuthenticatorTransport::HYBRID => Any::from("hybrid"),
            AuthenticatorTransport::INTERNAL => Any::from("internal"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "required" => Self::REQUIRED,
            "preferred" => Self::PREFERRED,
            "discouraged" => Self::DISCOURAGED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<UserVerificationRequirement> for Any {
    fn from(s: UserVerificationRequirement) -> Any {
         match s {
            UserVerificationRequirement::REQUIRED => Any::from("required"),
            UserVerificationRequirement::PREFERRED => Any::from("preferred"),
            UserVerificationRequirement::DISCOURAGED => Any::from("discouraged"),
         }
    }
}
impl From<&UserVerificationRequirement> for Any {
    fn from(s: &UserVerificationRequirement) -> Any {
         match *s {
            UserVerificationRequirement::REQUIRED => Any::from("required"),
            UserVerificationRequirement::PREFERRED => Any::from("preferred"),
            UserVerificationRequirement::DISCOURAGED => Any::from("discouraged"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ClientCapability> for Any {
    fn from(s: ClientCapability) -> Any {
         match s {
            ClientCapability::CONDITIONAL_CREATE => Any::from("conditionalCreate"),
            ClientCapability::CONDITIONAL_GET => Any::from("conditionalGet"),
            ClientCapability::HYBRID_TRANSPORT => Any::from("hybridTransport"),
            ClientCapability::PASSKEY_PLATFORM_AUTHENTICATOR => Any::from("passkeyPlatformAuthenticator"),
            ClientCapability::USER_VERIFYING_PLATFORM_AUTHENTICATOR => Any::from("userVerifyingPlatformAuthenticator"),
            ClientCapability::RELATED_ORIGINS => Any::from("relatedOrigins"),
            ClientCapability::SIGNAL_ALL_ACCEPTED_CREDENTIALS => Any::from("signalAllAcceptedCredentials"),
            ClientCapability::SIGNAL_CURRENT_USER_DETAILS => Any::from("signalCurrentUserDetails"),
            ClientCapability::SIGNAL_UNKNOWN_CREDENTIAL => Any::from("signalUnknownCredential"),
         }
    }
}
impl From<&ClientCapability> for Any {
    fn from(s: &ClientCapability) -> Any {
         match *s {
            ClientCapability::CONDITIONAL_CREATE => Any::from("conditionalCreate"),
            ClientCapability::CONDITIONAL_GET => Any::from("conditionalGet"),
            ClientCapability::HYBRID_TRANSPORT => Any::from("hybridTransport"),
            ClientCapability::PASSKEY_PLATFORM_AUTHENTICATOR => Any::from("passkeyPlatformAuthenticator"),
            ClientCapability::USER_VERIFYING_PLATFORM_AUTHENTICATOR => Any::from("userVerifyingPlatformAuthenticator"),
            ClientCapability::RELATED_ORIGINS => Any::from("relatedOrigins"),
            ClientCapability::SIGNAL_ALL_ACCEPTED_CREDENTIALS => Any::from("signalAllAcceptedCredentials"),
            ClientCapability::SIGNAL_CURRENT_USER_DETAILS => Any::from("signalCurrentUserDetails"),
            ClientCapability::SIGNAL_UNKNOWN_CREDENTIAL => Any::from("signalUnknownCredential"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "security-key" => Self::SECURITY_KEY,
            "client-device" => Self::CLIENT_DEVICE,
            "hybrid" => Self::HYBRID,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PublicKeyCredentialHint> for Any {
    fn from(s: PublicKeyCredentialHint) -> Any {
         match s {
            PublicKeyCredentialHint::SECURITY_KEY => Any::from("security-key"),
            PublicKeyCredentialHint::CLIENT_DEVICE => Any::from("client-device"),
            PublicKeyCredentialHint::HYBRID => Any::from("hybrid"),
         }
    }
}
impl From<&PublicKeyCredentialHint> for Any {
    fn from(s: &PublicKeyCredentialHint) -> Any {
         match *s {
            PublicKeyCredentialHint::SECURITY_KEY => Any::from("security-key"),
            PublicKeyCredentialHint::CLIENT_DEVICE => Any::from("client-device"),
            PublicKeyCredentialHint::HYBRID => Any::from("hybrid"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LargeBlobSupport {
    REQUIRED,
    PREFERRED,
}
impl FromVal for LargeBlobSupport {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "required" => Self::REQUIRED,
            "preferred" => Self::PREFERRED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<LargeBlobSupport> for Any {
    fn from(s: LargeBlobSupport) -> Any {
         match s {
            LargeBlobSupport::REQUIRED => Any::from("required"),
            LargeBlobSupport::PREFERRED => Any::from("preferred"),
         }
    }
}
impl From<&LargeBlobSupport> for Any {
    fn from(s: &LargeBlobSupport) -> Any {
         match *s {
            LargeBlobSupport::REQUIRED => Any::from("required"),
            LargeBlobSupport::PREFERRED => Any::from("preferred"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AacBitstreamFormat {
    AAC,
    ADTS,
}
impl FromVal for AacBitstreamFormat {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "aac" => Self::AAC,
            "adts" => Self::ADTS,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AacBitstreamFormat> for Any {
    fn from(s: AacBitstreamFormat) -> Any {
         match s {
            AacBitstreamFormat::AAC => Any::from("aac"),
            AacBitstreamFormat::ADTS => Any::from("adts"),
         }
    }
}
impl From<&AacBitstreamFormat> for Any {
    fn from(s: &AacBitstreamFormat) -> Any {
         match *s {
            AacBitstreamFormat::AAC => Any::from("aac"),
            AacBitstreamFormat::ADTS => Any::from("adts"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AvcBitstreamFormat {
    ANNEXB,
    AVC,
}
impl FromVal for AvcBitstreamFormat {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "annexb" => Self::ANNEXB,
            "avc" => Self::AVC,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AvcBitstreamFormat> for Any {
    fn from(s: AvcBitstreamFormat) -> Any {
         match s {
            AvcBitstreamFormat::ANNEXB => Any::from("annexb"),
            AvcBitstreamFormat::AVC => Any::from("avc"),
         }
    }
}
impl From<&AvcBitstreamFormat> for Any {
    fn from(s: &AvcBitstreamFormat) -> Any {
         match *s {
            AvcBitstreamFormat::ANNEXB => Any::from("annexb"),
            AvcBitstreamFormat::AVC => Any::from("avc"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum HevcBitstreamFormat {
    ANNEXB,
    HEVC,
}
impl FromVal for HevcBitstreamFormat {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "annexb" => Self::ANNEXB,
            "hevc" => Self::HEVC,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<HevcBitstreamFormat> for Any {
    fn from(s: HevcBitstreamFormat) -> Any {
         match s {
            HevcBitstreamFormat::ANNEXB => Any::from("annexb"),
            HevcBitstreamFormat::HEVC => Any::from("hevc"),
         }
    }
}
impl From<&HevcBitstreamFormat> for Any {
    fn from(s: &HevcBitstreamFormat) -> Any {
         match *s {
            HevcBitstreamFormat::ANNEXB => Any::from("annexb"),
            HevcBitstreamFormat::HEVC => Any::from("hevc"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum OpusBitstreamFormat {
    OPUS,
    OGG,
}
impl FromVal for OpusBitstreamFormat {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "opus" => Self::OPUS,
            "ogg" => Self::OGG,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OpusBitstreamFormat> for Any {
    fn from(s: OpusBitstreamFormat) -> Any {
         match s {
            OpusBitstreamFormat::OPUS => Any::from("opus"),
            OpusBitstreamFormat::OGG => Any::from("ogg"),
         }
    }
}
impl From<&OpusBitstreamFormat> for Any {
    fn from(s: &OpusBitstreamFormat) -> Any {
         match *s {
            OpusBitstreamFormat::OPUS => Any::from("opus"),
            OpusBitstreamFormat::OGG => Any::from("ogg"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
            "music" => Self::MUSIC,
            "voice" => Self::VOICE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OpusSignal> for Any {
    fn from(s: OpusSignal) -> Any {
         match s {
            OpusSignal::AUTO => Any::from("auto"),
            OpusSignal::MUSIC => Any::from("music"),
            OpusSignal::VOICE => Any::from("voice"),
         }
    }
}
impl From<&OpusSignal> for Any {
    fn from(s: &OpusSignal) -> Any {
         match *s {
            OpusSignal::AUTO => Any::from("auto"),
            OpusSignal::MUSIC => Any::from("music"),
            OpusSignal::VOICE => Any::from("voice"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "voip" => Self::VOIP,
            "audio" => Self::AUDIO,
            "lowdelay" => Self::LOWDELAY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<OpusApplication> for Any {
    fn from(s: OpusApplication) -> Any {
         match s {
            OpusApplication::VOIP => Any::from("voip"),
            OpusApplication::AUDIO => Any::from("audio"),
            OpusApplication::LOWDELAY => Any::from("lowdelay"),
         }
    }
}
impl From<&OpusApplication> for Any {
    fn from(s: &OpusApplication) -> Any {
         match *s {
            OpusApplication::VOIP => Any::from("voip"),
            OpusApplication::AUDIO => Any::from("audio"),
            OpusApplication::LOWDELAY => Any::from("lowdelay"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "no-preference" => Self::NO_PREFERENCE,
            "prefer-hardware" => Self::PREFER_HARDWARE,
            "prefer-software" => Self::PREFER_SOFTWARE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<HardwareAcceleration> for Any {
    fn from(s: HardwareAcceleration) -> Any {
         match s {
            HardwareAcceleration::NO_PREFERENCE => Any::from("no-preference"),
            HardwareAcceleration::PREFER_HARDWARE => Any::from("prefer-hardware"),
            HardwareAcceleration::PREFER_SOFTWARE => Any::from("prefer-software"),
         }
    }
}
impl From<&HardwareAcceleration> for Any {
    fn from(s: &HardwareAcceleration) -> Any {
         match *s {
            HardwareAcceleration::NO_PREFERENCE => Any::from("no-preference"),
            HardwareAcceleration::PREFER_HARDWARE => Any::from("prefer-hardware"),
            HardwareAcceleration::PREFER_SOFTWARE => Any::from("prefer-software"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AlphaOption {
    KEEP,
    DISCARD,
}
impl FromVal for AlphaOption {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "keep" => Self::KEEP,
            "discard" => Self::DISCARD,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AlphaOption> for Any {
    fn from(s: AlphaOption) -> Any {
         match s {
            AlphaOption::KEEP => Any::from("keep"),
            AlphaOption::DISCARD => Any::from("discard"),
         }
    }
}
impl From<&AlphaOption> for Any {
    fn from(s: &AlphaOption) -> Any {
         match *s {
            AlphaOption::KEEP => Any::from("keep"),
            AlphaOption::DISCARD => Any::from("discard"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum LatencyMode {
    QUALITY,
    REALTIME,
}
impl FromVal for LatencyMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "quality" => Self::QUALITY,
            "realtime" => Self::REALTIME,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<LatencyMode> for Any {
    fn from(s: LatencyMode) -> Any {
         match s {
            LatencyMode::QUALITY => Any::from("quality"),
            LatencyMode::REALTIME => Any::from("realtime"),
         }
    }
}
impl From<&LatencyMode> for Any {
    fn from(s: &LatencyMode) -> Any {
         match *s {
            LatencyMode::QUALITY => Any::from("quality"),
            LatencyMode::REALTIME => Any::from("realtime"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "constant" => Self::CONSTANT,
            "variable" => Self::VARIABLE,
            "quantizer" => Self::QUANTIZER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<VideoEncoderBitrateMode> for Any {
    fn from(s: VideoEncoderBitrateMode) -> Any {
         match s {
            VideoEncoderBitrateMode::CONSTANT => Any::from("constant"),
            VideoEncoderBitrateMode::VARIABLE => Any::from("variable"),
            VideoEncoderBitrateMode::QUANTIZER => Any::from("quantizer"),
         }
    }
}
impl From<&VideoEncoderBitrateMode> for Any {
    fn from(s: &VideoEncoderBitrateMode) -> Any {
         match *s {
            VideoEncoderBitrateMode::CONSTANT => Any::from("constant"),
            VideoEncoderBitrateMode::VARIABLE => Any::from("variable"),
            VideoEncoderBitrateMode::QUANTIZER => Any::from("quantizer"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "unconfigured" => Self::UNCONFIGURED,
            "configured" => Self::CONFIGURED,
            "closed" => Self::CLOSED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<CodecState> for Any {
    fn from(s: CodecState) -> Any {
         match s {
            CodecState::UNCONFIGURED => Any::from("unconfigured"),
            CodecState::CONFIGURED => Any::from("configured"),
            CodecState::CLOSED => Any::from("closed"),
         }
    }
}
impl From<&CodecState> for Any {
    fn from(s: &CodecState) -> Any {
         match *s {
            CodecState::UNCONFIGURED => Any::from("unconfigured"),
            CodecState::CONFIGURED => Any::from("configured"),
            CodecState::CLOSED => Any::from("closed"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EncodedAudioChunkType {
    KEY,
    DELTA,
}
impl FromVal for EncodedAudioChunkType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "key" => Self::KEY,
            "delta" => Self::DELTA,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<EncodedAudioChunkType> for Any {
    fn from(s: EncodedAudioChunkType) -> Any {
         match s {
            EncodedAudioChunkType::KEY => Any::from("key"),
            EncodedAudioChunkType::DELTA => Any::from("delta"),
         }
    }
}
impl From<&EncodedAudioChunkType> for Any {
    fn from(s: &EncodedAudioChunkType) -> Any {
         match *s {
            EncodedAudioChunkType::KEY => Any::from("key"),
            EncodedAudioChunkType::DELTA => Any::from("delta"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum EncodedVideoChunkType {
    KEY,
    DELTA,
}
impl FromVal for EncodedVideoChunkType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "key" => Self::KEY,
            "delta" => Self::DELTA,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<EncodedVideoChunkType> for Any {
    fn from(s: EncodedVideoChunkType) -> Any {
         match s {
            EncodedVideoChunkType::KEY => Any::from("key"),
            EncodedVideoChunkType::DELTA => Any::from("delta"),
         }
    }
}
impl From<&EncodedVideoChunkType> for Any {
    fn from(s: &EncodedVideoChunkType) -> Any {
         match *s {
            EncodedVideoChunkType::KEY => Any::from("key"),
            EncodedVideoChunkType::DELTA => Any::from("delta"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AudioSampleFormat> for Any {
    fn from(s: AudioSampleFormat) -> Any {
         match s {
            AudioSampleFormat::U8_ => Any::from("u8"),
            AudioSampleFormat::S16 => Any::from("s16"),
            AudioSampleFormat::S32 => Any::from("s32"),
            AudioSampleFormat::F32_ => Any::from("f32"),
            AudioSampleFormat::U8_PLANAR => Any::from("u8-planar"),
            AudioSampleFormat::S16_PLANAR => Any::from("s16-planar"),
            AudioSampleFormat::S32_PLANAR => Any::from("s32-planar"),
            AudioSampleFormat::F32_PLANAR => Any::from("f32-planar"),
         }
    }
}
impl From<&AudioSampleFormat> for Any {
    fn from(s: &AudioSampleFormat) -> Any {
         match *s {
            AudioSampleFormat::U8_ => Any::from("u8"),
            AudioSampleFormat::S16 => Any::from("s16"),
            AudioSampleFormat::S32 => Any::from("s32"),
            AudioSampleFormat::F32_ => Any::from("f32"),
            AudioSampleFormat::U8_PLANAR => Any::from("u8-planar"),
            AudioSampleFormat::S16_PLANAR => Any::from("s16-planar"),
            AudioSampleFormat::S32_PLANAR => Any::from("s32-planar"),
            AudioSampleFormat::F32_PLANAR => Any::from("f32-planar"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<VideoPixelFormat> for Any {
    fn from(s: VideoPixelFormat) -> Any {
         match s {
            VideoPixelFormat::I420 => Any::from("I420"),
            VideoPixelFormat::I420_P10 => Any::from("I420P10"),
            VideoPixelFormat::I420_P12 => Any::from("I420P12"),
            VideoPixelFormat::I420_A => Any::from("I420A"),
            VideoPixelFormat::I420_AP10 => Any::from("I420AP10"),
            VideoPixelFormat::I420_AP12 => Any::from("I420AP12"),
            VideoPixelFormat::I422 => Any::from("I422"),
            VideoPixelFormat::I422_P10 => Any::from("I422P10"),
            VideoPixelFormat::I422_P12 => Any::from("I422P12"),
            VideoPixelFormat::I422_A => Any::from("I422A"),
            VideoPixelFormat::I422_AP10 => Any::from("I422AP10"),
            VideoPixelFormat::I422_AP12 => Any::from("I422AP12"),
            VideoPixelFormat::I444 => Any::from("I444"),
            VideoPixelFormat::I444_P10 => Any::from("I444P10"),
            VideoPixelFormat::I444_P12 => Any::from("I444P12"),
            VideoPixelFormat::I444_A => Any::from("I444A"),
            VideoPixelFormat::I444_AP10 => Any::from("I444AP10"),
            VideoPixelFormat::I444_AP12 => Any::from("I444AP12"),
            VideoPixelFormat::NV12 => Any::from("NV12"),
            VideoPixelFormat::RGBA => Any::from("RGBA"),
            VideoPixelFormat::RGBX => Any::from("RGBX"),
            VideoPixelFormat::BGRA => Any::from("BGRA"),
            VideoPixelFormat::BGRX => Any::from("BGRX"),
         }
    }
}
impl From<&VideoPixelFormat> for Any {
    fn from(s: &VideoPixelFormat) -> Any {
         match *s {
            VideoPixelFormat::I420 => Any::from("I420"),
            VideoPixelFormat::I420_P10 => Any::from("I420P10"),
            VideoPixelFormat::I420_P12 => Any::from("I420P12"),
            VideoPixelFormat::I420_A => Any::from("I420A"),
            VideoPixelFormat::I420_AP10 => Any::from("I420AP10"),
            VideoPixelFormat::I420_AP12 => Any::from("I420AP12"),
            VideoPixelFormat::I422 => Any::from("I422"),
            VideoPixelFormat::I422_P10 => Any::from("I422P10"),
            VideoPixelFormat::I422_P12 => Any::from("I422P12"),
            VideoPixelFormat::I422_A => Any::from("I422A"),
            VideoPixelFormat::I422_AP10 => Any::from("I422AP10"),
            VideoPixelFormat::I422_AP12 => Any::from("I422AP12"),
            VideoPixelFormat::I444 => Any::from("I444"),
            VideoPixelFormat::I444_P10 => Any::from("I444P10"),
            VideoPixelFormat::I444_P12 => Any::from("I444P12"),
            VideoPixelFormat::I444_A => Any::from("I444A"),
            VideoPixelFormat::I444_AP10 => Any::from("I444AP10"),
            VideoPixelFormat::I444_AP12 => Any::from("I444AP12"),
            VideoPixelFormat::NV12 => Any::from("NV12"),
            VideoPixelFormat::RGBA => Any::from("RGBA"),
            VideoPixelFormat::RGBX => Any::from("RGBX"),
            VideoPixelFormat::BGRA => Any::from("BGRA"),
            VideoPixelFormat::BGRX => Any::from("BGRX"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "bt709" => Self::BT709,
            "bt470bg" => Self::BT470BG,
            "smpte170m" => Self::SMPTE170M,
            "bt2020" => Self::BT2020,
            "smpte432" => Self::SMPTE432,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<VideoColorPrimaries> for Any {
    fn from(s: VideoColorPrimaries) -> Any {
         match s {
            VideoColorPrimaries::BT709 => Any::from("bt709"),
            VideoColorPrimaries::BT470BG => Any::from("bt470bg"),
            VideoColorPrimaries::SMPTE170M => Any::from("smpte170m"),
            VideoColorPrimaries::BT2020 => Any::from("bt2020"),
            VideoColorPrimaries::SMPTE432 => Any::from("smpte432"),
         }
    }
}
impl From<&VideoColorPrimaries> for Any {
    fn from(s: &VideoColorPrimaries) -> Any {
         match *s {
            VideoColorPrimaries::BT709 => Any::from("bt709"),
            VideoColorPrimaries::BT470BG => Any::from("bt470bg"),
            VideoColorPrimaries::SMPTE170M => Any::from("smpte170m"),
            VideoColorPrimaries::BT2020 => Any::from("bt2020"),
            VideoColorPrimaries::SMPTE432 => Any::from("smpte432"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "bt709" => Self::BT709,
            "smpte170m" => Self::SMPTE170M,
            "iec61966-2-1" => Self::IEC61966_2_1,
            "linear" => Self::LINEAR,
            "pq" => Self::PQ,
            "hlg" => Self::HLG,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<VideoTransferCharacteristics> for Any {
    fn from(s: VideoTransferCharacteristics) -> Any {
         match s {
            VideoTransferCharacteristics::BT709 => Any::from("bt709"),
            VideoTransferCharacteristics::SMPTE170M => Any::from("smpte170m"),
            VideoTransferCharacteristics::IEC61966_2_1 => Any::from("iec61966-2-1"),
            VideoTransferCharacteristics::LINEAR => Any::from("linear"),
            VideoTransferCharacteristics::PQ => Any::from("pq"),
            VideoTransferCharacteristics::HLG => Any::from("hlg"),
         }
    }
}
impl From<&VideoTransferCharacteristics> for Any {
    fn from(s: &VideoTransferCharacteristics) -> Any {
         match *s {
            VideoTransferCharacteristics::BT709 => Any::from("bt709"),
            VideoTransferCharacteristics::SMPTE170M => Any::from("smpte170m"),
            VideoTransferCharacteristics::IEC61966_2_1 => Any::from("iec61966-2-1"),
            VideoTransferCharacteristics::LINEAR => Any::from("linear"),
            VideoTransferCharacteristics::PQ => Any::from("pq"),
            VideoTransferCharacteristics::HLG => Any::from("hlg"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "rgb" => Self::RGB,
            "bt709" => Self::BT709,
            "bt470bg" => Self::BT470BG,
            "smpte170m" => Self::SMPTE170M,
            "bt2020-ncl" => Self::BT2020_NCL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<VideoMatrixCoefficients> for Any {
    fn from(s: VideoMatrixCoefficients) -> Any {
         match s {
            VideoMatrixCoefficients::RGB => Any::from("rgb"),
            VideoMatrixCoefficients::BT709 => Any::from("bt709"),
            VideoMatrixCoefficients::BT470BG => Any::from("bt470bg"),
            VideoMatrixCoefficients::SMPTE170M => Any::from("smpte170m"),
            VideoMatrixCoefficients::BT2020_NCL => Any::from("bt2020-ncl"),
         }
    }
}
impl From<&VideoMatrixCoefficients> for Any {
    fn from(s: &VideoMatrixCoefficients) -> Any {
         match *s {
            VideoMatrixCoefficients::RGB => Any::from("rgb"),
            VideoMatrixCoefficients::BT709 => Any::from("bt709"),
            VideoMatrixCoefficients::BT470BG => Any::from("bt470bg"),
            VideoMatrixCoefficients::SMPTE170M => Any::from("smpte170m"),
            VideoMatrixCoefficients::BT2020_NCL => Any::from("bt2020-ncl"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "public" => Self::PUBLIC,
            "private" => Self::PRIVATE,
            "secret" => Self::SECRET,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<KeyType> for Any {
    fn from(s: KeyType) -> Any {
         match s {
            KeyType::PUBLIC => Any::from("public"),
            KeyType::PRIVATE => Any::from("private"),
            KeyType::SECRET => Any::from("secret"),
         }
    }
}
impl From<&KeyType> for Any {
    fn from(s: &KeyType) -> Any {
         match *s {
            KeyType::PUBLIC => Any::from("public"),
            KeyType::PRIVATE => Any::from("private"),
            KeyType::SECRET => Any::from("secret"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<KeyUsage> for Any {
    fn from(s: KeyUsage) -> Any {
         match s {
            KeyUsage::ENCRYPT => Any::from("encrypt"),
            KeyUsage::DECRYPT => Any::from("decrypt"),
            KeyUsage::SIGN => Any::from("sign"),
            KeyUsage::VERIFY => Any::from("verify"),
            KeyUsage::DERIVE_KEY => Any::from("deriveKey"),
            KeyUsage::DERIVE_BITS => Any::from("deriveBits"),
            KeyUsage::WRAP_KEY => Any::from("wrapKey"),
            KeyUsage::UNWRAP_KEY => Any::from("unwrapKey"),
         }
    }
}
impl From<&KeyUsage> for Any {
    fn from(s: &KeyUsage) -> Any {
         match *s {
            KeyUsage::ENCRYPT => Any::from("encrypt"),
            KeyUsage::DECRYPT => Any::from("decrypt"),
            KeyUsage::SIGN => Any::from("sign"),
            KeyUsage::VERIFY => Any::from("verify"),
            KeyUsage::DERIVE_KEY => Any::from("deriveKey"),
            KeyUsage::DERIVE_BITS => Any::from("deriveBits"),
            KeyUsage::WRAP_KEY => Any::from("wrapKey"),
            KeyUsage::UNWRAP_KEY => Any::from("unwrapKey"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "raw" => Self::RAW_,
            "spki" => Self::SPKI,
            "pkcs8" => Self::PKCS8,
            "jwk" => Self::JWK,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<KeyFormat> for Any {
    fn from(s: KeyFormat) -> Any {
         match s {
            KeyFormat::RAW_ => Any::from("raw"),
            KeyFormat::SPKI => Any::from("spki"),
            KeyFormat::PKCS8 => Any::from("pkcs8"),
            KeyFormat::JWK => Any::from("jwk"),
         }
    }
}
impl From<&KeyFormat> for Any {
    fn from(s: &KeyFormat) -> Any {
         match *s {
            KeyFormat::RAW_ => Any::from("raw"),
            KeyFormat::SPKI => Any::from("spki"),
            KeyFormat::PKCS8 => Any::from("pkcs8"),
            KeyFormat::JWK => Any::from("jwk"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "default" => Self::DEFAULT,
            "low-power" => Self::LOW_POWER,
            "high-performance" => Self::HIGH_PERFORMANCE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WebGLPowerPreference> for Any {
    fn from(s: WebGLPowerPreference) -> Any {
         match s {
            WebGLPowerPreference::DEFAULT => Any::from("default"),
            WebGLPowerPreference::LOW_POWER => Any::from("low-power"),
            WebGLPowerPreference::HIGH_PERFORMANCE => Any::from("high-performance"),
         }
    }
}
impl From<&WebGLPowerPreference> for Any {
    fn from(s: &WebGLPowerPreference) -> Any {
         match *s {
            WebGLPowerPreference::DEFAULT => Any::from("default"),
            WebGLPowerPreference::LOW_POWER => Any::from("low-power"),
            WebGLPowerPreference::HIGH_PERFORMANCE => Any::from("high-performance"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUPowerPreference {
    LOW_POWER,
    HIGH_PERFORMANCE,
}
impl FromVal for GPUPowerPreference {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "low-power" => Self::LOW_POWER,
            "high-performance" => Self::HIGH_PERFORMANCE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUPowerPreference> for Any {
    fn from(s: GPUPowerPreference) -> Any {
         match s {
            GPUPowerPreference::LOW_POWER => Any::from("low-power"),
            GPUPowerPreference::HIGH_PERFORMANCE => Any::from("high-performance"),
         }
    }
}
impl From<&GPUPowerPreference> for Any {
    fn from(s: &GPUPowerPreference) -> Any {
         match *s {
            GPUPowerPreference::LOW_POWER => Any::from("low-power"),
            GPUPowerPreference::HIGH_PERFORMANCE => Any::from("high-performance"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUFeatureName> for Any {
    fn from(s: GPUFeatureName) -> Any {
         match s {
            GPUFeatureName::CORE_FEATURES_AND_LIMITS => Any::from("core-features-and-limits"),
            GPUFeatureName::DEPTH_CLIP_CONTROL => Any::from("depth-clip-control"),
            GPUFeatureName::DEPTH32FLOAT_STENCIL8 => Any::from("depth32float-stencil8"),
            GPUFeatureName::TEXTURE_COMPRESSION_BC => Any::from("texture-compression-bc"),
            GPUFeatureName::TEXTURE_COMPRESSION_BC_SLICED_3D => Any::from("texture-compression-bc-sliced-3d"),
            GPUFeatureName::TEXTURE_COMPRESSION_ETC2 => Any::from("texture-compression-etc2"),
            GPUFeatureName::TEXTURE_COMPRESSION_ASTC => Any::from("texture-compression-astc"),
            GPUFeatureName::TEXTURE_COMPRESSION_ASTC_SLICED_3D => Any::from("texture-compression-astc-sliced-3d"),
            GPUFeatureName::TIMESTAMP_QUERY => Any::from("timestamp-query"),
            GPUFeatureName::INDIRECT_FIRST_INSTANCE => Any::from("indirect-first-instance"),
            GPUFeatureName::SHADER_F16 => Any::from("shader-f16"),
            GPUFeatureName::RG11B10UFLOAT_RENDERABLE => Any::from("rg11b10ufloat-renderable"),
            GPUFeatureName::BGRA8UNORM_STORAGE => Any::from("bgra8unorm-storage"),
            GPUFeatureName::FLOAT32_FILTERABLE => Any::from("float32-filterable"),
            GPUFeatureName::FLOAT32_BLENDABLE => Any::from("float32-blendable"),
            GPUFeatureName::CLIP_DISTANCES => Any::from("clip-distances"),
            GPUFeatureName::DUAL_SOURCE_BLENDING => Any::from("dual-source-blending"),
            GPUFeatureName::SUBGROUPS => Any::from("subgroups"),
            GPUFeatureName::TEXTURE_FORMATS_TIER1 => Any::from("texture-formats-tier1"),
            GPUFeatureName::TEXTURE_FORMATS_TIER2 => Any::from("texture-formats-tier2"),
         }
    }
}
impl From<&GPUFeatureName> for Any {
    fn from(s: &GPUFeatureName) -> Any {
         match *s {
            GPUFeatureName::CORE_FEATURES_AND_LIMITS => Any::from("core-features-and-limits"),
            GPUFeatureName::DEPTH_CLIP_CONTROL => Any::from("depth-clip-control"),
            GPUFeatureName::DEPTH32FLOAT_STENCIL8 => Any::from("depth32float-stencil8"),
            GPUFeatureName::TEXTURE_COMPRESSION_BC => Any::from("texture-compression-bc"),
            GPUFeatureName::TEXTURE_COMPRESSION_BC_SLICED_3D => Any::from("texture-compression-bc-sliced-3d"),
            GPUFeatureName::TEXTURE_COMPRESSION_ETC2 => Any::from("texture-compression-etc2"),
            GPUFeatureName::TEXTURE_COMPRESSION_ASTC => Any::from("texture-compression-astc"),
            GPUFeatureName::TEXTURE_COMPRESSION_ASTC_SLICED_3D => Any::from("texture-compression-astc-sliced-3d"),
            GPUFeatureName::TIMESTAMP_QUERY => Any::from("timestamp-query"),
            GPUFeatureName::INDIRECT_FIRST_INSTANCE => Any::from("indirect-first-instance"),
            GPUFeatureName::SHADER_F16 => Any::from("shader-f16"),
            GPUFeatureName::RG11B10UFLOAT_RENDERABLE => Any::from("rg11b10ufloat-renderable"),
            GPUFeatureName::BGRA8UNORM_STORAGE => Any::from("bgra8unorm-storage"),
            GPUFeatureName::FLOAT32_FILTERABLE => Any::from("float32-filterable"),
            GPUFeatureName::FLOAT32_BLENDABLE => Any::from("float32-blendable"),
            GPUFeatureName::CLIP_DISTANCES => Any::from("clip-distances"),
            GPUFeatureName::DUAL_SOURCE_BLENDING => Any::from("dual-source-blending"),
            GPUFeatureName::SUBGROUPS => Any::from("subgroups"),
            GPUFeatureName::TEXTURE_FORMATS_TIER1 => Any::from("texture-formats-tier1"),
            GPUFeatureName::TEXTURE_FORMATS_TIER2 => Any::from("texture-formats-tier2"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "unmapped" => Self::UNMAPPED,
            "pending" => Self::PENDING,
            "mapped" => Self::MAPPED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUBufferMapState> for Any {
    fn from(s: GPUBufferMapState) -> Any {
         match s {
            GPUBufferMapState::UNMAPPED => Any::from("unmapped"),
            GPUBufferMapState::PENDING => Any::from("pending"),
            GPUBufferMapState::MAPPED => Any::from("mapped"),
         }
    }
}
impl From<&GPUBufferMapState> for Any {
    fn from(s: &GPUBufferMapState) -> Any {
         match *s {
            GPUBufferMapState::UNMAPPED => Any::from("unmapped"),
            GPUBufferMapState::PENDING => Any::from("pending"),
            GPUBufferMapState::MAPPED => Any::from("mapped"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "1d" => Self::_1D,
            "2d" => Self::_2D,
            "3d" => Self::_3D,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUTextureDimension> for Any {
    fn from(s: GPUTextureDimension) -> Any {
         match s {
            GPUTextureDimension::_1D => Any::from("1d"),
            GPUTextureDimension::_2D => Any::from("2d"),
            GPUTextureDimension::_3D => Any::from("3d"),
         }
    }
}
impl From<&GPUTextureDimension> for Any {
    fn from(s: &GPUTextureDimension) -> Any {
         match *s {
            GPUTextureDimension::_1D => Any::from("1d"),
            GPUTextureDimension::_2D => Any::from("2d"),
            GPUTextureDimension::_3D => Any::from("3d"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "1d" => Self::_1D,
            "2d" => Self::_2D,
            "2d-array" => Self::_2D_ARRAY,
            "cube" => Self::CUBE,
            "cube-array" => Self::CUBE_ARRAY,
            "3d" => Self::_3D,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUTextureViewDimension> for Any {
    fn from(s: GPUTextureViewDimension) -> Any {
         match s {
            GPUTextureViewDimension::_1D => Any::from("1d"),
            GPUTextureViewDimension::_2D => Any::from("2d"),
            GPUTextureViewDimension::_2D_ARRAY => Any::from("2d-array"),
            GPUTextureViewDimension::CUBE => Any::from("cube"),
            GPUTextureViewDimension::CUBE_ARRAY => Any::from("cube-array"),
            GPUTextureViewDimension::_3D => Any::from("3d"),
         }
    }
}
impl From<&GPUTextureViewDimension> for Any {
    fn from(s: &GPUTextureViewDimension) -> Any {
         match *s {
            GPUTextureViewDimension::_1D => Any::from("1d"),
            GPUTextureViewDimension::_2D => Any::from("2d"),
            GPUTextureViewDimension::_2D_ARRAY => Any::from("2d-array"),
            GPUTextureViewDimension::CUBE => Any::from("cube"),
            GPUTextureViewDimension::CUBE_ARRAY => Any::from("cube-array"),
            GPUTextureViewDimension::_3D => Any::from("3d"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "all" => Self::ALL,
            "stencil-only" => Self::STENCIL_ONLY,
            "depth-only" => Self::DEPTH_ONLY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUTextureAspect> for Any {
    fn from(s: GPUTextureAspect) -> Any {
         match s {
            GPUTextureAspect::ALL => Any::from("all"),
            GPUTextureAspect::STENCIL_ONLY => Any::from("stencil-only"),
            GPUTextureAspect::DEPTH_ONLY => Any::from("depth-only"),
         }
    }
}
impl From<&GPUTextureAspect> for Any {
    fn from(s: &GPUTextureAspect) -> Any {
         match *s {
            GPUTextureAspect::ALL => Any::from("all"),
            GPUTextureAspect::STENCIL_ONLY => Any::from("stencil-only"),
            GPUTextureAspect::DEPTH_ONLY => Any::from("depth-only"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUTextureFormat> for Any {
    fn from(s: GPUTextureFormat) -> Any {
         match s {
            GPUTextureFormat::R8UNORM => Any::from("r8unorm"),
            GPUTextureFormat::R8SNORM => Any::from("r8snorm"),
            GPUTextureFormat::R8UINT => Any::from("r8uint"),
            GPUTextureFormat::R8SINT => Any::from("r8sint"),
            GPUTextureFormat::R16UNORM => Any::from("r16unorm"),
            GPUTextureFormat::R16SNORM => Any::from("r16snorm"),
            GPUTextureFormat::R16UINT => Any::from("r16uint"),
            GPUTextureFormat::R16SINT => Any::from("r16sint"),
            GPUTextureFormat::R16FLOAT => Any::from("r16float"),
            GPUTextureFormat::RG8UNORM => Any::from("rg8unorm"),
            GPUTextureFormat::RG8SNORM => Any::from("rg8snorm"),
            GPUTextureFormat::RG8UINT => Any::from("rg8uint"),
            GPUTextureFormat::RG8SINT => Any::from("rg8sint"),
            GPUTextureFormat::R32UINT => Any::from("r32uint"),
            GPUTextureFormat::R32SINT => Any::from("r32sint"),
            GPUTextureFormat::R32FLOAT => Any::from("r32float"),
            GPUTextureFormat::RG16UNORM => Any::from("rg16unorm"),
            GPUTextureFormat::RG16SNORM => Any::from("rg16snorm"),
            GPUTextureFormat::RG16UINT => Any::from("rg16uint"),
            GPUTextureFormat::RG16SINT => Any::from("rg16sint"),
            GPUTextureFormat::RG16FLOAT => Any::from("rg16float"),
            GPUTextureFormat::RGBA8UNORM => Any::from("rgba8unorm"),
            GPUTextureFormat::RGBA8UNORM_SRGB => Any::from("rgba8unorm-srgb"),
            GPUTextureFormat::RGBA8SNORM => Any::from("rgba8snorm"),
            GPUTextureFormat::RGBA8UINT => Any::from("rgba8uint"),
            GPUTextureFormat::RGBA8SINT => Any::from("rgba8sint"),
            GPUTextureFormat::BGRA8UNORM => Any::from("bgra8unorm"),
            GPUTextureFormat::BGRA8UNORM_SRGB => Any::from("bgra8unorm-srgb"),
            GPUTextureFormat::RGB9E5UFLOAT => Any::from("rgb9e5ufloat"),
            GPUTextureFormat::RGB10A2UINT => Any::from("rgb10a2uint"),
            GPUTextureFormat::RGB10A2UNORM => Any::from("rgb10a2unorm"),
            GPUTextureFormat::RG11B10UFLOAT => Any::from("rg11b10ufloat"),
            GPUTextureFormat::RG32UINT => Any::from("rg32uint"),
            GPUTextureFormat::RG32SINT => Any::from("rg32sint"),
            GPUTextureFormat::RG32FLOAT => Any::from("rg32float"),
            GPUTextureFormat::RGBA16UNORM => Any::from("rgba16unorm"),
            GPUTextureFormat::RGBA16SNORM => Any::from("rgba16snorm"),
            GPUTextureFormat::RGBA16UINT => Any::from("rgba16uint"),
            GPUTextureFormat::RGBA16SINT => Any::from("rgba16sint"),
            GPUTextureFormat::RGBA16FLOAT => Any::from("rgba16float"),
            GPUTextureFormat::RGBA32UINT => Any::from("rgba32uint"),
            GPUTextureFormat::RGBA32SINT => Any::from("rgba32sint"),
            GPUTextureFormat::RGBA32FLOAT => Any::from("rgba32float"),
            GPUTextureFormat::STENCIL8 => Any::from("stencil8"),
            GPUTextureFormat::DEPTH16UNORM => Any::from("depth16unorm"),
            GPUTextureFormat::DEPTH24PLUS => Any::from("depth24plus"),
            GPUTextureFormat::DEPTH24PLUS_STENCIL8 => Any::from("depth24plus-stencil8"),
            GPUTextureFormat::DEPTH32FLOAT => Any::from("depth32float"),
            GPUTextureFormat::DEPTH32FLOAT_STENCIL8 => Any::from("depth32float-stencil8"),
            GPUTextureFormat::BC1_RGBA_UNORM => Any::from("bc1-rgba-unorm"),
            GPUTextureFormat::BC1_RGBA_UNORM_SRGB => Any::from("bc1-rgba-unorm-srgb"),
            GPUTextureFormat::BC2_RGBA_UNORM => Any::from("bc2-rgba-unorm"),
            GPUTextureFormat::BC2_RGBA_UNORM_SRGB => Any::from("bc2-rgba-unorm-srgb"),
            GPUTextureFormat::BC3_RGBA_UNORM => Any::from("bc3-rgba-unorm"),
            GPUTextureFormat::BC3_RGBA_UNORM_SRGB => Any::from("bc3-rgba-unorm-srgb"),
            GPUTextureFormat::BC4_R_UNORM => Any::from("bc4-r-unorm"),
            GPUTextureFormat::BC4_R_SNORM => Any::from("bc4-r-snorm"),
            GPUTextureFormat::BC5_RG_UNORM => Any::from("bc5-rg-unorm"),
            GPUTextureFormat::BC5_RG_SNORM => Any::from("bc5-rg-snorm"),
            GPUTextureFormat::BC6H_RGB_UFLOAT => Any::from("bc6h-rgb-ufloat"),
            GPUTextureFormat::BC6H_RGB_FLOAT => Any::from("bc6h-rgb-float"),
            GPUTextureFormat::BC7_RGBA_UNORM => Any::from("bc7-rgba-unorm"),
            GPUTextureFormat::BC7_RGBA_UNORM_SRGB => Any::from("bc7-rgba-unorm-srgb"),
            GPUTextureFormat::ETC2_RGB8UNORM => Any::from("etc2-rgb8unorm"),
            GPUTextureFormat::ETC2_RGB8UNORM_SRGB => Any::from("etc2-rgb8unorm-srgb"),
            GPUTextureFormat::ETC2_RGB8A1UNORM => Any::from("etc2-rgb8a1unorm"),
            GPUTextureFormat::ETC2_RGB8A1UNORM_SRGB => Any::from("etc2-rgb8a1unorm-srgb"),
            GPUTextureFormat::ETC2_RGBA8UNORM => Any::from("etc2-rgba8unorm"),
            GPUTextureFormat::ETC2_RGBA8UNORM_SRGB => Any::from("etc2-rgba8unorm-srgb"),
            GPUTextureFormat::EAC_R11UNORM => Any::from("eac-r11unorm"),
            GPUTextureFormat::EAC_R11SNORM => Any::from("eac-r11snorm"),
            GPUTextureFormat::EAC_RG11UNORM => Any::from("eac-rg11unorm"),
            GPUTextureFormat::EAC_RG11SNORM => Any::from("eac-rg11snorm"),
            GPUTextureFormat::ASTC_4X4_UNORM => Any::from("astc-4x4-unorm"),
            GPUTextureFormat::ASTC_4X4_UNORM_SRGB => Any::from("astc-4x4-unorm-srgb"),
            GPUTextureFormat::ASTC_5X4_UNORM => Any::from("astc-5x4-unorm"),
            GPUTextureFormat::ASTC_5X4_UNORM_SRGB => Any::from("astc-5x4-unorm-srgb"),
            GPUTextureFormat::ASTC_5X5_UNORM => Any::from("astc-5x5-unorm"),
            GPUTextureFormat::ASTC_5X5_UNORM_SRGB => Any::from("astc-5x5-unorm-srgb"),
            GPUTextureFormat::ASTC_6X5_UNORM => Any::from("astc-6x5-unorm"),
            GPUTextureFormat::ASTC_6X5_UNORM_SRGB => Any::from("astc-6x5-unorm-srgb"),
            GPUTextureFormat::ASTC_6X6_UNORM => Any::from("astc-6x6-unorm"),
            GPUTextureFormat::ASTC_6X6_UNORM_SRGB => Any::from("astc-6x6-unorm-srgb"),
            GPUTextureFormat::ASTC_8X5_UNORM => Any::from("astc-8x5-unorm"),
            GPUTextureFormat::ASTC_8X5_UNORM_SRGB => Any::from("astc-8x5-unorm-srgb"),
            GPUTextureFormat::ASTC_8X6_UNORM => Any::from("astc-8x6-unorm"),
            GPUTextureFormat::ASTC_8X6_UNORM_SRGB => Any::from("astc-8x6-unorm-srgb"),
            GPUTextureFormat::ASTC_8X8_UNORM => Any::from("astc-8x8-unorm"),
            GPUTextureFormat::ASTC_8X8_UNORM_SRGB => Any::from("astc-8x8-unorm-srgb"),
            GPUTextureFormat::ASTC_10X5_UNORM => Any::from("astc-10x5-unorm"),
            GPUTextureFormat::ASTC_10X5_UNORM_SRGB => Any::from("astc-10x5-unorm-srgb"),
            GPUTextureFormat::ASTC_10X6_UNORM => Any::from("astc-10x6-unorm"),
            GPUTextureFormat::ASTC_10X6_UNORM_SRGB => Any::from("astc-10x6-unorm-srgb"),
            GPUTextureFormat::ASTC_10X8_UNORM => Any::from("astc-10x8-unorm"),
            GPUTextureFormat::ASTC_10X8_UNORM_SRGB => Any::from("astc-10x8-unorm-srgb"),
            GPUTextureFormat::ASTC_10X10_UNORM => Any::from("astc-10x10-unorm"),
            GPUTextureFormat::ASTC_10X10_UNORM_SRGB => Any::from("astc-10x10-unorm-srgb"),
            GPUTextureFormat::ASTC_12X10_UNORM => Any::from("astc-12x10-unorm"),
            GPUTextureFormat::ASTC_12X10_UNORM_SRGB => Any::from("astc-12x10-unorm-srgb"),
            GPUTextureFormat::ASTC_12X12_UNORM => Any::from("astc-12x12-unorm"),
            GPUTextureFormat::ASTC_12X12_UNORM_SRGB => Any::from("astc-12x12-unorm-srgb"),
         }
    }
}
impl From<&GPUTextureFormat> for Any {
    fn from(s: &GPUTextureFormat) -> Any {
         match *s {
            GPUTextureFormat::R8UNORM => Any::from("r8unorm"),
            GPUTextureFormat::R8SNORM => Any::from("r8snorm"),
            GPUTextureFormat::R8UINT => Any::from("r8uint"),
            GPUTextureFormat::R8SINT => Any::from("r8sint"),
            GPUTextureFormat::R16UNORM => Any::from("r16unorm"),
            GPUTextureFormat::R16SNORM => Any::from("r16snorm"),
            GPUTextureFormat::R16UINT => Any::from("r16uint"),
            GPUTextureFormat::R16SINT => Any::from("r16sint"),
            GPUTextureFormat::R16FLOAT => Any::from("r16float"),
            GPUTextureFormat::RG8UNORM => Any::from("rg8unorm"),
            GPUTextureFormat::RG8SNORM => Any::from("rg8snorm"),
            GPUTextureFormat::RG8UINT => Any::from("rg8uint"),
            GPUTextureFormat::RG8SINT => Any::from("rg8sint"),
            GPUTextureFormat::R32UINT => Any::from("r32uint"),
            GPUTextureFormat::R32SINT => Any::from("r32sint"),
            GPUTextureFormat::R32FLOAT => Any::from("r32float"),
            GPUTextureFormat::RG16UNORM => Any::from("rg16unorm"),
            GPUTextureFormat::RG16SNORM => Any::from("rg16snorm"),
            GPUTextureFormat::RG16UINT => Any::from("rg16uint"),
            GPUTextureFormat::RG16SINT => Any::from("rg16sint"),
            GPUTextureFormat::RG16FLOAT => Any::from("rg16float"),
            GPUTextureFormat::RGBA8UNORM => Any::from("rgba8unorm"),
            GPUTextureFormat::RGBA8UNORM_SRGB => Any::from("rgba8unorm-srgb"),
            GPUTextureFormat::RGBA8SNORM => Any::from("rgba8snorm"),
            GPUTextureFormat::RGBA8UINT => Any::from("rgba8uint"),
            GPUTextureFormat::RGBA8SINT => Any::from("rgba8sint"),
            GPUTextureFormat::BGRA8UNORM => Any::from("bgra8unorm"),
            GPUTextureFormat::BGRA8UNORM_SRGB => Any::from("bgra8unorm-srgb"),
            GPUTextureFormat::RGB9E5UFLOAT => Any::from("rgb9e5ufloat"),
            GPUTextureFormat::RGB10A2UINT => Any::from("rgb10a2uint"),
            GPUTextureFormat::RGB10A2UNORM => Any::from("rgb10a2unorm"),
            GPUTextureFormat::RG11B10UFLOAT => Any::from("rg11b10ufloat"),
            GPUTextureFormat::RG32UINT => Any::from("rg32uint"),
            GPUTextureFormat::RG32SINT => Any::from("rg32sint"),
            GPUTextureFormat::RG32FLOAT => Any::from("rg32float"),
            GPUTextureFormat::RGBA16UNORM => Any::from("rgba16unorm"),
            GPUTextureFormat::RGBA16SNORM => Any::from("rgba16snorm"),
            GPUTextureFormat::RGBA16UINT => Any::from("rgba16uint"),
            GPUTextureFormat::RGBA16SINT => Any::from("rgba16sint"),
            GPUTextureFormat::RGBA16FLOAT => Any::from("rgba16float"),
            GPUTextureFormat::RGBA32UINT => Any::from("rgba32uint"),
            GPUTextureFormat::RGBA32SINT => Any::from("rgba32sint"),
            GPUTextureFormat::RGBA32FLOAT => Any::from("rgba32float"),
            GPUTextureFormat::STENCIL8 => Any::from("stencil8"),
            GPUTextureFormat::DEPTH16UNORM => Any::from("depth16unorm"),
            GPUTextureFormat::DEPTH24PLUS => Any::from("depth24plus"),
            GPUTextureFormat::DEPTH24PLUS_STENCIL8 => Any::from("depth24plus-stencil8"),
            GPUTextureFormat::DEPTH32FLOAT => Any::from("depth32float"),
            GPUTextureFormat::DEPTH32FLOAT_STENCIL8 => Any::from("depth32float-stencil8"),
            GPUTextureFormat::BC1_RGBA_UNORM => Any::from("bc1-rgba-unorm"),
            GPUTextureFormat::BC1_RGBA_UNORM_SRGB => Any::from("bc1-rgba-unorm-srgb"),
            GPUTextureFormat::BC2_RGBA_UNORM => Any::from("bc2-rgba-unorm"),
            GPUTextureFormat::BC2_RGBA_UNORM_SRGB => Any::from("bc2-rgba-unorm-srgb"),
            GPUTextureFormat::BC3_RGBA_UNORM => Any::from("bc3-rgba-unorm"),
            GPUTextureFormat::BC3_RGBA_UNORM_SRGB => Any::from("bc3-rgba-unorm-srgb"),
            GPUTextureFormat::BC4_R_UNORM => Any::from("bc4-r-unorm"),
            GPUTextureFormat::BC4_R_SNORM => Any::from("bc4-r-snorm"),
            GPUTextureFormat::BC5_RG_UNORM => Any::from("bc5-rg-unorm"),
            GPUTextureFormat::BC5_RG_SNORM => Any::from("bc5-rg-snorm"),
            GPUTextureFormat::BC6H_RGB_UFLOAT => Any::from("bc6h-rgb-ufloat"),
            GPUTextureFormat::BC6H_RGB_FLOAT => Any::from("bc6h-rgb-float"),
            GPUTextureFormat::BC7_RGBA_UNORM => Any::from("bc7-rgba-unorm"),
            GPUTextureFormat::BC7_RGBA_UNORM_SRGB => Any::from("bc7-rgba-unorm-srgb"),
            GPUTextureFormat::ETC2_RGB8UNORM => Any::from("etc2-rgb8unorm"),
            GPUTextureFormat::ETC2_RGB8UNORM_SRGB => Any::from("etc2-rgb8unorm-srgb"),
            GPUTextureFormat::ETC2_RGB8A1UNORM => Any::from("etc2-rgb8a1unorm"),
            GPUTextureFormat::ETC2_RGB8A1UNORM_SRGB => Any::from("etc2-rgb8a1unorm-srgb"),
            GPUTextureFormat::ETC2_RGBA8UNORM => Any::from("etc2-rgba8unorm"),
            GPUTextureFormat::ETC2_RGBA8UNORM_SRGB => Any::from("etc2-rgba8unorm-srgb"),
            GPUTextureFormat::EAC_R11UNORM => Any::from("eac-r11unorm"),
            GPUTextureFormat::EAC_R11SNORM => Any::from("eac-r11snorm"),
            GPUTextureFormat::EAC_RG11UNORM => Any::from("eac-rg11unorm"),
            GPUTextureFormat::EAC_RG11SNORM => Any::from("eac-rg11snorm"),
            GPUTextureFormat::ASTC_4X4_UNORM => Any::from("astc-4x4-unorm"),
            GPUTextureFormat::ASTC_4X4_UNORM_SRGB => Any::from("astc-4x4-unorm-srgb"),
            GPUTextureFormat::ASTC_5X4_UNORM => Any::from("astc-5x4-unorm"),
            GPUTextureFormat::ASTC_5X4_UNORM_SRGB => Any::from("astc-5x4-unorm-srgb"),
            GPUTextureFormat::ASTC_5X5_UNORM => Any::from("astc-5x5-unorm"),
            GPUTextureFormat::ASTC_5X5_UNORM_SRGB => Any::from("astc-5x5-unorm-srgb"),
            GPUTextureFormat::ASTC_6X5_UNORM => Any::from("astc-6x5-unorm"),
            GPUTextureFormat::ASTC_6X5_UNORM_SRGB => Any::from("astc-6x5-unorm-srgb"),
            GPUTextureFormat::ASTC_6X6_UNORM => Any::from("astc-6x6-unorm"),
            GPUTextureFormat::ASTC_6X6_UNORM_SRGB => Any::from("astc-6x6-unorm-srgb"),
            GPUTextureFormat::ASTC_8X5_UNORM => Any::from("astc-8x5-unorm"),
            GPUTextureFormat::ASTC_8X5_UNORM_SRGB => Any::from("astc-8x5-unorm-srgb"),
            GPUTextureFormat::ASTC_8X6_UNORM => Any::from("astc-8x6-unorm"),
            GPUTextureFormat::ASTC_8X6_UNORM_SRGB => Any::from("astc-8x6-unorm-srgb"),
            GPUTextureFormat::ASTC_8X8_UNORM => Any::from("astc-8x8-unorm"),
            GPUTextureFormat::ASTC_8X8_UNORM_SRGB => Any::from("astc-8x8-unorm-srgb"),
            GPUTextureFormat::ASTC_10X5_UNORM => Any::from("astc-10x5-unorm"),
            GPUTextureFormat::ASTC_10X5_UNORM_SRGB => Any::from("astc-10x5-unorm-srgb"),
            GPUTextureFormat::ASTC_10X6_UNORM => Any::from("astc-10x6-unorm"),
            GPUTextureFormat::ASTC_10X6_UNORM_SRGB => Any::from("astc-10x6-unorm-srgb"),
            GPUTextureFormat::ASTC_10X8_UNORM => Any::from("astc-10x8-unorm"),
            GPUTextureFormat::ASTC_10X8_UNORM_SRGB => Any::from("astc-10x8-unorm-srgb"),
            GPUTextureFormat::ASTC_10X10_UNORM => Any::from("astc-10x10-unorm"),
            GPUTextureFormat::ASTC_10X10_UNORM_SRGB => Any::from("astc-10x10-unorm-srgb"),
            GPUTextureFormat::ASTC_12X10_UNORM => Any::from("astc-12x10-unorm"),
            GPUTextureFormat::ASTC_12X10_UNORM_SRGB => Any::from("astc-12x10-unorm-srgb"),
            GPUTextureFormat::ASTC_12X12_UNORM => Any::from("astc-12x12-unorm"),
            GPUTextureFormat::ASTC_12X12_UNORM_SRGB => Any::from("astc-12x12-unorm-srgb"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "clamp-to-edge" => Self::CLAMP_TO_EDGE,
            "repeat" => Self::REPEAT,
            "mirror-repeat" => Self::MIRROR_REPEAT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUAddressMode> for Any {
    fn from(s: GPUAddressMode) -> Any {
         match s {
            GPUAddressMode::CLAMP_TO_EDGE => Any::from("clamp-to-edge"),
            GPUAddressMode::REPEAT => Any::from("repeat"),
            GPUAddressMode::MIRROR_REPEAT => Any::from("mirror-repeat"),
         }
    }
}
impl From<&GPUAddressMode> for Any {
    fn from(s: &GPUAddressMode) -> Any {
         match *s {
            GPUAddressMode::CLAMP_TO_EDGE => Any::from("clamp-to-edge"),
            GPUAddressMode::REPEAT => Any::from("repeat"),
            GPUAddressMode::MIRROR_REPEAT => Any::from("mirror-repeat"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUFilterMode {
    NEAREST,
    LINEAR,
}
impl FromVal for GPUFilterMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "nearest" => Self::NEAREST,
            "linear" => Self::LINEAR,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUFilterMode> for Any {
    fn from(s: GPUFilterMode) -> Any {
         match s {
            GPUFilterMode::NEAREST => Any::from("nearest"),
            GPUFilterMode::LINEAR => Any::from("linear"),
         }
    }
}
impl From<&GPUFilterMode> for Any {
    fn from(s: &GPUFilterMode) -> Any {
         match *s {
            GPUFilterMode::NEAREST => Any::from("nearest"),
            GPUFilterMode::LINEAR => Any::from("linear"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUMipmapFilterMode {
    NEAREST,
    LINEAR,
}
impl FromVal for GPUMipmapFilterMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "nearest" => Self::NEAREST,
            "linear" => Self::LINEAR,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUMipmapFilterMode> for Any {
    fn from(s: GPUMipmapFilterMode) -> Any {
         match s {
            GPUMipmapFilterMode::NEAREST => Any::from("nearest"),
            GPUMipmapFilterMode::LINEAR => Any::from("linear"),
         }
    }
}
impl From<&GPUMipmapFilterMode> for Any {
    fn from(s: &GPUMipmapFilterMode) -> Any {
         match *s {
            GPUMipmapFilterMode::NEAREST => Any::from("nearest"),
            GPUMipmapFilterMode::LINEAR => Any::from("linear"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUCompareFunction> for Any {
    fn from(s: GPUCompareFunction) -> Any {
         match s {
            GPUCompareFunction::NEVER => Any::from("never"),
            GPUCompareFunction::LESS => Any::from("less"),
            GPUCompareFunction::EQUAL => Any::from("equal"),
            GPUCompareFunction::LESS_EQUAL => Any::from("less-equal"),
            GPUCompareFunction::GREATER => Any::from("greater"),
            GPUCompareFunction::NOT_EQUAL => Any::from("not-equal"),
            GPUCompareFunction::GREATER_EQUAL => Any::from("greater-equal"),
            GPUCompareFunction::ALWAYS => Any::from("always"),
         }
    }
}
impl From<&GPUCompareFunction> for Any {
    fn from(s: &GPUCompareFunction) -> Any {
         match *s {
            GPUCompareFunction::NEVER => Any::from("never"),
            GPUCompareFunction::LESS => Any::from("less"),
            GPUCompareFunction::EQUAL => Any::from("equal"),
            GPUCompareFunction::LESS_EQUAL => Any::from("less-equal"),
            GPUCompareFunction::GREATER => Any::from("greater"),
            GPUCompareFunction::NOT_EQUAL => Any::from("not-equal"),
            GPUCompareFunction::GREATER_EQUAL => Any::from("greater-equal"),
            GPUCompareFunction::ALWAYS => Any::from("always"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "uniform" => Self::UNIFORM,
            "storage" => Self::STORAGE,
            "read-only-storage" => Self::READ_ONLY_STORAGE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUBufferBindingType> for Any {
    fn from(s: GPUBufferBindingType) -> Any {
         match s {
            GPUBufferBindingType::UNIFORM => Any::from("uniform"),
            GPUBufferBindingType::STORAGE => Any::from("storage"),
            GPUBufferBindingType::READ_ONLY_STORAGE => Any::from("read-only-storage"),
         }
    }
}
impl From<&GPUBufferBindingType> for Any {
    fn from(s: &GPUBufferBindingType) -> Any {
         match *s {
            GPUBufferBindingType::UNIFORM => Any::from("uniform"),
            GPUBufferBindingType::STORAGE => Any::from("storage"),
            GPUBufferBindingType::READ_ONLY_STORAGE => Any::from("read-only-storage"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "filtering" => Self::FILTERING,
            "non-filtering" => Self::NON_FILTERING,
            "comparison" => Self::COMPARISON,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUSamplerBindingType> for Any {
    fn from(s: GPUSamplerBindingType) -> Any {
         match s {
            GPUSamplerBindingType::FILTERING => Any::from("filtering"),
            GPUSamplerBindingType::NON_FILTERING => Any::from("non-filtering"),
            GPUSamplerBindingType::COMPARISON => Any::from("comparison"),
         }
    }
}
impl From<&GPUSamplerBindingType> for Any {
    fn from(s: &GPUSamplerBindingType) -> Any {
         match *s {
            GPUSamplerBindingType::FILTERING => Any::from("filtering"),
            GPUSamplerBindingType::NON_FILTERING => Any::from("non-filtering"),
            GPUSamplerBindingType::COMPARISON => Any::from("comparison"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "float" => Self::FLOAT,
            "unfilterable-float" => Self::UNFILTERABLE_FLOAT,
            "depth" => Self::DEPTH,
            "sint" => Self::SINT,
            "uint" => Self::UINT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUTextureSampleType> for Any {
    fn from(s: GPUTextureSampleType) -> Any {
         match s {
            GPUTextureSampleType::FLOAT => Any::from("float"),
            GPUTextureSampleType::UNFILTERABLE_FLOAT => Any::from("unfilterable-float"),
            GPUTextureSampleType::DEPTH => Any::from("depth"),
            GPUTextureSampleType::SINT => Any::from("sint"),
            GPUTextureSampleType::UINT => Any::from("uint"),
         }
    }
}
impl From<&GPUTextureSampleType> for Any {
    fn from(s: &GPUTextureSampleType) -> Any {
         match *s {
            GPUTextureSampleType::FLOAT => Any::from("float"),
            GPUTextureSampleType::UNFILTERABLE_FLOAT => Any::from("unfilterable-float"),
            GPUTextureSampleType::DEPTH => Any::from("depth"),
            GPUTextureSampleType::SINT => Any::from("sint"),
            GPUTextureSampleType::UINT => Any::from("uint"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "write-only" => Self::WRITE_ONLY,
            "read-only" => Self::READ_ONLY,
            "read-write" => Self::READ_WRITE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUStorageTextureAccess> for Any {
    fn from(s: GPUStorageTextureAccess) -> Any {
         match s {
            GPUStorageTextureAccess::WRITE_ONLY => Any::from("write-only"),
            GPUStorageTextureAccess::READ_ONLY => Any::from("read-only"),
            GPUStorageTextureAccess::READ_WRITE => Any::from("read-write"),
         }
    }
}
impl From<&GPUStorageTextureAccess> for Any {
    fn from(s: &GPUStorageTextureAccess) -> Any {
         match *s {
            GPUStorageTextureAccess::WRITE_ONLY => Any::from("write-only"),
            GPUStorageTextureAccess::READ_ONLY => Any::from("read-only"),
            GPUStorageTextureAccess::READ_WRITE => Any::from("read-write"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "error" => Self::ERROR,
            "warning" => Self::WARNING,
            "info" => Self::INFO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUCompilationMessageType> for Any {
    fn from(s: GPUCompilationMessageType) -> Any {
         match s {
            GPUCompilationMessageType::ERROR => Any::from("error"),
            GPUCompilationMessageType::WARNING => Any::from("warning"),
            GPUCompilationMessageType::INFO => Any::from("info"),
         }
    }
}
impl From<&GPUCompilationMessageType> for Any {
    fn from(s: &GPUCompilationMessageType) -> Any {
         match *s {
            GPUCompilationMessageType::ERROR => Any::from("error"),
            GPUCompilationMessageType::WARNING => Any::from("warning"),
            GPUCompilationMessageType::INFO => Any::from("info"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUPipelineErrorReason {
    VALIDATION,
    INTERNAL,
}
impl FromVal for GPUPipelineErrorReason {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "validation" => Self::VALIDATION,
            "internal" => Self::INTERNAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUPipelineErrorReason> for Any {
    fn from(s: GPUPipelineErrorReason) -> Any {
         match s {
            GPUPipelineErrorReason::VALIDATION => Any::from("validation"),
            GPUPipelineErrorReason::INTERNAL => Any::from("internal"),
         }
    }
}
impl From<&GPUPipelineErrorReason> for Any {
    fn from(s: &GPUPipelineErrorReason) -> Any {
         match *s {
            GPUPipelineErrorReason::VALIDATION => Any::from("validation"),
            GPUPipelineErrorReason::INTERNAL => Any::from("internal"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUAutoLayoutMode {
    AUTO,
}
impl FromVal for GPUAutoLayoutMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUAutoLayoutMode> for Any {
    fn from(s: GPUAutoLayoutMode) -> Any {
         match s {
            GPUAutoLayoutMode::AUTO => Any::from("auto"),
         }
    }
}
impl From<&GPUAutoLayoutMode> for Any {
    fn from(s: &GPUAutoLayoutMode) -> Any {
         match *s {
            GPUAutoLayoutMode::AUTO => Any::from("auto"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "point-list" => Self::POINT_LIST,
            "line-list" => Self::LINE_LIST,
            "line-strip" => Self::LINE_STRIP,
            "triangle-list" => Self::TRIANGLE_LIST,
            "triangle-strip" => Self::TRIANGLE_STRIP,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUPrimitiveTopology> for Any {
    fn from(s: GPUPrimitiveTopology) -> Any {
         match s {
            GPUPrimitiveTopology::POINT_LIST => Any::from("point-list"),
            GPUPrimitiveTopology::LINE_LIST => Any::from("line-list"),
            GPUPrimitiveTopology::LINE_STRIP => Any::from("line-strip"),
            GPUPrimitiveTopology::TRIANGLE_LIST => Any::from("triangle-list"),
            GPUPrimitiveTopology::TRIANGLE_STRIP => Any::from("triangle-strip"),
         }
    }
}
impl From<&GPUPrimitiveTopology> for Any {
    fn from(s: &GPUPrimitiveTopology) -> Any {
         match *s {
            GPUPrimitiveTopology::POINT_LIST => Any::from("point-list"),
            GPUPrimitiveTopology::LINE_LIST => Any::from("line-list"),
            GPUPrimitiveTopology::LINE_STRIP => Any::from("line-strip"),
            GPUPrimitiveTopology::TRIANGLE_LIST => Any::from("triangle-list"),
            GPUPrimitiveTopology::TRIANGLE_STRIP => Any::from("triangle-strip"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUFrontFace {
    CCW,
    CW,
}
impl FromVal for GPUFrontFace {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "ccw" => Self::CCW,
            "cw" => Self::CW,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUFrontFace> for Any {
    fn from(s: GPUFrontFace) -> Any {
         match s {
            GPUFrontFace::CCW => Any::from("ccw"),
            GPUFrontFace::CW => Any::from("cw"),
         }
    }
}
impl From<&GPUFrontFace> for Any {
    fn from(s: &GPUFrontFace) -> Any {
         match *s {
            GPUFrontFace::CCW => Any::from("ccw"),
            GPUFrontFace::CW => Any::from("cw"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "front" => Self::FRONT,
            "back" => Self::BACK,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUCullMode> for Any {
    fn from(s: GPUCullMode) -> Any {
         match s {
            GPUCullMode::NONE => Any::from("none"),
            GPUCullMode::FRONT => Any::from("front"),
            GPUCullMode::BACK => Any::from("back"),
         }
    }
}
impl From<&GPUCullMode> for Any {
    fn from(s: &GPUCullMode) -> Any {
         match *s {
            GPUCullMode::NONE => Any::from("none"),
            GPUCullMode::FRONT => Any::from("front"),
            GPUCullMode::BACK => Any::from("back"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUBlendFactor> for Any {
    fn from(s: GPUBlendFactor) -> Any {
         match s {
            GPUBlendFactor::ZERO => Any::from("zero"),
            GPUBlendFactor::ONE => Any::from("one"),
            GPUBlendFactor::SRC => Any::from("src"),
            GPUBlendFactor::ONE_MINUS_SRC => Any::from("one-minus-src"),
            GPUBlendFactor::SRC_ALPHA => Any::from("src-alpha"),
            GPUBlendFactor::ONE_MINUS_SRC_ALPHA => Any::from("one-minus-src-alpha"),
            GPUBlendFactor::DST => Any::from("dst"),
            GPUBlendFactor::ONE_MINUS_DST => Any::from("one-minus-dst"),
            GPUBlendFactor::DST_ALPHA => Any::from("dst-alpha"),
            GPUBlendFactor::ONE_MINUS_DST_ALPHA => Any::from("one-minus-dst-alpha"),
            GPUBlendFactor::SRC_ALPHA_SATURATED => Any::from("src-alpha-saturated"),
            GPUBlendFactor::CONSTANT => Any::from("constant"),
            GPUBlendFactor::ONE_MINUS_CONSTANT => Any::from("one-minus-constant"),
            GPUBlendFactor::SRC1 => Any::from("src1"),
            GPUBlendFactor::ONE_MINUS_SRC1 => Any::from("one-minus-src1"),
            GPUBlendFactor::SRC1_ALPHA => Any::from("src1-alpha"),
            GPUBlendFactor::ONE_MINUS_SRC1_ALPHA => Any::from("one-minus-src1-alpha"),
         }
    }
}
impl From<&GPUBlendFactor> for Any {
    fn from(s: &GPUBlendFactor) -> Any {
         match *s {
            GPUBlendFactor::ZERO => Any::from("zero"),
            GPUBlendFactor::ONE => Any::from("one"),
            GPUBlendFactor::SRC => Any::from("src"),
            GPUBlendFactor::ONE_MINUS_SRC => Any::from("one-minus-src"),
            GPUBlendFactor::SRC_ALPHA => Any::from("src-alpha"),
            GPUBlendFactor::ONE_MINUS_SRC_ALPHA => Any::from("one-minus-src-alpha"),
            GPUBlendFactor::DST => Any::from("dst"),
            GPUBlendFactor::ONE_MINUS_DST => Any::from("one-minus-dst"),
            GPUBlendFactor::DST_ALPHA => Any::from("dst-alpha"),
            GPUBlendFactor::ONE_MINUS_DST_ALPHA => Any::from("one-minus-dst-alpha"),
            GPUBlendFactor::SRC_ALPHA_SATURATED => Any::from("src-alpha-saturated"),
            GPUBlendFactor::CONSTANT => Any::from("constant"),
            GPUBlendFactor::ONE_MINUS_CONSTANT => Any::from("one-minus-constant"),
            GPUBlendFactor::SRC1 => Any::from("src1"),
            GPUBlendFactor::ONE_MINUS_SRC1 => Any::from("one-minus-src1"),
            GPUBlendFactor::SRC1_ALPHA => Any::from("src1-alpha"),
            GPUBlendFactor::ONE_MINUS_SRC1_ALPHA => Any::from("one-minus-src1-alpha"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "add" => Self::ADD,
            "subtract" => Self::SUBTRACT,
            "reverse-subtract" => Self::REVERSE_SUBTRACT,
            "min" => Self::MIN,
            "max" => Self::MAX,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUBlendOperation> for Any {
    fn from(s: GPUBlendOperation) -> Any {
         match s {
            GPUBlendOperation::ADD => Any::from("add"),
            GPUBlendOperation::SUBTRACT => Any::from("subtract"),
            GPUBlendOperation::REVERSE_SUBTRACT => Any::from("reverse-subtract"),
            GPUBlendOperation::MIN => Any::from("min"),
            GPUBlendOperation::MAX => Any::from("max"),
         }
    }
}
impl From<&GPUBlendOperation> for Any {
    fn from(s: &GPUBlendOperation) -> Any {
         match *s {
            GPUBlendOperation::ADD => Any::from("add"),
            GPUBlendOperation::SUBTRACT => Any::from("subtract"),
            GPUBlendOperation::REVERSE_SUBTRACT => Any::from("reverse-subtract"),
            GPUBlendOperation::MIN => Any::from("min"),
            GPUBlendOperation::MAX => Any::from("max"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUStencilOperation> for Any {
    fn from(s: GPUStencilOperation) -> Any {
         match s {
            GPUStencilOperation::KEEP => Any::from("keep"),
            GPUStencilOperation::ZERO => Any::from("zero"),
            GPUStencilOperation::REPLACE => Any::from("replace"),
            GPUStencilOperation::INVERT => Any::from("invert"),
            GPUStencilOperation::INCREMENT_CLAMP => Any::from("increment-clamp"),
            GPUStencilOperation::DECREMENT_CLAMP => Any::from("decrement-clamp"),
            GPUStencilOperation::INCREMENT_WRAP => Any::from("increment-wrap"),
            GPUStencilOperation::DECREMENT_WRAP => Any::from("decrement-wrap"),
         }
    }
}
impl From<&GPUStencilOperation> for Any {
    fn from(s: &GPUStencilOperation) -> Any {
         match *s {
            GPUStencilOperation::KEEP => Any::from("keep"),
            GPUStencilOperation::ZERO => Any::from("zero"),
            GPUStencilOperation::REPLACE => Any::from("replace"),
            GPUStencilOperation::INVERT => Any::from("invert"),
            GPUStencilOperation::INCREMENT_CLAMP => Any::from("increment-clamp"),
            GPUStencilOperation::DECREMENT_CLAMP => Any::from("decrement-clamp"),
            GPUStencilOperation::INCREMENT_WRAP => Any::from("increment-wrap"),
            GPUStencilOperation::DECREMENT_WRAP => Any::from("decrement-wrap"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUIndexFormat {
    UINT16,
    UINT32,
}
impl FromVal for GPUIndexFormat {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "uint16" => Self::UINT16,
            "uint32" => Self::UINT32,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUIndexFormat> for Any {
    fn from(s: GPUIndexFormat) -> Any {
         match s {
            GPUIndexFormat::UINT16 => Any::from("uint16"),
            GPUIndexFormat::UINT32 => Any::from("uint32"),
         }
    }
}
impl From<&GPUIndexFormat> for Any {
    fn from(s: &GPUIndexFormat) -> Any {
         match *s {
            GPUIndexFormat::UINT16 => Any::from("uint16"),
            GPUIndexFormat::UINT32 => Any::from("uint32"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUVertexFormat> for Any {
    fn from(s: GPUVertexFormat) -> Any {
         match s {
            GPUVertexFormat::UINT8 => Any::from("uint8"),
            GPUVertexFormat::UINT8X2 => Any::from("uint8x2"),
            GPUVertexFormat::UINT8X4 => Any::from("uint8x4"),
            GPUVertexFormat::SINT8 => Any::from("sint8"),
            GPUVertexFormat::SINT8X2 => Any::from("sint8x2"),
            GPUVertexFormat::SINT8X4 => Any::from("sint8x4"),
            GPUVertexFormat::UNORM8 => Any::from("unorm8"),
            GPUVertexFormat::UNORM8X2 => Any::from("unorm8x2"),
            GPUVertexFormat::UNORM8X4 => Any::from("unorm8x4"),
            GPUVertexFormat::SNORM8 => Any::from("snorm8"),
            GPUVertexFormat::SNORM8X2 => Any::from("snorm8x2"),
            GPUVertexFormat::SNORM8X4 => Any::from("snorm8x4"),
            GPUVertexFormat::UINT16 => Any::from("uint16"),
            GPUVertexFormat::UINT16X2 => Any::from("uint16x2"),
            GPUVertexFormat::UINT16X4 => Any::from("uint16x4"),
            GPUVertexFormat::SINT16 => Any::from("sint16"),
            GPUVertexFormat::SINT16X2 => Any::from("sint16x2"),
            GPUVertexFormat::SINT16X4 => Any::from("sint16x4"),
            GPUVertexFormat::UNORM16 => Any::from("unorm16"),
            GPUVertexFormat::UNORM16X2 => Any::from("unorm16x2"),
            GPUVertexFormat::UNORM16X4 => Any::from("unorm16x4"),
            GPUVertexFormat::SNORM16 => Any::from("snorm16"),
            GPUVertexFormat::SNORM16X2 => Any::from("snorm16x2"),
            GPUVertexFormat::SNORM16X4 => Any::from("snorm16x4"),
            GPUVertexFormat::FLOAT16 => Any::from("float16"),
            GPUVertexFormat::FLOAT16X2 => Any::from("float16x2"),
            GPUVertexFormat::FLOAT16X4 => Any::from("float16x4"),
            GPUVertexFormat::FLOAT32 => Any::from("float32"),
            GPUVertexFormat::FLOAT32X2 => Any::from("float32x2"),
            GPUVertexFormat::FLOAT32X3 => Any::from("float32x3"),
            GPUVertexFormat::FLOAT32X4 => Any::from("float32x4"),
            GPUVertexFormat::UINT32 => Any::from("uint32"),
            GPUVertexFormat::UINT32X2 => Any::from("uint32x2"),
            GPUVertexFormat::UINT32X3 => Any::from("uint32x3"),
            GPUVertexFormat::UINT32X4 => Any::from("uint32x4"),
            GPUVertexFormat::SINT32 => Any::from("sint32"),
            GPUVertexFormat::SINT32X2 => Any::from("sint32x2"),
            GPUVertexFormat::SINT32X3 => Any::from("sint32x3"),
            GPUVertexFormat::SINT32X4 => Any::from("sint32x4"),
            GPUVertexFormat::UNORM10_10_10_2 => Any::from("unorm10-10-10-2"),
            GPUVertexFormat::UNORM8X4_BGRA => Any::from("unorm8x4-bgra"),
         }
    }
}
impl From<&GPUVertexFormat> for Any {
    fn from(s: &GPUVertexFormat) -> Any {
         match *s {
            GPUVertexFormat::UINT8 => Any::from("uint8"),
            GPUVertexFormat::UINT8X2 => Any::from("uint8x2"),
            GPUVertexFormat::UINT8X4 => Any::from("uint8x4"),
            GPUVertexFormat::SINT8 => Any::from("sint8"),
            GPUVertexFormat::SINT8X2 => Any::from("sint8x2"),
            GPUVertexFormat::SINT8X4 => Any::from("sint8x4"),
            GPUVertexFormat::UNORM8 => Any::from("unorm8"),
            GPUVertexFormat::UNORM8X2 => Any::from("unorm8x2"),
            GPUVertexFormat::UNORM8X4 => Any::from("unorm8x4"),
            GPUVertexFormat::SNORM8 => Any::from("snorm8"),
            GPUVertexFormat::SNORM8X2 => Any::from("snorm8x2"),
            GPUVertexFormat::SNORM8X4 => Any::from("snorm8x4"),
            GPUVertexFormat::UINT16 => Any::from("uint16"),
            GPUVertexFormat::UINT16X2 => Any::from("uint16x2"),
            GPUVertexFormat::UINT16X4 => Any::from("uint16x4"),
            GPUVertexFormat::SINT16 => Any::from("sint16"),
            GPUVertexFormat::SINT16X2 => Any::from("sint16x2"),
            GPUVertexFormat::SINT16X4 => Any::from("sint16x4"),
            GPUVertexFormat::UNORM16 => Any::from("unorm16"),
            GPUVertexFormat::UNORM16X2 => Any::from("unorm16x2"),
            GPUVertexFormat::UNORM16X4 => Any::from("unorm16x4"),
            GPUVertexFormat::SNORM16 => Any::from("snorm16"),
            GPUVertexFormat::SNORM16X2 => Any::from("snorm16x2"),
            GPUVertexFormat::SNORM16X4 => Any::from("snorm16x4"),
            GPUVertexFormat::FLOAT16 => Any::from("float16"),
            GPUVertexFormat::FLOAT16X2 => Any::from("float16x2"),
            GPUVertexFormat::FLOAT16X4 => Any::from("float16x4"),
            GPUVertexFormat::FLOAT32 => Any::from("float32"),
            GPUVertexFormat::FLOAT32X2 => Any::from("float32x2"),
            GPUVertexFormat::FLOAT32X3 => Any::from("float32x3"),
            GPUVertexFormat::FLOAT32X4 => Any::from("float32x4"),
            GPUVertexFormat::UINT32 => Any::from("uint32"),
            GPUVertexFormat::UINT32X2 => Any::from("uint32x2"),
            GPUVertexFormat::UINT32X3 => Any::from("uint32x3"),
            GPUVertexFormat::UINT32X4 => Any::from("uint32x4"),
            GPUVertexFormat::SINT32 => Any::from("sint32"),
            GPUVertexFormat::SINT32X2 => Any::from("sint32x2"),
            GPUVertexFormat::SINT32X3 => Any::from("sint32x3"),
            GPUVertexFormat::SINT32X4 => Any::from("sint32x4"),
            GPUVertexFormat::UNORM10_10_10_2 => Any::from("unorm10-10-10-2"),
            GPUVertexFormat::UNORM8X4_BGRA => Any::from("unorm8x4-bgra"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUVertexStepMode {
    VERTEX,
    INSTANCE,
}
impl FromVal for GPUVertexStepMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "vertex" => Self::VERTEX,
            "instance" => Self::INSTANCE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUVertexStepMode> for Any {
    fn from(s: GPUVertexStepMode) -> Any {
         match s {
            GPUVertexStepMode::VERTEX => Any::from("vertex"),
            GPUVertexStepMode::INSTANCE => Any::from("instance"),
         }
    }
}
impl From<&GPUVertexStepMode> for Any {
    fn from(s: &GPUVertexStepMode) -> Any {
         match *s {
            GPUVertexStepMode::VERTEX => Any::from("vertex"),
            GPUVertexStepMode::INSTANCE => Any::from("instance"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPULoadOp {
    LOAD,
    CLEAR,
}
impl FromVal for GPULoadOp {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "load" => Self::LOAD,
            "clear" => Self::CLEAR,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPULoadOp> for Any {
    fn from(s: GPULoadOp) -> Any {
         match s {
            GPULoadOp::LOAD => Any::from("load"),
            GPULoadOp::CLEAR => Any::from("clear"),
         }
    }
}
impl From<&GPULoadOp> for Any {
    fn from(s: &GPULoadOp) -> Any {
         match *s {
            GPULoadOp::LOAD => Any::from("load"),
            GPULoadOp::CLEAR => Any::from("clear"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUStoreOp {
    STORE,
    DISCARD,
}
impl FromVal for GPUStoreOp {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "store" => Self::STORE,
            "discard" => Self::DISCARD,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUStoreOp> for Any {
    fn from(s: GPUStoreOp) -> Any {
         match s {
            GPUStoreOp::STORE => Any::from("store"),
            GPUStoreOp::DISCARD => Any::from("discard"),
         }
    }
}
impl From<&GPUStoreOp> for Any {
    fn from(s: &GPUStoreOp) -> Any {
         match *s {
            GPUStoreOp::STORE => Any::from("store"),
            GPUStoreOp::DISCARD => Any::from("discard"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUQueryType {
    OCCLUSION,
    TIMESTAMP,
}
impl FromVal for GPUQueryType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "occlusion" => Self::OCCLUSION,
            "timestamp" => Self::TIMESTAMP,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUQueryType> for Any {
    fn from(s: GPUQueryType) -> Any {
         match s {
            GPUQueryType::OCCLUSION => Any::from("occlusion"),
            GPUQueryType::TIMESTAMP => Any::from("timestamp"),
         }
    }
}
impl From<&GPUQueryType> for Any {
    fn from(s: &GPUQueryType) -> Any {
         match *s {
            GPUQueryType::OCCLUSION => Any::from("occlusion"),
            GPUQueryType::TIMESTAMP => Any::from("timestamp"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUCanvasAlphaMode {
    OPAQUE,
    PREMULTIPLIED,
}
impl FromVal for GPUCanvasAlphaMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "opaque" => Self::OPAQUE,
            "premultiplied" => Self::PREMULTIPLIED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUCanvasAlphaMode> for Any {
    fn from(s: GPUCanvasAlphaMode) -> Any {
         match s {
            GPUCanvasAlphaMode::OPAQUE => Any::from("opaque"),
            GPUCanvasAlphaMode::PREMULTIPLIED => Any::from("premultiplied"),
         }
    }
}
impl From<&GPUCanvasAlphaMode> for Any {
    fn from(s: &GPUCanvasAlphaMode) -> Any {
         match *s {
            GPUCanvasAlphaMode::OPAQUE => Any::from("opaque"),
            GPUCanvasAlphaMode::PREMULTIPLIED => Any::from("premultiplied"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUCanvasToneMappingMode {
    STANDARD,
    EXTENDED,
}
impl FromVal for GPUCanvasToneMappingMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "standard" => Self::STANDARD,
            "extended" => Self::EXTENDED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUCanvasToneMappingMode> for Any {
    fn from(s: GPUCanvasToneMappingMode) -> Any {
         match s {
            GPUCanvasToneMappingMode::STANDARD => Any::from("standard"),
            GPUCanvasToneMappingMode::EXTENDED => Any::from("extended"),
         }
    }
}
impl From<&GPUCanvasToneMappingMode> for Any {
    fn from(s: &GPUCanvasToneMappingMode) -> Any {
         match *s {
            GPUCanvasToneMappingMode::STANDARD => Any::from("standard"),
            GPUCanvasToneMappingMode::EXTENDED => Any::from("extended"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum GPUDeviceLostReason {
    UNKNOWN,
    DESTROYED,
}
impl FromVal for GPUDeviceLostReason {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "unknown" => Self::UNKNOWN,
            "destroyed" => Self::DESTROYED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUDeviceLostReason> for Any {
    fn from(s: GPUDeviceLostReason) -> Any {
         match s {
            GPUDeviceLostReason::UNKNOWN => Any::from("unknown"),
            GPUDeviceLostReason::DESTROYED => Any::from("destroyed"),
         }
    }
}
impl From<&GPUDeviceLostReason> for Any {
    fn from(s: &GPUDeviceLostReason) -> Any {
         match *s {
            GPUDeviceLostReason::UNKNOWN => Any::from("unknown"),
            GPUDeviceLostReason::DESTROYED => Any::from("destroyed"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "validation" => Self::VALIDATION,
            "out-of-memory" => Self::OUT_OF_MEMORY,
            "internal" => Self::INTERNAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<GPUErrorFilter> for Any {
    fn from(s: GPUErrorFilter) -> Any {
         match s {
            GPUErrorFilter::VALIDATION => Any::from("validation"),
            GPUErrorFilter::OUT_OF_MEMORY => Any::from("out-of-memory"),
            GPUErrorFilter::INTERNAL => Any::from("internal"),
         }
    }
}
impl From<&GPUErrorFilter> for Any {
    fn from(s: &GPUErrorFilter) -> Any {
         match *s {
            GPUErrorFilter::VALIDATION => Any::from("validation"),
            GPUErrorFilter::OUT_OF_MEMORY => Any::from("out-of-memory"),
            GPUErrorFilter::INTERNAL => Any::from("internal"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<HIDUnitSystem> for Any {
    fn from(s: HIDUnitSystem) -> Any {
         match s {
            HIDUnitSystem::NONE => Any::from("none"),
            HIDUnitSystem::SI_LINEAR => Any::from("si-linear"),
            HIDUnitSystem::SI_ROTATION => Any::from("si-rotation"),
            HIDUnitSystem::ENGLISH_LINEAR => Any::from("english-linear"),
            HIDUnitSystem::ENGLISH_ROTATION => Any::from("english-rotation"),
            HIDUnitSystem::VENDOR_DEFINED => Any::from("vendor-defined"),
            HIDUnitSystem::RESERVED => Any::from("reserved"),
         }
    }
}
impl From<&HIDUnitSystem> for Any {
    fn from(s: &HIDUnitSystem) -> Any {
         match *s {
            HIDUnitSystem::NONE => Any::from("none"),
            HIDUnitSystem::SI_LINEAR => Any::from("si-linear"),
            HIDUnitSystem::SI_ROTATION => Any::from("si-rotation"),
            HIDUnitSystem::ENGLISH_LINEAR => Any::from("english-linear"),
            HIDUnitSystem::ENGLISH_ROTATION => Any::from("english-rotation"),
            HIDUnitSystem::VENDOR_DEFINED => Any::from("vendor-defined"),
            HIDUnitSystem::RESERVED => Any::from("reserved"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MIDIPortType {
    INPUT,
    OUTPUT,
}
impl FromVal for MIDIPortType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "input" => Self::INPUT,
            "output" => Self::OUTPUT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MIDIPortType> for Any {
    fn from(s: MIDIPortType) -> Any {
         match s {
            MIDIPortType::INPUT => Any::from("input"),
            MIDIPortType::OUTPUT => Any::from("output"),
         }
    }
}
impl From<&MIDIPortType> for Any {
    fn from(s: &MIDIPortType) -> Any {
         match *s {
            MIDIPortType::INPUT => Any::from("input"),
            MIDIPortType::OUTPUT => Any::from("output"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MIDIPortDeviceState {
    DISCONNECTED,
    CONNECTED,
}
impl FromVal for MIDIPortDeviceState {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "disconnected" => Self::DISCONNECTED,
            "connected" => Self::CONNECTED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MIDIPortDeviceState> for Any {
    fn from(s: MIDIPortDeviceState) -> Any {
         match s {
            MIDIPortDeviceState::DISCONNECTED => Any::from("disconnected"),
            MIDIPortDeviceState::CONNECTED => Any::from("connected"),
         }
    }
}
impl From<&MIDIPortDeviceState> for Any {
    fn from(s: &MIDIPortDeviceState) -> Any {
         match *s {
            MIDIPortDeviceState::DISCONNECTED => Any::from("disconnected"),
            MIDIPortDeviceState::CONNECTED => Any::from("connected"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "open" => Self::OPEN,
            "closed" => Self::CLOSED,
            "pending" => Self::PENDING,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MIDIPortConnectionState> for Any {
    fn from(s: MIDIPortConnectionState) -> Any {
         match s {
            MIDIPortConnectionState::OPEN => Any::from("open"),
            MIDIPortConnectionState::CLOSED => Any::from("closed"),
            MIDIPortConnectionState::PENDING => Any::from("pending"),
         }
    }
}
impl From<&MIDIPortConnectionState> for Any {
    fn from(s: &MIDIPortConnectionState) -> Any {
         match *s {
            MIDIPortConnectionState::OPEN => Any::from("open"),
            MIDIPortConnectionState::CLOSED => Any::from("closed"),
            MIDIPortConnectionState::PENDING => Any::from("pending"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "default" => Self::DEFAULT,
            "high-performance" => Self::HIGH_PERFORMANCE,
            "low-power" => Self::LOW_POWER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLPowerPreference> for Any {
    fn from(s: MLPowerPreference) -> Any {
         match s {
            MLPowerPreference::DEFAULT => Any::from("default"),
            MLPowerPreference::HIGH_PERFORMANCE => Any::from("high-performance"),
            MLPowerPreference::LOW_POWER => Any::from("low-power"),
         }
    }
}
impl From<&MLPowerPreference> for Any {
    fn from(s: &MLPowerPreference) -> Any {
         match *s {
            MLPowerPreference::DEFAULT => Any::from("default"),
            MLPowerPreference::HIGH_PERFORMANCE => Any::from("high-performance"),
            MLPowerPreference::LOW_POWER => Any::from("low-power"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLInputOperandLayout {
    NCHW,
    NHWC,
}
impl FromVal for MLInputOperandLayout {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "nchw" => Self::NCHW,
            "nhwc" => Self::NHWC,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLInputOperandLayout> for Any {
    fn from(s: MLInputOperandLayout) -> Any {
         match s {
            MLInputOperandLayout::NCHW => Any::from("nchw"),
            MLInputOperandLayout::NHWC => Any::from("nhwc"),
         }
    }
}
impl From<&MLInputOperandLayout> for Any {
    fn from(s: &MLInputOperandLayout) -> Any {
         match *s {
            MLInputOperandLayout::NCHW => Any::from("nchw"),
            MLInputOperandLayout::NHWC => Any::from("nhwc"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLOperandDataType> for Any {
    fn from(s: MLOperandDataType) -> Any {
         match s {
            MLOperandDataType::FLOAT32 => Any::from("float32"),
            MLOperandDataType::FLOAT16 => Any::from("float16"),
            MLOperandDataType::INT32 => Any::from("int32"),
            MLOperandDataType::UINT32 => Any::from("uint32"),
            MLOperandDataType::INT64 => Any::from("int64"),
            MLOperandDataType::UINT64 => Any::from("uint64"),
            MLOperandDataType::INT8 => Any::from("int8"),
            MLOperandDataType::UINT8 => Any::from("uint8"),
         }
    }
}
impl From<&MLOperandDataType> for Any {
    fn from(s: &MLOperandDataType) -> Any {
         match *s {
            MLOperandDataType::FLOAT32 => Any::from("float32"),
            MLOperandDataType::FLOAT16 => Any::from("float16"),
            MLOperandDataType::INT32 => Any::from("int32"),
            MLOperandDataType::UINT32 => Any::from("uint32"),
            MLOperandDataType::INT64 => Any::from("int64"),
            MLOperandDataType::UINT64 => Any::from("uint64"),
            MLOperandDataType::INT8 => Any::from("int8"),
            MLOperandDataType::UINT8 => Any::from("uint8"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "oihw" => Self::OIHW,
            "hwio" => Self::HWIO,
            "ohwi" => Self::OHWI,
            "ihwo" => Self::IHWO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLConv2dFilterOperandLayout> for Any {
    fn from(s: MLConv2dFilterOperandLayout) -> Any {
         match s {
            MLConv2dFilterOperandLayout::OIHW => Any::from("oihw"),
            MLConv2dFilterOperandLayout::HWIO => Any::from("hwio"),
            MLConv2dFilterOperandLayout::OHWI => Any::from("ohwi"),
            MLConv2dFilterOperandLayout::IHWO => Any::from("ihwo"),
         }
    }
}
impl From<&MLConv2dFilterOperandLayout> for Any {
    fn from(s: &MLConv2dFilterOperandLayout) -> Any {
         match *s {
            MLConv2dFilterOperandLayout::OIHW => Any::from("oihw"),
            MLConv2dFilterOperandLayout::HWIO => Any::from("hwio"),
            MLConv2dFilterOperandLayout::OHWI => Any::from("ohwi"),
            MLConv2dFilterOperandLayout::IHWO => Any::from("ihwo"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "iohw" => Self::IOHW,
            "hwoi" => Self::HWOI,
            "ohwi" => Self::OHWI,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLConvTranspose2dFilterOperandLayout> for Any {
    fn from(s: MLConvTranspose2dFilterOperandLayout) -> Any {
         match s {
            MLConvTranspose2dFilterOperandLayout::IOHW => Any::from("iohw"),
            MLConvTranspose2dFilterOperandLayout::HWOI => Any::from("hwoi"),
            MLConvTranspose2dFilterOperandLayout::OHWI => Any::from("ohwi"),
         }
    }
}
impl From<&MLConvTranspose2dFilterOperandLayout> for Any {
    fn from(s: &MLConvTranspose2dFilterOperandLayout) -> Any {
         match *s {
            MLConvTranspose2dFilterOperandLayout::IOHW => Any::from("iohw"),
            MLConvTranspose2dFilterOperandLayout::HWOI => Any::from("hwoi"),
            MLConvTranspose2dFilterOperandLayout::OHWI => Any::from("ohwi"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLGruWeightLayout {
    ZRN,
    RZN,
}
impl FromVal for MLGruWeightLayout {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "zrn" => Self::ZRN,
            "rzn" => Self::RZN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLGruWeightLayout> for Any {
    fn from(s: MLGruWeightLayout) -> Any {
         match s {
            MLGruWeightLayout::ZRN => Any::from("zrn"),
            MLGruWeightLayout::RZN => Any::from("rzn"),
         }
    }
}
impl From<&MLGruWeightLayout> for Any {
    fn from(s: &MLGruWeightLayout) -> Any {
         match *s {
            MLGruWeightLayout::ZRN => Any::from("zrn"),
            MLGruWeightLayout::RZN => Any::from("rzn"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "relu" => Self::RELU,
            "sigmoid" => Self::SIGMOID,
            "tanh" => Self::TANH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLRecurrentNetworkActivation> for Any {
    fn from(s: MLRecurrentNetworkActivation) -> Any {
         match s {
            MLRecurrentNetworkActivation::RELU => Any::from("relu"),
            MLRecurrentNetworkActivation::SIGMOID => Any::from("sigmoid"),
            MLRecurrentNetworkActivation::TANH => Any::from("tanh"),
         }
    }
}
impl From<&MLRecurrentNetworkActivation> for Any {
    fn from(s: &MLRecurrentNetworkActivation) -> Any {
         match *s {
            MLRecurrentNetworkActivation::RELU => Any::from("relu"),
            MLRecurrentNetworkActivation::SIGMOID => Any::from("sigmoid"),
            MLRecurrentNetworkActivation::TANH => Any::from("tanh"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "forward" => Self::FORWARD,
            "backward" => Self::BACKWARD,
            "both" => Self::BOTH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLRecurrentNetworkDirection> for Any {
    fn from(s: MLRecurrentNetworkDirection) -> Any {
         match s {
            MLRecurrentNetworkDirection::FORWARD => Any::from("forward"),
            MLRecurrentNetworkDirection::BACKWARD => Any::from("backward"),
            MLRecurrentNetworkDirection::BOTH => Any::from("both"),
         }
    }
}
impl From<&MLRecurrentNetworkDirection> for Any {
    fn from(s: &MLRecurrentNetworkDirection) -> Any {
         match *s {
            MLRecurrentNetworkDirection::FORWARD => Any::from("forward"),
            MLRecurrentNetworkDirection::BACKWARD => Any::from("backward"),
            MLRecurrentNetworkDirection::BOTH => Any::from("both"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLLstmWeightLayout {
    IOFG,
    IFGO,
}
impl FromVal for MLLstmWeightLayout {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "iofg" => Self::IOFG,
            "ifgo" => Self::IFGO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLLstmWeightLayout> for Any {
    fn from(s: MLLstmWeightLayout) -> Any {
         match s {
            MLLstmWeightLayout::IOFG => Any::from("iofg"),
            MLLstmWeightLayout::IFGO => Any::from("ifgo"),
         }
    }
}
impl From<&MLLstmWeightLayout> for Any {
    fn from(s: &MLLstmWeightLayout) -> Any {
         match *s {
            MLLstmWeightLayout::IOFG => Any::from("iofg"),
            MLLstmWeightLayout::IFGO => Any::from("ifgo"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "constant" => Self::CONSTANT,
            "edge" => Self::EDGE,
            "reflection" => Self::REFLECTION,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLPaddingMode> for Any {
    fn from(s: MLPaddingMode) -> Any {
         match s {
            MLPaddingMode::CONSTANT => Any::from("constant"),
            MLPaddingMode::EDGE => Any::from("edge"),
            MLPaddingMode::REFLECTION => Any::from("reflection"),
         }
    }
}
impl From<&MLPaddingMode> for Any {
    fn from(s: &MLPaddingMode) -> Any {
         match *s {
            MLPaddingMode::CONSTANT => Any::from("constant"),
            MLPaddingMode::EDGE => Any::from("edge"),
            MLPaddingMode::REFLECTION => Any::from("reflection"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLRoundingType {
    FLOOR,
    CEIL,
}
impl FromVal for MLRoundingType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "floor" => Self::FLOOR,
            "ceil" => Self::CEIL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLRoundingType> for Any {
    fn from(s: MLRoundingType) -> Any {
         match s {
            MLRoundingType::FLOOR => Any::from("floor"),
            MLRoundingType::CEIL => Any::from("ceil"),
         }
    }
}
impl From<&MLRoundingType> for Any {
    fn from(s: &MLRoundingType) -> Any {
         match *s {
            MLRoundingType::FLOOR => Any::from("floor"),
            MLRoundingType::CEIL => Any::from("ceil"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MLInterpolationMode {
    NEAREST_NEIGHBOR,
    LINEAR,
}
impl FromVal for MLInterpolationMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "nearest-neighbor" => Self::NEAREST_NEIGHBOR,
            "linear" => Self::LINEAR,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<MLInterpolationMode> for Any {
    fn from(s: MLInterpolationMode) -> Any {
         match s {
            MLInterpolationMode::NEAREST_NEIGHBOR => Any::from("nearest-neighbor"),
            MLInterpolationMode::LINEAR => Any::from("linear"),
         }
    }
}
impl From<&MLInterpolationMode> for Any {
    fn from(s: &MLInterpolationMode) -> Any {
         match *s {
            MLInterpolationMode::NEAREST_NEIGHBOR => Any::from("nearest-neighbor"),
            MLInterpolationMode::LINEAR => Any::from("linear"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SFrameTransformRole {
    ENCRYPT,
    DECRYPT,
}
impl FromVal for SFrameTransformRole {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "encrypt" => Self::ENCRYPT,
            "decrypt" => Self::DECRYPT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SFrameTransformRole> for Any {
    fn from(s: SFrameTransformRole) -> Any {
         match s {
            SFrameTransformRole::ENCRYPT => Any::from("encrypt"),
            SFrameTransformRole::DECRYPT => Any::from("decrypt"),
         }
    }
}
impl From<&SFrameTransformRole> for Any {
    fn from(s: &SFrameTransformRole) -> Any {
         match *s {
            SFrameTransformRole::ENCRYPT => Any::from("encrypt"),
            SFrameTransformRole::DECRYPT => Any::from("decrypt"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "authentication" => Self::AUTHENTICATION,
            "keyID" => Self::KEY_ID,
            "syntax" => Self::SYNTAX,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SFrameTransformErrorEventType> for Any {
    fn from(s: SFrameTransformErrorEventType) -> Any {
         match s {
            SFrameTransformErrorEventType::AUTHENTICATION => Any::from("authentication"),
            SFrameTransformErrorEventType::KEY_ID => Any::from("keyID"),
            SFrameTransformErrorEventType::SYNTAX => Any::from("syntax"),
         }
    }
}
impl From<&SFrameTransformErrorEventType> for Any {
    fn from(s: &SFrameTransformErrorEventType) -> Any {
         match *s {
            SFrameTransformErrorEventType::AUTHENTICATION => Any::from("authentication"),
            SFrameTransformErrorEventType::KEY_ID => Any::from("keyID"),
            SFrameTransformErrorEventType::SYNTAX => Any::from("syntax"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "empty" => Self::EMPTY,
            "key" => Self::KEY,
            "delta" => Self::DELTA,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCEncodedVideoFrameType> for Any {
    fn from(s: RTCEncodedVideoFrameType) -> Any {
         match s {
            RTCEncodedVideoFrameType::EMPTY => Any::from("empty"),
            RTCEncodedVideoFrameType::KEY => Any::from("key"),
            RTCEncodedVideoFrameType::DELTA => Any::from("delta"),
         }
    }
}
impl From<&RTCEncodedVideoFrameType> for Any {
    fn from(s: &RTCEncodedVideoFrameType) -> Any {
         match *s {
            RTCEncodedVideoFrameType::EMPTY => Any::from("empty"),
            RTCEncodedVideoFrameType::KEY => Any::from("key"),
            RTCEncodedVideoFrameType::DELTA => Any::from("delta"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCErrorDetailTypeIdp> for Any {
    fn from(s: RTCErrorDetailTypeIdp) -> Any {
         match s {
            RTCErrorDetailTypeIdp::IDP_BAD_SCRIPT_FAILURE => Any::from("idp-bad-script-failure"),
            RTCErrorDetailTypeIdp::IDP_EXECUTION_FAILURE => Any::from("idp-execution-failure"),
            RTCErrorDetailTypeIdp::IDP_LOAD_FAILURE => Any::from("idp-load-failure"),
            RTCErrorDetailTypeIdp::IDP_NEED_LOGIN => Any::from("idp-need-login"),
            RTCErrorDetailTypeIdp::IDP_TIMEOUT => Any::from("idp-timeout"),
            RTCErrorDetailTypeIdp::IDP_TLS_FAILURE => Any::from("idp-tls-failure"),
            RTCErrorDetailTypeIdp::IDP_TOKEN_EXPIRED => Any::from("idp-token-expired"),
            RTCErrorDetailTypeIdp::IDP_TOKEN_INVALID => Any::from("idp-token-invalid"),
         }
    }
}
impl From<&RTCErrorDetailTypeIdp> for Any {
    fn from(s: &RTCErrorDetailTypeIdp) -> Any {
         match *s {
            RTCErrorDetailTypeIdp::IDP_BAD_SCRIPT_FAILURE => Any::from("idp-bad-script-failure"),
            RTCErrorDetailTypeIdp::IDP_EXECUTION_FAILURE => Any::from("idp-execution-failure"),
            RTCErrorDetailTypeIdp::IDP_LOAD_FAILURE => Any::from("idp-load-failure"),
            RTCErrorDetailTypeIdp::IDP_NEED_LOGIN => Any::from("idp-need-login"),
            RTCErrorDetailTypeIdp::IDP_TIMEOUT => Any::from("idp-timeout"),
            RTCErrorDetailTypeIdp::IDP_TLS_FAILURE => Any::from("idp-tls-failure"),
            RTCErrorDetailTypeIdp::IDP_TOKEN_EXPIRED => Any::from("idp-token-expired"),
            RTCErrorDetailTypeIdp::IDP_TOKEN_INVALID => Any::from("idp-token-invalid"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "very-low" => Self::VERY_LOW,
            "low" => Self::LOW,
            "medium" => Self::MEDIUM,
            "high" => Self::HIGH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCPriorityType> for Any {
    fn from(s: RTCPriorityType) -> Any {
         match s {
            RTCPriorityType::VERY_LOW => Any::from("very-low"),
            RTCPriorityType::LOW => Any::from("low"),
            RTCPriorityType::MEDIUM => Any::from("medium"),
            RTCPriorityType::HIGH => Any::from("high"),
         }
    }
}
impl From<&RTCPriorityType> for Any {
    fn from(s: &RTCPriorityType) -> Any {
         match *s {
            RTCPriorityType::VERY_LOW => Any::from("very-low"),
            RTCPriorityType::LOW => Any::from("low"),
            RTCPriorityType::MEDIUM => Any::from("medium"),
            RTCPriorityType::HIGH => Any::from("high"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCStatsType> for Any {
    fn from(s: RTCStatsType) -> Any {
         match s {
            RTCStatsType::CODEC => Any::from("codec"),
            RTCStatsType::INBOUND_RTP => Any::from("inbound-rtp"),
            RTCStatsType::OUTBOUND_RTP => Any::from("outbound-rtp"),
            RTCStatsType::REMOTE_INBOUND_RTP => Any::from("remote-inbound-rtp"),
            RTCStatsType::REMOTE_OUTBOUND_RTP => Any::from("remote-outbound-rtp"),
            RTCStatsType::MEDIA_SOURCE => Any::from("media-source"),
            RTCStatsType::MEDIA_PLAYOUT => Any::from("media-playout"),
            RTCStatsType::PEER_CONNECTION => Any::from("peer-connection"),
            RTCStatsType::DATA_CHANNEL => Any::from("data-channel"),
            RTCStatsType::TRANSPORT => Any::from("transport"),
            RTCStatsType::CANDIDATE_PAIR => Any::from("candidate-pair"),
            RTCStatsType::LOCAL_CANDIDATE => Any::from("local-candidate"),
            RTCStatsType::REMOTE_CANDIDATE => Any::from("remote-candidate"),
            RTCStatsType::CERTIFICATE => Any::from("certificate"),
         }
    }
}
impl From<&RTCStatsType> for Any {
    fn from(s: &RTCStatsType) -> Any {
         match *s {
            RTCStatsType::CODEC => Any::from("codec"),
            RTCStatsType::INBOUND_RTP => Any::from("inbound-rtp"),
            RTCStatsType::OUTBOUND_RTP => Any::from("outbound-rtp"),
            RTCStatsType::REMOTE_INBOUND_RTP => Any::from("remote-inbound-rtp"),
            RTCStatsType::REMOTE_OUTBOUND_RTP => Any::from("remote-outbound-rtp"),
            RTCStatsType::MEDIA_SOURCE => Any::from("media-source"),
            RTCStatsType::MEDIA_PLAYOUT => Any::from("media-playout"),
            RTCStatsType::PEER_CONNECTION => Any::from("peer-connection"),
            RTCStatsType::DATA_CHANNEL => Any::from("data-channel"),
            RTCStatsType::TRANSPORT => Any::from("transport"),
            RTCStatsType::CANDIDATE_PAIR => Any::from("candidate-pair"),
            RTCStatsType::LOCAL_CANDIDATE => Any::from("local-candidate"),
            RTCStatsType::REMOTE_CANDIDATE => Any::from("remote-candidate"),
            RTCStatsType::CERTIFICATE => Any::from("certificate"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "cpu" => Self::CPU,
            "bandwidth" => Self::BANDWIDTH,
            "other" => Self::OTHER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCQualityLimitationReason> for Any {
    fn from(s: RTCQualityLimitationReason) -> Any {
         match s {
            RTCQualityLimitationReason::NONE => Any::from("none"),
            RTCQualityLimitationReason::CPU => Any::from("cpu"),
            RTCQualityLimitationReason::BANDWIDTH => Any::from("bandwidth"),
            RTCQualityLimitationReason::OTHER => Any::from("other"),
         }
    }
}
impl From<&RTCQualityLimitationReason> for Any {
    fn from(s: &RTCQualityLimitationReason) -> Any {
         match *s {
            RTCQualityLimitationReason::NONE => Any::from("none"),
            RTCQualityLimitationReason::CPU => Any::from("cpu"),
            RTCQualityLimitationReason::BANDWIDTH => Any::from("bandwidth"),
            RTCQualityLimitationReason::OTHER => Any::from("other"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "client" => Self::CLIENT,
            "server" => Self::SERVER,
            "unknown" => Self::UNKNOWN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCDtlsRole> for Any {
    fn from(s: RTCDtlsRole) -> Any {
         match s {
            RTCDtlsRole::CLIENT => Any::from("client"),
            RTCDtlsRole::SERVER => Any::from("server"),
            RTCDtlsRole::UNKNOWN => Any::from("unknown"),
         }
    }
}
impl From<&RTCDtlsRole> for Any {
    fn from(s: &RTCDtlsRole) -> Any {
         match *s {
            RTCDtlsRole::CLIENT => Any::from("client"),
            RTCDtlsRole::SERVER => Any::from("server"),
            RTCDtlsRole::UNKNOWN => Any::from("unknown"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "frozen" => Self::FROZEN,
            "waiting" => Self::WAITING,
            "in-progress" => Self::IN_PROGRESS,
            "failed" => Self::FAILED,
            "succeeded" => Self::SUCCEEDED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCStatsIceCandidatePairState> for Any {
    fn from(s: RTCStatsIceCandidatePairState) -> Any {
         match s {
            RTCStatsIceCandidatePairState::FROZEN => Any::from("frozen"),
            RTCStatsIceCandidatePairState::WAITING => Any::from("waiting"),
            RTCStatsIceCandidatePairState::IN_PROGRESS => Any::from("in-progress"),
            RTCStatsIceCandidatePairState::FAILED => Any::from("failed"),
            RTCStatsIceCandidatePairState::SUCCEEDED => Any::from("succeeded"),
         }
    }
}
impl From<&RTCStatsIceCandidatePairState> for Any {
    fn from(s: &RTCStatsIceCandidatePairState) -> Any {
         match *s {
            RTCStatsIceCandidatePairState::FROZEN => Any::from("frozen"),
            RTCStatsIceCandidatePairState::WAITING => Any::from("waiting"),
            RTCStatsIceCandidatePairState::IN_PROGRESS => Any::from("in-progress"),
            RTCStatsIceCandidatePairState::FAILED => Any::from("failed"),
            RTCStatsIceCandidatePairState::SUCCEEDED => Any::from("succeeded"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceTransportPolicy {
    RELAY,
    ALL,
}
impl FromVal for RTCIceTransportPolicy {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "relay" => Self::RELAY,
            "all" => Self::ALL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceTransportPolicy> for Any {
    fn from(s: RTCIceTransportPolicy) -> Any {
         match s {
            RTCIceTransportPolicy::RELAY => Any::from("relay"),
            RTCIceTransportPolicy::ALL => Any::from("all"),
         }
    }
}
impl From<&RTCIceTransportPolicy> for Any {
    fn from(s: &RTCIceTransportPolicy) -> Any {
         match *s {
            RTCIceTransportPolicy::RELAY => Any::from("relay"),
            RTCIceTransportPolicy::ALL => Any::from("all"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "balanced" => Self::BALANCED,
            "max-compat" => Self::MAX_COMPAT,
            "max-bundle" => Self::MAX_BUNDLE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCBundlePolicy> for Any {
    fn from(s: RTCBundlePolicy) -> Any {
         match s {
            RTCBundlePolicy::BALANCED => Any::from("balanced"),
            RTCBundlePolicy::MAX_COMPAT => Any::from("max-compat"),
            RTCBundlePolicy::MAX_BUNDLE => Any::from("max-bundle"),
         }
    }
}
impl From<&RTCBundlePolicy> for Any {
    fn from(s: &RTCBundlePolicy) -> Any {
         match *s {
            RTCBundlePolicy::BALANCED => Any::from("balanced"),
            RTCBundlePolicy::MAX_COMPAT => Any::from("max-compat"),
            RTCBundlePolicy::MAX_BUNDLE => Any::from("max-bundle"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCRtcpMuxPolicy {
    REQUIRE,
}
impl FromVal for RTCRtcpMuxPolicy {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "require" => Self::REQUIRE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCRtcpMuxPolicy> for Any {
    fn from(s: RTCRtcpMuxPolicy) -> Any {
         match s {
            RTCRtcpMuxPolicy::REQUIRE => Any::from("require"),
         }
    }
}
impl From<&RTCRtcpMuxPolicy> for Any {
    fn from(s: &RTCRtcpMuxPolicy) -> Any {
         match *s {
            RTCRtcpMuxPolicy::REQUIRE => Any::from("require"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "stable" => Self::STABLE,
            "have-local-offer" => Self::HAVE_LOCAL_OFFER,
            "have-remote-offer" => Self::HAVE_REMOTE_OFFER,
            "have-local-pranswer" => Self::HAVE_LOCAL_PRANSWER,
            "have-remote-pranswer" => Self::HAVE_REMOTE_PRANSWER,
            "closed" => Self::CLOSED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCSignalingState> for Any {
    fn from(s: RTCSignalingState) -> Any {
         match s {
            RTCSignalingState::STABLE => Any::from("stable"),
            RTCSignalingState::HAVE_LOCAL_OFFER => Any::from("have-local-offer"),
            RTCSignalingState::HAVE_REMOTE_OFFER => Any::from("have-remote-offer"),
            RTCSignalingState::HAVE_LOCAL_PRANSWER => Any::from("have-local-pranswer"),
            RTCSignalingState::HAVE_REMOTE_PRANSWER => Any::from("have-remote-pranswer"),
            RTCSignalingState::CLOSED => Any::from("closed"),
         }
    }
}
impl From<&RTCSignalingState> for Any {
    fn from(s: &RTCSignalingState) -> Any {
         match *s {
            RTCSignalingState::STABLE => Any::from("stable"),
            RTCSignalingState::HAVE_LOCAL_OFFER => Any::from("have-local-offer"),
            RTCSignalingState::HAVE_REMOTE_OFFER => Any::from("have-remote-offer"),
            RTCSignalingState::HAVE_LOCAL_PRANSWER => Any::from("have-local-pranswer"),
            RTCSignalingState::HAVE_REMOTE_PRANSWER => Any::from("have-remote-pranswer"),
            RTCSignalingState::CLOSED => Any::from("closed"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "new" => Self::NEW,
            "gathering" => Self::GATHERING,
            "complete" => Self::COMPLETE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceGatheringState> for Any {
    fn from(s: RTCIceGatheringState) -> Any {
         match s {
            RTCIceGatheringState::NEW => Any::from("new"),
            RTCIceGatheringState::GATHERING => Any::from("gathering"),
            RTCIceGatheringState::COMPLETE => Any::from("complete"),
         }
    }
}
impl From<&RTCIceGatheringState> for Any {
    fn from(s: &RTCIceGatheringState) -> Any {
         match *s {
            RTCIceGatheringState::NEW => Any::from("new"),
            RTCIceGatheringState::GATHERING => Any::from("gathering"),
            RTCIceGatheringState::COMPLETE => Any::from("complete"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "closed" => Self::CLOSED,
            "failed" => Self::FAILED,
            "disconnected" => Self::DISCONNECTED,
            "new" => Self::NEW,
            "connecting" => Self::CONNECTING,
            "connected" => Self::CONNECTED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCPeerConnectionState> for Any {
    fn from(s: RTCPeerConnectionState) -> Any {
         match s {
            RTCPeerConnectionState::CLOSED => Any::from("closed"),
            RTCPeerConnectionState::FAILED => Any::from("failed"),
            RTCPeerConnectionState::DISCONNECTED => Any::from("disconnected"),
            RTCPeerConnectionState::NEW => Any::from("new"),
            RTCPeerConnectionState::CONNECTING => Any::from("connecting"),
            RTCPeerConnectionState::CONNECTED => Any::from("connected"),
         }
    }
}
impl From<&RTCPeerConnectionState> for Any {
    fn from(s: &RTCPeerConnectionState) -> Any {
         match *s {
            RTCPeerConnectionState::CLOSED => Any::from("closed"),
            RTCPeerConnectionState::FAILED => Any::from("failed"),
            RTCPeerConnectionState::DISCONNECTED => Any::from("disconnected"),
            RTCPeerConnectionState::NEW => Any::from("new"),
            RTCPeerConnectionState::CONNECTING => Any::from("connecting"),
            RTCPeerConnectionState::CONNECTED => Any::from("connected"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceConnectionState> for Any {
    fn from(s: RTCIceConnectionState) -> Any {
         match s {
            RTCIceConnectionState::CLOSED => Any::from("closed"),
            RTCIceConnectionState::FAILED => Any::from("failed"),
            RTCIceConnectionState::DISCONNECTED => Any::from("disconnected"),
            RTCIceConnectionState::NEW => Any::from("new"),
            RTCIceConnectionState::CHECKING => Any::from("checking"),
            RTCIceConnectionState::COMPLETED => Any::from("completed"),
            RTCIceConnectionState::CONNECTED => Any::from("connected"),
         }
    }
}
impl From<&RTCIceConnectionState> for Any {
    fn from(s: &RTCIceConnectionState) -> Any {
         match *s {
            RTCIceConnectionState::CLOSED => Any::from("closed"),
            RTCIceConnectionState::FAILED => Any::from("failed"),
            RTCIceConnectionState::DISCONNECTED => Any::from("disconnected"),
            RTCIceConnectionState::NEW => Any::from("new"),
            RTCIceConnectionState::CHECKING => Any::from("checking"),
            RTCIceConnectionState::COMPLETED => Any::from("completed"),
            RTCIceConnectionState::CONNECTED => Any::from("connected"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "offer" => Self::OFFER,
            "pranswer" => Self::PRANSWER,
            "answer" => Self::ANSWER,
            "rollback" => Self::ROLLBACK,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCSdpType> for Any {
    fn from(s: RTCSdpType) -> Any {
         match s {
            RTCSdpType::OFFER => Any::from("offer"),
            RTCSdpType::PRANSWER => Any::from("pranswer"),
            RTCSdpType::ANSWER => Any::from("answer"),
            RTCSdpType::ROLLBACK => Any::from("rollback"),
         }
    }
}
impl From<&RTCSdpType> for Any {
    fn from(s: &RTCSdpType) -> Any {
         match *s {
            RTCSdpType::OFFER => Any::from("offer"),
            RTCSdpType::PRANSWER => Any::from("pranswer"),
            RTCSdpType::ANSWER => Any::from("answer"),
            RTCSdpType::ROLLBACK => Any::from("rollback"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceProtocol {
    UDP,
    TCP,
}
impl FromVal for RTCIceProtocol {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "udp" => Self::UDP,
            "tcp" => Self::TCP,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceProtocol> for Any {
    fn from(s: RTCIceProtocol) -> Any {
         match s {
            RTCIceProtocol::UDP => Any::from("udp"),
            RTCIceProtocol::TCP => Any::from("tcp"),
         }
    }
}
impl From<&RTCIceProtocol> for Any {
    fn from(s: &RTCIceProtocol) -> Any {
         match *s {
            RTCIceProtocol::UDP => Any::from("udp"),
            RTCIceProtocol::TCP => Any::from("tcp"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "active" => Self::ACTIVE,
            "passive" => Self::PASSIVE,
            "so" => Self::SO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceTcpCandidateType> for Any {
    fn from(s: RTCIceTcpCandidateType) -> Any {
         match s {
            RTCIceTcpCandidateType::ACTIVE => Any::from("active"),
            RTCIceTcpCandidateType::PASSIVE => Any::from("passive"),
            RTCIceTcpCandidateType::SO => Any::from("so"),
         }
    }
}
impl From<&RTCIceTcpCandidateType> for Any {
    fn from(s: &RTCIceTcpCandidateType) -> Any {
         match *s {
            RTCIceTcpCandidateType::ACTIVE => Any::from("active"),
            RTCIceTcpCandidateType::PASSIVE => Any::from("passive"),
            RTCIceTcpCandidateType::SO => Any::from("so"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "host" => Self::HOST,
            "srflx" => Self::SRFLX,
            "prflx" => Self::PRFLX,
            "relay" => Self::RELAY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceCandidateType> for Any {
    fn from(s: RTCIceCandidateType) -> Any {
         match s {
            RTCIceCandidateType::HOST => Any::from("host"),
            RTCIceCandidateType::SRFLX => Any::from("srflx"),
            RTCIceCandidateType::PRFLX => Any::from("prflx"),
            RTCIceCandidateType::RELAY => Any::from("relay"),
         }
    }
}
impl From<&RTCIceCandidateType> for Any {
    fn from(s: &RTCIceCandidateType) -> Any {
         match *s {
            RTCIceCandidateType::HOST => Any::from("host"),
            RTCIceCandidateType::SRFLX => Any::from("srflx"),
            RTCIceCandidateType::PRFLX => Any::from("prflx"),
            RTCIceCandidateType::RELAY => Any::from("relay"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "udp" => Self::UDP,
            "tcp" => Self::TCP,
            "tls" => Self::TLS,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceServerTransportProtocol> for Any {
    fn from(s: RTCIceServerTransportProtocol) -> Any {
         match s {
            RTCIceServerTransportProtocol::UDP => Any::from("udp"),
            RTCIceServerTransportProtocol::TCP => Any::from("tcp"),
            RTCIceServerTransportProtocol::TLS => Any::from("tls"),
         }
    }
}
impl From<&RTCIceServerTransportProtocol> for Any {
    fn from(s: &RTCIceServerTransportProtocol) -> Any {
         match *s {
            RTCIceServerTransportProtocol::UDP => Any::from("udp"),
            RTCIceServerTransportProtocol::TCP => Any::from("tcp"),
            RTCIceServerTransportProtocol::TLS => Any::from("tls"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "sendrecv" => Self::SENDRECV,
            "sendonly" => Self::SENDONLY,
            "recvonly" => Self::RECVONLY,
            "inactive" => Self::INACTIVE,
            "stopped" => Self::STOPPED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCRtpTransceiverDirection> for Any {
    fn from(s: RTCRtpTransceiverDirection) -> Any {
         match s {
            RTCRtpTransceiverDirection::SENDRECV => Any::from("sendrecv"),
            RTCRtpTransceiverDirection::SENDONLY => Any::from("sendonly"),
            RTCRtpTransceiverDirection::RECVONLY => Any::from("recvonly"),
            RTCRtpTransceiverDirection::INACTIVE => Any::from("inactive"),
            RTCRtpTransceiverDirection::STOPPED => Any::from("stopped"),
         }
    }
}
impl From<&RTCRtpTransceiverDirection> for Any {
    fn from(s: &RTCRtpTransceiverDirection) -> Any {
         match *s {
            RTCRtpTransceiverDirection::SENDRECV => Any::from("sendrecv"),
            RTCRtpTransceiverDirection::SENDONLY => Any::from("sendonly"),
            RTCRtpTransceiverDirection::RECVONLY => Any::from("recvonly"),
            RTCRtpTransceiverDirection::INACTIVE => Any::from("inactive"),
            RTCRtpTransceiverDirection::STOPPED => Any::from("stopped"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "new" => Self::NEW,
            "connecting" => Self::CONNECTING,
            "connected" => Self::CONNECTED,
            "closed" => Self::CLOSED,
            "failed" => Self::FAILED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCDtlsTransportState> for Any {
    fn from(s: RTCDtlsTransportState) -> Any {
         match s {
            RTCDtlsTransportState::NEW => Any::from("new"),
            RTCDtlsTransportState::CONNECTING => Any::from("connecting"),
            RTCDtlsTransportState::CONNECTED => Any::from("connected"),
            RTCDtlsTransportState::CLOSED => Any::from("closed"),
            RTCDtlsTransportState::FAILED => Any::from("failed"),
         }
    }
}
impl From<&RTCDtlsTransportState> for Any {
    fn from(s: &RTCDtlsTransportState) -> Any {
         match *s {
            RTCDtlsTransportState::NEW => Any::from("new"),
            RTCDtlsTransportState::CONNECTING => Any::from("connecting"),
            RTCDtlsTransportState::CONNECTED => Any::from("connected"),
            RTCDtlsTransportState::CLOSED => Any::from("closed"),
            RTCDtlsTransportState::FAILED => Any::from("failed"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "new" => Self::NEW,
            "gathering" => Self::GATHERING,
            "complete" => Self::COMPLETE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceGathererState> for Any {
    fn from(s: RTCIceGathererState) -> Any {
         match s {
            RTCIceGathererState::NEW => Any::from("new"),
            RTCIceGathererState::GATHERING => Any::from("gathering"),
            RTCIceGathererState::COMPLETE => Any::from("complete"),
         }
    }
}
impl From<&RTCIceGathererState> for Any {
    fn from(s: &RTCIceGathererState) -> Any {
         match *s {
            RTCIceGathererState::NEW => Any::from("new"),
            RTCIceGathererState::GATHERING => Any::from("gathering"),
            RTCIceGathererState::COMPLETE => Any::from("complete"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceTransportState> for Any {
    fn from(s: RTCIceTransportState) -> Any {
         match s {
            RTCIceTransportState::CLOSED => Any::from("closed"),
            RTCIceTransportState::FAILED => Any::from("failed"),
            RTCIceTransportState::DISCONNECTED => Any::from("disconnected"),
            RTCIceTransportState::NEW => Any::from("new"),
            RTCIceTransportState::CHECKING => Any::from("checking"),
            RTCIceTransportState::COMPLETED => Any::from("completed"),
            RTCIceTransportState::CONNECTED => Any::from("connected"),
         }
    }
}
impl From<&RTCIceTransportState> for Any {
    fn from(s: &RTCIceTransportState) -> Any {
         match *s {
            RTCIceTransportState::CLOSED => Any::from("closed"),
            RTCIceTransportState::FAILED => Any::from("failed"),
            RTCIceTransportState::DISCONNECTED => Any::from("disconnected"),
            RTCIceTransportState::NEW => Any::from("new"),
            RTCIceTransportState::CHECKING => Any::from("checking"),
            RTCIceTransportState::COMPLETED => Any::from("completed"),
            RTCIceTransportState::CONNECTED => Any::from("connected"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "unknown" => Self::UNKNOWN,
            "controlling" => Self::CONTROLLING,
            "controlled" => Self::CONTROLLED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceRole> for Any {
    fn from(s: RTCIceRole) -> Any {
         match s {
            RTCIceRole::UNKNOWN => Any::from("unknown"),
            RTCIceRole::CONTROLLING => Any::from("controlling"),
            RTCIceRole::CONTROLLED => Any::from("controlled"),
         }
    }
}
impl From<&RTCIceRole> for Any {
    fn from(s: &RTCIceRole) -> Any {
         match *s {
            RTCIceRole::UNKNOWN => Any::from("unknown"),
            RTCIceRole::CONTROLLING => Any::from("controlling"),
            RTCIceRole::CONTROLLED => Any::from("controlled"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum RTCIceComponent {
    RTP,
    RTCP,
}
impl FromVal for RTCIceComponent {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "rtp" => Self::RTP,
            "rtcp" => Self::RTCP,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCIceComponent> for Any {
    fn from(s: RTCIceComponent) -> Any {
         match s {
            RTCIceComponent::RTP => Any::from("rtp"),
            RTCIceComponent::RTCP => Any::from("rtcp"),
         }
    }
}
impl From<&RTCIceComponent> for Any {
    fn from(s: &RTCIceComponent) -> Any {
         match *s {
            RTCIceComponent::RTP => Any::from("rtp"),
            RTCIceComponent::RTCP => Any::from("rtcp"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "connecting" => Self::CONNECTING,
            "connected" => Self::CONNECTED,
            "closed" => Self::CLOSED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCSctpTransportState> for Any {
    fn from(s: RTCSctpTransportState) -> Any {
         match s {
            RTCSctpTransportState::CONNECTING => Any::from("connecting"),
            RTCSctpTransportState::CONNECTED => Any::from("connected"),
            RTCSctpTransportState::CLOSED => Any::from("closed"),
         }
    }
}
impl From<&RTCSctpTransportState> for Any {
    fn from(s: &RTCSctpTransportState) -> Any {
         match *s {
            RTCSctpTransportState::CONNECTING => Any::from("connecting"),
            RTCSctpTransportState::CONNECTED => Any::from("connected"),
            RTCSctpTransportState::CLOSED => Any::from("closed"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "connecting" => Self::CONNECTING,
            "open" => Self::OPEN,
            "closing" => Self::CLOSING,
            "closed" => Self::CLOSED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCDataChannelState> for Any {
    fn from(s: RTCDataChannelState) -> Any {
         match s {
            RTCDataChannelState::CONNECTING => Any::from("connecting"),
            RTCDataChannelState::OPEN => Any::from("open"),
            RTCDataChannelState::CLOSING => Any::from("closing"),
            RTCDataChannelState::CLOSED => Any::from("closed"),
         }
    }
}
impl From<&RTCDataChannelState> for Any {
    fn from(s: &RTCDataChannelState) -> Any {
         match *s {
            RTCDataChannelState::CONNECTING => Any::from("connecting"),
            RTCDataChannelState::OPEN => Any::from("open"),
            RTCDataChannelState::CLOSING => Any::from("closing"),
            RTCDataChannelState::CLOSED => Any::from("closed"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RTCErrorDetailType> for Any {
    fn from(s: RTCErrorDetailType) -> Any {
         match s {
            RTCErrorDetailType::DATA_CHANNEL_FAILURE => Any::from("data-channel-failure"),
            RTCErrorDetailType::DTLS_FAILURE => Any::from("dtls-failure"),
            RTCErrorDetailType::FINGERPRINT_FAILURE => Any::from("fingerprint-failure"),
            RTCErrorDetailType::SCTP_FAILURE => Any::from("sctp-failure"),
            RTCErrorDetailType::SDP_SYNTAX_ERROR => Any::from("sdp-syntax-error"),
            RTCErrorDetailType::HARDWARE_ENCODER_NOT_AVAILABLE => Any::from("hardware-encoder-not-available"),
            RTCErrorDetailType::HARDWARE_ENCODER_ERROR => Any::from("hardware-encoder-error"),
         }
    }
}
impl From<&RTCErrorDetailType> for Any {
    fn from(s: &RTCErrorDetailType) -> Any {
         match *s {
            RTCErrorDetailType::DATA_CHANNEL_FAILURE => Any::from("data-channel-failure"),
            RTCErrorDetailType::DTLS_FAILURE => Any::from("dtls-failure"),
            RTCErrorDetailType::FINGERPRINT_FAILURE => Any::from("fingerprint-failure"),
            RTCErrorDetailType::SCTP_FAILURE => Any::from("sctp-failure"),
            RTCErrorDetailType::SDP_SYNTAX_ERROR => Any::from("sdp-syntax-error"),
            RTCErrorDetailType::HARDWARE_ENCODER_NOT_AVAILABLE => Any::from("hardware-encoder-not-available"),
            RTCErrorDetailType::HARDWARE_ENCODER_ERROR => Any::from("hardware-encoder-error"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum BinaryType {
    BLOB,
    ARRAYBUFFER,
}
impl FromVal for BinaryType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "blob" => Self::BLOB,
            "arraybuffer" => Self::ARRAYBUFFER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<BinaryType> for Any {
    fn from(s: BinaryType) -> Any {
         match s {
            BinaryType::BLOB => Any::from("blob"),
            BinaryType::ARRAYBUFFER => Any::from("arraybuffer"),
         }
    }
}
impl From<&BinaryType> for Any {
    fn from(s: &BinaryType) -> Any {
         match *s {
            BinaryType::BLOB => Any::from("blob"),
            BinaryType::ARRAYBUFFER => Any::from("arraybuffer"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "pending" => Self::PENDING,
            "reliable-only" => Self::RELIABLE_ONLY,
            "supports-unreliable" => Self::SUPPORTS_UNRELIABLE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WebTransportReliabilityMode> for Any {
    fn from(s: WebTransportReliabilityMode) -> Any {
         match s {
            WebTransportReliabilityMode::PENDING => Any::from("pending"),
            WebTransportReliabilityMode::RELIABLE_ONLY => Any::from("reliable-only"),
            WebTransportReliabilityMode::SUPPORTS_UNRELIABLE => Any::from("supports-unreliable"),
         }
    }
}
impl From<&WebTransportReliabilityMode> for Any {
    fn from(s: &WebTransportReliabilityMode) -> Any {
         match *s {
            WebTransportReliabilityMode::PENDING => Any::from("pending"),
            WebTransportReliabilityMode::RELIABLE_ONLY => Any::from("reliable-only"),
            WebTransportReliabilityMode::SUPPORTS_UNRELIABLE => Any::from("supports-unreliable"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "default" => Self::DEFAULT,
            "throughput" => Self::THROUGHPUT,
            "low-latency" => Self::LOW_LATENCY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WebTransportCongestionControl> for Any {
    fn from(s: WebTransportCongestionControl) -> Any {
         match s {
            WebTransportCongestionControl::DEFAULT => Any::from("default"),
            WebTransportCongestionControl::THROUGHPUT => Any::from("throughput"),
            WebTransportCongestionControl::LOW_LATENCY => Any::from("low-latency"),
         }
    }
}
impl From<&WebTransportCongestionControl> for Any {
    fn from(s: &WebTransportCongestionControl) -> Any {
         match *s {
            WebTransportCongestionControl::DEFAULT => Any::from("default"),
            WebTransportCongestionControl::THROUGHPUT => Any::from("throughput"),
            WebTransportCongestionControl::LOW_LATENCY => Any::from("low-latency"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WebTransportErrorSource {
    STREAM,
    SESSION,
}
impl FromVal for WebTransportErrorSource {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "stream" => Self::STREAM,
            "session" => Self::SESSION,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WebTransportErrorSource> for Any {
    fn from(s: WebTransportErrorSource) -> Any {
         match s {
            WebTransportErrorSource::STREAM => Any::from("stream"),
            WebTransportErrorSource::SESSION => Any::from("session"),
         }
    }
}
impl From<&WebTransportErrorSource> for Any {
    fn from(s: &WebTransportErrorSource) -> Any {
         match *s {
            WebTransportErrorSource::STREAM => Any::from("stream"),
            WebTransportErrorSource::SESSION => Any::from("session"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "ok" => Self::OK,
            "stall" => Self::STALL,
            "babble" => Self::BABBLE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<USBTransferStatus> for Any {
    fn from(s: USBTransferStatus) -> Any {
         match s {
            USBTransferStatus::OK => Any::from("ok"),
            USBTransferStatus::STALL => Any::from("stall"),
            USBTransferStatus::BABBLE => Any::from("babble"),
         }
    }
}
impl From<&USBTransferStatus> for Any {
    fn from(s: &USBTransferStatus) -> Any {
         match *s {
            USBTransferStatus::OK => Any::from("ok"),
            USBTransferStatus::STALL => Any::from("stall"),
            USBTransferStatus::BABBLE => Any::from("babble"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "standard" => Self::STANDARD,
            "class" => Self::CLASS,
            "vendor" => Self::VENDOR,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<USBRequestType> for Any {
    fn from(s: USBRequestType) -> Any {
         match s {
            USBRequestType::STANDARD => Any::from("standard"),
            USBRequestType::CLASS => Any::from("class"),
            USBRequestType::VENDOR => Any::from("vendor"),
         }
    }
}
impl From<&USBRequestType> for Any {
    fn from(s: &USBRequestType) -> Any {
         match *s {
            USBRequestType::STANDARD => Any::from("standard"),
            USBRequestType::CLASS => Any::from("class"),
            USBRequestType::VENDOR => Any::from("vendor"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "device" => Self::DEVICE,
            "interface" => Self::INTERFACE,
            "endpoint" => Self::ENDPOINT,
            "other" => Self::OTHER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<USBRecipient> for Any {
    fn from(s: USBRecipient) -> Any {
         match s {
            USBRecipient::DEVICE => Any::from("device"),
            USBRecipient::INTERFACE => Any::from("interface"),
            USBRecipient::ENDPOINT => Any::from("endpoint"),
            USBRecipient::OTHER => Any::from("other"),
         }
    }
}
impl From<&USBRecipient> for Any {
    fn from(s: &USBRecipient) -> Any {
         match *s {
            USBRecipient::DEVICE => Any::from("device"),
            USBRecipient::INTERFACE => Any::from("interface"),
            USBRecipient::ENDPOINT => Any::from("endpoint"),
            USBRecipient::OTHER => Any::from("other"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum USBDirection {
    IN_,
    OUT,
}
impl FromVal for USBDirection {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "in" => Self::IN_,
            "out" => Self::OUT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<USBDirection> for Any {
    fn from(s: USBDirection) -> Any {
         match s {
            USBDirection::IN_ => Any::from("in"),
            USBDirection::OUT => Any::from("out"),
         }
    }
}
impl From<&USBDirection> for Any {
    fn from(s: &USBDirection) -> Any {
         match *s {
            USBDirection::IN_ => Any::from("in"),
            USBDirection::OUT => Any::from("out"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "bulk" => Self::BULK,
            "interrupt" => Self::INTERRUPT,
            "isochronous" => Self::ISOCHRONOUS,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<USBEndpointType> for Any {
    fn from(s: USBEndpointType) -> Any {
         match s {
            USBEndpointType::BULK => Any::from("bulk"),
            USBEndpointType::INTERRUPT => Any::from("interrupt"),
            USBEndpointType::ISOCHRONOUS => Any::from("isochronous"),
         }
    }
}
impl From<&USBEndpointType> for Any {
    fn from(s: &USBEndpointType) -> Any {
         match *s {
            USBEndpointType::BULK => Any::from("bulk"),
            USBEndpointType::INTERRUPT => Any::from("interrupt"),
            USBEndpointType::ISOCHRONOUS => Any::from("isochronous"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AutoKeyword {
    AUTO,
}
impl FromVal for AutoKeyword {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "auto" => Self::AUTO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AutoKeyword> for Any {
    fn from(s: AutoKeyword) -> Any {
         match s {
            AutoKeyword::AUTO => Any::from("auto"),
         }
    }
}
impl From<&AutoKeyword> for Any {
    fn from(s: &AutoKeyword) -> Any {
         match *s {
            AutoKeyword::AUTO => Any::from("auto"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "" => Self::NONE,
            "rl" => Self::RL,
            "lr" => Self::LR,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<DirectionSetting> for Any {
    fn from(s: DirectionSetting) -> Any {
         match s {
            DirectionSetting::NONE => Any::from(""),
            DirectionSetting::RL => Any::from("rl"),
            DirectionSetting::LR => Any::from("lr"),
         }
    }
}
impl From<&DirectionSetting> for Any {
    fn from(s: &DirectionSetting) -> Any {
         match *s {
            DirectionSetting::NONE => Any::from(""),
            DirectionSetting::RL => Any::from("rl"),
            DirectionSetting::LR => Any::from("lr"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "start" => Self::START,
            "center" => Self::CENTER,
            "end" => Self::END,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<LineAlignSetting> for Any {
    fn from(s: LineAlignSetting) -> Any {
         match s {
            LineAlignSetting::START => Any::from("start"),
            LineAlignSetting::CENTER => Any::from("center"),
            LineAlignSetting::END => Any::from("end"),
         }
    }
}
impl From<&LineAlignSetting> for Any {
    fn from(s: &LineAlignSetting) -> Any {
         match *s {
            LineAlignSetting::START => Any::from("start"),
            LineAlignSetting::CENTER => Any::from("center"),
            LineAlignSetting::END => Any::from("end"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "line-left" => Self::LINE_LEFT,
            "center" => Self::CENTER,
            "line-right" => Self::LINE_RIGHT,
            "auto" => Self::AUTO,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<PositionAlignSetting> for Any {
    fn from(s: PositionAlignSetting) -> Any {
         match s {
            PositionAlignSetting::LINE_LEFT => Any::from("line-left"),
            PositionAlignSetting::CENTER => Any::from("center"),
            PositionAlignSetting::LINE_RIGHT => Any::from("line-right"),
            PositionAlignSetting::AUTO => Any::from("auto"),
         }
    }
}
impl From<&PositionAlignSetting> for Any {
    fn from(s: &PositionAlignSetting) -> Any {
         match *s {
            PositionAlignSetting::LINE_LEFT => Any::from("line-left"),
            PositionAlignSetting::CENTER => Any::from("center"),
            PositionAlignSetting::LINE_RIGHT => Any::from("line-right"),
            PositionAlignSetting::AUTO => Any::from("auto"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "start" => Self::START,
            "center" => Self::CENTER,
            "end" => Self::END,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<AlignSetting> for Any {
    fn from(s: AlignSetting) -> Any {
         match s {
            AlignSetting::START => Any::from("start"),
            AlignSetting::CENTER => Any::from("center"),
            AlignSetting::END => Any::from("end"),
            AlignSetting::LEFT => Any::from("left"),
            AlignSetting::RIGHT => Any::from("right"),
         }
    }
}
impl From<&AlignSetting> for Any {
    fn from(s: &AlignSetting) -> Any {
         match *s {
            AlignSetting::START => Any::from("start"),
            AlignSetting::CENTER => Any::from("center"),
            AlignSetting::END => Any::from("end"),
            AlignSetting::LEFT => Any::from("left"),
            AlignSetting::RIGHT => Any::from("right"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ScrollSetting {
    NONE,
    UP,
}
impl FromVal for ScrollSetting {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "" => Self::NONE,
            "up" => Self::UP,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<ScrollSetting> for Any {
    fn from(s: ScrollSetting) -> Any {
         match s {
            ScrollSetting::NONE => Any::from(""),
            ScrollSetting::UP => Any::from("up"),
         }
    }
}
impl From<&ScrollSetting> for Any {
    fn from(s: &ScrollSetting) -> Any {
         match *s {
            ScrollSetting::NONE => Any::from(""),
            ScrollSetting::UP => Any::from("up"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "opaque" => Self::OPAQUE,
            "alpha-blend" => Self::ALPHA_BLEND,
            "additive" => Self::ADDITIVE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XREnvironmentBlendMode> for Any {
    fn from(s: XREnvironmentBlendMode) -> Any {
         match s {
            XREnvironmentBlendMode::OPAQUE => Any::from("opaque"),
            XREnvironmentBlendMode::ALPHA_BLEND => Any::from("alpha-blend"),
            XREnvironmentBlendMode::ADDITIVE => Any::from("additive"),
         }
    }
}
impl From<&XREnvironmentBlendMode> for Any {
    fn from(s: &XREnvironmentBlendMode) -> Any {
         match *s {
            XREnvironmentBlendMode::OPAQUE => Any::from("opaque"),
            XREnvironmentBlendMode::ALPHA_BLEND => Any::from("alpha-blend"),
            XREnvironmentBlendMode::ADDITIVE => Any::from("additive"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRInteractionMode {
    SCREEN_SPACE,
    WORLD_SPACE,
}
impl FromVal for XRInteractionMode {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "screen-space" => Self::SCREEN_SPACE,
            "world-space" => Self::WORLD_SPACE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRInteractionMode> for Any {
    fn from(s: XRInteractionMode) -> Any {
         match s {
            XRInteractionMode::SCREEN_SPACE => Any::from("screen-space"),
            XRInteractionMode::WORLD_SPACE => Any::from("world-space"),
         }
    }
}
impl From<&XRInteractionMode> for Any {
    fn from(s: &XRInteractionMode) -> Any {
         match *s {
            XRInteractionMode::SCREEN_SPACE => Any::from("screen-space"),
            XRInteractionMode::WORLD_SPACE => Any::from("world-space"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRDepthType {
    RAW_,
    SMOOTH,
}
impl FromVal for XRDepthType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "raw" => Self::RAW_,
            "smooth" => Self::SMOOTH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRDepthType> for Any {
    fn from(s: XRDepthType) -> Any {
         match s {
            XRDepthType::RAW_ => Any::from("raw"),
            XRDepthType::SMOOTH => Any::from("smooth"),
         }
    }
}
impl From<&XRDepthType> for Any {
    fn from(s: &XRDepthType) -> Any {
         match *s {
            XRDepthType::RAW_ => Any::from("raw"),
            XRDepthType::SMOOTH => Any::from("smooth"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRDepthUsage {
    CPU_OPTIMIZED,
    GPU_OPTIMIZED,
}
impl FromVal for XRDepthUsage {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "cpu-optimized" => Self::CPU_OPTIMIZED,
            "gpu-optimized" => Self::GPU_OPTIMIZED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRDepthUsage> for Any {
    fn from(s: XRDepthUsage) -> Any {
         match s {
            XRDepthUsage::CPU_OPTIMIZED => Any::from("cpu-optimized"),
            XRDepthUsage::GPU_OPTIMIZED => Any::from("gpu-optimized"),
         }
    }
}
impl From<&XRDepthUsage> for Any {
    fn from(s: &XRDepthUsage) -> Any {
         match *s {
            XRDepthUsage::CPU_OPTIMIZED => Any::from("cpu-optimized"),
            XRDepthUsage::GPU_OPTIMIZED => Any::from("gpu-optimized"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "luminance-alpha" => Self::LUMINANCE_ALPHA,
            "float32" => Self::FLOAT32,
            "unsigned-short" => Self::UNSIGNED_SHORT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRDepthDataFormat> for Any {
    fn from(s: XRDepthDataFormat) -> Any {
         match s {
            XRDepthDataFormat::LUMINANCE_ALPHA => Any::from("luminance-alpha"),
            XRDepthDataFormat::FLOAT32 => Any::from("float32"),
            XRDepthDataFormat::UNSIGNED_SHORT => Any::from("unsigned-short"),
         }
    }
}
impl From<&XRDepthDataFormat> for Any {
    fn from(s: &XRDepthDataFormat) -> Any {
         match *s {
            XRDepthDataFormat::LUMINANCE_ALPHA => Any::from("luminance-alpha"),
            XRDepthDataFormat::FLOAT32 => Any::from("float32"),
            XRDepthDataFormat::UNSIGNED_SHORT => Any::from("unsigned-short"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "screen" => Self::SCREEN,
            "floating" => Self::FLOATING,
            "head-locked" => Self::HEAD_LOCKED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRDOMOverlayType> for Any {
    fn from(s: XRDOMOverlayType) -> Any {
         match s {
            XRDOMOverlayType::SCREEN => Any::from("screen"),
            XRDOMOverlayType::FLOATING => Any::from("floating"),
            XRDOMOverlayType::HEAD_LOCKED => Any::from("head-locked"),
         }
    }
}
impl From<&XRDOMOverlayType> for Any {
    fn from(s: &XRDOMOverlayType) -> Any {
         match *s {
            XRDOMOverlayType::SCREEN => Any::from("screen"),
            XRDOMOverlayType::FLOATING => Any::from("floating"),
            XRDOMOverlayType::HEAD_LOCKED => Any::from("head-locked"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
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
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRHandJoint> for Any {
    fn from(s: XRHandJoint) -> Any {
         match s {
            XRHandJoint::WRIST => Any::from("wrist"),
            XRHandJoint::THUMB_METACARPAL => Any::from("thumb-metacarpal"),
            XRHandJoint::THUMB_PHALANX_PROXIMAL => Any::from("thumb-phalanx-proximal"),
            XRHandJoint::THUMB_PHALANX_DISTAL => Any::from("thumb-phalanx-distal"),
            XRHandJoint::THUMB_TIP => Any::from("thumb-tip"),
            XRHandJoint::INDEX_FINGER_METACARPAL => Any::from("index-finger-metacarpal"),
            XRHandJoint::INDEX_FINGER_PHALANX_PROXIMAL => Any::from("index-finger-phalanx-proximal"),
            XRHandJoint::INDEX_FINGER_PHALANX_INTERMEDIATE => Any::from("index-finger-phalanx-intermediate"),
            XRHandJoint::INDEX_FINGER_PHALANX_DISTAL => Any::from("index-finger-phalanx-distal"),
            XRHandJoint::INDEX_FINGER_TIP => Any::from("index-finger-tip"),
            XRHandJoint::MIDDLE_FINGER_METACARPAL => Any::from("middle-finger-metacarpal"),
            XRHandJoint::MIDDLE_FINGER_PHALANX_PROXIMAL => Any::from("middle-finger-phalanx-proximal"),
            XRHandJoint::MIDDLE_FINGER_PHALANX_INTERMEDIATE => Any::from("middle-finger-phalanx-intermediate"),
            XRHandJoint::MIDDLE_FINGER_PHALANX_DISTAL => Any::from("middle-finger-phalanx-distal"),
            XRHandJoint::MIDDLE_FINGER_TIP => Any::from("middle-finger-tip"),
            XRHandJoint::RING_FINGER_METACARPAL => Any::from("ring-finger-metacarpal"),
            XRHandJoint::RING_FINGER_PHALANX_PROXIMAL => Any::from("ring-finger-phalanx-proximal"),
            XRHandJoint::RING_FINGER_PHALANX_INTERMEDIATE => Any::from("ring-finger-phalanx-intermediate"),
            XRHandJoint::RING_FINGER_PHALANX_DISTAL => Any::from("ring-finger-phalanx-distal"),
            XRHandJoint::RING_FINGER_TIP => Any::from("ring-finger-tip"),
            XRHandJoint::PINKY_FINGER_METACARPAL => Any::from("pinky-finger-metacarpal"),
            XRHandJoint::PINKY_FINGER_PHALANX_PROXIMAL => Any::from("pinky-finger-phalanx-proximal"),
            XRHandJoint::PINKY_FINGER_PHALANX_INTERMEDIATE => Any::from("pinky-finger-phalanx-intermediate"),
            XRHandJoint::PINKY_FINGER_PHALANX_DISTAL => Any::from("pinky-finger-phalanx-distal"),
            XRHandJoint::PINKY_FINGER_TIP => Any::from("pinky-finger-tip"),
         }
    }
}
impl From<&XRHandJoint> for Any {
    fn from(s: &XRHandJoint) -> Any {
         match *s {
            XRHandJoint::WRIST => Any::from("wrist"),
            XRHandJoint::THUMB_METACARPAL => Any::from("thumb-metacarpal"),
            XRHandJoint::THUMB_PHALANX_PROXIMAL => Any::from("thumb-phalanx-proximal"),
            XRHandJoint::THUMB_PHALANX_DISTAL => Any::from("thumb-phalanx-distal"),
            XRHandJoint::THUMB_TIP => Any::from("thumb-tip"),
            XRHandJoint::INDEX_FINGER_METACARPAL => Any::from("index-finger-metacarpal"),
            XRHandJoint::INDEX_FINGER_PHALANX_PROXIMAL => Any::from("index-finger-phalanx-proximal"),
            XRHandJoint::INDEX_FINGER_PHALANX_INTERMEDIATE => Any::from("index-finger-phalanx-intermediate"),
            XRHandJoint::INDEX_FINGER_PHALANX_DISTAL => Any::from("index-finger-phalanx-distal"),
            XRHandJoint::INDEX_FINGER_TIP => Any::from("index-finger-tip"),
            XRHandJoint::MIDDLE_FINGER_METACARPAL => Any::from("middle-finger-metacarpal"),
            XRHandJoint::MIDDLE_FINGER_PHALANX_PROXIMAL => Any::from("middle-finger-phalanx-proximal"),
            XRHandJoint::MIDDLE_FINGER_PHALANX_INTERMEDIATE => Any::from("middle-finger-phalanx-intermediate"),
            XRHandJoint::MIDDLE_FINGER_PHALANX_DISTAL => Any::from("middle-finger-phalanx-distal"),
            XRHandJoint::MIDDLE_FINGER_TIP => Any::from("middle-finger-tip"),
            XRHandJoint::RING_FINGER_METACARPAL => Any::from("ring-finger-metacarpal"),
            XRHandJoint::RING_FINGER_PHALANX_PROXIMAL => Any::from("ring-finger-phalanx-proximal"),
            XRHandJoint::RING_FINGER_PHALANX_INTERMEDIATE => Any::from("ring-finger-phalanx-intermediate"),
            XRHandJoint::RING_FINGER_PHALANX_DISTAL => Any::from("ring-finger-phalanx-distal"),
            XRHandJoint::RING_FINGER_TIP => Any::from("ring-finger-tip"),
            XRHandJoint::PINKY_FINGER_METACARPAL => Any::from("pinky-finger-metacarpal"),
            XRHandJoint::PINKY_FINGER_PHALANX_PROXIMAL => Any::from("pinky-finger-phalanx-proximal"),
            XRHandJoint::PINKY_FINGER_PHALANX_INTERMEDIATE => Any::from("pinky-finger-phalanx-intermediate"),
            XRHandJoint::PINKY_FINGER_PHALANX_DISTAL => Any::from("pinky-finger-phalanx-distal"),
            XRHandJoint::PINKY_FINGER_TIP => Any::from("pinky-finger-tip"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "point" => Self::POINT,
            "plane" => Self::PLANE,
            "mesh" => Self::MESH,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRHitTestTrackableType> for Any {
    fn from(s: XRHitTestTrackableType) -> Any {
         match s {
            XRHitTestTrackableType::POINT => Any::from("point"),
            XRHitTestTrackableType::PLANE => Any::from("plane"),
            XRHitTestTrackableType::MESH => Any::from("mesh"),
         }
    }
}
impl From<&XRHitTestTrackableType> for Any {
    fn from(s: &XRHitTestTrackableType) -> Any {
         match *s {
            XRHitTestTrackableType::POINT => Any::from("point"),
            XRHitTestTrackableType::PLANE => Any::from("plane"),
            XRHitTestTrackableType::MESH => Any::from("mesh"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRReflectionFormat {
    SRGBA8,
    RGBA16F,
}
impl FromVal for XRReflectionFormat {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "srgba8" => Self::SRGBA8,
            "rgba16f" => Self::RGBA16F,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRReflectionFormat> for Any {
    fn from(s: XRReflectionFormat) -> Any {
         match s {
            XRReflectionFormat::SRGBA8 => Any::from("srgba8"),
            XRReflectionFormat::RGBA16F => Any::from("rgba16f"),
         }
    }
}
impl From<&XRReflectionFormat> for Any {
    fn from(s: &XRReflectionFormat) -> Any {
         match *s {
            XRReflectionFormat::SRGBA8 => Any::from("srgba8"),
            XRReflectionFormat::RGBA16F => Any::from("rgba16f"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRPlaneOrientation {
    HORIZONTAL,
    VERTICAL,
}
impl FromVal for XRPlaneOrientation {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "horizontal" => Self::HORIZONTAL,
            "vertical" => Self::VERTICAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRPlaneOrientation> for Any {
    fn from(s: XRPlaneOrientation) -> Any {
         match s {
            XRPlaneOrientation::HORIZONTAL => Any::from("horizontal"),
            XRPlaneOrientation::VERTICAL => Any::from("vertical"),
         }
    }
}
impl From<&XRPlaneOrientation> for Any {
    fn from(s: &XRPlaneOrientation) -> Any {
         match *s {
            XRPlaneOrientation::HORIZONTAL => Any::from("horizontal"),
            XRPlaneOrientation::VERTICAL => Any::from("vertical"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "inline" => Self::INLINE,
            "immersive-vr" => Self::IMMERSIVE_VR,
            "immersive-ar" => Self::IMMERSIVE_AR,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRSessionMode> for Any {
    fn from(s: XRSessionMode) -> Any {
         match s {
            XRSessionMode::INLINE => Any::from("inline"),
            XRSessionMode::IMMERSIVE_VR => Any::from("immersive-vr"),
            XRSessionMode::IMMERSIVE_AR => Any::from("immersive-ar"),
         }
    }
}
impl From<&XRSessionMode> for Any {
    fn from(s: &XRSessionMode) -> Any {
         match *s {
            XRSessionMode::INLINE => Any::from("inline"),
            XRSessionMode::IMMERSIVE_VR => Any::from("immersive-vr"),
            XRSessionMode::IMMERSIVE_AR => Any::from("immersive-ar"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "visible" => Self::VISIBLE,
            "visible-blurred" => Self::VISIBLE_BLURRED,
            "hidden" => Self::HIDDEN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRVisibilityState> for Any {
    fn from(s: XRVisibilityState) -> Any {
         match s {
            XRVisibilityState::VISIBLE => Any::from("visible"),
            XRVisibilityState::VISIBLE_BLURRED => Any::from("visible-blurred"),
            XRVisibilityState::HIDDEN => Any::from("hidden"),
         }
    }
}
impl From<&XRVisibilityState> for Any {
    fn from(s: &XRVisibilityState) -> Any {
         match *s {
            XRVisibilityState::VISIBLE => Any::from("visible"),
            XRVisibilityState::VISIBLE_BLURRED => Any::from("visible-blurred"),
            XRVisibilityState::HIDDEN => Any::from("hidden"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "viewer" => Self::VIEWER,
            "local" => Self::LOCAL,
            "local-floor" => Self::LOCAL_FLOOR,
            "bounded-floor" => Self::BOUNDED_FLOOR,
            "unbounded" => Self::UNBOUNDED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRReferenceSpaceType> for Any {
    fn from(s: XRReferenceSpaceType) -> Any {
         match s {
            XRReferenceSpaceType::VIEWER => Any::from("viewer"),
            XRReferenceSpaceType::LOCAL => Any::from("local"),
            XRReferenceSpaceType::LOCAL_FLOOR => Any::from("local-floor"),
            XRReferenceSpaceType::BOUNDED_FLOOR => Any::from("bounded-floor"),
            XRReferenceSpaceType::UNBOUNDED => Any::from("unbounded"),
         }
    }
}
impl From<&XRReferenceSpaceType> for Any {
    fn from(s: &XRReferenceSpaceType) -> Any {
         match *s {
            XRReferenceSpaceType::VIEWER => Any::from("viewer"),
            XRReferenceSpaceType::LOCAL => Any::from("local"),
            XRReferenceSpaceType::LOCAL_FLOOR => Any::from("local-floor"),
            XRReferenceSpaceType::BOUNDED_FLOOR => Any::from("bounded-floor"),
            XRReferenceSpaceType::UNBOUNDED => Any::from("unbounded"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XREye> for Any {
    fn from(s: XREye) -> Any {
         match s {
            XREye::NONE => Any::from("none"),
            XREye::LEFT => Any::from("left"),
            XREye::RIGHT => Any::from("right"),
         }
    }
}
impl From<&XREye> for Any {
    fn from(s: &XREye) -> Any {
         match *s {
            XREye::NONE => Any::from("none"),
            XREye::LEFT => Any::from("left"),
            XREye::RIGHT => Any::from("right"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "none" => Self::NONE,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRHandedness> for Any {
    fn from(s: XRHandedness) -> Any {
         match s {
            XRHandedness::NONE => Any::from("none"),
            XRHandedness::LEFT => Any::from("left"),
            XRHandedness::RIGHT => Any::from("right"),
         }
    }
}
impl From<&XRHandedness> for Any {
    fn from(s: &XRHandedness) -> Any {
         match *s {
            XRHandedness::NONE => Any::from("none"),
            XRHandedness::LEFT => Any::from("left"),
            XRHandedness::RIGHT => Any::from("right"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "gaze" => Self::GAZE,
            "tracked-pointer" => Self::TRACKED_POINTER,
            "screen" => Self::SCREEN,
            "transient-pointer" => Self::TRANSIENT_POINTER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRTargetRayMode> for Any {
    fn from(s: XRTargetRayMode) -> Any {
         match s {
            XRTargetRayMode::GAZE => Any::from("gaze"),
            XRTargetRayMode::TRACKED_POINTER => Any::from("tracked-pointer"),
            XRTargetRayMode::SCREEN => Any::from("screen"),
            XRTargetRayMode::TRANSIENT_POINTER => Any::from("transient-pointer"),
         }
    }
}
impl From<&XRTargetRayMode> for Any {
    fn from(s: &XRTargetRayMode) -> Any {
         match *s {
            XRTargetRayMode::GAZE => Any::from("gaze"),
            XRTargetRayMode::TRACKED_POINTER => Any::from("tracked-pointer"),
            XRTargetRayMode::SCREEN => Any::from("screen"),
            XRTargetRayMode::TRANSIENT_POINTER => Any::from("transient-pointer"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "default" => Self::DEFAULT,
            "mono" => Self::MONO,
            "stereo" => Self::STEREO,
            "stereo-left-right" => Self::STEREO_LEFT_RIGHT,
            "stereo-top-bottom" => Self::STEREO_TOP_BOTTOM,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRLayerLayout> for Any {
    fn from(s: XRLayerLayout) -> Any {
         match s {
            XRLayerLayout::DEFAULT => Any::from("default"),
            XRLayerLayout::MONO => Any::from("mono"),
            XRLayerLayout::STEREO => Any::from("stereo"),
            XRLayerLayout::STEREO_LEFT_RIGHT => Any::from("stereo-left-right"),
            XRLayerLayout::STEREO_TOP_BOTTOM => Any::from("stereo-top-bottom"),
         }
    }
}
impl From<&XRLayerLayout> for Any {
    fn from(s: &XRLayerLayout) -> Any {
         match *s {
            XRLayerLayout::DEFAULT => Any::from("default"),
            XRLayerLayout::MONO => Any::from("mono"),
            XRLayerLayout::STEREO => Any::from("stereo"),
            XRLayerLayout::STEREO_LEFT_RIGHT => Any::from("stereo-left-right"),
            XRLayerLayout::STEREO_TOP_BOTTOM => Any::from("stereo-top-bottom"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "default" => Self::DEFAULT,
            "text-optimized" => Self::TEXT_OPTIMIZED,
            "graphics-optimized" => Self::GRAPHICS_OPTIMIZED,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRLayerQuality> for Any {
    fn from(s: XRLayerQuality) -> Any {
         match s {
            XRLayerQuality::DEFAULT => Any::from("default"),
            XRLayerQuality::TEXT_OPTIMIZED => Any::from("text-optimized"),
            XRLayerQuality::GRAPHICS_OPTIMIZED => Any::from("graphics-optimized"),
         }
    }
}
impl From<&XRLayerQuality> for Any {
    fn from(s: &XRLayerQuality) -> Any {
         match *s {
            XRLayerQuality::DEFAULT => Any::from("default"),
            XRLayerQuality::TEXT_OPTIMIZED => Any::from("text-optimized"),
            XRLayerQuality::GRAPHICS_OPTIMIZED => Any::from("graphics-optimized"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum XRTextureType {
    TEXTURE,
    TEXTURE_ARRAY,
}
impl FromVal for XRTextureType {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "texture" => Self::TEXTURE,
            "texture-array" => Self::TEXTURE_ARRAY,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XRTextureType> for Any {
    fn from(s: XRTextureType) -> Any {
         match s {
            XRTextureType::TEXTURE => Any::from("texture"),
            XRTextureType::TEXTURE_ARRAY => Any::from("texture-array"),
         }
    }
}
impl From<&XRTextureType> for Any {
    fn from(s: &XRTextureType) -> Any {
         match *s {
            XRTextureType::TEXTURE => Any::from("texture"),
            XRTextureType::TEXTURE_ARRAY => Any::from("texture-array"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "tldr" => Self::TLDR,
            "teaser" => Self::TEASER,
            "key-points" => Self::KEY_POINTS,
            "headline" => Self::HEADLINE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SummarizerType> for Any {
    fn from(s: SummarizerType) -> Any {
         match s {
            SummarizerType::TLDR => Any::from("tldr"),
            SummarizerType::TEASER => Any::from("teaser"),
            SummarizerType::KEY_POINTS => Any::from("key-points"),
            SummarizerType::HEADLINE => Any::from("headline"),
         }
    }
}
impl From<&SummarizerType> for Any {
    fn from(s: &SummarizerType) -> Any {
         match *s {
            SummarizerType::TLDR => Any::from("tldr"),
            SummarizerType::TEASER => Any::from("teaser"),
            SummarizerType::KEY_POINTS => Any::from("key-points"),
            SummarizerType::HEADLINE => Any::from("headline"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SummarizerFormat {
    PLAIN_TEXT,
    MARKDOWN,
}
impl FromVal for SummarizerFormat {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "plain-text" => Self::PLAIN_TEXT,
            "markdown" => Self::MARKDOWN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SummarizerFormat> for Any {
    fn from(s: SummarizerFormat) -> Any {
         match s {
            SummarizerFormat::PLAIN_TEXT => Any::from("plain-text"),
            SummarizerFormat::MARKDOWN => Any::from("markdown"),
         }
    }
}
impl From<&SummarizerFormat> for Any {
    fn from(s: &SummarizerFormat) -> Any {
         match *s {
            SummarizerFormat::PLAIN_TEXT => Any::from("plain-text"),
            SummarizerFormat::MARKDOWN => Any::from("markdown"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "short" => Self::SHORT,
            "medium" => Self::MEDIUM,
            "long" => Self::LONG,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<SummarizerLength> for Any {
    fn from(s: SummarizerLength) -> Any {
         match s {
            SummarizerLength::SHORT => Any::from("short"),
            SummarizerLength::MEDIUM => Any::from("medium"),
            SummarizerLength::LONG => Any::from("long"),
         }
    }
}
impl From<&SummarizerLength> for Any {
    fn from(s: &SummarizerLength) -> Any {
         match *s {
            SummarizerLength::SHORT => Any::from("short"),
            SummarizerLength::MEDIUM => Any::from("medium"),
            SummarizerLength::LONG => Any::from("long"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "formal" => Self::FORMAL,
            "neutral" => Self::NEUTRAL,
            "casual" => Self::CASUAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WriterTone> for Any {
    fn from(s: WriterTone) -> Any {
         match s {
            WriterTone::FORMAL => Any::from("formal"),
            WriterTone::NEUTRAL => Any::from("neutral"),
            WriterTone::CASUAL => Any::from("casual"),
         }
    }
}
impl From<&WriterTone> for Any {
    fn from(s: &WriterTone) -> Any {
         match *s {
            WriterTone::FORMAL => Any::from("formal"),
            WriterTone::NEUTRAL => Any::from("neutral"),
            WriterTone::CASUAL => Any::from("casual"),
         }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WriterFormat {
    PLAIN_TEXT,
    MARKDOWN,
}
impl FromVal for WriterFormat {
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "plain-text" => Self::PLAIN_TEXT,
            "markdown" => Self::MARKDOWN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WriterFormat> for Any {
    fn from(s: WriterFormat) -> Any {
         match s {
            WriterFormat::PLAIN_TEXT => Any::from("plain-text"),
            WriterFormat::MARKDOWN => Any::from("markdown"),
         }
    }
}
impl From<&WriterFormat> for Any {
    fn from(s: &WriterFormat) -> Any {
         match *s {
            WriterFormat::PLAIN_TEXT => Any::from("plain-text"),
            WriterFormat::MARKDOWN => Any::from("markdown"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "short" => Self::SHORT,
            "medium" => Self::MEDIUM,
            "long" => Self::LONG,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<WriterLength> for Any {
    fn from(s: WriterLength) -> Any {
         match s {
            WriterLength::SHORT => Any::from("short"),
            WriterLength::MEDIUM => Any::from("medium"),
            WriterLength::LONG => Any::from("long"),
         }
    }
}
impl From<&WriterLength> for Any {
    fn from(s: &WriterLength) -> Any {
         match *s {
            WriterLength::SHORT => Any::from("short"),
            WriterLength::MEDIUM => Any::from("medium"),
            WriterLength::LONG => Any::from("long"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "as-is" => Self::AS_IS,
            "more-formal" => Self::MORE_FORMAL,
            "more-casual" => Self::MORE_CASUAL,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RewriterTone> for Any {
    fn from(s: RewriterTone) -> Any {
         match s {
            RewriterTone::AS_IS => Any::from("as-is"),
            RewriterTone::MORE_FORMAL => Any::from("more-formal"),
            RewriterTone::MORE_CASUAL => Any::from("more-casual"),
         }
    }
}
impl From<&RewriterTone> for Any {
    fn from(s: &RewriterTone) -> Any {
         match *s {
            RewriterTone::AS_IS => Any::from("as-is"),
            RewriterTone::MORE_FORMAL => Any::from("more-formal"),
            RewriterTone::MORE_CASUAL => Any::from("more-casual"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "as-is" => Self::AS_IS,
            "plain-text" => Self::PLAIN_TEXT,
            "markdown" => Self::MARKDOWN,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RewriterFormat> for Any {
    fn from(s: RewriterFormat) -> Any {
         match s {
            RewriterFormat::AS_IS => Any::from("as-is"),
            RewriterFormat::PLAIN_TEXT => Any::from("plain-text"),
            RewriterFormat::MARKDOWN => Any::from("markdown"),
         }
    }
}
impl From<&RewriterFormat> for Any {
    fn from(s: &RewriterFormat) -> Any {
         match *s {
            RewriterFormat::AS_IS => Any::from("as-is"),
            RewriterFormat::PLAIN_TEXT => Any::from("plain-text"),
            RewriterFormat::MARKDOWN => Any::from("markdown"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "as-is" => Self::AS_IS,
            "shorter" => Self::SHORTER,
            "longer" => Self::LONGER,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<RewriterLength> for Any {
    fn from(s: RewriterLength) -> Any {
         match s {
            RewriterLength::AS_IS => Any::from("as-is"),
            RewriterLength::SHORTER => Any::from("shorter"),
            RewriterLength::LONGER => Any::from("longer"),
         }
    }
}
impl From<&RewriterLength> for Any {
    fn from(s: &RewriterLength) -> Any {
         match *s {
            RewriterLength::AS_IS => Any::from("as-is"),
            RewriterLength::SHORTER => Any::from("shorter"),
            RewriterLength::LONGER => Any::from("longer"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "unavailable" => Self::UNAVAILABLE,
            "downloadable" => Self::DOWNLOADABLE,
            "downloading" => Self::DOWNLOADING,
            "available" => Self::AVAILABLE,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<Availability> for Any {
    fn from(s: Availability) -> Any {
         match s {
            Availability::UNAVAILABLE => Any::from("unavailable"),
            Availability::DOWNLOADABLE => Any::from("downloadable"),
            Availability::DOWNLOADING => Any::from("downloading"),
            Availability::AVAILABLE => Any::from("available"),
         }
    }
}
impl From<&Availability> for Any {
    fn from(s: &Availability) -> Any {
         match *s {
            Availability::UNAVAILABLE => Any::from("unavailable"),
            Availability::DOWNLOADABLE => Any::from("downloadable"),
            Availability::DOWNLOADING => Any::from("downloading"),
            Availability::AVAILABLE => Any::from("available"),
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
    fn from_val(v: &Any) -> Self {
         match v.as_::<Option<&str>>().unwrap() {
            "" => Self::NONE,
            "arraybuffer" => Self::ARRAYBUFFER,
            "blob" => Self::BLOB,
            "document" => Self::DOCUMENT,
            "json" => Self::JSON,
            "text" => Self::TEXT,
             _ => unreachable!(),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        Any::from(*self).as_handle()
    }
}
impl From<XMLHttpRequestResponseType> for Any {
    fn from(s: XMLHttpRequestResponseType) -> Any {
         match s {
            XMLHttpRequestResponseType::NONE => Any::from(""),
            XMLHttpRequestResponseType::ARRAYBUFFER => Any::from("arraybuffer"),
            XMLHttpRequestResponseType::BLOB => Any::from("blob"),
            XMLHttpRequestResponseType::DOCUMENT => Any::from("document"),
            XMLHttpRequestResponseType::JSON => Any::from("json"),
            XMLHttpRequestResponseType::TEXT => Any::from("text"),
         }
    }
}
impl From<&XMLHttpRequestResponseType> for Any {
    fn from(s: &XMLHttpRequestResponseType) -> Any {
         match *s {
            XMLHttpRequestResponseType::NONE => Any::from(""),
            XMLHttpRequestResponseType::ARRAYBUFFER => Any::from("arraybuffer"),
            XMLHttpRequestResponseType::BLOB => Any::from("blob"),
            XMLHttpRequestResponseType::DOCUMENT => Any::from("document"),
            XMLHttpRequestResponseType::JSON => Any::from("json"),
            XMLHttpRequestResponseType::TEXT => Any::from("text"),
         }
    }
}


