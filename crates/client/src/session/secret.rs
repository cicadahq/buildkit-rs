use std::{collections::HashMap, path::PathBuf};

use buildkit_rs_proto::moby::buildkit::secrets::v1::{
    secrets_server::{Secrets, SecretsServer},
    GetSecretRequest, GetSecretResponse,
};
use tonic::{Request, Response, Status};

/// The source of a secret.
#[derive(Debug, Clone)]
pub enum SecretSource {
    /// The secret is stored in an environment variable.
    Env(String),
    /// The secret is stored in a file.
    File(PathBuf),
    /// The secret is stored in memory.
    Memory(Vec<u8>),
}

#[derive(Debug)]
pub struct SecretService {
    secrets: HashMap<String, SecretSource>,
}

impl SecretService {
    pub fn new<I, K, V>(secrets: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<SecretSource>,
    {
        Self {
            secrets: secrets
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }

    pub fn into_server(self) -> SecretsServer<Self> {
        SecretsServer::new(self)
    }
}

#[tonic::async_trait]
impl Secrets for SecretService {
    #[tracing::instrument(skip_all)]
    async fn get_secret(
        &self,
        request: Request<GetSecretRequest>,
    ) -> Result<Response<GetSecretResponse>, Status> {
        let GetSecretRequest { id, .. } = request.into_inner();

        match self.secrets.get(&id) {
            Some(secret) => {
                let data = match secret {
                    SecretSource::Env(key) => match std::env::var(key) {
                        Ok(value) => value.into_bytes(),
                        Err(err) => {
                            return Err(Status::not_found(format!(
                                "Secret with id {id} not found: {err}",
                            )));
                        }
                    },
                    SecretSource::File(path) => tokio::fs::read(path).await.map_err(|err| {
                        Status::not_found(format!("Secret with id {id} not found: {err}",))
                    })?,
                    SecretSource::Memory(val) => val.clone(),
                };

                Ok(Response::new(GetSecretResponse { data }))
            }
            None => {
                return Err(Status::not_found(format!("Secret with id {id} not found",)));
            }
        }
    }
}
