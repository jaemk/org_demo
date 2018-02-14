begin transaction;

drop index linode_name_index;
drop index user_email_index;
drop index org_name_index;

drop table linode;
drop table user_org;
drop table user;
drop table org;

commit;

