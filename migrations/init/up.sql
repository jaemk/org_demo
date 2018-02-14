-- sqlite specific
pragma foreign_keys = on;

begin transaction;

create table org (
    id integer PRIMARY KEY,
    name text UNIQUE NOT NULL COLLATE NOCASE
);

create table user (
    id integer PRIMARY KEY,
    email text UNIQUE NOT NULL COLLATE NOCASE
);

create table user_org (
    id integer PRIMARY KEY,
    user integer,
    org integer,
    FOREIGN KEY (user) REFERENCES user(id) ON DELETE CASCADE,
    FOREIGN KEY (org) REFERENCES org(id) ON DELETE CASCADE
);

create table linode (
    id integer PRIMARY KEY,
    name text UNIQUE NOT NULL COLLATE NOCASE,
    org integer,
    FOREIGN KEY (org) REFERENCES org(id) ON DELETE CASCADE
);

create index org_name_index on org (name COLLATE NOCASE);
create index user_email_index on user (email COLLATE NOCASE);
create index linode_name_index on linode (name COLLATE NOCASE);

commit;


/*
-- Select all linodes available to a user by their email

SELECT name FROM linode WHERE org in (
    SELECT org.id FROM org
        INNER JOIN user_org on user_org.org=org.id
        INNER JOIN user on user_org.user=user.id
        WHERE user.email = {EMAIL}
);

SELECT linode.name, org.name, user.email FROM org
    INNER JOIN user_org ON org.id=user_org.org
    INNER JOIN user ON user_org.user=user.id
    INNER JOIN linode ON user_org.org=linode.org
    WHERE user.email = {EMAIL};
*/

