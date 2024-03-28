use std::ops::Add;
use diesel::{self, Queryable};
use diesel::prelude::*;
use std::time::{Duration, SystemTime};
use diesel::data_types::PgInterval;
use rand::Rng;
use rand::distributions::Alphanumeric;
use crate::db::{DBError, DBPool, DBResult};
use crate::db::users::User;


// Define a struct representing the row in the table
#[derive(Queryable)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created_at: SystemTime,
    pub duration: PgInterval,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::sessions)]
pub struct NewSession {
    pub user_id: i32,
    pub token: String,
    pub duration: PgInterval,
}

impl Session {
    pub fn create(pool: &DBPool, p_user_id: i32, p_duration: Duration) -> DBResult<String> {
        use crate::schema::sessions::dsl::*;
        let tok: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        let new_session = NewSession {
            user_id: p_user_id,
            token: tok.clone(),
            duration: PgInterval::from_microseconds(p_duration.as_micros() as i64),
        };
        let mut conn =  pool.get()?;

        diesel::insert_into(sessions)
            .values(new_session)
            .execute(&mut conn)?;
        Ok(tok)
    }

    pub fn from_token(pool: &DBPool, p_token: &str) -> DBResult<User> {
        use crate::schema::sessions::dsl::*;
        let mut conn =  pool.get()?;
        let session = sessions
            .filter(token.eq(p_token))
            .first::<Session>(&mut conn)?;

        let current_time = SystemTime::now();
        let expiration_time = session.created_at.add(Duration::from_micros(session.duration.microseconds as u64));
        if current_time > expiration_time {
            return Err(DBError::SessionExpired)
        }
        let user = User::from_id(pool, session.user_id)?;
        Ok(user)
    }
    fn del_id(pool: &DBPool, p_id: i32) -> DBResult<()> {
        use crate::schema::sessions::dsl::*;
        let mut conn =  pool.get()?;
        diesel::delete(sessions.filter(id.eq(p_id)))
            .execute(&mut conn)?;
        Ok(())
    }

    fn del_token(pool: &mut DBPool, p_token: &str) -> DBResult<()> {
        use crate::schema::sessions::dsl::*;
        let mut conn = pool.get()?;
        diesel::delete(sessions.filter(token.eq(p_token)))
            .execute(&mut conn).map_err(DBError::DatabaseErr)?;
        Ok(())
    }
}


