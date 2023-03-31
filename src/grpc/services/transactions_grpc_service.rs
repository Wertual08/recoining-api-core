pub mod api_transactions {
    tonic::include_proto!("api_core.transactions");
}

pub use api_transactions::transactions_server::TransactionsServer;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use tonic::{Request, Response, Status};

use std::sync::Arc;

use crate::{
    domain::{
        ServiceFactory, 
        transactions::{TransactionModel, TransactionStateModel},
    }, 
    logging::Logger,
};

use self::api_transactions::{
    transactions_server::Transactions, 
    SendBasicRequest, 
    SendResponse, 
    send_response::{Payload, Retry, Pending, Success}, 
    TransactionResource,
};

use super::{extensions::{StatusResult, AuthorizedRequest}};


#[derive(Debug)]
pub struct TransactionsGrpcService {
    logger: Arc<Logger>,
    service_factory: Arc<ServiceFactory>,
}

impl TransactionsGrpcService {
    pub fn new(
        logger: Arc<Logger>,
        service_factory: Arc<ServiceFactory>,
    ) -> Self {
        Self {
            logger,
            service_factory,
        }
    }
}

#[tonic::async_trait]
impl Transactions for TransactionsGrpcService {
    async fn send_basic(&self, request: Request<SendBasicRequest>) -> Result<Response<SendResponse>, Status> {
        let request_data = request.get_ref();
        if request_data.amount <= 0.0 {
            return Err(Status::invalid_argument("Amount must be grater than zero"));
        }

        let amount_option = BigDecimal::from_f64(request_data.amount);
        if amount_option.is_none() {
            return Err(Status::invalid_argument("Amount can not be converted to decimal value"));
        }
        let amount = amount_option.unwrap();

        if request_data.currency.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err(Status::invalid_argument("Currency must contain only alphabetic ascii chars"));
        }

        if request_data.currency.chars().count() > 8 {
            return Err(Status::invalid_argument("Currency must consist of 8 chars at max."));
        }

        if request_data.label.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err(Status::invalid_argument("Label must contain only alphabetic ascii chars"));
        }

        if request_data.label.chars().count() > 16 {
            return Err(Status::invalid_argument("Label must consist of 16 chars at max."));
        }

        let token = request.authorize(&self.logger, &self.service_factory.token())?;

        let registry_service = self.service_factory.registry();
        let registry_option = registry_service.find(
            request_data.registry_id,
        ).await.consume_error(&self.logger)?;
        if registry_option.is_none() {
            return Err(Status::not_found("Registry not found"));
        }
        let registry = registry_option.unwrap();

        let registry_user_service = self.service_factory.registry_user();

        let count = registry_user_service.count(registry.id, &[
            token.sub,
            request_data.user_id,
        ]).await.consume_error(&self.logger)?;
        if count != 2 {
            return Err(Status::permission_denied("One of the users is not connected with the specified registry"));
        }
        let transaction_service = self.service_factory.transaction();

        let result = transaction_service.send_basic(
            &registry, 
            token.sub,
            request_data.user_id,
            amount, 
            request_data.currency.clone(), 
            request_data.label.clone(), 
            request_data.description.clone(),
        ).await.consume_error(&self.logger)?;

        Ok(Response::new(SendResponse { 
            payload: Some(match result {
                TransactionStateModel::Fail => Payload::Retry(Retry {}),
                TransactionStateModel::Pending(transaction) => Payload::Pending(Pending {
                    transaction: Some(transaction.into()),
                }),
                TransactionStateModel::Sent(transaction) => Payload::Success(Success {
                    transaction: Some(transaction.into()),
                }),
            }),
        }))
    }
}

impl From<TransactionModel> for TransactionResource {
    fn from(model: TransactionModel) -> Self {
        Self {
            registry_id: model.registry_id,
            pack: model.pack,
            created_at: model.created_at,
            source_user_id: model.source_user_id,
            target_user_id: model.target_user_id,
            sequence: model.sequence as i32,
            variant: i16::from(model.variant) as i32,
            amount: model.amount.to_f64().unwrap(),
            currency: model.currency,
            label: model.label,
            description: model.description,
            hash: model.hash,
        }
    }
}