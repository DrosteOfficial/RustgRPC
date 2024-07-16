use tonic::transport::Channel;
use tonic::{transport::Server, Request, Response, Status};
use tonic::transport::Error as TonicTransportError;
use crate::generated::calculator::CalculationRequest;
use crate::generated::calculator::calculator_client::CalculatorClient;
use crate::generated::pow::pow_client::PowClient;
use crate::generated::pow::PowRequest;
use crate::generated::user::CreateUserRequest;
use crate::generated::user::user_service_client::UserServiceClient;


// mod generated;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:5051").connect().await?;
    let mut calc_client = CalculatorClient::new(channel.clone());
    let mut pow_client = PowClient::new(channel.clone());
    let mut user_client = UserServiceClient::new(channel.clone());

    let sum_request = tonic::Request::new(CalculationRequest { a: 5, b: 3 });
    let pow_request = tonic::Request::new(PowRequest { a: 2, b: 10 });
    let user_request = tonic::Request::new(CreateUserRequest {
        username: "adrian".to_string(),
        password: "KtoWieJaki".to_string(),
        email: "KtoWieJaki@adrian.hehe".to_string()
    });
    // let message_request = CreateMessageRequest { // Removed as it's not used
    //     message: "Hello".to_string(),
    //     sender: 1,
    //     receiver: 2,
    //     timestamp: Utc::now().timestamp(),
    // };

    let sum_response = calc_client.add(sum_request).await?.into_inner();
    let pow_response = pow_client.powerfn(pow_request).await?.into_inner();
    let user_response = user_client.create_user(user_request).await?.into_inner();
    println!("Sum Response: {:?}", sum_response.result);
    println!("Pow Response: {:?}", pow_response.result);
    println!("User Response: {:?}", user_response.id);


    Ok(())
}

