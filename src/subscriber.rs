use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    client::{Client, Response},
    error::NovuError,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriberPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub subscriber_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribersResponse {
    pub page: i32,
    pub total_count: i32,
    pub page_size: i32,
    pub data: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubscriberPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub subscriber_id: String,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSubscriberResponse {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub organization_id: Option<String>,
    #[serde(rename = "_environmentId")]
    pub environment_id: Option<String>,
    pub channels: Option<Vec<HashMap<String, serde_json::Value>>>,
    pub deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "subscriberId")]
    pub subscriber_id: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "lastOnlineAt")]
    pub last_online_at: Option<String>,
    pub locale: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "__v")]
    pub version: Option<i64>,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubscriberResponse {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub organization_id: Option<String>,
    #[serde(rename = "_environmentId")]
    pub environment_id: Option<String>,
    pub channels: Option<Vec<HashMap<String, serde_json::Value>>>,
    pub deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "subscriberId")]
    pub subscriber_id: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "lastOnlineAt")]
    pub last_online_at: Option<String>,
    pub locale: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "__v")]
    pub version: Option<i64>,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscriberResponse {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub organization_id: Option<String>,
    #[serde(rename = "_environmentId")]
    pub environment_id: Option<String>,
    pub channels: Option<Vec<HashMap<String, serde_json::Value>>>,
    pub deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "subscriberId")]
    pub subscriber_id: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "lastOnlineAt")]
    pub last_online_at: Option<String>,
    pub locale: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "__v")]
    pub version: Option<i64>,
    pub data: Option<HashMap<String, serde_json::Value>>,
}

pub struct Subscribers {
    client: Client,
}

impl Subscribers {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, page: i32) -> Result<SubscribersResponse, NovuError> {
        let endpoint = format!("/subscribers/?page={}", page);
        let result: Response<SubscribersResponse> = self.client.get(endpoint).await?;

        match result {
            crate::client::Response::Success(data) => Ok(data.data),
            crate::client::Response::Error(err) => todo!("{:?}", err),
            crate::client::Response::Messages(err) => todo!("{:?}", err),
        }
    }

    pub async fn get_subscriber(
        &self,
        subscriber_id: String,
    ) -> Result<GetSubscriberResponse, NovuError> {
        let endpoint = format!("/subscribers/{}", subscriber_id);
        let result = self.client.get(endpoint).await?;

        match result {
            crate::client::Response::Success(data) => Ok(data.data),
            crate::client::Response::Error(err) => todo!("{:?}", err),
            crate::client::Response::Messages(err) => todo!("{:?}", err),
        }
    }

    pub async fn update(
        &self,
        subscriber_id: String,
        data: SubscriberPayload,
    ) -> Result<UpdateSubscriberResponse, NovuError> {
        let endpoint = format!("/subscribers/{}", subscriber_id);
        let result = self.client.put(endpoint, &data).await?;

        match result {
            crate::client::Response::Success(data) => Ok(data.data),
            crate::client::Response::Error(err) => todo!("{:?}", err),
            crate::client::Response::Messages(err) => todo!("{:?}", err),
        }
    }

    pub async fn create(
        &self,
        data: CreateSubscriberPayload,
    ) -> Result<CreateSubscriberResponse, NovuError> {
        let endpoint = "/subscribers/".to_string();
        let result = self.client.post(endpoint, Some(&data)).await?;

        match result {
            crate::client::Response::Success(data) => Ok(data.data),
            crate::client::Response::Error(err) => todo!("{:?}", err),
            crate::client::Response::Messages(err) => todo!("{:?}", err),
        }
    }
}
