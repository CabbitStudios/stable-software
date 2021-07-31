mod layout;
mod horses;
mod utils;

use std::future::{ready, Ready};

use actix_web::FromRequest;

pub use layout::*;
pub use horses::*;
pub use utils::*;

pub struct ViewContext {
    path: String,
}

impl ViewContext {
    pub fn path_starts_with(&self, pat: &str) -> bool {
        self.path.starts_with(pat)
    }
}

impl FromRequest for ViewContext {
    type Error = ();

    type Future = Ready<Result<Self, ()>>;

    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ready(Ok(ViewContext {
            path: req.path().to_string(),
        }))
    }
}
