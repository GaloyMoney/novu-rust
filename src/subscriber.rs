use serde::{Deserialize, Serialize};

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
pub struct UpdateSubscriberResponse {
    #[serde(rename = "__v")]
    pub version: Option<String>,
    #[serde(rename = "_environmentId")]
    pub environment_id: String,
    pub id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub organization_id: String,
    // pub avatar: Option<String>,
    pub channels: Vec<Channel>,
    pub created_at: String,
    pub deleted: bool,
    pub email: Option<String>,
    pub first_name: Option<String>,
    // pub is_online: Option<bool>,
    pub last_name: Option<String>,
    // #[serde(rename = "lastOnlineAt")]
    // pub last_online_at: Option<String>,
    // pub locale: Option<String>,
    // pub phone: Option<String>,
    #[serde(rename = "subscriberId")]
    pub subscriber_id: String,
    pub updated_at: String,
    pub _id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "_integrationId")]
    pub integration_id: String,
    pub credentials: ChannelCredentials,
    #[serde(rename = "integrationIdentifier")]
    pub integration_identifier: String,
    #[serde(rename = "providerId")]
    pub provider_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelCredentials {
    pub channel: String,
    #[serde(rename = "deviceTokens")]
    pub device_tokens: Vec<String>,
    #[serde(rename = "webhookUrl")]
    pub webhook_url: String,
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

    // pub async fn identify(&self, data: SubscriberPayload) -> Result<reqwest::Response, NovuError> {
    //     let url = format!("{}/subscribers");
    //     let response = self.client.post(&url).json(&data).send().await?;
    //     Ok(response)
    // }

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
}
