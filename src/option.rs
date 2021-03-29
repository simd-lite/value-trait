use crate::ValueAccess;

impl<V> ValueAccess for Option<V>
where
    V: ValueAccess,
{
    type Target = V::Target;
    type Key = V::Key;
    type Array = V::Array;

    type Object = V::Object;

    fn as_bool(&self) -> Option<bool> {
        self.as_ref().and_then(|v| v.as_bool())
    }

    fn as_i64(&self) -> Option<i64> {
        self.as_ref().and_then(|v| v.as_i64())
    }

    fn as_u64(&self) -> Option<u64> {
        self.as_ref().and_then(|v| v.as_u64())
    }

    fn as_f64(&self) -> Option<f64> {
        self.as_ref().and_then(|v| v.as_f64())
    }

    fn as_str(&self) -> Option<&str> {
        self.as_ref().and_then(|v| v.as_str())
    }

    fn as_array(&self) -> Option<&Self::Array> {
        self.as_ref().and_then(|v| v.as_array())
    }

    fn as_object(&self) -> Option<&Self::Object> {
        self.as_ref().and_then(|v| v.as_object())
    }
}

impl<V, E> ValueAccess for Result<V, E>
where
    V: ValueAccess,
{
    type Target = V::Target;
    type Key = V::Key;
    type Array = V::Array;
    type Object = V::Object;

    fn as_bool(&self) -> Option<bool> {
        self.as_ref().ok().and_then(|v| v.as_bool())
    }

    fn as_i64(&self) -> Option<i64> {
        self.as_ref().ok().and_then(|v| v.as_i64())
    }

    fn as_u64(&self) -> Option<u64> {
        self.as_ref().ok().and_then(|v| v.as_u64())
    }

    fn as_f64(&self) -> Option<f64> {
        self.as_ref().ok().and_then(|v| v.as_f64())
    }

    fn as_str(&self) -> Option<&str> {
        self.as_ref().ok().and_then(|v| v.as_str())
    }

    fn as_array(&self) -> Option<&Self::Array> {
        self.as_ref().ok().and_then(|v| v.as_array())
    }

    fn as_object(&self) -> Option<&Self::Object> {
        self.as_ref().ok().and_then(|v| v.as_object())
    }
}

impl<V> ValueAccess for &V
where
    V: ValueAccess,
{
    type Target = V::Target;

    type Key = V::Key;

    type Array = V::Array;

    type Object = V::Object;

    fn as_bool(&self) -> Option<bool> {
        (*self).as_bool()
    }

    fn as_i64(&self) -> Option<i64> {
        (*self).as_i64()
    }

    fn as_u64(&self) -> Option<u64> {
        (*self).as_u64()
    }

    fn as_f64(&self) -> Option<f64> {
        (*self).as_f64()
    }

    fn as_str(&self) -> Option<&str> {
        (*self).as_str()
    }

    fn as_array(&self) -> Option<&Self::Array> {
        (*self).as_array()
    }

    fn as_object(&self) -> Option<&Self::Object> {
        (*self).as_object()
    }
}
