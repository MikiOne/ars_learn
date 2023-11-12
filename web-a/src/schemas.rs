// @generated automatically by Diesel CLI.

diesel::table! {
    wa_users (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 200]
        pwd_hash -> Nullable<Varchar>,
        #[max_length = 500]
        remark -> Nullable<Varchar>,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}
