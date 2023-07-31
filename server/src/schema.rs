// @generated automatically by Diesel CLI.

diesel::table! {
    attraction (id) {
        id -> Int4,
        description -> Varchar,
        city_id -> Int4,
        latitude -> Nullable<Varchar>,
        longitude -> Nullable<Varchar>,
        attraction_type_id -> Int4,
    }
}

diesel::table! {
    attraction_rating (id) {
        id -> Int4,
        at -> Timestamp,
        attraction_id -> Int4,
        rate -> Numeric,
    }
}

diesel::table! {
    attraction_rating_aggregate (id) {
        id -> Int4,
        attraction_id -> Int4,
        at -> Timestamp,
        average -> Numeric,
        ninety_five_percentile -> Numeric,
        ninety_nine_percentile -> Numeric,
    }
}

diesel::table! {
    attraction_type (id) {
        id -> Int4,
        code -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    city (id) {
        id -> Int4,
        description -> Varchar,
        country_id -> Int4,
    }
}

diesel::table! {
    country (id) {
        id -> Int4,
        iso_code -> Varchar,
        description -> Varchar,
    }
}

diesel::joinable!(attraction -> attraction_type (attraction_type_id));
diesel::joinable!(attraction -> city (city_id));
diesel::joinable!(attraction_rating -> attraction (attraction_id));
diesel::joinable!(attraction_rating_aggregate -> attraction (attraction_id));
diesel::joinable!(city -> country (country_id));

diesel::allow_tables_to_appear_in_same_query!(
  attraction,
  attraction_rating,
  attraction_rating_aggregate,
  attraction_type,
  city,
  country,
);
