use crate::msg::{
    CommitRequest, CommitResponse, GetRequest, GetResponse, PrewriteRequest, PrewriteResponse,
    TimestampRequest, TimestampResponse,RollbackRequest,RollbackResponse,
};

service! {
    service timestamp {
        rpc get_timestamp(TimestampRequest) returns (TimestampResponse);
    }
}

pub use timestamp::{add_service as add_tso_service, Client as TSOClient, Service};

service! {
    service transaction {
        rpc get(GetRequest) returns (GetResponse);
        rpc prewrite(PrewriteRequest) returns (PrewriteResponse);
        rpc commit(CommitRequest) returns (CommitResponse);
        rpc rollback(RollbackRequest) returns(RollbackResponse);
    }
}

pub use transaction::{add_service as add_transaction_service, Client as TransactionClient};
