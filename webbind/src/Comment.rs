use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Comment {
    inner: CharacterData,
}
impl FromVal for Comment {
    fn from_val(v: &emlite::Val) -> Self {
        Comment {
            inner: CharacterData::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Comment {
    type Target = CharacterData;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Comment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Comment {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Comment {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Comment> for emlite::Val {
    fn from(s: Comment) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Comment);

impl Comment {
    pub fn new0() -> Comment {
        Self {
            inner: emlite::Val::global("Comment")
                .new(&[])
                .as_::<CharacterData>(),
        }
    }

    pub fn new1(data: DOMString) -> Comment {
        Self {
            inner: emlite::Val::global("Comment")
                .new(&[data.into()])
                .as_::<CharacterData>(),
        }
    }
}
