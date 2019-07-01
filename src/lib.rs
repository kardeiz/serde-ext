pub use serde_ext_macros::*;

pub mod de {

    #[derive(derive_more::Deref, derive_more::DerefMut)]
    pub struct Parsable<T>(pub T);

    impl<'de, T> serde::Deserialize<'de> for Parsable<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            let s: std::borrow::Cow<str> = serde::Deserialize::deserialize(deserializer)?;
            Ok(Parsable(s.parse().map_err(serde::de::Error::custom)?))
        }
    }

    pub fn parsable<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::de::Deserializer<'de>,
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        let out: Parsable<T> = serde::Deserialize::deserialize(deserializer)?;
        Ok(out.0)
    }

    pub struct NonEmptyString(pub Option<String>);

    impl<'de> serde::Deserialize<'de> for NonEmptyString
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            let s: Option<String> = serde::Deserialize::deserialize(deserializer)?;
            Ok(NonEmptyString(s.and_then(|t| if t.is_empty() { None } else { Some(t) })))
        }
    }

    pub fn non_empty_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let out: NonEmptyString = serde::Deserialize::deserialize(deserializer)?;
        Ok(out.0)
    }

}

#[cfg(feature = "base64")]
pub mod base64 {

    #[derive(derive_more::Deref, derive_more::DerefMut)]
    pub struct Base64(pub Vec<u8>);

    impl<'de> serde::Deserialize<'de> for Base64 where {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            let s: std::borrow::Cow<str> = serde::Deserialize::deserialize(deserializer)?;
            Ok(Base64(base64::decode(s.as_ref()).map_err(serde::de::Error::custom)?))
        }
    }

    impl serde::Serialize for Base64 {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serialize(&self.0, serializer)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let out: Base64 = serde::Deserialize::deserialize(deserializer)?;
        Ok(out.0)
    }

    fn serialize<S>(base64_bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::Serialize;
        base64::encode(&base64_bytes).serialize(serializer)
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
