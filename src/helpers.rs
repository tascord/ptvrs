use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};

pub fn clean(s: String) -> String {
    let mut s = s;
    s = s.trim().to_string();
    s = s.trim_start_matches('"').to_string();
    s = s.trim_end_matches('"').to_string();
    s
}

pub fn to_query<T: Serialize>(s: T) -> String {
    serde_json::to_value(s)
        .unwrap()
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| {
            // If v is an array, define k={v[0]}&k={v[1]}&...
            if v.is_array() {
                v.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| format!("{}={}", k, clean(v.to_string())))
                    .collect::<Vec<String>>()
                    .join("&")
            } else {
                format!("{}={}", k, clean(v.to_string()))
            }
        })
        .collect::<Vec<String>>()
        .join("&")
}

pub fn de_iso_8601<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    Ok(NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
        .map_err(|e| serde::de::Error::custom(format!("Error deser iso_8601 '{s}': {e:?}")))?)
}

pub fn ser_iso_8601<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date {
        Some(date) => serializer.serialize_str(&date.format("%Y-%m-%dT%H:%M:%S").to_string()),
        None => serializer.serialize_none(),
    }
}

/// 24 hour clock format (HH:MM:SS) AEDT/AEST
pub fn de_service_time<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => Ok(Some(
            NaiveDateTime::parse_from_str(&s, "%H:%M:%S").map_err(|e| {
                serde::de::Error::custom(format!("Error deser service_time '{s}': {e:?}"))
            })?,
        )),
        None => Ok(None),
    }
}

// yyyy-MM-dd HH:mm
pub fn ser_touch_utc<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date {
        Some(date) => serializer.serialize_str(&date.format("%Y-%m-%d %H:%M").to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn de_rfc3339<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    Ok(NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.3fZ")
        .map_err(|e| serde::de::Error::custom(format!("Error deser rfc3339 '{s}': {e:?}")))?)
}

pub fn opt_de_rfc3339<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => Ok(Some(
            NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.3fZ").map_err(|e| {
                serde::de::Error::custom(format!("Error deser rfc3339 '{s}': {e:?}"))
            })?,
        )),
        None => Ok(None),
    }
}
