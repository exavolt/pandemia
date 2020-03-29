//! Dao implementation for Record
//!

use chrono::prelude::*;
use diesel::prelude::*;
use diesel::sql_types;

use crate::{models::Record, result::Result, schema::records, types::EntriesResult, types::LocKind, ID};

/// This model structure modeled after data from https://www.worldometers.info/coronavirus/
#[derive(Insertable)]
#[table_name = "records"]
struct NewRecord<'a> {
    pub loc: &'a str,
    pub loc_kind: i16,
    pub total_cases: i32,
    pub total_deaths: i32,
    pub total_recovered: i32,
    pub active_cases: i32,
    pub critical_cases: i32,
    pub cases_to_pop: f64,
    pub meta: &'a Vec<&'a str>,
    pub latest:bool
}

/// Data Access Object for Record
#[derive(Dao)]
#[table_name = "records"]
pub struct RecordDao<'a> {
    db: &'a PgConnection,
}

impl<'a> RecordDao<'a> {
    /// Create new Record
    pub fn create(
        &self,
        loc: &'a str,
        loc_kind: LocKind,
        total_cases: i32,
        total_deaths: i32,
        total_recovered: i32,
        active_cases: i32,
        critical_cases: i32,
        cases_to_pop: f64,
        meta: &'a Vec<&'a str>,
    ) -> Result<Record> {
        use crate::schema::records::{self, dsl};

        self.db.build_transaction().read_write().run::<_, _, _>(|| {
            // reset dulu yang ada flag latest-nya ke false
            diesel::update(dsl::records.filter(dsl::latest.eq(true)))
                .set(dsl::latest.eq(false))
                .execute(self.db)?;

            // tambahkan record baru dengan latest=true
            diesel::insert_into(records::table)
                .values(&NewRecord {
                    loc,
                    loc_kind: loc_kind as i16,
                    total_cases,
                    total_deaths,
                    total_recovered,
                    active_cases,
                    critical_cases,
                    cases_to_pop,
                    meta,
                    latest: true,
                })
                .get_result(self.db)
                .map_err(From::from)
        })
    }

    /// Get stock histories based on Record
    pub fn get_latest_records(&self, loc: Option<&str>, offset: i64, limit: i64) -> Result<Vec<Record>> {
        use crate::schema::records::dsl;

        assert!(offset > -1, "Invalid offset");
        assert!(limit > -1, "Invalid limit");
        assert!(limit < 1_000_000, "Invalid limit");

        if let Some(loc) = loc {
            dsl::records
                .filter(dsl::loc.eq(loc))
                .order(dsl::last_updated.desc())
                .offset(offset)
                .limit(limit)
                .load(self.db)
                .map_err(From::from)
        } else {
            dsl::records
                .order(dsl::last_updated.desc())
                .offset(offset)
                .limit(limit)
                .load(self.db)
                .map_err(From::from)
        }
    }

    /// Search for specific records only take the latest one for each location
    pub fn search(&self, query: &str, offset: i64, limit: i64) -> Result<EntriesResult<Record>> {
        use crate::schema::records::{self, dsl};

        // select * from (select *, rank() OVER (PARTITION BY loc order by last_updated desc) from records) as d where d.rank=1;

        let like_clause = format!("%{}%", query);
        let mut filterer: Box<dyn BoxableExpression<records::table, _, SqlType = sql_types::Bool>> =
            Box::new(dsl::id.ne(0).and(dsl::latest.eq(true)) );

        filterer = Box::new(filterer.and(dsl::loc.like(&like_clause)));

        Ok(EntriesResult::new(
            dsl::records
                .filter(&filterer)
                .offset(offset)
                .limit(limit)
                .load::<Record>(self.db)?,
            dsl::records
                .filter(filterer)
                .select(diesel::dsl::count(dsl::id))
                .first(self.db)?,
        ))
    }
}
