use captcha_oxide_derive::proxy_task;
use std::borrow::Cow;
use url::Url;

use crate::{captcha_types::empty_data::Empty, CaptchaTask};

/// Represents the data required by the 2captcha API to solve a
/// reCaptcha V2 Enterprise challenge
///
/// # Example
/// ```
/// use captcha_oxide::{
///     Error,
///     CaptchaTask,
///     captcha_types::recaptcha::RecaptchaV2Enterprise
/// };
///
/// # fn main() -> Result<(), Error> {
/// let captcha = <RecaptchaV2Enterprise>::builder()
///     .website_url("http://someurl.com")
///     .website_key("SOME_KEY")
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// The angle brackets (`<>`) around [`RecaptchaV2Enterprise`] allow the
/// use of the default type provided to the generic argument, so you don't
/// need to create a serializable unit struct if you don't plan to use the
/// [`RecaptchaV2Enterprise::enterprise_payload`] field
#[proxy_task(
    with_proxy = "RecaptchaV2EnterpriseTask",
    proxyless = "RecaptchaV2EnterpriseTaskProxyless",
    crate = crate,
)]
#[derive(serde::Serialize, CaptchaTask)]
#[task(timeout = 20, solution = super::super::solution::RecaptchaSolution<'a>, crate = crate)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaV2Enterprise<'a, T = Empty>
where
    T: serde::Serialize,
{
    /// The full URL of target web page where the captcha is loaded.
    /// We do not open the page, so it is not a problem if it is available
    /// only for authenticated users
    #[serde(rename = "websiteURL")]
    #[task(builder_type = &'a str, parse_with = { fallible({ path = url::Url::parse }) })]
    pub(super) website_url: Url,

    /// reCAPTCHA sitekey. Can be found inside `data-sitekey` property of the reCAPTCHA
    /// `div` element or inside the `k` parameter of the requests to the reCAPTHCHA API.
    /// You can also use [this script](https://gist.github.com/2captcha/2ee70fa1130e756e1693a5d4be4d8c70)
    /// to find the value
    pub(super) website_key: Cow<'a, str>,

    /// Additional parameters passed to `grecaptcha.enterprise.render` call. For example,
    /// there can be an object containing and `s` value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) enterprise_payload: Option<T>,

    /// Indicates the use of the invisible version of reCAPTCHA - a case when you
    /// don't see the checkbox, but the challenge appears. Mostly used with a
    /// callback function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_invisible: Option<bool>,

    /// User-Agent your browser will be used to load the captcha.
    /// Use only modern browsers' User-Agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user_agent: Option<Cow<'a, str>>,

    /// Your cookies will be set in a browser of our worker. Suitable
    /// for captcha on Google services.
    ///
    /// May be passed in as an iterable (array, slice or [Vec]) of
    /// [`crate::cookie::Cookie`] or [`(impl ToString, impl ToString)`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[task(builder_type = Option<crate::cookie::Cookies<'a>>, parse_with = { infallible({ path = crate::cookie::Cookies::stringify, parse_ref }) })]
    pub(super) cookies: Option<Cow<'a, str>>,

    /// Domain used to load the captcha: `google.com` or `recaptcha.net`.
    /// Default value: `google.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) api_domain: Option<Cow<'a, str>>,
}
