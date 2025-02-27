use log::{error, info, log_enabled, Level};
use tonic::{Request, Response, Status};

use crate::calculator::calculator_server::Calculator;
use crate::calculator::{CalculationRequest, CalculationResponse};

#[derive(Debug, Default)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: Request<CalculationRequest>,
    ) -> Result<Response<CalculationResponse>, Status> {
        info!("Received request: {:?}", request);

        let input = request.get_ref();
        let response = CalculationResponse {
            result: input.a + input.b,
        };
        info!("Returning response: {:?}", response);
        Ok(Response::new(response))
    }
}
