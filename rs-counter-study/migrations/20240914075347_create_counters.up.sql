-- Add up migration script here
create table counters (
    id SERIAL primary key not null,
    user_id character varying not null,
    name text not null,
    value integer not null,
    step integer not null,
    input_step integer not null,
    sequence integer not null,
    created_at TIMESTAMP not null default current_timestamp,
    updated_at TIMESTAMP not null default current_timestamp
);

create index counters_user_id_index on counters (user_id, sequence desc);

create table counter_records (
    id SERIAL primary key not null,
    counter_id integer not null,
    step integer not null,
    "begin" integer not null,
    "end" integer not null,
    created_at TIMESTAMP not null default current_timestamp,
    updated_at TIMESTAMP not null default current_timestamp
);
create index counter_records_counter_id_index on counter_records (counter_id);
