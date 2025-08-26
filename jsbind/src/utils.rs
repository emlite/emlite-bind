macro_rules! bind {
    ($i:ident) => {
        impl emlite::FromVal for $i {
            fn from_val(v: &$crate::prelude::Any) -> Self {
                $i { inner: v.clone() }
            }
            fn take_ownership(v: emlite::common::Handle) -> Self {
                Self::from_val(&$crate::prelude::Any::take_ownership(v))
            }
            fn as_handle(&self) -> emlite::common::Handle {
                self.inner.as_handle()
            }
        }

        impl core::ops::Deref for $i {
            type Target = $crate::prelude::Any;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl core::ops::DerefMut for $i {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl AsRef<$crate::prelude::Any> for $i {
            fn as_ref(&self) -> &$crate::prelude::Any {
                &self.inner
            }
        }

        impl AsMut<$crate::prelude::Any> for $i {
            fn as_mut(&mut self) -> &mut $crate::prelude::Any {
                &mut self.inner
            }
        }

        impl From<$i> for $crate::prelude::Any {
            fn from(x: $i) -> $crate::prelude::Any {
                use emlite::FromVal;
                let handle = x.inner.as_handle();
                core::mem::forget(x);
                $crate::prelude::Any::take_ownership(handle)
            }
        }

        impl From<&$i> for $crate::prelude::Any {
            fn from(x: &$i) -> $crate::prelude::Any {
                x.inner.clone()
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
            fn instanceof(val: &$crate::prelude::Any) -> bool {
                // assumes `$crate::prelude::Any::instance_of(&ctor)` exists
                let ctor = $crate::prelude::Any::global(stringify!($ty));
                val.instanceof(ctor)
            }
            #[inline]
            fn unchecked_from_val(v: $crate::prelude::Any) -> Self {
                v.as_::<Self>() // zero-cost new-type cast
            }
            #[inline]
            fn unchecked_from_val_ref(v: &$crate::prelude::Any) -> &Self {
                unsafe { &*(v as *const $crate::prelude::Any as *const Self) }
            }
            #[inline]
            fn unchecked_from_val_mut(v: &mut $crate::prelude::Any) -> &mut Self {
                unsafe { &mut *(v as *mut $crate::prelude::Any as *mut Self) }
            }
        }
    };
    ($ty:ty, $global_ctor:expr) => {
        impl $crate::prelude::DynCast for $ty {
            #[inline]
            fn instanceof(val: &$crate::prelude::Any) -> bool {
                // assumes `$crate::prelude::Any::instance_of(&ctor)` exists
                let ctor = $crate::prelude::Any::global($global_ctor);
                val.instanceof(ctor)
            }
            #[inline]
            fn unchecked_from_val(v: $crate::prelude::Any) -> Self {
                v.as_::<Self>() // zero-cost new-type cast
            }
            #[inline]
            fn unchecked_from_val_ref(v: &$crate::prelude::Any) -> &Self {
                unsafe { &*(v as *const $crate::prelude::Any as *const Self) }
            }
            #[inline]
            fn unchecked_from_val_mut(v: &mut $crate::prelude::Any) -> &mut Self {
                unsafe { &mut *(v as *mut $crate::prelude::Any as *mut Self) }
            }
        }
    };
}

pub use impl_dyn_cast;
