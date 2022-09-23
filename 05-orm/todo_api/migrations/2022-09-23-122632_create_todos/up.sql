-- Your SQL goes here
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT 'f',
  completed_at TIMESTAMP,
  created_at TIMESTAMP
)
