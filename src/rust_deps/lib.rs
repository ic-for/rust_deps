
use ic_cdk::export::candid;
use ic_cdk::export::{candid::{CandidType, Deserialize, Nat}, Principal};
use ic_cdk_macros::*;

#[import(canister = "multiply_deps")]
struct CounterCanister;

#[derive(Debug, CandidType, Deserialize)]
pub struct DeleteCommand {
    id: candid::Nat
}


#[update]
async fn greet2(name: String) -> String {
    CounterCanister::greet(name).await.0
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
async fn now() -> candid::Int {
    CounterCanister::now().await.0
}

#[update]
async fn read() -> candid::Nat {
    CounterCanister::read().await.0
}

