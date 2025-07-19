create table if not exists users (
  id serial primary key,
  username varchar(50) unique not null,
  email varchar(255) unique not null,
  public_key varchar(4096) unique not null,
  active boolean default(false)
);
