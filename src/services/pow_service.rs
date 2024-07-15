use tonic::{Request, Response, Status};
use crate::pow::pow_server::{Pow, PowServer};
use crate::pow::{PowRequest, PowResponse};

#[derive(Debug, Default)]
pub struct PowService {}

#[tonic::async_trait]
impl Pow for PowService {
    async fn powerfn(
        &self,
        request: Request<PowRequest>,
    ) -> Result<Response<PowResponse>, Status> {
        println!("Got a request: {:?}", request);

        let input = request.get_ref();
        let result = input.a.pow(input.b as u32); // Assuming `a` and `b` are non-negative for simplicity

        let response = PowResponse {
            result,
        };

        Ok(Response::new(response))
    }
}