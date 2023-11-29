use std::borrow::Cow;

use captcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct CyberSiARACaptchaSolution<'a> {
    pub token: Cow<'a, str>,
}
