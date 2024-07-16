use tonic::{Request, Response, Status};
use crate::pow::pow_server::{Pow, PowServer};
use crate::pow::{PowRequest, PowResponse};
use log::{info, error};

#[derive(Debug, Default)]
pub struct PowService {}

#[tonic::async_trait]
impl Pow for PowService {
    async fn powerfn(
        &self,
        request: Request<PowRequest>,
    ) -> Result<Response<PowResponse>, Status> {
        info!("Received power function request: {:?}", request);

        let input = request.get_ref();

        // Example of input validation
        if input.b < 0 {
            error!("Negative exponent received: {}", input.b);
            return Err(Status::invalid_argument("Exponent must be non-negative"));
        }

        let result = input.a.pow(input.b as u32);

        Ok(Response::new(PowResponse { result }))
    }
}