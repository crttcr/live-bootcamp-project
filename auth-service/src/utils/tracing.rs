
use axum::{body::Body, extract::Request, response::Response};
use tracing;
use tracing::{Level, Span};
use tracing_subscriber;
use std::time::Duration;

pub fn init_tracing() {
	let level = Level::DEBUG;
	tracing_subscriber::fmt()
		.compact()
		.with_max_level(level)
		.init();
}


// Creates a new tracing span with a unique request ID for each incoming request.
// This helps in tracking and correlating logs for individual requests.
//
pub fn make_span_with_request_id(request: &Request<Body>) -> Span 
{
	let _request_id = uuid::Uuid::new_v4();
	let request_id  = ulid::Ulid::new().to_string();
	let method      = request.method();
	let uri         = request.uri();
	let version     = request.version();
	
	tracing::span!(
        Level::INFO,
        "[REQUEST]",
        method          = tracing::field::display(method),
        uri             = tracing::field::display(uri),
        version         = tracing::field::debug(version),
        request_id      = tracing::field::display(request_id),
    )
}

// Logs an event indicating the start of a request.
//
pub fn on_request(_request: &Request<Body>, _span: &Span) {
	tracing::event!(Level::INFO, "[REQUEST START]");
}

// Logs an event indicating the end of a request, including its latency and status code.
// If the status code indicates an error (4xx or 5xx), it logs at the ERROR level.
//
pub fn on_response(response: &Response, latency: Duration, _span: &Span) {
	let status            = response.status();
	let status_code       = status.as_u16();
	let status_code_class = status_code / 100;
	match status_code_class {
		4..=5 => {
			tracing::event!(
                Level::ERROR,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
		}
		_ => {
			tracing::event!(
                Level::INFO,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
		}
	};
}