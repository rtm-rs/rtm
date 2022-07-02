use eventually::{aggregate, aggregate::Root as AggregateRoot, message};
use rust_decimal::Decimal;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
};

pub type BankAccountId = String;

#[derive(Debug, Clone)]
pub struct BankAccount {
    id: BankAccountId,
    current_balance: Decimal,
    pending_transactions: HashMap<TransactionId, Transaction>,
    is_closed: bool,
}
