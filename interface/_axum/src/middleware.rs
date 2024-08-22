use axum::extract::FromRequest;
use axum::{async_trait, http::StatusCode};
use common::functions::decode_token;
use serde::{Deserialize, Serialize};
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

#[async_trait]
impl<B> FromRequest<B> for AuthorizationService
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let auth = req
            .headers()
            .and_then(|headers| headers.get("Authorization"))
            .and_then(|header| header.to_str().ok());

        match auth {
            Some(token) => {
                let id = decode_token(token).unwrap_or_default();
                Ok(AuthorizationService { id: id.to_string() })
            }
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}

#[derive(Debug)]
pub struct XOrganizationService {
    pub organization_id: String,
}

#[async_trait]
impl<B> FromRequest<B> for XOrganizationService
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let organization_id = req
            .headers()
            .and_then(|headers| headers.get("X-Organization-ID"))
            .and_then(|header| header.to_str().ok());

        match organization_id {
            Some(organization_id) => Ok(XOrganizationService {
                organization_id: organization_id.to_string(),
            }),
            _ => Err(StatusCode::NOT_ACCEPTABLE),
        }
    }
}

#[derive(Debug)]
pub struct XBranchService {
    pub branch_id: String,
}

#[async_trait]
impl<B> FromRequest<B> for XBranchService
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let branch_id = req
            .headers()
            .and_then(|headers| headers.get("X-Branch-ID"))
            .and_then(|header| header.to_str().ok());

        match branch_id {
            Some(branch_id) => Ok(XBranchService {
                branch_id: branch_id.to_string(),
            }),
            _ => Err(StatusCode::NOT_ACCEPTABLE),
        }
    }
}
