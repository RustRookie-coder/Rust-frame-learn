-- Add up migration script here
CREATE TYPE living_lab_users_role AS ENUM ('admin', 'user');
create table living_lab_users (
                       id SERIAL primary key not null,
                       user_id character varying not null,
                       openid character varying not null,
                       session_key character varying not null,
                       name character varying not null,
                       email character varying not null,
                       password character varying not null,
                       verification_token VARCHAR(255),
                       role living_lab_users_role NOT NULL DEFAULT 'user',
                       created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                       updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

create unique index living_lab_users_openid_index on living_lab_users (openid)
