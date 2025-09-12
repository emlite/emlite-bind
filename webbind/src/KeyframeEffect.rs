use super::*;

/// The KeyframeEffect class.
/// [`KeyframeEffect`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeyframeEffect {
    inner: AnimationEffect,
}

impl FromVal for KeyframeEffect {
    fn from_val(v: &Any) -> Self {
        KeyframeEffect {
            inner: AnimationEffect::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for KeyframeEffect {
    type Target = AnimationEffect;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KeyframeEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KeyframeEffect {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KeyframeEffect {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<KeyframeEffect> for Any {
    fn from(s: KeyframeEffect) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KeyframeEffect> for Any {
    fn from(s: &KeyframeEffect) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(KeyframeEffect);

impl KeyframeEffect {
    /// Getter of the `target` attribute.
    /// [`KeyframeEffect.target`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)
    pub fn target(&self) -> Element {
        self.inner.get("target").as_::<Element>()
    }

    /// Setter of the `target` attribute.
    /// [`KeyframeEffect.target`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)
    pub fn set_target(&mut self, value: &Element) {
        self.inner.set("target", value);
    }
}
impl KeyframeEffect {
    /// Getter of the `pseudoElement` attribute.
    /// [`KeyframeEffect.pseudoElement`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/pseudoElement)
    pub fn pseudo_element(&self) -> JsString {
        self.inner.get("pseudoElement").as_::<JsString>()
    }

    /// Setter of the `pseudoElement` attribute.
    /// [`KeyframeEffect.pseudoElement`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/pseudoElement)
    pub fn set_pseudo_element(&mut self, value: &JsString) {
        self.inner.set("pseudoElement", value);
    }
}
impl KeyframeEffect {
    /// Getter of the `composite` attribute.
    /// [`KeyframeEffect.composite`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)
    pub fn composite(&self) -> CompositeOperation {
        self.inner.get("composite").as_::<CompositeOperation>()
    }

    /// Setter of the `composite` attribute.
    /// [`KeyframeEffect.composite`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)
    pub fn set_composite(&mut self, value: &CompositeOperation) {
        self.inner.set("composite", value);
    }
}
impl KeyframeEffect {
    /// Getter of the `iterationComposite` attribute.
    /// [`KeyframeEffect.iterationComposite`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)
    pub fn iteration_composite(&self) -> IterationCompositeOperation {
        self.inner
            .get("iterationComposite")
            .as_::<IterationCompositeOperation>()
    }

    /// Setter of the `iterationComposite` attribute.
    /// [`KeyframeEffect.iterationComposite`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)
    pub fn set_iteration_composite(&mut self, value: &IterationCompositeOperation) {
        self.inner.set("iterationComposite", value);
    }
}

impl KeyframeEffect {
    /// The `new KeyframeEffect(..)` constructor, creating a new KeyframeEffect instance
    pub fn new0(target: &Element, keyframes: &Object) -> KeyframeEffect {
        Self {
            inner: Any::global("KeyframeEffect")
                .new(&[target.into(), keyframes.into()])
                .as_::<AnimationEffect>(),
        }
    }

    /// The `new KeyframeEffect(..)` constructor, creating a new KeyframeEffect instance
    pub fn new1(target: &Element, keyframes: &Object, options: &Any) -> KeyframeEffect {
        Self {
            inner: Any::global("KeyframeEffect")
                .new(&[target.into(), keyframes.into(), options.into()])
                .as_::<AnimationEffect>(),
        }
    }
}

impl KeyframeEffect {
    /// The `new KeyframeEffect(..)` constructor, creating a new KeyframeEffect instance
    pub fn new2(source: &KeyframeEffect) -> KeyframeEffect {
        Self {
            inner: Any::global("KeyframeEffect")
                .new(&[source.into()])
                .as_::<AnimationEffect>(),
        }
    }
}
impl KeyframeEffect {
    /// The getKeyframes method.
    /// [`KeyframeEffect.getKeyframes`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/getKeyframes)
    pub fn get_keyframes(&self) -> TypedArray<Object> {
        self.inner
            .call("getKeyframes", &[])
            .as_::<TypedArray<Object>>()
    }
}
impl KeyframeEffect {
    /// The setKeyframes method.
    /// [`KeyframeEffect.setKeyframes`](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/setKeyframes)
    pub fn set_keyframes(&self, keyframes: &Object) -> Undefined {
        self.inner
            .call("setKeyframes", &[keyframes.into()])
            .as_::<Undefined>()
    }
}
