use chrono::{DateTime, Utc};
use serde::{de::Error, Deserialize, Deserializer, Serializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let data = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&data)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|err| Error::custom(err))
}
pub fn serialize<S>(data_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&data_time.to_rfc3339())
}
