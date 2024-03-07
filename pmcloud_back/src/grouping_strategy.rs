use std::collections::HashMap;

pub struct GroupingStrategy {
    filter: GroupingFilterStrategy,
    groupings: Vec<GroupingType>,
    preserve_unicity: bool, // If true, a picture will not be able to appear in two different groups.
}

pub struct GroupingFilterStrategy {
    filters: Vec<Vec<FilterType>>
    // First vec is a list of group of filters, at least one filter must be passed.
    // Second vec is a list of filters, all filters must be passed.
}

// EXIF RELATED DATA

pub enum ExifDataTypeValue {
    // CreationDate(Vec<DateTime>),
    // EditionDate(Vec<DateTime>),
    Latitude(Vec<f64>),
    Longitude(Vec<f64>),
    Altitude(Vec<f64>),
    Orientation(Vec<Orientation>),
    Width(Vec<i32>),
    Height(Vec<i32>),
    CameraBrand(Vec<String>),
    CameraModel(Vec<String>),
    FocalLength(Vec<f64>),
    ExposureTime(Vec<(u32, u32)>),
    IsoSpeed(Vec<i32>),
    FNumber(Vec<f64>)
}
pub enum Orientation {
    #[default]
    Unspecified,
    Normal,
    HorizontalFlip,
    Rotate180,
    VerticalFlip,
    Rotate90HorizontalFlip,
    Rotate90,
    Rotate90VerticalFlip,
    Rotate270,
}

// FILTERING

pub enum FilterType {
    All,
    IncludeTags(Vec<[u8; 16]>),
    ExcludeTags(Vec<[u8; 16]>),
    IncludeSubgroups(Vec<[u8; 16]>),
    ExcludeSubgroups(Vec<[u8; 16]>),
    ExifEqualTo(ExifDataTypeValue), // Equal to any of the values
    ExifNotEqualTo(ExifDataTypeValue), // Not equal to all the values
    ExifInInterval(ExifDataTypeValue), // Interval composed of two first values
    ExifNotInInterval(ExifDataTypeValue), // Interval composed of two first values
}

// GROUPING

pub enum GroupingType {
    GroupByFilter(FilterGrouping),
    GroupByTags(TagGrouping),
    GroupByExifValues(ExifValuesGrouping),
    GroupByExifInterval(ExifIntervalGrouping),
    GroupByLocation(LocationGrouping)
}
pub struct FilterGrouping {
    filters: Vec<(GroupingFilterStrategy, u64)> // Value is the key of the corresponding subgroup
}
pub struct TagGrouping {
    tag_group_id: u64,
    tag_id_to_subgroup_id: HashMap<u64, u64>,
    subgroup_names_format: String
}
pub struct ExifValuesGrouping {
    data_type: ExifDataTypeValue, // data vec is empty
    values_to_subgroup_id: HashMap<ExifDataTypeValue, u64>,
    subgroup_names_format: String // Include value format
}
pub struct ExifIntervalGrouping {
    interval: ExifDataTypeValue, // First value is origin, second is interval
    subgroup_names_format: String // Datetime format or number format.
}
pub struct LocationGrouping {
    clusters_ids: Vec<u64>,
    is_date_ordered: bool,
    sharpness: u32,
}
