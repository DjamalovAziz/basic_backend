use diesel::table;

table! {
    organizations (id) {
        id -> Varchar,

        name -> Varchar,

        created_at -> Varchar,
        updated_at -> Nullable<Varchar>,
    }
}

table! {
    branchs (id) {
        id -> Varchar,

        name -> Varchar,

        branch_location -> Nullable<Varchar>,
        for_call -> Nullable<Varchar>, // Vec<ForCall>

        organization_id -> Varchar,

        created_at -> Varchar,
        updated_at -> Nullable<Varchar>,
    }
}
