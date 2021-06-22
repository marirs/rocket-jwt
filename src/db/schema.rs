table! {
    users (username) {
        username -> Text,
        email -> Text,
        password -> Text,
        is_admin -> Bool,
        token -> Nullable<Text>,
    }
}
