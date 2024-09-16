use diesel::table;

table! {
    telegram_groups (id) {
        id -> Varchar,

        group_id -> Varchar,

        name -> Nullable<Varchar>,

        organization_id -> Varchar,
        branch_id -> Varchar,

        created_at -> Varchar,
        updated_at -> Nullable<Varchar>,
    }
}

table! {
    fcm_subscriptions (id) {
        id -> Varchar,

        fcm_token -> Varchar,

        organization_id -> Varchar,
        branch_id -> Varchar,
        user_id -> Varchar,

        created_at -> Varchar,
    }
}

table! {
    subscriptions (id) {
        id -> Varchar,

        endpoint -> Varchar,
        expirationTime -> Nullable<Varchar>,

        p256dh -> Varchar,
        auth -> Varchar,

        organization_id -> Varchar,
        branch_id -> Varchar,
        user_id -> Varchar,

        created_at -> Varchar,
    }
}
