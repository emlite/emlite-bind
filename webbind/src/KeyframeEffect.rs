use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeyframeEffect {
    inner: AnimationEffect,
}
impl FromVal for KeyframeEffect {
    fn from_val(v: &emlite::Val) -> Self {
        KeyframeEffect {
            inner: AnimationEffect::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for KeyframeEffect {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for KeyframeEffect {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<KeyframeEffect> for emlite::Val {
    fn from(s: KeyframeEffect) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&KeyframeEffect> for emlite::Val {
    fn from(s: &KeyframeEffect) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(KeyframeEffect);

impl KeyframeEffect {
    pub fn new(source: &KeyframeEffect) -> KeyframeEffect {
        Self {
            inner: emlite::Val::global("KeyframeEffect")
                .new(&[source.into()])
                .as_::<AnimationEffect>(),
        }
    }
}
impl KeyframeEffect {
    pub fn target(&self) -> Element {
        self.inner.get("target").as_::<Element>()
    }

    pub fn set_target(&mut self, value: &Element) {
        self.inner.set("target", value);
    }
}
impl KeyframeEffect {
    pub fn pseudo_element(&self) -> String {
        self.inner.get("pseudoElement").as_::<String>()
    }

    pub fn set_pseudo_element(&mut self, value: &str) {
        self.inner.set("pseudoElement", value);
    }
}
impl KeyframeEffect {
    pub fn composite(&self) -> CompositeOperation {
        self.inner.get("composite").as_::<CompositeOperation>()
    }

    pub fn set_composite(&mut self, value: &CompositeOperation) {
        self.inner.set("composite", value);
    }
}
impl KeyframeEffect {
    pub fn get_keyframes(&self) -> Sequence<Object> {
        self.inner
            .call("getKeyframes", &[])
            .as_::<Sequence<Object>>()
    }
}
impl KeyframeEffect {
    pub fn set_keyframes(&self, keyframes: &Object) -> Undefined {
        self.inner
            .call("setKeyframes", &[keyframes.into()])
            .as_::<Undefined>()
    }
}
impl KeyframeEffect {
    pub fn iteration_composite(&self) -> IterationCompositeOperation {
        self.inner
            .get("iterationComposite")
            .as_::<IterationCompositeOperation>()
    }

    pub fn set_iteration_composite(&mut self, value: &IterationCompositeOperation) {
        self.inner.set("iterationComposite", value);
    }
}
