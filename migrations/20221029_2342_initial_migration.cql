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
    image text,
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
) with clustering order by (id asc);

create table recoining.registries (
    id bigint primary key,
    current_pack bigint,
    current_sequence smallint,
    name text,
    image text,
    variant smallint,
    created_at bigint,
    updated_at bigint
);

create table recoining.registry_users (
    registry_id bigint,
    user_id bigint,
    updated_at bigint,
    current_pack bigint,
    current_sequence smallint,
    balance map<text, decimal>,
    primary key (registry_id, user_id)
) with clustering order by (user_id asc);

create materialized view recoining.user_registries
as select 
    user_id, 
    registry_id, 
    updated_at
from recoining.registry_users
where user_id is not null
and updated_at is not null
primary key (user_id, updated_at, registry_id)
with clustering order by (updated_at desc);

create table recoining.transactions (
    registry_id bigint,
    pack bigint,
    sequence smallint,
    created_at bigint,
    source_user_id bigint,
    target_user_id bigint,
    variant smallint,
    amount decimal,
    currency text,
    label text,
    description text,
    hash blob,
    primary key ((registry_id, pack), sequence)
) with clustering order by (sequence desc);