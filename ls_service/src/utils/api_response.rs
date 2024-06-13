use std::fmt::{self, Display, Formatter};

use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder, ResponseError};


#[derive(Debug)]
pub struct ApiResponse{
    pub _status_code: u16,
    pub body: String,
    response_code: StatusCode
}

impl ApiResponse{
    pub fn new(_status_code: u16, body: String) -> Self {
        ApiResponse{
            _status_code,
            body,
            response_code: StatusCode::from_u16(_status_code).unwrap()
        }
    }
}

impl Responder for ApiResponse{
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        println!("req is {}", req.path());
        HttpResponse::new(self.response_code).set_body(body)
    }
}

impl Display for ApiResponse{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {} \n Status Code: {}", self.body, self._status_code)
    }
}

impl ResponseError for ApiResponse{

    fn status_code(&self) -> StatusCode {
        self.response_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.status_code()).set_body(body)
    }
}


// // response codes
// enum ResponseCode{
//     Ok = 200,
//     Created = 201,
//     Accepted = 202,
//     NoContent = 204,
//     Found = 302,
//     NotModified = 304,
//     BadRequest = 400,
//     Unauthorized = 401,
//     Forbidden = 403,
//     NotFound = 404,
//     MethodNotAllowed = 405,
//     RequestTimeout = 408,
//     Confict = 409,
//     TooManyRequests = 429,
//     InternalServerError = 500,
//     NotImplemented = 501,
//     BadGateway = 502,
//     ServiceUnavailable = 503,
//     GatewayTimeout = 504
// }