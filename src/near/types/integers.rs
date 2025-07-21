use near_sdk::serde::{self, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[near_sdk::near(serializers=[borsh])]
#[serde(crate = "near_sdk::serde")]
pub struct U64(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
#[near_sdk::near(serializers=[borsh])]
pub struct U128(pub u128);

impl From<u64> for U64 {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<u128> for U128 {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl near_sdk::schemars::JsonSchema for U64 {
    fn schema_name() -> String {
        "U64".to_string()
    }

    fn json_schema(
        generator: &mut near_sdk::schemars::r#gen::SchemaGenerator,
    ) -> near_sdk::schemars::schema::Schema {
        <u64 as near_sdk::schemars::JsonSchema>::json_schema(generator)
    }
}

impl near_sdk::schemars::JsonSchema for U128 {
    fn schema_name() -> String {
        "U128".to_string()
    }

    fn json_schema(
        generator: &mut near_sdk::schemars::r#gen::SchemaGenerator,
    ) -> near_sdk::schemars::schema::Schema {
        <String as near_sdk::schemars::JsonSchema>::json_schema(generator)
    }
}

impl Serialize for U128 {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for U64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrNumberVisitor;

        impl serde::de::Visitor<'_> for StringOrNumberVisitor {
            type Value = U64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or a number")
            }

            fn visit_str<E>(self, value: &str) -> Result<U64, E>
            where
                E: serde::de::Error,
            {
                value
                    .parse::<u64>()
                    .map(U64)
                    .map_err(serde::de::Error::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<U64, E>
            where
                E: serde::de::Error,
            {
                Ok(U64(value))
            }
        }

        deserializer.deserialize_any(StringOrNumberVisitor)
    }
}

impl<'de> Deserialize<'de> for U128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrNumberVisitor;

        impl serde::de::Visitor<'_> for StringOrNumberVisitor {
            type Value = U128;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or a number 128")
            }

            fn visit_str<E>(self, value: &str) -> Result<U128, E>
            where
                E: serde::de::Error,
            {
                value
                    .parse::<u128>()
                    .map(U128)
                    .map_err(serde::de::Error::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<U128, E>
            where
                E: serde::de::Error,
            {
                Ok(U128(value as u128))
            }

            fn visit_u128<E>(self, value: u128) -> Result<U128, E>
            where
                E: serde::de::Error,
            {
                Ok(U128(value))
            }
        }

        deserializer.deserialize_any(StringOrNumberVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::borsh::{self, BorshDeserialize};
    use near_sdk::serde_json;

    #[test]
    fn test_u64_struct_from_u64() {
        let u64_value = 1234567890;
        let u64_from_u64: U64 = u64_value.into();

        assert_eq!(u64_from_u64.0, u64_value);
    }

    #[test]
    fn test_u128_struct_from_u128() {
        let u128_value = 12345678901234567890;
        let u128_from_u128: U128 = u128_value.into();

        assert_eq!(u128_from_u128.0, u128_value);
    }

    #[test]
    fn test_u64_struct_from_u128() {
        let u128_value = 12345678901234567890;
        let u64_from_u128: U64 = u128_value.into();

        assert_eq!(u64_from_u128.0, u128_value);
    }

    #[test]
    fn test_u128_struct_from_u64() {
        let u64_value = 1234567890;
        let u128_from_u64: U128 = u64_value.into();

        assert_eq!(u128_from_u64.0, u64_value);
    }

    #[test]
    fn test_u64_serde() {
        let u64_value = U64(1234567890);
        let serialized = serde_json::to_string(&u64_value).unwrap();

        assert_eq!(serialized, "1234567890");
    }

    #[test]
    fn test_u128_serde() {
        let u128_value = U128(12345678901234567890);
        let serialized = serde_json::to_string(&u128_value).unwrap();

        assert_eq!(serialized, "\"12345678901234567890\"");
    }

    #[test]
    fn test_u64_from_str() {
        let u64_value = "12345678901234567890";
        let deserialized: U64 = serde_json::from_str(u64_value).unwrap();

        assert_eq!(deserialized, U64(12345678901234567890));
    }

    #[test]
    fn test_u128_from_str() {
        let u128_value = "12345678901234567890";
        let deserialized: U128 = serde_json::from_str(u128_value).unwrap();

        assert_eq!(deserialized, U128(12345678901234567890));
    }

    #[test]
    fn test_u64_deserde() {
        let u64_value = 1234567890;
        let u64_value_str = format!("\"{u64_value}\"");
        let deserialized: U64 = serde_json::from_str(&u64_value_str).unwrap();

        assert_eq!(deserialized.0, u64_value);
    }

    #[test]
    fn test_u128_deserde() {
        let u128_value = 12345678901234567890;
        let u128_value_str = format!("\"{u128_value}\"");
        let deserialized: U128 = serde_json::from_str(&u128_value_str).unwrap();

        assert_eq!(deserialized.0, u128_value);
    }

    #[test]
    fn test_u64_borsh() {
        let u64_value = U64(1234567890);
        let serialized = borsh::to_vec(&u64_value).unwrap();
        let deserialized = U64::try_from_slice(&serialized).unwrap();

        assert_eq!(deserialized, u64_value);
    }

    #[test]
    fn test_u128_borsh() {
        let u128_value = U128(12345678901234567890u128);
        let serialized = borsh::to_vec(&u128_value).unwrap();
        let deserialized = U128::try_from_slice(&serialized).unwrap();

        assert_eq!(deserialized, u128_value);
    }
}
