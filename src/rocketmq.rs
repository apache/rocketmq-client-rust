use crate::pb::{
    messaging_service_client::MessagingServiceClient, QueryRouteRequest, QueryRouteResponse,
};
use rustls::client::ServerCertVerifier;
use tonic::{
    transport::{Channel, ClientTlsConfig},
    Request, Response,
};

pub struct RpcClient {
    stub: MessagingServiceClient<Channel>,
    remote_address: String,
}

struct TrustAllCertVerifier;

impl ServerCertVerifier for TrustAllCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &rustls::Certificate,
        intermediates: &[rustls::Certificate],
        server_name: &rustls::ServerName,
        scts: &mut dyn Iterator<Item = &[u8]>,
        ocsp_response: &[u8],
        now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

impl RpcClient {
    pub async fn new(target: &'static str) -> Result<RpcClient, Box<dyn std::error::Error>> {
        let remote_address = String::from(target);

        let mut channel = Channel::from_shared(target)?
            .tcp_nodelay(true)
            .connect_timeout(std::time::Duration::from_secs(3));
        if remote_address.starts_with("https://") {
            let verifier = std::sync::Arc::new(TrustAllCertVerifier {});
            let rustls_config = rustls::client::ClientConfig::builder()
                .with_safe_defaults()
                .with_custom_certificate_verifier(verifier)
                .with_no_client_auth();
            //TODO: Disable verify server certificate
            let tls_config = ClientTlsConfig::new();
            channel = channel.tls_config(tls_config)?;
        }
        let channel = channel.connect().await?;
        let stub = MessagingServiceClient::new(channel);
        Ok(RpcClient {
            stub,
            remote_address,
        })
    }

    pub async fn query_route(
        &mut self,
        request: QueryRouteRequest,
    ) -> Result<Response<QueryRouteResponse>, Box<dyn std::error::Error>> {
        let req = Request::new(request);
        Ok(self.stub.query_route(req).await?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pb::{Resource, Code};

    #[tokio::test]
    async fn test_connect() {
        let target = "http://127.0.0.1:5001";
        let mut rpc_client = RpcClient::new(target)
            .await
            .expect("Should be able to connect");
    }

    #[tokio::test]
    async fn test_query_route() {
        let target = "http://127.0.0.1:5001";
        let mut rpc_client = RpcClient::new(target)
            .await
            .expect("Should be able to connect");
        let topic = Resource {
            resource_namespace: String::from("arn"),
            name: String::from("TestTopic"),
        };
        let request = QueryRouteRequest {
            topic: Some(topic),
            endpoints: None,
        };

        let reply = rpc_client
            .query_route(request)
            .await
            .expect("Failed to query route");
        let route_response = reply.into_inner();
        assert_eq!(route_response.status.unwrap().code, Code::Ok as i32);
    }
}
