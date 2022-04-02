use crate::pb::{
    messaging_service_client::MessagingServiceClient, QueryRouteRequest, QueryRouteResponse,
    SendMessageRequest, SendMessageResponse,
};
use tonic::{
    metadata::MetadataMap,
    transport::{Channel, ClientTlsConfig},
    Request, Response,
};

#[derive(Debug, PartialEq)]
pub struct Credentials {
    access_key: String,
    access_secret: String,
    session_token: Option<String>,
}

pub trait CredentialProvider {
    fn get_credentials(&self) -> Credentials;
}

pub struct StaticCredentialProvider {
    access_key: String,
    access_secret: String,
}

impl StaticCredentialProvider {
    pub fn new(access_key: &str, access_secret: &str) -> Self {
        Self {
            access_key: access_key.to_owned(),
            access_secret: access_secret.to_owned(),
        }
    }
}

impl CredentialProvider for StaticCredentialProvider {
    fn get_credentials(&self) -> Credentials {
        Credentials {
            access_key: self.access_key.clone(),
            access_secret: self.access_secret.clone(),
            session_token: None,
        }
    }
}

#[derive(Default)]
struct ClientConfig {
    region: String,
    service_name: String,
    resource_namespace: String,
    credential_provider: Option<Box<dyn CredentialProvider>>,
    tenant_id: String,
    io_timeout: std::time::Duration,
    long_polling_timeout: std::time::Duration,
    group: String,
    client_id: String,
    tracing: bool,
}

pub struct RpcClient {
    stub: MessagingServiceClient<Channel>,
    peer_address: String,
    // client_config: std::rc::Rc<ClientConfig>,
}

impl RpcClient {
    pub async fn new(target: String) -> Result<RpcClient, Box<dyn std::error::Error>> {
        let mut channel = Channel::from_shared(target.clone())?
            .tcp_nodelay(true)
            .connect_timeout(std::time::Duration::from_secs(3));
        if target.starts_with("https://") {
            channel = channel.tls_config(ClientTlsConfig::new())?;
        }
        let channel = channel.connect().await?;
        let stub = MessagingServiceClient::new(channel);
        Ok(RpcClient {
            stub,
            peer_address: target,
        })
    }

    fn add_metadata(meta: &mut MetadataMap) {}

    pub async fn query_route(
        &mut self,
        request: QueryRouteRequest,
    ) -> Result<Response<QueryRouteResponse>, Box<dyn std::error::Error>> {
        let mut req = Request::new(request);
        RpcClient::add_metadata(req.metadata_mut());
        Ok(self.stub.query_route(req).await?)
    }

    pub async fn send_message(
        &mut self,
        request: SendMessageRequest,
    ) -> Result<Response<SendMessageResponse>, Box<dyn std::error::Error>> {
        let mut req = Request::new(request);
        Ok(self.stub.send_message(req).await?)
    }
}

#[derive(Default)]
pub struct ClientManager {}

impl ClientManager {
    pub async fn start(&self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
        let handle = tokio::spawn(async move {
            for i in 0..3 {
                interval.tick().await;
                println!("Tick");
            }
        });
        let _result = handle.await;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pb::{Code, Resource};

    #[tokio::test]
    async fn test_connect() {
        let target = "http://127.0.0.1:5001";
        let _rpc_client = RpcClient::new(target.to_owned())
            .await
            .expect("Should be able to connect");
    }

    #[tokio::test]
    async fn test_connect_staging() {
        let target = "https://mq-inst-1080056302921134-bxuibml7.mq.cn-hangzhou.aliyuncs.com:80";
        let _rpc_client = RpcClient::new(target.to_owned())
            .await
            .expect("Failed to connect to staging proxy server");
    }

    #[tokio::test]
    async fn test_query_route() {
        let target = "http://127.0.0.1:5001";
        let mut rpc_client = RpcClient::new(target.to_owned())
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

    #[test]
    fn test_static_credentials_provider() {
        let provider = StaticCredentialProvider::new("ak", "as");
        let credentials = provider.get_credentials();
        assert_eq!(
            credentials,
            Credentials {
                access_key: String::from("ak"),
                access_secret: String::from("as"),
                session_token: None,
            }
        );
    }

    #[tokio::test]
    async fn test_periodic_task() {
        let client_manager = ClientManager::default();
        client_manager.start().await;
    }
}
