pub mod connhelper;
mod error;

use buildkit_rs_proto::moby::buildkit::v1::frontend::llb_bridge_client::LlbBridgeClient;
use connhelper::docker::docker_connect;
use error::Error;
use tonic::{
    transport::{Channel, Uri},
    IntoRequest, Request,
};
use tower::service_fn;

type LlbBridge = LlbBridgeClient<Channel>;

async fn connect() -> Result<LlbBridge, Error> {
    let channel = Channel::from_static("http://[::1]:50051")
        .connect_with_connector(service_fn(|_: Uri| docker_connect("buildkitd")))
        .await?;

    Ok(LlbBridgeClient::new(channel))
}

trait BuildkitRequestBuilder<T> {
    fn with_buildid(self, buildid: &str) -> Request<T>;
}

impl<T: IntoRequest<Req>, Req> BuildkitRequestBuilder<Req> for T {
    fn with_buildid(self, buildid: &str) -> Request<Req> {
        let mut request = self.into_request();
        request
            .metadata_mut()
            .insert("buildkit-controlapi-buildid", buildid.parse().unwrap());
        request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        use buildkit_rs_proto::moby::buildkit::v1::frontend::PingRequest;

        let mut conn = connect().await.unwrap();
        dbg!(conn.ping(PingRequest {}.with_buildid("0")).await);
    }
}
