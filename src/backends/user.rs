use crate::{
    backends::Backend,
    db::{
        model::{User, UserCredentials},
        schema::users::dsl::{self, users},
    },
    error::Error,
    Result,
};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

impl Backend {
    /// Search/Find a user with user/pass
    pub fn find_user(&self, credentials: UserCredentials) -> Result<User> {
        let mut conn = self.get_connection()?;

        users
            .find(credentials.username)
            .filter(dsl::password.eq(credentials.password))
            .first(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => Error::NotFound,
                _ => e.into(),
            })
    }

    /// Search/Find a user by token
    pub fn find_user_by_token(&self, token: &str) -> Result<User> {
        let mut conn = self.get_connection()?;

        users
            .filter(dsl::token.eq(token))
            .first(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => Error::NotFound,
                _ => e.into(),
            })
    }

    /// Add a new user
    pub fn add_user(&self, new_user: User) -> Result<()> {
        let mut conn = self.get_connection()?;

        Ok(diesel::insert_into(users)
            .values(new_user)
            .execute(&mut conn)
            .map(|_| ())?)
    }

    /// Update a user
    pub fn update_user(&self, user: User) -> Result<User> {
        let mut conn = self.get_connection()?;

        match diesel::update(&user).set(&user).execute(&mut conn)? {
            0 => Err(Error::NotFound),
            1 => Ok(user),
            i => Err(Error::InvalidResult(format!(
                "Updated {} rows in users table instead of exactly 1",
                i,
            ))),
        }
    }

    /// Delete a given user
    pub fn delete_user(&self, username: &str) -> Result<()> {
        let mut conn = self.get_connection()?;

        match diesel::delete(users.find(username)).execute(&mut conn)? {
            0 => Err(Error::NotFound),
            1 => Ok(()),
            i => Err(Error::InvalidResult(format!(
                "Deleted {} rows in users table instead of exactly 1",
                i,
            ))),
        }
    }

    /// Retrieve/list all users
    pub fn list_users(&self) -> Result<Vec<User>> {
        let mut conn = self.get_connection()?;

        Ok(users.load(&mut conn)?)
    }
}
