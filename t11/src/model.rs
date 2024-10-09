use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub(crate) struct Event {
    pub(crate) ulid: Ulid,
    pub(crate) description: String,
}

impl Event {
    pub(crate) fn new(ulid: Ulid, description: String) -> Self {
        Self { ulid, description }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Data {
    pub(crate) user_id: u64,
    pub(crate) date: NaiveDate,
    pub(crate) description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct UpdateData {
    #[serde(flatten)]
    pub(crate) data: Data,
    pub(crate) ulid: Ulid,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct DeleteData {
    pub(crate) user_id: u64,
    pub(crate) date: NaiveDate,
    pub(crate) ulid: Ulid,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ListEvents {
    pub(crate) user_id: u64,
    pub(crate) date: NaiveDate,
}
