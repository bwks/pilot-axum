CREATE TABLE requests(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  name TEXT NOT NULL,
  requested_at timestamptz NOT NULL
);