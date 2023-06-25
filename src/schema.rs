// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        first_name -> Text,
        middle_name -> Nullable<Text>,
        last_name -> Text,
    }
}

diesel::table! {
    blog_comment (id) {
        id -> Uuid,
        post_id -> Uuid,
        user_id -> Uuid,
        is_published -> Bool,
        is_archived -> Bool,
        date_archived -> Nullable<Timestamptz>,
        date_created -> Timestamptz,
        date_modified -> Nullable<Timestamptz>,
        date_published -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    blog_post (id) {
        id -> Uuid,
        publisher -> Uuid,
        title -> Text,
        content -> Text,
        link -> Nullable<Text>,
        is_published -> Bool,
        is_archived -> Bool,
        date_archived -> Nullable<Timestamptz>,
        date_created -> Timestamptz,
        date_modified -> Nullable<Timestamptz>,
        date_published -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    blog_subcomment (id) {
        id -> Uuid,
        parent_comment -> Uuid,
        child_comment -> Uuid,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Text,
        ibsn_13 -> Text,
        volume -> Nullable<Int4>,
    }
}

diesel::table! {
    books_authors (book_id, author_id) {
        book_id -> Int4,
        author_id -> Int4,
    }
}

diesel::table! {
    comment (id) {
        id -> Uuid,
        blog_post_id -> Uuid,
        publisher -> Uuid,
        content -> Text,
        date_created -> Timestamptz,
        date_modified -> Nullable<Timestamptz>,
        date_published -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    comment_mention (id) {
        id -> Uuid,
        comment_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    comment_reaction (id) {
        id -> Uuid,
        comment_id -> Uuid,
        user_id -> Uuid,
        date_created -> Timestamptz,
        reaction -> Bpchar,
    }
}

diesel::table! {
    comment_review (id) {
        id -> Uuid,
        comment_id -> Uuid,
        review_status -> Bpchar,
        is_published -> Bool,
        is_archived -> Bool,
        date_archived -> Nullable<Timestamptz>,
        date_created -> Timestamptz,
        date_modified -> Nullable<Timestamptz>,
        date_published -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    country (id) {
        id -> Int4,
        country_code -> Varchar,
        country_abbreviation -> Varchar,
        country_name -> Varchar,
    }
}

diesel::table! {
    create_page_view (idempotent_id) {
        id -> Int4,
        idempotent_id -> Uuid,
        page_title -> Text,
        page_location -> Text,
        page_path -> Text,
        page_referrer -> Nullable<Text>,
        user_agent -> Text,
        page_encoding -> Text,
        engagement_time_msec -> Int8,
        anonymous_id -> Uuid,
    }
}

diesel::table! {
    ethnicity (id) {
        id -> Bpchar,
        description -> Bpchar,
    }
}

diesel::table! {
    feature_environment (id) {
        id -> Uuid,
        environment_name -> Bpchar,
    }
}

diesel::table! {
    friends (id) {
        id -> Uuid,
        relationship_status -> Int4,
        sender -> Uuid,
        receiver -> Uuid,
    }
}

diesel::table! {
    group_available (id) {
        id -> Uuid,
        group_name -> Text,
        is_active -> Bool,
        date_modified -> Nullable<Timestamptz>,
        date_created -> Timestamptz,
    }
}

diesel::table! {
    page_view (id) {
        id -> Int4,
        page_title -> Text,
        page_location -> Text,
        page_path -> Text,
        page_referrer -> Nullable<Text>,
        user_agent -> Text,
        page_encoding -> Text,
        engagement_time_msec -> Int8,
        anonymous_id -> Uuid,
    }
}

diesel::table! {
    pages (id) {
        id -> Int4,
        page_number -> Int4,
        content -> Text,
        book_id -> Int4,
    }
}

diesel::table! {
    peril (id) {
        id -> Int4,
        peril_code -> Bpchar,
        description -> Bpchar,
    }
}

diesel::table! {
    post_mention (id) {
        id -> Uuid,
        post_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    post_reaction (id) {
        id -> Uuid,
        post_id -> Uuid,
        user_id -> Uuid,
        date_created -> Timestamptz,
        reaction -> Bpchar,
    }
}

diesel::table! {
    post_review (id) {
        id -> Uuid,
        post_id -> Uuid,
        review_status -> Bpchar,
        is_published -> Bool,
        is_archived -> Bool,
        date_archived -> Nullable<Timestamptz>,
        date_created -> Timestamptz,
        date_modified -> Nullable<Timestamptz>,
        date_published -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    postal_county (id) {
        id -> Int4,
        state_fips -> Bpchar,
        county_fips -> Bpchar,
        county_name -> Bpchar,
        notes -> Nullable<Bpchar>,
    }
}

diesel::table! {
    postal_states (fips_code) {
        id -> Int4,
        ams_id -> Int4,
        fips_code -> Bpchar,
        state_name -> Bpchar,
        state_code -> Bpchar,
        sovereignty -> Bpchar,
    }
}

diesel::table! {
    user_group (id) {
        id -> Uuid,
        group_id -> Uuid,
        user_id -> Uuid,
        date_modified -> Nullable<Timestamptz>,
        date_created -> Timestamptz,
    }
}

diesel::table! {
    user_profile (id) {
        id -> Uuid,
        user_id -> Uuid,
        date_modified -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Text,
        middle_name -> Nullable<Text>,
        last_name -> Text,
        username -> Text,
        email -> Text,
        hashed_password -> Text,
        password_changed -> Bool,
        email_changed -> Bool,
        is_active -> Bool,
        date_created -> Timestamptz,
        date_modified -> Nullable<Timestamptz>,
        suffix -> Nullable<Text>,
        idempotent_key -> Nullable<Uuid>,
    }
}

diesel::table! {
    users_login (id) {
        id -> Uuid,
        user_id -> Uuid,
        login_time -> Timestamptz,
        logout_time -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users_login_audit (id) {
        id -> Int4,
        user_id -> Uuid,
        uid -> Uuid,
        date_modified -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    workspace (id) {
        id -> Int4,
        workspace_name -> Text,
        workspace_description -> Nullable<Text>,
    }
}

diesel::joinable!(blog_post -> users (publisher));
diesel::joinable!(books_authors -> authors (author_id));
diesel::joinable!(books_authors -> books (book_id));
diesel::joinable!(comment_review -> blog_comment (comment_id));
diesel::joinable!(pages -> books (book_id));
diesel::joinable!(postal_county -> postal_states (state_fips));
diesel::joinable!(user_group -> group_available (group_id));
diesel::joinable!(user_group -> users (user_id));
diesel::joinable!(user_profile -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors, blog_comment, blog_post, blog_subcomment, books, books_authors, comment,
    comment_mention, comment_reaction, comment_review, country, create_page_view, ethnicity,
    feature_environment, friends, group_available, page_view, pages, peril, post_mention,
    post_reaction, post_review, postal_county, postal_states, user_group, user_profile, users,
    users_login, users_login_audit, workspace,
);
