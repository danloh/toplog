table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        content -> Text,
        author -> Varchar,
        ty -> Int4,
        language -> Varchar,
        topic -> Varchar,
        link -> Varchar,
        link_host -> Varchar,
        post_by -> Varchar,
        post_at -> Timestamp,
        pub_at -> Timestamp,
        vote -> Int4,
    }
}

table! {
    blogs (id) {
        id -> Int4,
        aname -> Varchar,
        avatar -> Varchar,
        intro -> Text,
        topic -> Varchar,
        blog_link -> Varchar,
        blog_host -> Varchar,
        tw_link -> Varchar,
        gh_link -> Varchar,
        other_link -> Varchar,
        is_top -> Bool,
        karma -> Int4,
    }
}

table! {
    issuelabels (issue_id, label) {
        issue_id -> Int4,
        label -> Varchar,
        label_at -> Timestamp,
    }
}

table! {
    issues (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        content -> Text,
        author -> Varchar,
        post_at -> Timestamp,
        vote -> Int4,
        is_closed -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        uname -> Varchar,
        psw_hash -> Varchar,
        join_at -> Timestamp,
        last_seen -> Timestamp,
        avatar -> Varchar,
        email -> Varchar,
        link -> Varchar,
        intro -> Text,
        location -> Varchar,
        nickname -> Varchar,
        permission -> Int2,
        auth_from -> Varchar,
        email_confirmed -> Bool,
        karma -> Int4,
        is_pro -> Bool,
        can_push -> Bool,
        push_email -> Varchar,
    }
}

joinable!(issuelabels -> issues (issue_id));

allow_tables_to_appear_in_same_query!(
    articles,
    blogs,
    issuelabels,
    issues,
    users,
);
