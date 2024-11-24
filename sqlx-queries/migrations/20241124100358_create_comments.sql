create table comments (
	id uuid not null primary key default uuid_generate_v4(),
	content text not null,
	created_at timestamp with time zone not null default now(),
	updated_at timestamp with time zone default null,
	post_id uuid not null references posts (id) on delete cascade,
	user_id uuid not null references users (id) on delete cascade
);
