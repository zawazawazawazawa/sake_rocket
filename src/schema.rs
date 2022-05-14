table! {
    distilleries (id) {
        id -> Integer,
        whisky_type -> Varchar,
        region -> Nullable<Varchar>,
        name -> Varchar,
        name_ja -> Varchar,
        owner -> Varchar,
        owner_ja -> Varchar,
    }
}
