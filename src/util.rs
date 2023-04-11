//! Utilities
use serde::de::DeserializeOwned;

type DeserializeJsonError = serde_path_to_error::Error<serde_json::Error>;

/// Parses json with better errors provided by [`serde_path_to_error`]
pub fn parse_json<T: DeserializeOwned>(json: &str) -> Result<T, DeserializeJsonError> {
    let jd = &mut serde_json::Deserializer::from_str(json);
    serde_path_to_error::deserialize(jd)
}

/// A helper function to format an error with its source chain.
///
/// This function works with both `&Error` and `Box<dyn Error>`. When passing a boxed error,
/// make sure to dereference it using `&*e`.
pub fn error_chain(e: &(dyn std::error::Error + 'static)) -> String {
    use std::fmt::Write as _;

    let mut s = e.to_string();
    let mut current = e.source();
    if current.is_some() {
        s.push_str("\nCaused by:");
    }
    while let Some(cause) = current {
        write!(s, "\n\t{}", cause).ok();
        current = cause.source();
    }
    s
}

pub(crate) fn serialize_comma_separated_option<T, S>(
    data: &Option<Vec<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: ToString,
{
    let Some(items) = data else { return serializer.serialize_none() };
    let comma_separated = items
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",");

    serializer.serialize_str(&comma_separated)
}

pub(crate) fn serialize_bool_1_or_skip<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        false => serializer.serialize_none(),
        true => serializer.serialize_str("1"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    // use serde_urlencoded::to_string as qs;

    fn qs(v: &impl Serialize) -> String {
        serde_urlencoded::to_string(v).unwrap()
    }

    #[test]
    fn test_serialize_comma_separated_option() {
        /// Straight Odds query params
        #[derive(Serialize)]
        pub struct S {
            #[serde(serialize_with = "serialize_comma_separated_option")]
            ids: Option<Vec<i32>>,
        }

        assert_eq!(qs(&S { ids: None }), "");
        assert_eq!(
            qs(&S {
                ids: Some(vec![1, 2])
            }),
            "ids=1%2C2"
        );
    }

    #[test]
    fn test_serialize_bool_1_or_skip() {
        #[derive(Serialize)]
        pub struct S {
            #[serde(serialize_with = "serialize_bool_1_or_skip")]
            is: bool,
        }

        assert_eq!(qs(&S { is: false }), "");
        assert_eq!(qs(&S { is: true }), "is=1");
    }
}
