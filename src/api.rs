use crate::types::{Account, Network, Order};
use thiserror::Error;
use ethers::{
    prelude::Address, 
    types::U256,
};
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};

pub struct LooksRareApi {
    client: Client,
    network: Network,
}

impl LooksRareApi {
    pub fn new() -> Self {
        let builder = ClientBuilder::new();

        let client = builder.build().unwrap();

        Self {
            client,
            network: Network::Mainnet,
        }
    }

    pub async fn get_account(&self, req: AccountRequest) -> Result<Account, LooksRareApiError> {
        let api = self.network.api();
        let url = format!("{}/accounts", api);
        let mut map = std::collections::HashMap::new();
        map.insert("address", serde_json::to_value(req.address)?);

        let res = self.client.get(url).query(&map).send().await?;
        let text = res.text().await?;
        let resp: AccountResponse = serde_json::from_str(&text)?;
        let data: Account = resp.data.ok_or(LooksRareApiError::AccountNotFound {
            address: req.address
        })?;

        Ok(data)
    }

    pub async fn get_orders(&self, req: OrdersRequest) -> Result<Vec<Order>, LooksRareApiError> {
        let api = self.network.api();
        let url = format!("{}/orders", api);


        let mut query = vec![];

        if let Some(_a) = &req.is_order_ask { query.push(("isOrderAsk", serde_json::to_value(req.is_order_ask)?)); };
        if let Some(_b) = &req.collection { query.push(("collection", serde_json::to_value(req.collection)?)); };
        if let Some(_c) = &req.token_id { query.push(("tokenId", serde_json::to_value(req.token_id)?)); };
        if let Some(_d) = &req.signer { query.push(("signer", serde_json::to_value(req.signer)?)); };
        if let Some(_e) = &req.nonce { query.push(("nonce", serde_json::to_value(req.nonce)?)); };
        if let Some(_f) = &req.strategy { query.push(("strategy", serde_json::to_value(req.strategy)?)); };
        if let Some(_g) = &req.currency { query.push(("currency", serde_json::to_value(req.currency)?)); };
        if let Some(_h) = &req.price { query.push(("price", serde_json::to_value(req.price)?)); };
        if let Some(_i) = &req.start_time { query.push(("startTime", serde_json::to_value(req.start_time)?)); };
        
        if let Some(_j) = &req.status { 
            req.status.unwrap().iter_mut().for_each(|x| { query.push(("status[]", serde_json::to_value(x.to_str()).unwrap())) } ); 
        };
        
        if let Some(_k) = &req.pagination {
            if let Some(_first) = &req.pagination.clone().unwrap().first { 
                query.push(
                    ("pagination[first]",
                    serde_json::to_value(
                        req.pagination.clone().unwrap().first.unwrap().to_string()
                    )?)
                ); 
            };
            
            if let Some(_cursor) = &req.pagination.clone().unwrap().cursor { query.push(("pagination[cursor]", serde_json::to_value(req.pagination.clone().unwrap().cursor)?)); }; 
        };

        if let Some(_l) = &req.sort { query.push(("sort", serde_json::to_value(req.sort.unwrap().to_str())?)); };

        let res = self.client.get(url).query(&query).send().await?;
        let text = res.text().await?;

        let resp: OrdersResponse = serde_json::from_str(&text)?;
        let data: Vec<Order> = resp.data.ok_or(LooksRareApiError::OrdersNotFound)?;

        Ok(data)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountRequest {
    pub address: Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AccountResponse {
    success: bool,
    message: Option<String>,
    data: Option<Account>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrdersRequest {
    pub is_order_ask: Option<bool>,
    pub collection: Option<Address>,
    pub token_id: Option<String>,
    pub signer: Option<Address>,
    pub nonce: Option<String>,
    pub strategy: Option<Address>,
    pub currency: Option<Address>,
    pub price: Option<U256>,
    pub start_time: Option<u64>,
    pub status: Option<Vec<Status>>,
    pub pagination: Option<Pagination>,
    pub sort: Option<Sort>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OrdersResponse {
    success: bool,
    message: Option<String>,
    data: Option<Vec<Order>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    first: Option<u64>,
    cursor: Option<String>,
}



#[derive(Debug, Error)]
pub enum LooksRareApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("Account not found (address: {address}")]
    AccountNotFound { address: Address },
    #[error("Orders not found")]
    OrdersNotFound,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    Cancelled,
    Executed,
    Expired,
    Valid,
}

impl Status {
    fn to_str(&self) -> &str {
        match &self {
            Status::Cancelled => "CANCELLED",
            Status::Executed => "EXECUTED",
            Status::Expired => "EXPIRED",
            Status::Valid => "VALID",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Sort {
    ExpiringSoon,
    Newest,
    PriceAsc,
    PriceDesc,
}

impl Sort {
    fn to_str(&self) -> &str {
        match &self {
            Sort::ExpiringSoon => "EXPIRING_SOON",
            Sort::Newest => "NEWEST",
            Sort::PriceAsc => "PRICE_ASC",
            Sort::PriceDesc => "PRICE_DESC",
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::types::Account;

    use super::*;

    #[tokio::test]
    async fn can_get_account() {
        let api = LooksRareApi::new();

        let req = AccountRequest {
            address: "0x3d67b76CF3dcc881255eb2262E788BE03b2f5B9F"
                .parse()
                .unwrap(),
        };
        let input_address = req.address;
        let account: Account = api.get_account(req).await.unwrap();
        let output_address: Address = account.address;
        assert_eq!(input_address, output_address);
    }

    #[tokio::test]
    async fn can_get_orders() {
        let api = LooksRareApi::new();

        let req = OrdersRequest {
            is_order_ask: Some(true),
            collection: Some("0x34d85c9cdeb23fa97cb08333b511ac86e1c4e258".parse().unwrap()),
            token_id: Some(String::from("62962")),
            signer: Some("0x9E69b59b8d2A094CB1117f92Ff7DCf51Ed467B41".parse().unwrap()),
            nonce: Some(String::from("17832")),
            strategy: Some("0x579af6fd30bf83a5ac0d636bc619f98dbdeb930c".parse().unwrap()), 
            currency: None, 
            price: None, 
            start_time: None, 
            status: Some(vec![Status::Cancelled]),
            pagination: Some(Pagination {
                first: Some(4),
                cursor: None,
            }),
            sort: Some(Sort::Newest), 
        };
        
        let input_is_order_ask: bool = req.is_order_ask.unwrap();
        let input_collection: Address = req.collection.unwrap();
        let input_token_id: String = req.clone().token_id.unwrap();
        let input_signer: Address = req.signer.unwrap();
        let input_nonce: String = req.clone().nonce.unwrap();
        let input_strategy: Address = req.strategy.unwrap();
        // let input_currency
        // let input_price
        // let input_start_time
        let input_status: Vec<Status> = req.clone().status.unwrap();
        // let input_sort

        let orders: Vec<Order> = api.get_orders(req).await.unwrap();
        let first_order: Order = orders.into_iter().nth(0).unwrap();
        println!("{}",serde_json::to_string(&first_order).unwrap());

        let output_is_order_ask: bool = first_order.is_order_ask;
        let output_collection: Address = first_order.collection_address;
        let output_token_id: String = first_order.token_id;
        let output_signer: Address = first_order.signer;
        let output_nonce: String = first_order.nonce;
        let output_strategy: Address = first_order.strategy;
        // let input_currency
        // let input_price
        // let input_start_time
        let output_status: String = first_order.status;

        assert_eq!(input_is_order_ask, output_is_order_ask);
        assert_eq!(input_collection, output_collection);
        assert_eq!(input_token_id, output_token_id);
        assert_eq!(input_signer, output_signer);
        assert_eq!(input_nonce, output_nonce);
        assert_eq!(input_strategy, output_strategy);
        // test if output status is contained in list of input status
        assert!(input_status.iter().any(|i| i.to_str()==output_status));
    }

    #[tokio::test]
    async fn orders_pagination() {
        let api = LooksRareApi::new();

        let req = OrdersRequest {
            is_order_ask: None,
            collection: None,
            token_id: None,
            signer: None,
            nonce: None,
            strategy: None,
            currency: None, 
            price: None, 
            start_time: None, 
            status: None,
            pagination: Some(Pagination {
                first: Some(4),
                cursor: None,
            }),
            sort: None, 
        };
        
        let input_pagination_first: usize = req.clone().pagination.unwrap().first.unwrap().try_into().unwrap();
        // let input_pagination_cursor

        let orders: Vec<Order> = api.get_orders(req).await.unwrap();
        let orders_len: usize = orders.len();
        let first_order: Order = orders.into_iter().nth(0).unwrap();
        println!("{}",serde_json::to_string(&first_order).unwrap());

        let output_pagination_first: usize = orders_len;
        //let output_pagination_cursor = 

        assert_eq!(input_pagination_first, output_pagination_first);
    }
}

