
    use tonic::{Request, Response, Status};
    use crate::calculator::calculator_server::{Calculator, CalculatorServer};
    use crate::calculator::{CalculationRequest, CalculationResponse};

    #[derive(Debug, Default)]
    pub struct CalculatorService {}

    #[tonic::async_trait]
    impl Calculator for CalculatorService {
        async fn add(
            &self,
            request: Request<CalculationRequest>,
        ) -> Result<Response<CalculationResponse>, Status> {
            println!("Got a request: {:?}", request);

            let input = request.get_ref();
            let response = CalculationResponse {
                result: input.a + input.b,
            };

            Ok(Response::new(response))
        }
    }


