use super::*;

/// The GroupEffect class.
/// [`GroupEffect`](https://developer.mozilla.org/en-US/docs/Web/API/GroupEffect)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GroupEffect {
    inner: Any,
}

impl FromVal for GroupEffect {
    fn from_val(v: &Any) -> Self {
        GroupEffect {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GroupEffect {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GroupEffect {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GroupEffect {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GroupEffect> for Any {
    fn from(s: GroupEffect) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GroupEffect> for Any {
    fn from(s: &GroupEffect) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GroupEffect);

impl GroupEffect {
    /// Getter of the `children` attribute.
    /// [`GroupEffect.children`](https://developer.mozilla.org/en-US/docs/Web/API/GroupEffect/children)
    pub fn children(&self) -> AnimationNodeList {
        self.inner.get("children").as_::<AnimationNodeList>()
    }
}
impl GroupEffect {
    /// Getter of the `firstChild` attribute.
    /// [`GroupEffect.firstChild`](https://developer.mozilla.org/en-US/docs/Web/API/GroupEffect/firstChild)
    pub fn first_child(&self) -> AnimationEffect {
        self.inner.get("firstChild").as_::<AnimationEffect>()
    }
}
impl GroupEffect {
    /// Getter of the `lastChild` attribute.
    /// [`GroupEffect.lastChild`](https://developer.mozilla.org/en-US/docs/Web/API/GroupEffect/lastChild)
    pub fn last_child(&self) -> AnimationEffect {
        self.inner.get("lastChild").as_::<AnimationEffect>()
    }
}

impl GroupEffect {
    /// The `new GroupEffect(..)` constructor, creating a new GroupEffect instance
    pub fn new(children: &TypedArray<AnimationEffect>) -> GroupEffect {
        Self {
            inner: Any::global("GroupEffect")
                .new(&[children.into()])
                .as_::<Any>(),
        }
    }
}

impl GroupEffect {
    /// The `new GroupEffect(..)` constructor, creating a new GroupEffect instance
    pub fn new_with_timing(children: &TypedArray<AnimationEffect>, timing: &Any) -> GroupEffect {
        Self {
            inner: Any::global("GroupEffect")
                .new(&[children.into(), timing.into()])
                .as_::<Any>(),
        }
    }
}

impl GroupEffect {
    /// The clone method.
    /// [`GroupEffect.clone`](https://developer.mozilla.org/en-US/docs/Web/API/GroupEffect/clone)
    pub fn clone_(&self) -> GroupEffect {
        self.inner.call("clone", &[]).as_::<GroupEffect>()
    }
}
impl GroupEffect {
    /// The prepend method.
    /// [`GroupEffect.prepend`](https://developer.mozilla.org/en-US/docs/Web/API/GroupEffect/prepend)
    pub fn prepend(&self, effects: &AnimationEffect) -> Undefined {
        self.inner
            .call("prepend", &[effects.into()])
            .as_::<Undefined>()
    }
}
impl GroupEffect {
    /// The append method.
    /// [`GroupEffect.append`](https://developer.mozilla.org/en-US/docs/Web/API/GroupEffect/append)
    pub fn append(&self, effects: &AnimationEffect) -> Undefined {
        self.inner
            .call("append", &[effects.into()])
            .as_::<Undefined>()
    }
}
