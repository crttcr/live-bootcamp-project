use axum::http::header::SET_COOKIE;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_extra::extract::CookieJar;

#[derive(Debug)]
pub struct WithCookies<T>
	where T: IntoResponse {
	pub status:   StatusCode,
	pub cookies:  CookieJar,
	pub body:     T,
}

impl<T> IntoResponse for WithCookies<T>
where
	T: IntoResponse,
{
	fn into_response(self) -> Response {
		let mut response = (self.status, self.body).into_response();

		for cookie in self.cookies.iter().into_iter() {
			let c = cookie.to_string().parse().unwrap();
			response
				.headers_mut()
				.append(SET_COOKIE, c);
		}

		response
	}
}


/*
#[cfg(test)]
mod tests {
	use super::*;
	use crate::routes::LoginResponse::TwoFactorAuth;
	use crate::routes::TwoFactorAuthResponse;
	use axum::Json;
	use crate::domain::LoginAttemptId;
	
	#[test]
	fn it_works() {
		let status     = StatusCode::PARTIAL_CONTENT;
		let cookies    = CookieJar::new();
		let attempt_id = LoginAttemptId::new();
		let response   = TwoFactorAuthResponse::new(attempt_id);
		let response   = TwoFactorAuth(response);
		let body       = Json(response);
		let response   = WithCookies { status, cookies, body };
		println!("{:?}", response.into_response());
	}
}
*/