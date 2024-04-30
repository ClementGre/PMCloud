use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Queryable, Selectable};

use crate::database::schema::PictureOrientation;
use crate::database::schema::*;
use crate::database::user::User;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(table_name = pictures)]
pub struct Picture {
    pub id: u64,
    pub owner_id: u32,
    pub author_id: u32,
    pub deleted_date: Option<NaiveDateTime>,
    pub copied: bool,
    pub creation_date: NaiveDateTime,
    pub edition_date: NaiveDateTime,
    /// 6 decimals, maximum 100.000000°
    pub latitude: Option<BigDecimal>,
    /// 6 decimals, maximum 1000.000000°
    pub longitude: Option<BigDecimal>,
    pub altitude: Option<u16>,
    pub orientation: PictureOrientation,
    pub width: u16,
    pub height: u16,
    pub camera_brand: Option<String>,
    pub camera_model: Option<String>,
    /// 2 decimals, maximum 10000.00mm (10 m)
    pub focal_length: Option<BigDecimal>,
    pub exposure_time_num: Option<u32>,
    pub exposure_time_den: Option<u32>,
    pub iso_speed: Option<u32>,
    /// 1 decimal, maximum 1000.0
    pub f_number: Option<BigDecimal>,
}

impl Picture {}
