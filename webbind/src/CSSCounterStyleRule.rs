use super::*;

/// The CSSCounterStyleRule class.
/// [`CSSCounterStyleRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSCounterStyleRule {
    inner: CSSRule,
}
impl FromVal for CSSCounterStyleRule {
    fn from_val(v: &Any) -> Self {
        CSSCounterStyleRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSCounterStyleRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSCounterStyleRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSCounterStyleRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSCounterStyleRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSCounterStyleRule> for Any {
    fn from(s: CSSCounterStyleRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSCounterStyleRule> for Any {
    fn from(s: &CSSCounterStyleRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSCounterStyleRule);

impl CSSCounterStyleRule {
    /// Getter of the `name` attribute.
    /// [`CSSCounterStyleRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name)
    pub fn name(&self) -> CSSOMString {
        self.inner.get("name").as_::<CSSOMString>()
    }

    /// Setter of the `name` attribute.
    /// [`CSSCounterStyleRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name)
    pub fn set_name(&mut self, value: &CSSOMString) {
        self.inner.set("name", value);
    }
}
impl CSSCounterStyleRule {
    /// Getter of the `system` attribute.
    /// [`CSSCounterStyleRule.system`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system)
    pub fn system(&self) -> CSSOMString {
        self.inner.get("system").as_::<CSSOMString>()
    }

    /// Setter of the `system` attribute.
    /// [`CSSCounterStyleRule.system`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system)
    pub fn set_system(&mut self, value: &CSSOMString) {
        self.inner.set("system", value);
    }
}
impl CSSCounterStyleRule {
    /// Getter of the `symbols` attribute.
    /// [`CSSCounterStyleRule.symbols`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols)
    pub fn symbols(&self) -> CSSOMString {
        self.inner.get("symbols").as_::<CSSOMString>()
    }

    /// Setter of the `symbols` attribute.
    /// [`CSSCounterStyleRule.symbols`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols)
    pub fn set_symbols(&mut self, value: &CSSOMString) {
        self.inner.set("symbols", value);
    }
}
impl CSSCounterStyleRule {
    /// Getter of the `additiveSymbols` attribute.
    /// [`CSSCounterStyleRule.additiveSymbols`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols)
    pub fn additive_symbols(&self) -> CSSOMString {
        self.inner.get("additiveSymbols").as_::<CSSOMString>()
    }

    /// Setter of the `additiveSymbols` attribute.
    /// [`CSSCounterStyleRule.additiveSymbols`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols)
    pub fn set_additive_symbols(&mut self, value: &CSSOMString) {
        self.inner.set("additiveSymbols", value);
    }
}
impl CSSCounterStyleRule {
    /// Getter of the `negative` attribute.
    /// [`CSSCounterStyleRule.negative`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative)
    pub fn negative(&self) -> CSSOMString {
        self.inner.get("negative").as_::<CSSOMString>()
    }

    /// Setter of the `negative` attribute.
    /// [`CSSCounterStyleRule.negative`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative)
    pub fn set_negative(&mut self, value: &CSSOMString) {
        self.inner.set("negative", value);
    }
}
impl CSSCounterStyleRule {
    /// Getter of the `prefix` attribute.
    /// [`CSSCounterStyleRule.prefix`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix)
    pub fn prefix(&self) -> CSSOMString {
        self.inner.get("prefix").as_::<CSSOMString>()
    }

    /// Setter of the `prefix` attribute.
    /// [`CSSCounterStyleRule.prefix`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix)
    pub fn set_prefix(&mut self, value: &CSSOMString) {
        self.inner.set("prefix", value);
    }
}
impl CSSCounterStyleRule {
    /// Getter of the `suffix` attribute.
    /// [`CSSCounterStyleRule.suffix`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix)
    pub fn suffix(&self) -> CSSOMString {
        self.inner.get("suffix").as_::<CSSOMString>()
    }

    /// Setter of the `suffix` attribute.
    /// [`CSSCounterStyleRule.suffix`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix)
    pub fn set_suffix(&mut self, value: &CSSOMString) {
        self.inner.set("suffix", value);
    }
}
impl CSSCounterStyleRule {
    /// Getter of the `range` attribute.
    /// [`CSSCounterStyleRule.range`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/range)
    pub fn range(&self) -> CSSOMString {
        self.inner.get("range").as_::<CSSOMString>()
    }

    /// Setter of the `range` attribute.
    /// [`CSSCounterStyleRule.range`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/range)
    pub fn set_range(&mut self, value: &CSSOMString) {
        self.inner.set("range", value);
    }
}
impl CSSCounterStyleRule {
    /// Getter of the `pad` attribute.
    /// [`CSSCounterStyleRule.pad`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/pad)
    pub fn pad(&self) -> CSSOMString {
        self.inner.get("pad").as_::<CSSOMString>()
    }

    /// Setter of the `pad` attribute.
    /// [`CSSCounterStyleRule.pad`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/pad)
    pub fn set_pad(&mut self, value: &CSSOMString) {
        self.inner.set("pad", value);
    }
}
impl CSSCounterStyleRule {
    /// Getter of the `speakAs` attribute.
    /// [`CSSCounterStyleRule.speakAs`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs)
    pub fn speak_as(&self) -> CSSOMString {
        self.inner.get("speakAs").as_::<CSSOMString>()
    }

    /// Setter of the `speakAs` attribute.
    /// [`CSSCounterStyleRule.speakAs`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs)
    pub fn set_speak_as(&mut self, value: &CSSOMString) {
        self.inner.set("speakAs", value);
    }
}
impl CSSCounterStyleRule {
    /// Getter of the `fallback` attribute.
    /// [`CSSCounterStyleRule.fallback`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback)
    pub fn fallback(&self) -> CSSOMString {
        self.inner.get("fallback").as_::<CSSOMString>()
    }

    /// Setter of the `fallback` attribute.
    /// [`CSSCounterStyleRule.fallback`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback)
    pub fn set_fallback(&mut self, value: &CSSOMString) {
        self.inner.set("fallback", value);
    }
}
