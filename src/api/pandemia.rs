//! Koleksi query yang digunakan untuk operasi pada rest API Pandemia
#![allow(missing_docs)]

use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{
        error::{param_error, Error},
        ApiResult, Error as ApiError, HttpRequest as ApiHttpRequest,
    },
    auth,
    dao::RecordDao,
    error::{self, ErrorCode},
    eventstream::{self, Event::NewRecordUpdate},
    models,
    prelude::*,
    types::LocKind,
    ID,
};

#[derive(Deserialize, Validate)]
pub struct RecordUpdate {
    #[validate(range(min = 1, max = 9999999999))]
    pub id: i64,
    #[validate(length(min = 2, max = 1000))]
    pub loc: String,
    pub loc_kind: i16,
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
    pub active_cases: i32,
    pub critical_cases: i32,
    pub cases_to_pop: f64,
    pub meta: Vec<String>,
    pub last_updated: NaiveDateTime,
}

#[derive(Deserialize, Validate)]
pub struct UpdateRecords {
    records: Vec<RecordUpdate>,
}

#[derive(Serialize)]
pub struct InfoLocation {
    pub name: String,
    pub latest_record: models::Record,
    pub history: Vec<models::Record>,
}

#[derive(Deserialize, Validate)]
pub struct AddRecord {
    pub loc: String,
    pub loc_kind: i16,
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
    pub active_cases: i32,
    pub critical_cases: i32,
}

/// Holder untuk implementasi API endpoint publik untuk Pandemia.
pub struct PublicApi;

#[api_group("Pandemia", "public", base = "/pandemia/v1")]
impl PublicApi {

    /// Add record.
    #[api_endpoint(path = "/add_record", auth = "required", mutable, accessor = "admin")]
    pub fn add_record(query: AddRecord) -> ApiResult<models::Record> {
        query.validate()?;
        let conn = state.db();
        let dao = RecordDao::new(&conn);

        let record = dao.create(
            &query.loc,
            query.loc_kind.into(),
            query.total_cases,
            query.total_deaths,
            query.total_recovered,
            query.active_cases,
            query.critical_cases,
            0.0,
            &vec![],
            false,
        )?;

        eventstream::emit(NewRecordUpdate(None, record.clone()));

        Ok(ApiResult::success(record))
    }

    /// Get location info (single mode)
    #[api_endpoint(path = "/info_location", auth = "none")]
    pub fn get_info_location(query: LocationQuery) -> ApiResult<Option<models::Record>> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);
        let locs: Vec<&str> = vec![query.loc.as_str()];
        let mut rec = dao.get_latest_records(locs, 0, 3)?;

        if rec.first().is_some() {
            Ok(ApiResult::success(Some(rec.swap_remove(0))))
        } else {
            Ok(ApiResult::success(None))
        }
    }

    /// Get location info (multiple mode)
    #[api_endpoint(path = "/info_locations", auth = "none")]
    pub fn get_info_locations(query: LocationQuery) -> ApiResult<Vec<InfoLocation>> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);

        let locs: Vec<&str> = query.loc.split(',').collect();

        let records = dao.get_latest_records(locs, 0, 10)?;

        let mut result = vec![];

        for rec in records {
            let mut history: Vec<models::Record> = vec![];

            if query.with_history == Some(true) {
                history = dao.get_record_history(&rec.loc, 0, 30)?;
            }

            result.push(InfoLocation {
                name: rec.loc.to_owned(),
                latest_record: rec,
                history,
            });
        }

        Ok(ApiResult::success(result))
    }

    // /// Search for records
    // #[api_endpoint(path = "/latest_records", auth = "none")]
    // pub fn latest_records(query: QueryEntries) -> ApiResult<EntriesResult<models::Record>> {
    //     let conn = state.db();
    //     let dao = RecordDao::new(&conn);
    //     let entries = dao
    //         .get_latest_records(query.query.as_ref().map(|a| a.as_str()), query.offset, query.limit)?
    //         .into_iter()
    //         .map(|p| p.into())
    //         .collect();
    //     let count = dao.count()?;
    //     Ok(ApiResult::success(EntriesResult { count, entries }))
    // }

    /// Search for records
    #[api_endpoint(path = "/search_records", auth = "required", accessor = "admin")]
    pub fn search_records(query: QueryEntries) -> ApiResult<EntriesResult<models::Record>> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);

        let result = dao.search(&query.query.unwrap_or("".to_string()), query.offset, query.limit)?;

        Ok(ApiResult::success(EntriesResult {
            count: result.count,
            entries: result.entries,
        }))
    }

    /// Update multiple records at once.
    #[api_endpoint(path = "/update_records", auth = "required", mutable, accessor = "admin")]
    pub fn update_records(query: UpdateRecords) -> ApiResult<()> {
        use crate::schema::records::{self, dsl};
        query.validate()?;

        let conn = state.db();

        conn.build_transaction()
            .read_write()
            .run::<_, error::Error, _>(|| {
                let dao = RecordDao::new(&conn);

                for record in query.records {
                    let old_record = dao.get_latest_records(vec![record.loc.as_ref()], 0, 1)?.pop();

                    let new_record = dao.create(
                        &record.loc,
                        record.loc_kind.into(),
                        record.total_cases,
                        record.total_deaths,
                        record.total_recovered,
                        record.active_cases,
                        record.critical_cases,
                        record.cases_to_pop,
                        &record.meta.iter().map(|a| a.as_str()).collect(),
                        true,
                    )?;

                    if let Some(old_record) = old_record {
                        let diff = new_record.diff(&old_record);

                        if diff.new_cases > 0
                            || diff.new_deaths > 0
                            || diff.new_recovered > 0
                            || diff.new_critical > 0
                        {
                            eventstream::emit(NewRecordUpdate(Some(old_record.clone()), new_record.clone()));
                        }
                    }

                    debug!("updating record {}...", record.id);
                }

                Ok(())
            })?;

        Ok(ApiResult::success(()))
    }

    /// Delete record by id
    #[api_endpoint(path = "/delete_record", auth = "required", mutable, accessor = "admin")]
    pub fn delete_record(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = RecordDao::new(&conn);
        let rec = dao.get_by_id(query.id)?;
        dao.delete_by_id(rec.id)?;
        Ok(ApiResult::success(()))
    }
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Pandemia", "private", base = "/pandemia/v1")]
impl PrivateApi {}
