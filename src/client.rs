use crate::pb::{
    messaging_service_client::MessagingServiceClient, QueryRouteRequest, QueryRouteResponse,
};
use tonic::{
    transport::{Channel, ClientTlsConfig},
    Request, Response,
};

pub struct RpcClient {
    stub: MessagingServiceClient<Channel>,
    peer_address: String,
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
}
