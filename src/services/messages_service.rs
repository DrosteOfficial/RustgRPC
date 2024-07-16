use tonic::{transport::Server, Request, Response, Status};
use crate::generated::messages::{CreateMessageRequest, DeleteMessageRequest, GetMessageRequest, MessageResponse};
use crate::generated::messages::messages_server::Messages;



#[derive(Default)]
pub struct MyMessagesService {}

#[tonic::async_trait]
impl Messages for MyMessagesService {
    async fn create_message(
        &self,
        request: Request<CreateMessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        // Implement logic to handle message creation
        Ok(Response::new(MessageResponse {
            // Assuming a successful operation, populate the response accordingly
            user_id: 0, // Example value
            messages: vec![], // Example value
        }))
    }

    async fn get_messages(
        &self,
        request: Request<GetMessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        // Implement logic to retrieve messages
        Ok(Response::new(MessageResponse {
            // Assuming a successful operation, populate the response accordingly
            user_id: request.into_inner().user_id,
            messages: vec![], // Example value
        }))
    }

    async fn delete_message(
        &self,
        request: Request<DeleteMessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        // Implement logic to delete a message
        Ok(Response::new(MessageResponse {
            // Assuming a successful operation, populate the response accordingly
            user_id: 0, // Example value
            messages: vec![], // Example value
        }))
    }
}