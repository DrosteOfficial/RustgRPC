// mod generated;

use crate::generated::user::{CreateUserRequest, DeleteUserRequest, GetUserRequest, UpdateUserRequest};
use crate::generated::user::user_service_client::UserServiceClient;
use tonic::transport::Channel;
use crate::generated::calculator::CalculationRequest;
use crate::generated::calculator::calculator_client::CalculatorClient;
use crate::generated::messages::{CreateMessageRequest, DeleteMessageRequest, GetMessageRequest};
use crate::generated::messages::messages_client::MessagesClient;
use crate::generated::pow::pow_client::PowClient;
use crate::generated::pow::PowRequest;

// #[cfg(feature = "client")]


//po zerawniu połączenia z serverem ponowne połącznie tak aby odebrać response z reszty requestów

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:5051").connect().await?;
    let mut calc_client = CalculatorClient::new(channel.clone());
    let mut pow_client = PowClient::new(channel.clone());
    let user_client = UserServiceClient::new(channel.clone());
    let mut mess_client = MessagesClient::new(channel.clone());

    // Calculation requests
    let sum_request = tonic::Request::new(CalculationRequest { a: 5, b: 3 });
    let pow_request = tonic::Request::new(PowRequest { a: 2, b: 10 });

    // User requests
    let create_user_request = tonic::Request::new(CreateUserRequest {
        id: 2,
        username: "adrian".to_string(),
        password: "frost".to_string(),
        email: "frost@gmail.com".to_string(),
        gender: 0,
    });

    let update_user_request = tonic::Request::new(UpdateUserRequest {
        id: 6,
        username: "adrian_updated".to_string(),
        password: "frost_updated".to_string(),
        email: "frost_updated@gmail.com".to_string(),
        gender: 1,
    });

    let delete_user_request = tonic::Request::new(DeleteUserRequest {
        id: 8,
    });

    let get_user_request = tonic::Request::new(GetUserRequest {
        id: 8,
    });
    let get_message_request = tonic::Request::new(GetMessageRequest {
        user_id: 2,
    });

    let create_message_request = tonic::Request::new(CreateMessageRequest {
        message: "Hello".to_string(),
        sender: 6,
        receiver: 7,
        timestamp: 2137,
    });
    let delete_message_request = tonic::Request::new((
        DeleteMessageRequest {
            message_id: 1,
        }
    ));

    // Making requests
    let sum_response = calc_client.add(sum_request).await?.into_inner();

    let pow_response = pow_client.powerfn(pow_request).await?.into_inner();

    // let create_user_response = user_client.create_user(create_user_request).await?.into_inner();
    // let update_user_response = user_client.update_user(update_user_request).await?.into_inner();
    // let get_user_response = user_client.get_user(get_user_request).await?.into_inner();
    // let delete_user_response = user_client.delete_user(delete_user_request).await?.into_inner();

    let _create_message_response = mess_client.create_message(create_message_request).await?.into_inner();
    let _get_message_response = mess_client.get_messages(get_message_request).await?.into_inner();
    let _delete_message_response = mess_client.delete_message(delete_message_request).await?.into_inner();

    // Print responses
    println!("Sum Response: {:?}", sum_response.result);
    println!("Pow Response: {:?}", pow_response.result);

    // println!("Create User Response: {:?}", create_user_response.id);
    // println!("Update User Response: {:?}", update_user_response.id);
    // println!("Delete User Response: {:?}", delete_user_response.id);
    // println!("Get User Response: {:?}", get_user_response.id);
    println!("Create Message Response: {:?}", _create_message_response);
    print!("Get Message Response: {:?}", _get_message_response);
    println!("Delete Message Response: {:?}", _delete_message_response);

    Ok(())
}