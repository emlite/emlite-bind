macro_rules! bind {
    ($i:ident) => {
        impl emlite::FromVal for $i {
            fn from_val(v: &emlite::Val) -> Self {
                $i { inner: v.clone() }
            }
            fn take_ownership(v: emlite::env::Handle) -> Self {
                Self::from_val(&emlite::Val::take_ownership(v))
            }
            fn as_handle(&self) -> emlite::env::Handle {
                self.inner.as_handle()
            }
        }

        impl core::ops::Deref for $i {
            type Target = emlite::Val;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl core::ops::DerefMut for $i {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl From<$i> for emlite::Val {
            fn from(x: $i) -> emlite::Val {
                use emlite::FromVal;
                let handle = x.inner.as_handle();
                core::mem::forget(x);
                emlite::Val::take_ownership(handle)
            }
        }
    };
}

pub(crate) use bind;
