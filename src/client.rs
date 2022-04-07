use crate::pb::{
    messaging_service_client::MessagingServiceClient, QueryRouteRequest, QueryRouteResponse,
    SendMessageRequest, SendMessageResponse,
};
use rustls::client;
use tonic::{
    metadata::MetadataMap,
    transport::{Channel, ClientTlsConfig},
    Request, Response,
};

use crate::credentials::CredentialProvider;
use std::collections::HashMap;
use std::rc::Rc;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, RwLock,
    },
    thread,
};

static CLIENT_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

pub struct ClientConfig {
    region: String,
    service_name: String,
    resource_namespace: Option<String>,
    credential_provider: Option<Box<dyn CredentialProvider>>,
    tenant_id: Option<String>,
    connect_timeout: std::time::Duration,
    io_timeout: std::time::Duration,
    long_polling_timeout: std::time::Duration,
    group: Option<String>,
    client_id: String,
    tracing: bool,
}

fn build_client_id() -> String {
    let mut client_id = String::new();
    match gethostname::gethostname().into_string() {
        Ok(hostname) => {
            client_id.push_str(&hostname);
        }
        Err(_) => {
            client_id.push_str("localhost");
        }
    };
    client_id.push('@');
    let pid = std::process::id();
    client_id.push_str(&pid.to_string());
    client_id.push('#');
    let sequence = CLIENT_SEQUENCE.fetch_add(1usize, Ordering::Relaxed);
    client_id.push_str(&sequence.to_string());
    client_id
}

impl Default for ClientConfig {
    fn default() -> Self {
        let client_id = build_client_id();
        Self {
            region: String::from("cn-hangzhou"),
            service_name: String::from("RocketMQ"),
            resource_namespace: None,
            credential_provider: None,
            tenant_id: None,
            connect_timeout: std::time::Duration::from_secs(3),
            io_timeout: std::time::Duration::from_secs(3),
            long_polling_timeout: std::time::Duration::from_secs(3),
            group: None,
            client_id,
            tracing: false,
        }
    }
}

pub struct RpcClient {
    client_config: Arc<RwLock<ClientConfig>>,
    stub: MessagingServiceClient<Channel>,
    peer_address: String,
    // client_config: std::rc::Rc<ClientConfig>,
}

impl RpcClient {
    pub async fn new(
        target: String,
        client_config: Arc<RwLock<ClientConfig>>,
    ) -> Result<RpcClient, Box<dyn std::error::Error>> {
        let config = Arc::clone(&client_config);
        let mut channel = Channel::from_shared(target.clone())?
            .tcp_nodelay(true)
            .connect_timeout(config.read().unwrap().connect_timeout);
        if target.starts_with("https://") {
            channel = channel.tls_config(ClientTlsConfig::new())?;
        }
        let channel = channel.connect().await?;
        let stub = MessagingServiceClient::new(channel);
        Ok(RpcClient {
            client_config,
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
        let req = Request::new(request);
        Ok(self.stub.send_message(req).await?)
    }
}

#[derive(Default)]
pub struct ClientManager {
    client_config: Arc<RwLock<ClientConfig>>,
    clients: Mutex<HashMap<String, Rc<RpcClient>>>,
}

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

    pub async fn get_rpc_client(
        &'static mut self,
        endpoint: &str,
    ) -> Result<Rc<RpcClient>, Box<dyn std::error::Error>> {
        let mut rpc_clients = self.clients.lock()?;
        let key = endpoint.to_owned();
        match rpc_clients.get(&key) {
            Some(value) => {
                return Ok(Rc::clone(value));
            }
            None => {
                let rpc_client =
                    RpcClient::new(key.clone(), Arc::clone(&self.client_config)).await?;
                let client = Rc::new(rpc_client);
                rpc_clients.insert(key, Rc::clone(&client));
                Ok(client)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use crate::pb::{Code, Resource};

    #[tokio::test]
    async fn test_connect() {
        let target = "http://127.0.0.1:5001";
        let client_config = Arc::new(RwLock::new(ClientConfig::default()));
        let _rpc_client = RpcClient::new(target.to_owned(), client_config)
            .await
            .expect("Should be able to connect");
    }

    #[tokio::test]
    async fn test_connect_staging() {
        let client_config = Arc::new(RwLock::new(ClientConfig::default()));
        let target = "https://mq-inst-1080056302921134-bxuibml7.mq.cn-hangzhou.aliyuncs.com:80";
        let _rpc_client = RpcClient::new(target.to_owned(), client_config)
            .await
            .expect("Failed to connect to staging proxy server");
    }

    #[tokio::test]
    async fn test_query_route() {
        let target = "http://127.0.0.1:5001";
        let client_config = Arc::new(RwLock::new(ClientConfig::default()));
        let mut rpc_client = RpcClient::new(target.to_owned(), client_config)
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
    fn test_build_client_id() {
        let mut set = HashSet::new();
        let cnt = 1000;
        for _ in 0..cnt {
            let client_id = build_client_id();
            set.insert(client_id);
        }
        assert_eq!(cnt, set.len());
    }

    #[tokio::test]
    async fn test_periodic_task() {
        let client_manager = ClientManager::default();
        client_manager.start().await;
    }
}
