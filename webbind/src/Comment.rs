use super::*;

/// The Comment class.
/// [`Comment`](https://developer.mozilla.org/en-US/docs/Web/API/Comment)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Comment {
    inner: CharacterData,
}

impl FromVal for Comment {
    fn from_val(v: &Any) -> Self {
        Comment {
            inner: CharacterData::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for Comment {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Comment {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Comment> for Any {
    fn from(s: Comment) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Comment> for Any {
    fn from(s: &Comment) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Comment);

impl Comment {
    /// The `new Comment(..)` constructor, creating a new Comment instance
    pub fn new() -> Comment {
        Self {
            inner: Any::global("Comment").new(&[]).as_::<CharacterData>(),
        }
    }
}

impl Comment {
    /// The `new Comment(..)` constructor, creating a new Comment instance
    pub fn new_with_data(data: &JsString) -> Comment {
        Self {
            inner: Any::global("Comment")
                .new(&[data.into()])
                .as_::<CharacterData>(),
        }
    }
}
