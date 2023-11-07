// @generated automatically by Diesel CLI.

diesel::table! {
    hf_dict (id) {
        id -> Int4,
        group -> Int2,
        #[max_length = 10]
        code -> Nullable<Varchar>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 500]
        remark -> Nullable<Varchar>,
        create_time -> Timestamp,
    }
}
