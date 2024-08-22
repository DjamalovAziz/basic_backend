use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Token {
    pub access_token: String,
}

pub const DEFAULT_OFFSET: u32 = 1;
pub const DEFAULT_LIMIT: u32 = 25;
pub const DEFAULT_START: u32 = 0;

#[derive(Deserialize, Serialize, Debug)]
pub struct ResultPagingDB<DbModel> {
    pub total: usize,
    pub limit: usize,
    pub count: usize,
    pub page: usize,
    pub items: Vec<DbModel>,
}

impl<Model, DbModel> From<ResultPagingDB<DbModel>> for ResultPaging<Model>
where
    DbModel: Into<Model>,
{
    fn from(val: ResultPagingDB<DbModel>) -> Self {
        let total = val.total;
        let limit = val.limit;
        let page = val.page;
        let items = val.items.into_iter().map(DbModel::into).collect::<Vec<Model>>();
        ResultPaging::new(total, limit, page, items)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultPaging<T> {
    pub limit: usize,      // Default 25
    pub total: usize,      // Total records on db
    pub count: usize,      // Total returned records from db
    pub page: usize,       // Current page index
    pub page_count: usize, // Total page count
    pub items: Vec<T>,     // Returned items
}

impl<Model> ResultPaging<Model> {
    pub fn paging_from<ModelDTO: From<Model>>(self) -> ResultPaging<ModelDTO> {
        ResultPaging {
            limit: self.limit,
            total: self.total,
            count: self.count,
            page: self.page,
            page_count: self.page_count,
            items: self
                .items
                .into_iter()
                .map(|dto| -> ModelDTO { ModelDTO::from(dto) })
                .collect(),
        }
    }
}

impl<T> ResultPaging<T> {
    pub fn new(total: usize, limit: usize, page: usize, items: Vec<T>) -> Self {
        Self {
            limit,
            total,
            count: items.len(),
            page,
            page_count: (total as f64 / limit as f64).ceil() as usize,
            items,
        }
    }
}

pub trait QueryParams: Send + Sync {
    fn offset(&self) -> u32;
    fn limit(&self) -> u32;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParamsImpl {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

impl QueryParams for QueryParamsImpl {
    fn offset(&self) -> u32 {
        self.offset.unwrap_or(DEFAULT_OFFSET)
    }
    fn limit(&self) -> u32 {
        self.limit.unwrap_or(DEFAULT_LIMIT)
    }
}
