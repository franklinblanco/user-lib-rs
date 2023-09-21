CREATE TABLE IF NOT EXISTS "token" (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  auth_token TEXT NOT NULL,
  refresh_token TEXT NOT NULL,
  time_created TIMESTAMPTZ NOT NULL,
  last_updated TIMESTAMPTZ NOT NULL
)