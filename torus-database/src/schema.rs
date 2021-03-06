table! {
    sandbox (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
        birthday -> Nullable<Date>,
        height -> Nullable<Int4>,
        weight -> Nullable<Numeric>,
        enabled -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
