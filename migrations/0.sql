-- SCHEMA VERSION 0, 2019-12-26
-- "initial version", considered a "starting point" for the migrations

-- also the assumed database layout of someone either migrating from an older version of PK or starting a new instance,
-- so everything here *should* be idempotent given a schema version older than this or nonexistent.

-- Create proxy_tag compound type if it doesn't exist
do $$ begin
    create type proxy_tag as (
        prefix text,
        suffix text
    );
exception when duplicate_object then null;
end $$;

create table if not exists systems
(
    id          serial primary key,
    hid         char(5) unique not null,
    name        text,
    description text,
    tag         text,
    avatar_url  text,
    token       text,
    created     timestamptz      not null default (current_timestamp at time zone 'utc'),
    ui_tz       text           not null default 'UTC',
    description_privacy integer check (description_privacy in (1, 2)) not null default 1,
    member_list_privacy integer check (member_list_privacy in (1, 2)) not null default 1,
    front_privacy integer check (front_privacy in (1, 2)) not null default 1,
    front_history_privacy integer check (front_history_privacy in (1, 2)) not null default 1,
    pings_enabled bool not null default true,
    group_list_privacy integer check (group_list_privacy in (1, 2)) not null default 1
);

create table if not exists system_guild
(
    system serial not null references systems (id) on delete cascade,
    guild bigint not null,
    
    proxy_enabled bool not null default true,
    autoproxy_mode int check (autoproxy_mode in (1, 2, 3, 4)) not null default 1,
    autoproxy_member int references members (id) on delete set null
    
    primary key (system, guild)
);

create table if not exists members
(
    id           serial primary key,
    hid          char(5) unique not null,
    system       serial         not null references systems (id) on delete cascade,
    color        char(6),
    avatar_url   text,
    name         text           not null,
    display_name text,
    birthday     date,
    pronouns     text,
    description  text,
    proxy_tags   proxy_tag[]    not null default array[]::proxy_tag[], -- Rationale on making this an array rather than a separate table - we never need to query them individually, only access them as part of a selected Member struct
    keep_proxy   bool           not null default false, 
    created      timestamptz      not null default (current_timestamp at time zone 'utc'),
    message_count int not null default 0,
    description_privacy integer check (description_privacy in (1, 2)) not null default 1,
    name_privacy integer check (name_privacy in (1, 2)) not null default 1,
    avatar_privacy integer check (avatar_privacy in (1, 2)) not null default 1,
    birthday_privacy integer check (birthday_privacy in (1, 2)) not null default 1,
    pronoun_privacy integer check (pronoun_privacy in (1, 2)) not null default 1,
    metadata_privacy integer check (metadata_privacy in (1, 2)) not null default 1
);

create table if not exists member_guild
(
    member serial not null references members (id) on delete cascade,
    guild bigint not null,
    
    display_name text default null,
    avatar_url text
    
    primary key (member, guild)
);

create table if not exists accounts
(
    uid    bigint primary key,
    system serial not null references systems (id) on delete cascade
);

create table if not exists messages
(
    mid          bigint primary key,
    channel      bigint not null,
    member       serial not null references members (id) on delete cascade,
    sender       bigint not null,
    original_mid bigint,
    guild        bigint default null
);

create table if not exists switches
(
    id        serial primary key,
    system    serial    not null references systems (id) on delete cascade,
    timestamp timestamptz not null default (current_timestamp at time zone 'utc')
);

create table if not exists switch_members
(
    id     serial primary key,
    switch serial not null references switches (id) on delete cascade,
    member serial not null references members (id) on delete cascade
);

create table if not exists webhooks
(
    channel bigint primary key,
    webhook bigint not null,
    token   text   not null
);

create table if not exists servers
(
    id            bigint primary key,
    log_channel   bigint,
    log_blacklist bigint[] not null default array[]::bigint[],
    blacklist     bigint[] not null default array[]::bigint[],
    log_cleanup_enabled bool not null default false
);

create table groups (
    id int primary key generated always as identity,
    hid char(5) unique not null,
    system int not null references systems(id) on delete cascade,
    
    name text not null,
    display_name text,
    description text,
    icon text,
    
    -- Description columns follow the same pattern as usual: 1 = public, 2 = private
    description_privacy integer check (description_privacy in (1, 2)) not null default 1,
    icon_privacy integer check (icon_privacy in (1, 2)) not null default 1,
    list_privacy integer check (list_privacy in (1, 2)) not null default 1,
    visibility integer check (visibility in (1, 2)) not null default 1,

    created timestamptz not null default (current_timestamp at time zone 'utc')
);

create table group_members (
    group_id int not null references groups(id) on delete cascade,
    member_id int not null references members(id) on delete cascade,
    primary key (group_id, member_id)
);

create index if not exists idx_switches_system on switches using btree (system asc nulls last) include ("timestamp");
create index if not exists idx_switch_members_switch on switch_members using btree (switch asc nulls last) include (member);
create index if not exists idx_message_member on messages (member);

-- Create a trigger function to increment the message count on inserting to the messages table
create function trg_msgcount_increment() returns trigger as $$
begin
    update members set message_count = message_count + 1 where id = NEW.member;
    return NEW;
end;
$$ language plpgsql;

create trigger increment_member_message_count before insert on messages for each row execute procedure trg_msgcount_increment();


-- Create a trigger function to decrement the message count on deleting from the messages table
create function trg_msgcount_decrement() returns trigger as $$
begin
    -- Don't decrement if count <= zero (shouldn't happen, but we don't want negative message counts)
    update members set message_count = message_count - 1 where id = OLD.member and message_count > 0;
    return OLD;
end;
$$ language plpgsql;

create trigger decrement_member_message_count before delete on messages for each row execute procedure trg_msgcount_decrement();
