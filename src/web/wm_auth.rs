use crate::ctx::Ctx;
use crate::error::Error::{
    AuthFailCtxNotInRequestExt, AuthFailedNoAuthTokenCookie, AuthFailedTokenWrongFormat,
};
use crate::error::{Error, Result};
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    /*cookies: Cookies,*/ req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    // let (user_id, exp, sign) = auth_token
    //     .ok_or(AuthFailedNoAuthTokenCookie)
    //     .and_then(parse_token)?;

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>.
    let result_ctx = match auth_token
        .ok_or(AuthFailedNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            // TODO: Token components validations.
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err() && !matches!(result_ctx, Err(AuthFailedNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        // // User cookies extractor
        // let cookies = parts.extract::<Cookies>().await.unwrap();
        // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
        //
        // // Parse token
        // let (user_id, exp, sign) = auth_token
        //     .ok_or(AuthFailedNoAuthTokenCookie)
        //     .and_then(parse_token)?;
        //
        // // Token components validation
        // Ok(Ctx::new(user_id))

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) =
        regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(AuthFailedTokenWrongFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| AuthFailedTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
