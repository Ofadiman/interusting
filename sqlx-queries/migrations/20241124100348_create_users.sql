create table users (
	id uuid not null primary key default uuid_generate_v4(),
	first_name text not null,
	last_name text not null,
	created_at timestamp with time zone not null default now(),
	updated_at timestamp with time zone default null
);
