use diesel::table;

table! {
    users (id) {
        id -> Varchar,

        password -> Varchar,
        image_path -> Varchar,
        phone_number -> Varchar,

        email -> Nullable<Varchar>,

        created_at -> Varchar,
        updated_at -> Nullable<Varchar>,
    }
}

table! {
    relations (id) {
        id -> Varchar,

        organization_id -> Varchar,
        branch_id -> Varchar,
        user_id -> Varchar,
        role -> Varchar,
        relation_type -> Varchar,

        created_at -> Varchar,
        updated_at -> Nullable<Varchar>,
    }
}
