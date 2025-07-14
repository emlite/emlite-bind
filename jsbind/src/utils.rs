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

        impl AsRef<emlite::Val> for $i {
            fn as_ref(&self) -> &emlite::Val {
                &self.inner
            }
        }

        impl AsMut<emlite::Val> for $i {
            fn as_mut(&mut self) -> &mut emlite::Val {
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

#[macro_export]
macro_rules! impl_dyn_cast {
    ($ty:ty) => {
        impl $crate::prelude::DynCast for $ty {
            #[inline]
            fn instanceof(val: &emlite::Val) -> bool {
                // assumes `emlite::Val::instance_of(&ctor)` exists
                let ctor = emlite::Val::global(stringify!($ty));
                val.instanceof(ctor)
            }
            #[inline]
            fn unchecked_from_val(v: emlite::Val) -> Self {
                v.as_::<Self>() // zero-cost new-type cast
            }
            #[inline]
            fn unchecked_from_val_ref(v: &emlite::Val) -> &Self {
                unsafe { &*(v as *const emlite::Val as *const Self) }
            }
        }
    };
    ($ty:ty, $global_ctor:expr) => {
        impl $crate::prelude::DynCast for $ty {
            #[inline]
            fn instanceof(val: &emlite::Val) -> bool {
                // assumes `emlite::Val::instance_of(&ctor)` exists
                let ctor = emlite::Val::global($global_ctor);
                val.instanceof(ctor)
            }
            #[inline]
            fn unchecked_from_val(v: emlite::Val) -> Self {
                v.as_::<Self>() // zero-cost new-type cast
            }
            #[inline]
            fn unchecked_from_val_ref(v: &emlite::Val) -> &Self {
                unsafe { &*(v as *const emlite::Val as *const Self) }
            }
        }
    };
}

pub use impl_dyn_cast;
