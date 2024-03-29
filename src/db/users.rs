use crate::db::sessions::Session;
use crate::db::{DBError, DBPool, DBResult};
use argon2;
use diesel::prelude::*;
use diesel::Queryable;
use std::time::{Duration, SystemTime};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: SystemTime,
}

fn hash_password(password: &str, salt: &str) -> Result<String, &'static str> {
    let config = argon2::Config {
        hash_length: 32,
        ..argon2::Config::default()
    };
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)
        .map_err(|_| "Error hashing password")
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl NewUser {
    pub fn new(username: &str, email: &str, password: &str) -> Result<NewUser, &'static str> {
        let password_hash = hash_password(password, username)?;
        Ok(NewUser {
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
        })
    }
}

impl User {
    pub fn create(pool: &DBPool, newuser: NewUser) -> DBResult<User> {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get()?;
        let val = diesel::insert_into(users)
            .values(newuser)
            .returning((id, username, email, password_hash, created_at))
            .get_result::<User>(&mut conn)?;
        Ok(val)
    }

    pub fn from_id(pool: &DBPool, p_id: i32) -> DBResult<User> {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get()?;
        let user = users.filter(id.eq(p_id)).first::<User>(&mut conn)?;
        Ok(user)
    }

    pub fn authenticate(pool: &DBPool, p_username: &str, p_password: &str) -> DBResult<User> {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get()?;
        let hashed_pwd = hash_password(p_password, p_username)
            .map_err(|err| DBError::InternalErr(err.to_string()))?;
        let user = users
            .filter(username.eq(p_username))
            .filter(password_hash.eq(hashed_pwd))
            .first::<User>(&mut conn)?;
        Ok(user)
    }

    pub fn create_session(&self, pool: &DBPool, duration: Duration) -> DBResult<String> {
        Session::create(pool, self.id, duration)
    }

    fn delete(pool: &DBPool, p_id: i32) -> DBResult<()> {
        use crate::schema::users::dsl::*;
        let mut conn = pool.get()?;
        diesel::delete(users.filter(id.eq(p_id))).execute(&mut conn)?;
        Ok(())
    }
}
