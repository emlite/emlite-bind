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
    pub fn apply(target: &Function, this_arg: &Any, arguments: &Array) -> Result<Any, TypeError> {
        let result = Self::obj().call(
            "apply",
            &[
                target.clone().into(),
                this_arg.clone(),
                arguments.clone().into(),
            ],
        );
        result.as_::<Result<Any, TypeError>>()
    }

    /// [`Reflect.construct`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/construct)
    ///
    /// # Returns
    /// Result containing the constructed object or a TypeError if target is not constructible
    pub fn construct(
        target: &Function,
        args: &Array,
        new_target: Option<&Any>,
    ) -> Result<Any, TypeError> {
        let mut params: Vec<emlite::Val> = vec![target.clone().into(), args.clone().into()];
        if let Some(nt) = new_target {
            params.push(nt.clone());
        }
        let result = Self::obj().call("construct", &params);
        result.as_::<Result<Any, TypeError>>()
    }

    /// [`Reflect.defineProperty`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/defineProperty)
    ///
    /// # Returns
    /// Result containing success boolean or TypeError if target is not an object
    pub fn define_property(target: &Any, key: &Any, attributes: &Any) -> Result<bool, TypeError> {
        let result = Self::obj().call(
            "defineProperty",
            &[target.clone(), key.clone(), attributes.clone()],
        );
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.deleteProperty`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/deleteProperty)
    ///
    /// # Returns
    /// Result containing success boolean or TypeError if target is not an object
    pub fn delete_property(target: &Any, key: &Any) -> Result<bool, TypeError> {
        let result = Self::obj().call("deleteProperty", &[target.clone(), key.clone()]);
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.get`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/get)
    ///
    /// # Returns
    /// Result containing the property value or TypeError if target is not an object
    pub fn get(target: &Any, key: &Any, receiver: Option<&Any>) -> Result<Any, TypeError> {
        let mut params = vec![target.clone(), key.clone()];
        if let Some(r) = receiver {
            params.push(r.clone());
        }
        let result = Self::obj().call("get", &params);
        result.as_::<Result<Any, TypeError>>()
    }

    /// [`Reflect.set`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set)
    ///
    /// # Returns
    /// Result containing success boolean or TypeError if target is not an object
    pub fn set(
        target: &Any,
        key: &Any,
        value: &Any,
        receiver: Option<&Any>,
    ) -> Result<bool, TypeError> {
        let mut params = vec![target.clone(), key.clone(), value.clone()];
        if let Some(r) = receiver {
            params.push(r.clone());
        }
        let result = Self::obj().call("set", &params);
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.getOwnPropertyDescriptor`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getOwnPropertyDescriptor)
    ///
    /// # Returns
    /// Result containing the property descriptor or TypeError if target is not an object
    pub fn get_own_property_descriptor(target: &Any, key: &Any) -> Result<Option<Any>, TypeError> {
        let result = Self::obj().call("getOwnPropertyDescriptor", &[target.clone(), key.clone()]);
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
    pub fn get_prototype_of(target: &Any) -> Result<Option<Any>, TypeError> {
        let result = Self::obj().call("getPrototypeOf", &[target.clone()]);
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
    pub fn has(target: &Any, key: &Any) -> Result<bool, TypeError> {
        let result = Self::obj().call("has", &[target.clone(), key.clone()]);
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.isExtensible`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/isExtensible)
    ///
    /// # Returns
    /// Result containing boolean or TypeError if target is not an object
    pub fn is_extensible(target: &Any) -> Result<bool, TypeError> {
        let result = Self::obj().call("isExtensible", &[target.clone()]);
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.ownKeys`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/ownKeys)
    ///
    /// # Returns
    /// Result containing array of keys or TypeError if target is not an object
    pub fn own_keys(target: &Any) -> Result<Array, TypeError> {
        let result = Self::obj().call("ownKeys", &[target.clone()]);
        result.as_::<Result<Array, TypeError>>()
    }

    /// [`Reflect.preventExtensions`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/preventExtensions)
    ///
    /// # Returns
    /// Result containing success boolean or TypeError if target is not an object
    pub fn prevent_extensions(target: &Any) -> Result<bool, TypeError> {
        let result = Self::obj().call("preventExtensions", &[target.clone()]);
        result.as_::<Result<bool, TypeError>>()
    }

    /// [`Reflect.setPrototypeOf`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/setPrototypeOf)
    ///
    /// # Returns
    /// Result containing success boolean or TypeError if target is not an object
    pub fn set_prototype_of(target: &Any, proto: &Any) -> Result<bool, TypeError> {
        let result = Self::obj().call("setPrototypeOf", &[target.clone(), proto.clone()]);
        result.as_::<Result<bool, TypeError>>()
    }
}
