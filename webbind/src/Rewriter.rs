use super::*;

#[derive(Clone, Debug)]
pub struct RewriterCreateOptions {
    inner: emlite::Val,
}
impl FromVal for RewriterCreateOptions {
    fn from_val(v: &emlite::Val) -> Self {
        RewriterCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RewriterCreateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RewriterCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RewriterCreateOptions> for emlite::Val {
    fn from(s: RewriterCreateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RewriterCreateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl RewriterCreateOptions {
    pub fn monitor(&self) -> jsbind::Function {
        self.inner.get("monitor").as_::<jsbind::Function>()
    }

    pub fn set_monitor(&mut self, value: jsbind::Function) {
        self.inner.set("monitor", value);
    }
}
impl RewriterCreateOptions {
    pub fn shared_context(&self) -> jsbind::DOMString {
        self.inner.get("sharedContext").as_::<jsbind::DOMString>()
    }

    pub fn set_shared_context(&mut self, value: jsbind::DOMString) {
        self.inner.set("sharedContext", value);
    }
}
#[derive(Clone, Debug)]
pub struct RewriterCreateCoreOptions {
    inner: emlite::Val,
}
impl FromVal for RewriterCreateCoreOptions {
    fn from_val(v: &emlite::Val) -> Self {
        RewriterCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RewriterCreateCoreOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RewriterCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RewriterCreateCoreOptions> for emlite::Val {
    fn from(s: RewriterCreateCoreOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RewriterCreateCoreOptions {
    pub fn tone(&self) -> RewriterTone {
        self.inner.get("tone").as_::<RewriterTone>()
    }

    pub fn set_tone(&mut self, value: RewriterTone) {
        self.inner.set("tone", value);
    }
}
impl RewriterCreateCoreOptions {
    pub fn format(&self) -> RewriterFormat {
        self.inner.get("format").as_::<RewriterFormat>()
    }

    pub fn set_format(&mut self, value: RewriterFormat) {
        self.inner.set("format", value);
    }
}
impl RewriterCreateCoreOptions {
    pub fn length(&self) -> RewriterLength {
        self.inner.get("length").as_::<RewriterLength>()
    }

    pub fn set_length(&mut self, value: RewriterLength) {
        self.inner.set("length", value);
    }
}
impl RewriterCreateCoreOptions {
    pub fn expected_input_languages(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_expected_input_languages(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
impl RewriterCreateCoreOptions {
    pub fn expected_context_languages(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_expected_context_languages(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
        self.inner.set("expectedContextLanguages", value);
    }
}
impl RewriterCreateCoreOptions {
    pub fn output_language(&self) -> jsbind::DOMString {
        self.inner.get("outputLanguage").as_::<jsbind::DOMString>()
    }

    pub fn set_output_language(&mut self, value: jsbind::DOMString) {
        self.inner.set("outputLanguage", value);
    }
}
#[derive(Clone, Debug)]
pub struct RewriterRewriteOptions {
    inner: emlite::Val,
}
impl FromVal for RewriterRewriteOptions {
    fn from_val(v: &emlite::Val) -> Self {
        RewriterRewriteOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RewriterRewriteOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RewriterRewriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RewriterRewriteOptions> for emlite::Val {
    fn from(s: RewriterRewriteOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RewriterRewriteOptions {
    pub fn context(&self) -> jsbind::DOMString {
        self.inner.get("context").as_::<jsbind::DOMString>()
    }

    pub fn set_context(&mut self, value: jsbind::DOMString) {
        self.inner.set("context", value);
    }
}
impl RewriterRewriteOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug)]
pub struct Rewriter {
    inner: emlite::Val,
}
impl FromVal for Rewriter {
    fn from_val(v: &emlite::Val) -> Self {
        Rewriter {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for Rewriter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Rewriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Rewriter> for emlite::Val {
    fn from(s: Rewriter) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Rewriter {
    pub fn create0() -> jsbind::Promise {
        emlite::Val::global("rewriter")
            .call("create", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn create1(options: RewriterCreateOptions) -> jsbind::Promise {
        emlite::Val::global("rewriter")
            .call("create", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Rewriter {
    pub fn availability0() -> jsbind::Promise {
        emlite::Val::global("rewriter")
            .call("availability", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn availability1(options: RewriterCreateCoreOptions) -> jsbind::Promise {
        emlite::Val::global("rewriter")
            .call("availability", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Rewriter {
    pub fn rewrite0(&self, input: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("rewrite", &[input.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn rewrite1(
        &self,
        input: jsbind::DOMString,
        options: RewriterRewriteOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("rewrite", &[input.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Rewriter {
    pub fn rewrite_streaming0(&self, input: jsbind::DOMString) -> ReadableStream {
        self.inner
            .call("rewriteStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }

    pub fn rewrite_streaming1(
        &self,
        input: jsbind::DOMString,
        options: RewriterRewriteOptions,
    ) -> ReadableStream {
        self.inner
            .call("rewriteStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Rewriter {
    pub fn shared_context(&self) -> jsbind::DOMString {
        self.inner.get("sharedContext").as_::<jsbind::DOMString>()
    }
}
impl Rewriter {
    pub fn tone(&self) -> RewriterTone {
        self.inner.get("tone").as_::<RewriterTone>()
    }
}
impl Rewriter {
    pub fn format(&self) -> RewriterFormat {
        self.inner.get("format").as_::<RewriterFormat>()
    }
}
impl Rewriter {
    pub fn length(&self) -> RewriterLength {
        self.inner.get("length").as_::<RewriterLength>()
    }
}
impl Rewriter {
    pub fn expected_input_languages(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl Rewriter {
    pub fn expected_context_languages(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl Rewriter {
    pub fn output_language(&self) -> jsbind::DOMString {
        self.inner.get("outputLanguage").as_::<jsbind::DOMString>()
    }
}
impl Rewriter {
    pub fn measure_input_usage0(&self, input: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn measure_input_usage1(
        &self,
        input: jsbind::DOMString,
        options: RewriterRewriteOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Rewriter {
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Rewriter {
    pub fn destroy(&self) -> jsbind::Undefined {
        self.inner.call("destroy", &[]).as_::<jsbind::Undefined>()
    }
}
