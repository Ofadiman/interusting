create table posts (
	id uuid not null primary key default uuid_generate_v4(),
	title text not null,
	content text not null,
	created_at timestamp with time zone not null default now(),
	updated_at timestamp with time zone default null,
	user_id uuid not null references users (id) on delete cascade
);
