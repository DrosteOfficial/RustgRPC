use log::{debug, error, info, log_enabled, Level};
use tonic::transport::Channel;

use crate::generated::calculator::calculator_client::CalculatorClient;
use crate::generated::calculator::CalculationRequest;
use crate::generated::messages::messages_client::MessagesClient;
use crate::generated::messages::{CreateMessageRequest, DeleteMessageRequest, GetMessageRequest};
use crate::generated::pow::pow_client::PowClient;
use crate::generated::pow::PowRequest;
use crate::generated::user::user_service_client::UserServiceClient;
use crate::generated::user::{
    CreateUserRequest, DeleteUserRequest, GetUserRequest, SignInRequest, SignOutRequest,
    UpdateUserRequest,
};

mod generated;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let channel = Channel::from_static("http://[::1]:5051").connect().await?;
    let calc_client = CalculatorClient::new(channel.clone());
    let pow_client = PowClient::new(channel.clone());
    let user_client = UserServiceClient::new(channel.clone());
    let mess_client = MessagesClient::new(channel.clone());

    // Calculation requests
    let sum_request = tonic::Request::new(CalculationRequest { a: 5, b: 3 });
    let pow_request = tonic::Request::new(PowRequest { a: 2, b: 10 });

    let create_user_request = tonic::Request::new(CreateUserRequest {
        id: 0,
        username: "adrian1".to_string(),
        password: "frost1".to_string(),
        email: "frost@gmail.com1".to_string(),
        gender: 1,
    });
    //
    // let update_user_request = tonic::Request::new(UpdateUserRequest {
    //     id: 2,
    //     username: "adrian_updated1".to_string(),
    //     password: "frost_updated1".to_string(),
    //     email: "frost_updated@gmail.com1".to_string(),
    //     gender: 0,
    // });
    //
    // let delete_user_request = tonic::Request::new(DeleteUserRequest { id: 1 });
    //
    // let user_request = tonic::Request::new(GetUserRequest { id: 15 });
    //
    let mut singin_request = tonic::Request::new(SignInRequest {
        login_or_email: "adrian1".to_string(),
        password: "frost1".to_string(),
    });
    //
    // let get_message_request = tonic::Request::new(GetMessageRequest { user_id: 2 });
    //
    // let create_message_request = tonic::Request::new(CreateMessageRequest {
    //     message: "Hello".to_string(),
    //     sender: 2,
    //     receiver: 3,
    //     timestamp: 2000000,
    // });
    // let delete_message_request = tonic::Request::new(DeleteMessageRequest { message_id: 1 });
    //
    // // Making requests
    //
    // if let Err(err) = calc_client.clone().add(sum_request).await {
    //     error!("Error adding: {:?}", err);
    // } else {
    //     info!("Addition successful");
    // }
    //
    // if let Err(err) = pow_client.clone().powerfn(pow_request).await {
    //     error!("Error with power function: {:?}", err);
    // } else {
    //     info!("Power function successful");
    // }
    //
    if let Err(err) = user_client.clone().create_user(create_user_request).await {
        error!("Error creating user: {:?}", err);
    } else {
        info!("User created successfully");
    }
    // if let Err(err) = user_client.clone().update_user(update_user_request).await {
    //     error!("Error updating user: {:?}", err);
    // } else {
    //     info!("User updated successfully");
    // }
    // if let Err(err) = user_client.clone().get_user(user_request).await {
    //     error!("Error getting user: {:?}", err);
    // } else {
    //     info!("User retrieved successfully");
    // }
    // if let Err(err) = user_client.clone().delete_user(delete_user_request).await {
    //     error!("Error deleting user: {:?}", err);
    // } else {
    //     info!("User deleted successfully");
    // }
    //
    // if let Err(err) = mess_client
    //     .clone()
    //     .create_message(create_message_request)
    //     .await
    // {
    //     error!("Error creating message: {:?}", err);
    // } else {
    //     info!("Message created successfully");
    // }
    // if let Err(err) = mess_client.clone().get_messages(get_message_request).await {
    //     error!("Error getting messages: {:?}", err);
    // } else {
    //     info!("Messages retrieved successfully");
    // }
    // if let Err(err) = mess_client
    //     .clone()
    //     .delete_message(delete_message_request)
    //     .await
    // {
    //     error!("Error deleting message: {:?}", err);
    // } else {
    //     info!("Message deleted successfully");
    // }
    if let Err(err) = user_client.clone().sign_in(singin_request).await {
        error!("Error signing in: {:?}", err);
    } else {
        info!("Sign in successful");
    }
    let mut signout_request = tonic::Request::new(SignOutRequest {});
    signout_request.metadata_mut().append("authorization", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ7XCJpZFwiOjEsXCJ1c2VybmFtZVwiOlwiYWRyaWFuMVwiLFwicGFzc3dvcmRcIjpcImZyb3N0MVwiLFwiZW1haWxcIjpcImZyb3N0QGdtYWlsLmNvbTFcIixcImdlbmRlclwiOlwiRmVtYWxlXCJ9IiwiY29tcGFueSI6IkNvbnRyb2wiLCJleHAiOjE3MjE3MzQ3ODR9.FqwYRB7vxio52NFvMMvmhnVfXhZTAIv9lC7oYjS4UGg".parse().unwrap());
    if let Err(err) = user_client.clone().sign_out(signout_request).await {
        error!("Error signing out: {:?}", err);
    } else {
        info!("Sign out successful");
    }
    Ok(())
}
