use crate::pb::{
    messaging_service_server::MessagingService, AckMessageRequest, AckMessageResponse,
    ChangeInvisibleDurationRequest, ChangeInvisibleDurationResponse, EndTransactionRequest,
    EndTransactionResponse, ForwardMessageToDeadLetterQueueRequest,
    ForwardMessageToDeadLetterQueueResponse, HeartbeatRequest, HeartbeatResponse,
    NotifyClientTerminationRequest, NotifyClientTerminationResponse, PullMessageRequest,
    PullMessageResponse, QueryAssignmentRequest, QueryAssignmentResponse, QueryOffsetRequest,
    QueryOffsetResponse, QueryRouteRequest, QueryRouteResponse, ReceiveMessageRequest,
    ReceiveMessageResponse, SendMessageRequest, SendMessageResponse, TelemetryCommand,
};
use futures::Stream;
use tonic::{Request, Response, Status, Streaming};

#[derive(Default)]
pub struct ServerService {}

type ResponseStream =
    std::pin::Pin<Box<dyn Stream<Item = Result<TelemetryCommand, Status>> + Send>>;

#[tonic::async_trait]
impl MessagingService for ServerService {
    type TelemetryStream = ResponseStream;
    async fn query_route(
        &self,
        request: Request<QueryRouteRequest>,
    ) -> Result<Response<QueryRouteResponse>, Status> {
        println!("{:?}", request);
        let reply = QueryRouteResponse {
            status: Some(crate::pb::Status {
                code: crate::pb::Code::Ok as i32,
                message: String::from("OK"),
            }),
            message_queues: vec![],
        };
        Ok(Response::new(reply))
    }

    async fn heartbeat(
        &self,
        request: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        let reply = HeartbeatResponse { status: None };
        Ok(Response::new(reply))
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let reply = SendMessageResponse {
            status: None,
            receipts: vec![],
        };
        Ok(Response::new(reply))
    }

    async fn query_assignment(
        &self,
        request: Request<QueryAssignmentRequest>,
    ) -> Result<Response<QueryAssignmentResponse>, Status> {
        let reply = QueryAssignmentResponse {
            status: None,
            assignments: vec![],
        };
        Ok(Response::new(reply))
    }

    async fn receive_message(
        &self,
        request: Request<ReceiveMessageRequest>,
    ) -> Result<Response<ReceiveMessageResponse>, Status> {
        let reply = ReceiveMessageResponse {
            status: None,
            delivery_timestamp: None,
            invisible_duration: None,
            messages: vec![],
        };
        Ok(Response::new(reply))
    }

    async fn ack_message(
        &self,
        request: Request<AckMessageRequest>,
    ) -> Result<Response<AckMessageResponse>, Status> {
        let reply = AckMessageResponse { status: None };
        Ok(Response::new(reply))
    }

    async fn forward_message_to_dead_letter_queue(
        &self,
        request: Request<ForwardMessageToDeadLetterQueueRequest>,
    ) -> Result<Response<ForwardMessageToDeadLetterQueueResponse>, Status> {
        let reply = ForwardMessageToDeadLetterQueueResponse { status: None };
        Ok(Response::new(reply))
    }

    /// Commits or rollback one transactional message.
    async fn end_transaction(
        &self,
        request: Request<EndTransactionRequest>,
    ) -> Result<Response<EndTransactionResponse>, Status> {
        let reply = EndTransactionResponse { status: None };
        Ok(Response::new(reply))
    }

    async fn query_offset(
        &self,
        request: Request<QueryOffsetRequest>,
    ) -> Result<Response<QueryOffsetResponse>, Status> {
        let reply = QueryOffsetResponse {
            status: None,
            offset: 0,
        };
        Ok(Response::new(reply))
    }

    async fn pull_message(
        &self,
        request: Request<PullMessageRequest>,
    ) -> Result<Response<PullMessageResponse>, Status> {
        let reply = PullMessageResponse {
            status: None,
            min_offset: 0,
            next_offset: 0,
            max_offset: 10,
            messages: vec![],
        };
        Ok(Response::new(reply))
    }

    ///Server streaming response type for the Telemetry method.
    async fn telemetry(
        &self,
        request: Request<Streaming<TelemetryCommand>>,
    ) -> Result<Response<Self::TelemetryStream>, Status> {
        Err(Status::aborted("NotImplemented"))
    }

    /// Notify the server that the client is terminated.
    async fn notify_client_termination(
        &self,
        request: Request<NotifyClientTerminationRequest>,
    ) -> Result<Response<NotifyClientTerminationResponse>, Status> {
        let reply = NotifyClientTerminationResponse { status: None };
        Ok(Response::new(reply))
    }

    async fn change_invisible_duration(
        &self,
        request: Request<ChangeInvisibleDurationRequest>,
    ) -> Result<Response<ChangeInvisibleDurationResponse>, Status> {
        let reply = ChangeInvisibleDurationResponse {
            status: None,
            receipt_handle: String::from("receipt-handle"),
        };
        Ok(Response::new(reply))
    }
}
