mod prelude;
pub(crate) const SOFT_ID: u16 = 4143;

pub mod captcha_types;
pub mod error;
pub mod proxy;
pub mod solver;

pub use captcha_types::CaptchaTask;
pub use error::*;
pub use solver::Solver;
