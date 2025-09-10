use crate::error::TypeError;
use crate::{any::Any, array::Array, function::Function};
use alloc::vec;
use alloc::vec::Vec;

/// static Reflect struct, never constructed.
pub struct Reflect;

impl Reflect {
    /// Internal helper that returns the global `Reflect` object as an
    /// `emlite::Val`.
    #[inline]
    fn obj() -> emlite::Val {
        emlite::Val::global("Reflect")
    }

    /// [`Reflect.apply`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/apply)
    ///
    /// # Returns
    /// Result containing the return value or a TypeError if target is not callable
    pub fn apply<TThis, TArgs>(
        target: &Function,
        this_arg: TThis,
        arguments: TArgs,
    ) -> Result<Any, TypeError>
    where
        TThis: Into<emlite::Val>,
        TArgs: Into<emlite::Val>,
    {
        let result = Self::obj().call(
            "apply",
            &[target.clone().into(), this_arg.into(), arguments.into()],
        );
        result.as_::<Result<Any, TypeError>>()
    }

    /// [`Reflect.construct`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/construct)
    ///
    /// # Returns
    /// Result containing the constructed object or a TypeError if target is not constructible
    pub fn construct<TArgs, TNew>(
        target: &Function,
        args: TArgs,
        new_target: Option<TNew>,
    ) -> Result<Any, TypeError>
    where
        TArgs: Into<emlite::Val>,
        TNew: Into<emlite::Val>,
    {
        let mut params: Vec<emlite::Val> = vec![target.clone().into(), args.into()];
        if let Some(nt) = new_target {
            params.push(nt.into());
        }
        let result = Self::obj().call("construct", &params);
        result.as_::<Result<Any, TypeError>>()
    }

    /// [`Reflect.defineProperty`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/defineProperty)
    ///
    /// # Returns
    /// Result containing success boolean or TypeError if target is not an object
    pub fn define_property<T, K, A>(target: T, key: K, attributes: A) -> Result<bool, TypeError>
    where
        T: Into<emlite::Val>,
        K: Into<emlite::Val>,
        A: Into<emlite::Val>,
    {
        let result = Self::obj().call(
            "defineProperty",
            &[target.into(), key.into(), attributes.into()],
        );
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.deleteProperty`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/deleteProperty)
    ///
    /// # Returns
    /// Result containing success boolean or TypeError if target is not an object
    pub fn delete_property<T, K>(target: T, key: K) -> Result<bool, TypeError>
    where
        T: Into<emlite::Val>,
        K: Into<emlite::Val>,
    {
        let result = Self::obj().call("deleteProperty", &[target.into(), key.into()]);
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.get`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/get)
    ///
    /// # Returns
    /// Result containing the property value or TypeError if target is not an object
    pub fn get<T, K, R>(target: T, key: K, receiver: Option<R>) -> Result<Any, TypeError>
    where
        T: Into<emlite::Val>,
        K: Into<emlite::Val>,
        R: Into<emlite::Val>,
    {
        let mut params = vec![target.into(), key.into()];
        if let Some(r) = receiver {
            params.push(r.into());
        }
        let result = Self::obj().call("get", &params);
        result.as_::<Result<Any, TypeError>>()
    }

    /// [`Reflect.set`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set)
    ///
    /// # Returns
    /// Result containing success boolean or TypeError if target is not an object
    pub fn set<T, K, V, R>(
        target: T,
        key: K,
        value: V,
        receiver: Option<R>,
    ) -> Result<bool, TypeError>
    where
        T: Into<emlite::Val>,
        K: Into<emlite::Val>,
        V: Into<emlite::Val>,
        R: Into<emlite::Val>,
    {
        let mut params = vec![target.into(), key.into(), value.into()];
        if let Some(r) = receiver {
            params.push(r.into());
        }
        let result = Self::obj().call("set", &params);
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.getOwnPropertyDescriptor`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getOwnPropertyDescriptor)
    ///
    /// # Returns
    /// Result containing the property descriptor or TypeError if target is not an object
    pub fn get_own_property_descriptor<T, K>(target: T, key: K) -> Result<Option<Any>, TypeError>
    where
        T: Into<emlite::Val>,
        K: Into<emlite::Val>,
    {
        let result = Self::obj().call("getOwnPropertyDescriptor", &[target.into(), key.into()]);
        if result.is_error() {
            Err(result.as_::<TypeError>())
        } else if result.is_undefined() {
            Ok(None)
        } else {
            Ok(Some(result.as_::<Any>()))
        }
    }

    /// [`Reflect.getPrototypeOf`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getPrototypeOf)
    ///
    /// # Returns
    /// Result containing the prototype or TypeError if target is not an object
    pub fn get_prototype_of<T>(target: T) -> Result<Option<Any>, TypeError>
    where
        T: Into<emlite::Val>,
    {
        let result = Self::obj().call("getPrototypeOf", &[target.into()]);
        if result.is_error() {
            Err(result.as_::<TypeError>())
        } else if result.is_null() {
            Ok(None)
        } else {
            Ok(Some(result.as_::<Any>()))
        }
    }

    /// [`Reflect.has`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/has)
    ///
    /// # Returns
    /// Result containing boolean or TypeError if target is not an object
    pub fn has<T, K>(target: T, key: K) -> Result<bool, TypeError>
    where
        T: Into<emlite::Val>,
        K: Into<emlite::Val>,
    {
        let result = Self::obj().call("has", &[target.into(), key.into()]);
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.isExtensible`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/isExtensible)
    ///
    /// # Returns
    /// Result containing boolean or TypeError if target is not an object
    pub fn is_extensible<T>(target: T) -> Result<bool, TypeError>
    where
        T: Into<emlite::Val>,
    {
        let result = Self::obj().call("isExtensible", &[target.into()]);
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.ownKeys`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/ownKeys)
    ///
    /// # Returns
    /// Result containing array of keys or TypeError if target is not an object
    pub fn own_keys<T>(target: T) -> Result<Array, TypeError>
    where
        T: Into<emlite::Val>,
    {
        let result = Self::obj().call("ownKeys", &[target.into()]);
        result.as_::<Result<Array, TypeError>>()
    }

    /// [`Reflect.preventExtensions`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/preventExtensions)
    ///
    /// # Returns
    /// Result containing success boolean or TypeError if target is not an object
    pub fn prevent_extensions<T>(target: T) -> Result<bool, TypeError>
    where
        T: Into<emlite::Val>,
    {
        let result = Self::obj().call("preventExtensions", &[target.into()]);
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.setPrototypeOf`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/setPrototypeOf)
    ///
    /// # Returns
    /// Result containing success boolean or TypeError if target is not an object
    pub fn set_prototype_of<T, P>(target: T, proto: P) -> Result<bool, TypeError>
    where
        T: Into<emlite::Val>,
        P: Into<emlite::Val>,
    {
        let result = Self::obj().call("setPrototypeOf", &[target.into(), proto.into()]);
        result.as_::<Result<bool, TypeError>>()
    }
}
