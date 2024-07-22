use log::{error, log};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tonic::{IntoRequest, Request, Response, Status};

use crate::entities::message as messageEntity;
use crate::entities::message::Column;
use crate::generated::messages::messages_server::Messages;
use crate::generated::messages::{
    CreateMessageRequest, DeleteMessageRequest, GetMessageRequest, Message, MessageResponse,
};

#[derive(Debug, Default)]
pub struct MessagesService {
    db: DatabaseConnection,
}

impl MessagesService {
    pub fn new_messages_service(db: DatabaseConnection) -> MessagesService {
        MessagesService { db }
    }

    async fn get_db_connection(&self) -> Result<DatabaseConnection, Status> {
        Ok(self.db.clone())
    }

    async fn create_message(
        &self,
        request: Request<CreateMessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
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
                error!("Error inserting message: {:?}", e);
                Status::internal("Failed to insert message ")
            })?;

        println!("Message inserted");
        return Ok(Response::new(MessageResponse::default()));
    }

    async fn get_messages(
        &self,
        request: Request<GetMessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let db_conn = self.get_db_connection().await?;
        let det = request.into_inner();
        match messageEntity::Entity::find()
            .filter(Column::Sender.eq(det.user_id))
            .all(&db_conn)
            .await
        {
            Ok(messages) => {
                let mut message_list = Vec::new();
                for message in messages {
                    message_list.push(Message {
                        message: "".to_string(),
                        sender: message.sender,
                        receiver: message.receiver,
                        timestamp: message.timestamp,
                    });
                }
                return Ok(Response::new(MessageResponse {
                    user_id: det.clone().user_id,
                    messages: message_list,
                }));
            }
            Err(e) => {
                log::error!("Error getting messages: {:?}", e);
                return Err(Status::internal("Failed to get messages"));
            }
        }
    }

    async fn delete_message(
        &self,
        request: Request<DeleteMessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        let _db_conn = self.get_db_connection().await?;
        let det = request.into_inner();

        let _message_exist = self
            .get_messages(Request::new(GetMessageRequest {
                user_id: det.message_id,
            }))
            .await;
        if let Err(err) = _message_exist {
            log::error!("Error checking if message exists: {:?}", err);
            return Err(Status::internal("Failed to check if message exists"));
        }

        let _deleted_message = messageEntity::Entity::delete_by_id(det.message_id)
            .exec(&_db_conn)
            .await; //message id not user id
        if let Err(err) = _deleted_message {
            log::error!("Error deleting message: {:?}", err);
            return Err(Status::internal("Failed to delete message"));
        }
        return Ok(Response::new(MessageResponse::default()));
    }
}
#[tonic::async_trait]
impl Messages for MessagesService {
    async fn create_message(
        &self,
        request: Request<CreateMessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        self.create_message(request).await
    }

    async fn delete_message(
        &self,
        request: Request<DeleteMessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        self.delete_message(request).await
    }

    async fn get_messages(
        &self,
        request: Request<GetMessageRequest>,
    ) -> Result<Response<MessageResponse>, Status> {
        self.get_messages(request).await
    }
}
