use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRHand {
    inner: emlite::Val,
}
impl FromVal for XRHand {
    fn from_val(v: &emlite::Val) -> Self {
        XRHand {
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
impl core::ops::Deref for XRHand {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRHand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRHand> for emlite::Val {
    fn from(s: XRHand) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRHand {
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }
}
impl XRHand {
    pub fn get(&self, key: XRHandJoint) -> XRJointSpace {
        self.inner.call("get", &[key.into()]).as_::<XRJointSpace>()
    }
}
