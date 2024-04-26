use serde::{Serialize, Deserialize};
#[derive(Serialize , Deserialize, Debug)]
struct TRC20{
    wallet_id: String,
    wallet_publickey: String
}

