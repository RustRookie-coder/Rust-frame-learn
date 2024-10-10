-- Add down migration script here
drop index counters_user_id_index;

drop table counter_records;

drop index counter_records_counter_id_index;

drop table counters;
