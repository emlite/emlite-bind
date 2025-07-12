use super::*;

#[derive(Clone, Debug)]
pub struct TextFormat {
    inner: emlite::Val,
}
impl FromVal for TextFormat {
    fn from_val(v: &emlite::Val) -> Self {
        TextFormat {
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
impl std::ops::Deref for TextFormat {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TextFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextFormat> for emlite::Val {
    fn from(s: TextFormat) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextFormat {
    pub fn new0() -> TextFormat {
        Self {
            inner: emlite::Val::global("TextFormat")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(options: jsbind::Any) -> TextFormat {
        Self {
            inner: emlite::Val::global("TextFormat")
                .new(&[options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl TextFormat {
    pub fn range_start(&self) -> u32 {
        self.inner.get("rangeStart").as_::<u32>()
    }
}
impl TextFormat {
    pub fn range_end(&self) -> u32 {
        self.inner.get("rangeEnd").as_::<u32>()
    }
}
impl TextFormat {
    pub fn underline_style(&self) -> UnderlineStyle {
        self.inner.get("underlineStyle").as_::<UnderlineStyle>()
    }
}
impl TextFormat {
    pub fn underline_thickness(&self) -> UnderlineThickness {
        self.inner
            .get("underlineThickness")
            .as_::<UnderlineThickness>()
    }
}
