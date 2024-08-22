use actix_web::error::{ErrorNotAcceptable, ErrorUnauthorized};
use actix_web::{dev, Error, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};

use std::future::{ready, Future};
use std::pin::Pin;

use common::functions::decode_token;

// ==================== MANAGEMENT ==================== //

#[derive(Serialize, Debug, Deserialize)]
pub struct SignInResponse {
    pub message: String,
    pub status: bool,
    pub access_token: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Response {
    pub message: String,
    pub status: bool,
}

#[derive(Debug)]
pub struct AuthorizationService {
    pub id: String,
}

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<AuthorizationService, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth = req.headers().get("Authorization");
        Box::pin(ready(match auth {
            Some(token) => {
                let id = decode_token(token).unwrap_or_default();

                Ok(AuthorizationService { id: id.to_string() })
            }
            _ => Err(ErrorUnauthorized(format!(
                "{:#?}",
                std::collections::HashMap::from([("error", "None auth token")])
            ))),
        }))
    }
}

#[derive(Debug)]
pub struct XOrganizationService {
    pub organization_id: String,
}

impl FromRequest for XOrganizationService {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<XOrganizationService, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let organization_id = req.headers().get("X-Organization-ID");
        Box::pin(ready(match organization_id {
            Some(organization_id) => Ok(XOrganizationService {
                organization_id: organization_id.to_str().unwrap_or_default().to_string(),
            }),
            _ => Err(ErrorNotAcceptable(format!(
                "{:#?}",
                std::collections::HashMap::from([("error", "None organization_id")])
            ))),
        }))
    }
}

#[derive(Debug)]
pub struct XBranchService {
    pub branch_id: String,
}

impl FromRequest for XBranchService {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<XBranchService, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let branch_id = req.headers().get("X-Branch-ID");
        Box::pin(ready(match branch_id {
            Some(branch_id) => Ok(XBranchService {
                branch_id: branch_id.to_str().unwrap_or_default().to_string(),
            }),
            _ => Err(ErrorNotAcceptable(format!(
                "{:#?}",
                std::collections::HashMap::from([("error", "None branch_id")])
            ))),
        }))
    }
}
