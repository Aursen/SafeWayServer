CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE positions (
  id uuid DEFAULT uuid_generate_v4(),
  date TIMESTAMP NOT NULL,
  latitude DOUBLE PRECISION NOT NULL,
  longitude DOUBLE PRECISION NOT NULL,
  PRIMARY KEY (id)
)