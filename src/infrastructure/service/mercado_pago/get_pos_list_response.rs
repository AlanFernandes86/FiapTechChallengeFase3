use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPosListResponse {
    pub paging: Paging,
    pub results: Vec<PosResult>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paging {
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PosResult {
    pub user_id: i64,
    pub name: String,
    pub fixed_amount: bool,
    pub category: i64,
    pub store_id: String,
    pub external_id: String,
    pub id: i64,
    pub qr: Qr,
    pub date_created: String,
    pub date_last_updated: String,
    pub external_store_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Qr {
    pub image: String,
    pub template_document: String,
    pub template_image: String,
}