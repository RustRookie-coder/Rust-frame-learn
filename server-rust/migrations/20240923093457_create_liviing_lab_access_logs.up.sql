-- Add up migration script here
CREATE TABLE living_lab_access_logs (
                             id SERIAL primary key not null,
                             user_id character varying not null,
                             name character varying not null,
                             username character varying not null,
                             ip_addr integer NOT NULL,
                             method character varying not null,
                             request_url character varying not null,
                             device character varying not null,
                             execution_time integer,
                             error character varying not null,
                             created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                             updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );
