use crate::data;
use crate::{conf, error::Error};
use std::sync::Arc;

pub struct CreateTransactionOrderReq {
    user_id: i64,
    ref_id: i64,
    txn_type: String,
    payment_type: String,
}

pub struct EthereumService {
    conf: Arc<conf::Config>,
    data: Arc<data::Data>,
}

impl EthereumService {
    pub fn new(conf: Arc<conf::Config>, data: Arc<data::Data>) -> Self {
        EthereumService { conf, data }
    }

    pub fn handle_webhook_event() {}

    pub fn mint() {}
}

impl EthereumService {
    pub fn create_transaction_order(req: &CreateTransactionOrderReq) -> Result<(), Error> {
        if req.txn_type == *"mint".to_string() {
            Self::create_mint_tx_order(req)
        } else {
            Err(Error::BadRequest("".to_string()))
        }
    }

    pub fn create_mint_tx_order(req: &CreateTransactionOrderReq) -> Result<(), Error> {
        Err(Error::BadRequest("".to_string()))
    }
}
