
use crate::get_endpoint::{maybe_run_dot_env};
use crate::errors::AppError::{NoJwt};
use crate::types::Result;

pub fn get_jwt_from_env_vars() -> Result<String> {
    maybe_run_dot_env().unwrap();
    match std::env::var("JWT_TOKEN") {
        Ok(jwt) => Ok(jwt),
        Err(e) => Err(NoJwt(e)),
    }
}