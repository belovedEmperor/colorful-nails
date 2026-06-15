-- Add migration script here

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE appointments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    scheduled_at TIMESTAMPTZ NOT NULL,
    services TEXT,
    notes TEXT,
    accepted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Index for quick status lookups
create index if not exists idx_appointments_status on appointments(accepted);

-- Row Level Security (optional but recommended)
-- The service role key bypasses RLS, so these only matter if you
-- ever expose the anon key to the client.
alter table appointments enable row level security;
