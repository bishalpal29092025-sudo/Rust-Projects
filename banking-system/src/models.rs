#[derive(Debug, Clone)]
pub struct Account {
    pub id: u32,
    pub name: String,
    pub balance: f64,
}


#[derive(Debug, Clone)]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Transfer,
}

#[derive(Debug, Clone)]
pub struct Transaction{
    pub id: u32,
    pub account_id: u32,
    pub transaction_type: TransactionType,
    pub amount: f64,
}