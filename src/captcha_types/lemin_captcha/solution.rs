use std::borrow::Cow;

use catptcha_oxide_derive::captcha_solution;

#[captcha_solution]
pub struct LeminCaptchaSolution<'a> {
    pub answer: Cow<'a, str>,

    pub challenge_id: Cow<'a, str>,
}
