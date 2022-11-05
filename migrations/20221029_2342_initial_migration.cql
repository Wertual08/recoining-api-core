create keyspace if not exists recoining
with replication = {
    'class': 'NetworkTopologyStrategy',
    'datacenter1': 2
}
and durable_writes = true;

create table recoining.users (
    id bigint primary key,
    phone bigint,
    email text,
    login text,
    avatar text,
    balance map<text, decimal>
);

create index on recoining.users(phone);

create index on recoining.users(email);

create table recoining.phone_codes (
    phone bigint primary key,
    code bigint,
    created_at bigint,
    attempts smallint
);

create table recoining.email_codes (
    email text primary key,
    code bigint,
    created_at bigint,
    attempts smallint
);

create table recoining.user_tokens (
    user_id bigint,
    id text,
    primary key (user_id, id)
);

create table recoining.registries (
    id bigint primary key,
    current_batch bigint,
    current_sequence bigint
);

create table recoining.transactions (
    registry_id bigint,
    pack bigint,
    sequence smallint,
    created_at bigint,
    type smallint,
    amount decimal,
    currency text,
    label text,
    description text,
    hash blob,
    state tinyint,
    primary key ((registry_id, pack), sequence)
);

create table recoining.user_registries (
    user_id bigint,
    registry_id bigint,
    balance map<text, decimal>,
    primary key (user_id, registry_id)
);

create index on recoining.user_registries(registry_id);