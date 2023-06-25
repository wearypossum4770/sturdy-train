-- Your SQL goes here

CREATE TABLE IF NOT EXISTS create_page_view (
  id INTEGER NOT NULL DEFAULT 0,
idempotent_id UUID PRIMARY KEY NOT NULL,
page_title TEXT NOT NULL,
page_location TEXT NOT NULL,
page_path TEXT NOT NULL,
page_referrer TEXT NULL,
user_agent TEXT NOT NULL,
page_encoding TEXT NOT NULL DEFAULT 'UTF-8',
engagement_time_msec BIGINT NOT NULL,
anonymous_id UUID NOT NULL
);
CREATE TABLE IF NOT EXISTS page_view(
  id SERIAL PRIMARY KEY NOT NULL UNIQUE,
page_title TEXT NOT NULL,
page_location TEXT NOT NULL,
page_path TEXT NOT NULL,
page_referrer TEXT NULL,
user_agent TEXT NOT NULL,
page_encoding TEXT NOT NULL DEFAULT 'UTF-8',
engagement_time_msec BIGINT NOT NULL,
anonymous_id UUID NOT NULL
);