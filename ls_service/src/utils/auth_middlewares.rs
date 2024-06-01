// use crate::utils::{api_response::ApiResponse, jwt::decode_jwt};
// use actix_web::{body::MessageBody,
//                 dev::{ServiceRequest, ServiceResponse},
//                 Error};
// use actix_web_lab::middleware::Next;

// pub async fn check_auth_middleware(req: ServiceRequest,
//                                    next: Next<impl MessageBody>)
//                                    -> Result<ServiceResponse<impl MessageBody>, Error>
// {
//     // Check if the request has a header named "Authorization)
//     let auth = req.headers().get("AUTHORIZATION");

//     if auth.is_none()
//     {
//         return Err(Error::from(ApiResponse::new(401, "Unauthorized".to_string())));
//     }

//     let token = auth.unwrap()
//                     .to_str()
//                     .unwrap()
//                     .replace("Bearer ", "")
//                     .to_owned();
//     let claim = decode_jwt(token).unwrap();
//     println!("{:?}", claim);

//     next.call(req)
//         .await
//         .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())))
// }
