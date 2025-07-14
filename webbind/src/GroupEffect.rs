use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GroupEffect {
    inner: emlite::Val,
}
impl FromVal for GroupEffect {
    fn from_val(v: &emlite::Val) -> Self {
        GroupEffect {
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
impl core::ops::Deref for GroupEffect {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GroupEffect {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GroupEffect {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GroupEffect> for emlite::Val {
    fn from(s: GroupEffect) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GroupEffect);

impl GroupEffect {
    pub fn new0(children: jsbind::Sequence<AnimationEffect>) -> GroupEffect {
        Self {
            inner: emlite::Val::global("GroupEffect")
                .new(&[children.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(children: jsbind::Sequence<AnimationEffect>, timing: jsbind::Any) -> GroupEffect {
        Self {
            inner: emlite::Val::global("GroupEffect")
                .new(&[children.into(), timing.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl GroupEffect {
    pub fn children(&self) -> AnimationNodeList {
        self.inner.get("children").as_::<AnimationNodeList>()
    }
}
impl GroupEffect {
    pub fn first_child(&self) -> AnimationEffect {
        self.inner.get("firstChild").as_::<AnimationEffect>()
    }
}
impl GroupEffect {
    pub fn last_child(&self) -> AnimationEffect {
        self.inner.get("lastChild").as_::<AnimationEffect>()
    }
}
impl GroupEffect {
    pub fn clone_(&self) -> GroupEffect {
        self.inner.call("clone", &[]).as_::<GroupEffect>()
    }
}
impl GroupEffect {
    pub fn prepend(&self, effects: AnimationEffect) -> jsbind::Undefined {
        self.inner
            .call("prepend", &[effects.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl GroupEffect {
    pub fn append(&self, effects: AnimationEffect) -> jsbind::Undefined {
        self.inner
            .call("append", &[effects.into()])
            .as_::<jsbind::Undefined>()
    }
}
