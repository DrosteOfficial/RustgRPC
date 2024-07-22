use log::{error, info};
use tonic::{Request, Response, Status};

use crate::pow::pow_server::Pow;
use crate::pow::{PowRequest, PowResponse};

#[derive(Debug, Default)]
pub struct PowService {}

#[tonic::async_trait]
impl Pow for PowService {
    async fn powerfn(&self, request: Request<PowRequest>) -> Result<Response<PowResponse>, Status> {
        info!("Received power function request: {:?}", request);

        let input = request.get_ref();

        // Example of input validation
        if input.b < 0 {
            error!("Negative exponent received: {}", input.b);
            return Err(Status::invalid_argument("Exponent must be non-negative"));
        }

        let result = input.a.pow(input.b as u32);
        println!("Result: {}", result);
        match result {
            0 => {
                error!("Power function overflowed");
                return Err(Status::out_of_range("Result overflowed"));
            }
            _ => {
                let response = PowResponse { result };
                info!("Returning power function response: {:?}", response);
                Ok(Response::new(response))
            }
        }
    }
}
