use std::sync::Arc;
use sea_orm::{ ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use crate::generated::messages::{CreateMessageRequest, DeleteMessageRequest, GetMessageRequest, MessageResponse};
use crate::generated::messages::messages_server::Messages;
use crate::entities::message as messageEntity;
use crate::entities::message::Column;

#[derive(Debug, Default)]
pub struct MessagesService {
    db: Arc<Mutex<Option<DatabaseConnection>>>,
}

impl MessagesService {
    pub fn new_messages_service(db: Arc<Mutex<Option<DatabaseConnection>>>) -> MessagesService {
        MessagesService { db }
    }

    async fn get_db_connection(&self) -> Result<DatabaseConnection, Status> {
        let db_guard = self.db.lock().await;
        if let Some(ref db_conn) = *db_guard {
            Ok(db_conn.clone())
        } else {
            return Err(Status::internal("Database connection is not available"));
        }
    }

    async fn create_message(&self, request: Request<CreateMessageRequest>) -> Result<Response<MessageResponse>, Status> {
        let db_conn = self.get_db_connection().await?;
        let det = request.into_inner();
        let new_message = messageEntity::ActiveModel {
            id: ActiveValue::NotSet,
            message: Set(det.message),
            sender: Set(det.sender),
            receiver: Set(det.receiver),
            timestamp: Set(det.timestamp),
        };

        let _inserted_message = messageEntity::Entity::insert(new_message)
            .exec(&db_conn)
            .await
            .map_err(|e| {
                println!("Error inserting message: {:?}", e);
                Status::internal("Failed to insert message")
            })?;
        println!("Message inserted");
        return Ok(Response::new(MessageResponse::default()));
    }

    async fn get_messages(&self, request: Request<GetMessageRequest>) -> Result<Response<MessageResponse>, Status> {
        let db_conn = self.get_db_connection().await?;
        let det = request.into_inner();
        let _messages = messageEntity::Entity::find()
            .filter(Column::Sender.eq(det.user_id))
            .all(&db_conn)
            .await;

        if _messages.unwrap().iter().count() > 0
        {
            let response = MessageResponse::default(); // Modify this line to populate response
            println!("Messages found");
            return Ok(Response::new(response));
        } else {
            println!("No messages found");
            return Err(Status::internal("No messages found"));
        }
        // Assume you populate MessageResponse with fetched messages here
    }

    async fn delete_message(&self, request: Request<DeleteMessageRequest>) -> Result<Response<MessageResponse>, Status> {
        let _db_conn = self.get_db_connection().await?;
        let det = request.into_inner();
        let _deleted_message = messageEntity::Entity::delete_by_id(det.message_id); //message id not user id
        return Ok(Response::new(MessageResponse::default()));

    }
}
    #[tonic::async_trait]
    impl Messages for MessagesService {
        async fn create_message(&self, request: Request<CreateMessageRequest>) -> Result<Response<MessageResponse>, Status> {
            self.create_message(request).await
        }

        async fn delete_message(&self, request: Request<DeleteMessageRequest>) -> Result<Response<MessageResponse>, Status> {
            self.delete_message(request).await
        }

        async fn get_messages(&self, request: Request<GetMessageRequest>) -> Result<Response<MessageResponse>, Status> {
            self.get_messages(request).await
        }
    }
