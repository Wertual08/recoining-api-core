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

create table recoining.transactions (
    former_user_id bigint,
    second_user_id bigint,
    month bigint,
    id bigint,
    type int,
    amount decimal,
    currency text,
    label text,
    description text,
    primary key ((former_user_id, second_user_id, month), id)
);

create table recoining.user_p2p_accounts (
    former_user_id bigint,
    second_user_id bigint,
    balance map<text, decimal>,
    primary key ((former_user_id, second_user_id))
);