use bigdecimal::BigDecimal;
use diesel::{Associations, Identifiable, Queryable, Selectable};
use time::PrimitiveDateTime;

use crate::database::schema::PictureOrientation;
use crate::database::schema::*;
use crate::database::user::User;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = pictures)]
pub struct Picture {
    pub id: i64,
    pub user_id: i32,
    pub creation_date: PrimitiveDateTime,
    pub edition_date: PrimitiveDateTime,
    /// 6 decimals, maximum 100.000000°
    pub latitude: Option<BigDecimal>,
    /// 6 decimals, maximum 1000.000000°
    pub longitude: Option<BigDecimal>,
    pub altitude: Option<i16>,
    pub orientation: PictureOrientation,
    pub width: i16,
    pub height: i16,
    pub camera_brand: Option<String>,
    pub camera_model: Option<String>,
    /// 2 decimals, maximum 10000.00mm (10 m)
    pub focal_length: Option<BigDecimal>,
    pub exposure_time_num: Option<i32>,
    pub exposure_time_den: Option<i32>,
    pub iso_speed: Option<i32>,
    /// 1 decimal, maximum 1000.0
    pub f_number: Option<BigDecimal>,
}

impl Picture {}
