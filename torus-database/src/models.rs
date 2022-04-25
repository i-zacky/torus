use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Queryable, Debug)]
pub struct Sandbox {
    pub id: String,
    pub name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub height: Option<i32>,
    pub weight: Option<BigDecimal>,
    pub enabled: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
