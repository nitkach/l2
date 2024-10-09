use crate::model::{Data, Event};
use chrono::{Datelike, NaiveDate};
use dashmap::DashMap;
use std::{
    collections::{hash_map, HashMap},
    sync::{Arc, Mutex},
};
use ulid::Ulid;

pub(crate) struct Repository {
    repo: Arc<DashMap<u64, HashMap<NaiveDate, Vec<Event>>>>,
    ulid_generator: Arc<Mutex<ulid::Generator>>,
}

impl Repository {
    pub(crate) fn new() -> Self {
        Self {
            repo: Arc::new(DashMap::new()),
            ulid_generator: Arc::new(Mutex::new(ulid::Generator::new())),
        }
    }
}

impl Repository {
    pub(crate) async fn create_event(&self, data: Data) {
        let ulid = loop {
            let Ok(ulid) = self.ulid_generator.lock().unwrap().generate() else {
                tokio::task::yield_now().await;
                continue;
            };
            break ulid;
        };

        let event = Event::new(ulid, data.description);

        match self.repo.entry(data.user_id) {
            dashmap::Entry::Vacant(vacant_entry) => {
                let value: HashMap<NaiveDate, Vec<Event>> =
                    HashMap::from_iter([(data.date, vec![event])]);
                vacant_entry.insert(value);
            }
            dashmap::Entry::Occupied(mut occupied_entry) => {
                match occupied_entry.get_mut().entry(data.date) {
                    hash_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![event]);
                    }
                    hash_map::Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push(event);
                    }
                }
            }
        }
    }

    pub(crate) fn update_event(
        &self,
        user_id: u64,
        date: NaiveDate,
        description: String,
        ulid: Ulid,
    ) -> MutateState {
        match self.repo.entry(user_id) {
            dashmap::Entry::Occupied(mut occupied_entry) => {
                match occupied_entry.get_mut().entry(date) {
                    hash_map::Entry::Occupied(mut occupied_entry) => {
                        let Ok(nth) = occupied_entry
                            .get_mut()
                            .binary_search_by(|event| event.ulid.cmp(&ulid))
                        else {
                            return MutateState::UlidNotFound;
                        };
                        let (_, needle, _) = occupied_entry.get_mut().select_nth_unstable(nth);
                        needle.description = description;
                        MutateState::Success
                    }
                    hash_map::Entry::Vacant(_) => MutateState::DateNotFound,
                }
            }
            dashmap::Entry::Vacant(_) => MutateState::UserNotFound,
        }
    }

    pub(crate) fn delete_event(&self, user_id: u64, date: NaiveDate, ulid: Ulid) -> MutateState {
        match self.repo.entry(user_id) {
            dashmap::Entry::Occupied(mut occupied_entry) => {
                match occupied_entry.get_mut().entry(date) {
                    hash_map::Entry::Occupied(mut occupied_entry) => {
                        let Ok(nth) = occupied_entry
                            .get_mut()
                            .binary_search_by(|event| event.ulid.cmp(&ulid))
                        else {
                            return MutateState::UlidNotFound;
                        };
                        occupied_entry.get_mut().remove(nth);
                        MutateState::Success
                    }
                    hash_map::Entry::Vacant(_) => MutateState::DateNotFound,
                }
            }
            dashmap::Entry::Vacant(_) => MutateState::UserNotFound,
        }
    }

    pub(crate) fn events_for_day(&self, user_id: u64, date: NaiveDate) -> ReadState {
        let Some(user_dates) = self.repo.get(&user_id) else {
            return ReadState::UserNotFound;
        };

        let Some(events) = user_dates.get(&date).map(ToOwned::to_owned) else {
            return ReadState::DateNotFound;
        };

        ReadState::Success(events)
    }

    pub(crate) fn events_for_week(&self, user_id: u64, date: NaiveDate) -> ReadState {
        let Some(user_dates) = self.repo.get(&user_id) else {
            return ReadState::UserNotFound;
        };

        let start_of_week =
            date - chrono::Duration::days(i64::from(date.weekday().num_days_from_monday()));
        let end_of_week = start_of_week + chrono::Duration::days(6);

        let events = user_dates
            .iter()
            .filter(|(date, _)| &start_of_week <= date && date <= &&end_of_week)
            .flat_map(|(_, events)| events.to_owned())
            .collect::<Vec<_>>();

        ReadState::Success(events)
    }

    pub(crate) fn events_for_month(&self, user_id: u64, date: NaiveDate) -> ReadState {
        let Some(user_dates) = self.repo.get(&user_id) else {
            return ReadState::UserNotFound;
        };

        let start_of_month =
            NaiveDate::from_ymd_opt(date.year(), date.month(), 1).expect("all args are valid date");
        let end_of_month = NaiveDate::from_ymd_opt(
            date.year(),
            date.month(),
            date.with_day(1)
                .expect("first day exists for every month")
                .with_month(date.month() + 1)
                .expect("month has the first day on the month")
                .pred_opt()
                .expect("should not be NaiveDate::MIN")
                .day(),
        )
        .expect("all args are valid date");
        let events = user_dates
            .iter()
            .filter(|(date, _)| &start_of_month <= date && date <= &&end_of_month)
            .flat_map(|(_, events)| events.to_owned())
            .collect::<Vec<_>>();

        ReadState::Success(events)
    }
}

pub(crate) enum MutateState {
    Success,
    UlidNotFound,
    DateNotFound,
    UserNotFound,
}

pub(crate) enum ReadState {
    Success(Vec<Event>),
    DateNotFound,
    UserNotFound,
}
