use std::error::Error;

type BoxError = Box<dyn Error + Send + Sync>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StravaAuthError {
    /// The request could not be sent, or the response could not be read.
    #[error("request to Strava failed")]
    Request(#[source] BoxError),

    /// Strava responded with a non-success status code.
    #[error("Strava returned HTTP {status}: {body}")]
    Status { status: u16, body: String },

    /// The response body was not a valid token record.
    #[error("failed to decode Strava token response")]
    Decode(#[source] BoxError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_error_displays_code_and_body() {
        let err = StravaAuthError::Status {
            status: 401,
            body: String::from("Authorization Error"),
        };
        assert_eq!(
            err.to_string(),
            "Strava returned HTTP 401: Authorization Error"
        );
        assert!(err.source().is_none());
    }

    #[test]
    fn request_and_decode_errors_expose_source() {
        let request = StravaAuthError::Request("connection reset".into());
        assert_eq!(request.to_string(), "request to Strava failed");
        assert_eq!(request.source().unwrap().to_string(), "connection reset");

        let decode = StravaAuthError::Decode("missing field `access_token`".into());
        assert_eq!(decode.to_string(), "failed to decode Strava token response");
        assert_eq!(
            decode.source().unwrap().to_string(),
            "missing field `access_token`"
        );
    }

    #[test]
    fn error_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync + 'static>() {}
        assert_send_sync::<StravaAuthError>();
    }
}
