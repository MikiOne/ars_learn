use diesel::table;
table! {
    users (id) {
        id -> Text,
        name -> Text,
    }
}
