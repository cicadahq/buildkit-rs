use buildkit_rs_proto::moby::filesync::v1::{
    auth_server::AuthServer, CredentialsRequest, CredentialsResponse, FetchTokenRequest,
    FetchTokenResponse, GetTokenAuthorityRequest, GetTokenAuthorityResponse,
    VerifyTokenAuthorityRequest, VerifyTokenAuthorityResponse,
};
use tonic::{Request, Response, Status};
use tracing::debug;

#[derive(Debug, Clone, Default)]
pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_server(self) -> AuthServer<Self> {
        AuthServer::new(self)
    }
}

#[tonic::async_trait]
impl buildkit_rs_proto::moby::filesync::v1::auth_server::Auth for AuthService {
    #[tracing::instrument(skip_all)]
    async fn credentials(
        &self,
        request: Request<CredentialsRequest>,
    ) -> Result<Response<CredentialsResponse>, Status> {
        debug!(?request);
        Ok(Response::new(CredentialsResponse::default()))
    }

    #[tracing::instrument(skip_all)]
    async fn fetch_token(
        &self,
        request: Request<FetchTokenRequest>,
    ) -> Result<Response<FetchTokenResponse>, Status> {
        debug!(?request);
        Err(Status::unimplemented("Not implemented yet"))
    }

    #[tracing::instrument(skip_all)]
    async fn get_token_authority(
        &self,
        request: Request<GetTokenAuthorityRequest>,
    ) -> Result<Response<GetTokenAuthorityResponse>, Status> {
        debug!(?request);
        Err(Status::unimplemented("Not implemented yet"))
    }

    #[tracing::instrument(skip_all)]
    async fn verify_token_authority(
        &self,
        request: Request<VerifyTokenAuthorityRequest>,
    ) -> Result<Response<VerifyTokenAuthorityResponse>, Status> {
        debug!(?request);
        Err(Status::unimplemented("Not implemented yet"))
    }
}
