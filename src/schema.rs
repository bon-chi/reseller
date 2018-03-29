table! {
    resellers (id) {
        id -> Integer,
        seller_id -> Varchar,
        name -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    reseller_comments (id) {
        id -> Bigint,
        reseller_id -> Integer,
        comment -> Text,
        user_name -> Nullable<Varchar>,
        pass -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

joinable!(reseller_comments -> resellers (reseller_id));

allow_tables_to_appear_in_same_query!(
    resellers,
    reseller_comments,
);
