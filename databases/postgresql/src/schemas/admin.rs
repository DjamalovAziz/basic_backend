use diesel::table;

table! {
    admins (id) {
        id -> Varchar,

        password -> Varchar,
        role -> Varchar,
        phone_number -> Varchar,

        created_at -> Varchar,
        updated_at -> Nullable<Varchar>,
    }
}
