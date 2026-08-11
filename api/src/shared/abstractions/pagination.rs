
use crate::shared::abstractions::dto::Dto;



pub struct OffsetPagination<T:Dto> {
    pub page: u32,
    pub page_size: u32,
    pub data: Vec<T>,
}

pub struct KeySetPagination<T:Dto> {
    pub cursor: u32 ,
    pub page_size: u32,
    pub data: Vec<T>,
}

