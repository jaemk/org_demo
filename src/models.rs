use rusqlite::{Connection, Row};
use errors::*;


// ------------------------------------------
// ----------- Creating things --------------
// ------------------------------------------
pub struct NewOrg {
    pub name: String,
}
impl NewOrg {
    pub fn insert(self, conn: &Connection) -> Result<i64> {
        let stmt = "insert into org (name) values (?)";
        Ok(try_insert!(conn, stmt, &[&self.name]))
    }
}


pub struct NewUser {
    pub email: String,
}
impl NewUser {
    pub fn insert(self, conn: &Connection) -> Result<i64> {
        let stmt = "insert into user (email) values (?)";
        Ok(try_insert!(conn, stmt, &[&self.email]))
    }
}


pub struct NewUserOrg {
    pub user: i64,
    pub org: i64,
}
impl NewUserOrg {
    pub fn insert(self, conn: &Connection) -> Result<i64> {
        let stmt = "insert into user_org (user, org) values (?, ?)";
        Ok(try_insert!(conn, stmt, &[&self.user, &self.org]))
    }
}


pub struct NewLinode {
    pub name: String,
    pub org: i64,
}
impl NewLinode {
    pub fn insert(self, conn: &Connection) -> Result<i64> {
        let stmt = "insert into linode (name, org) values (?, ?)";
        Ok(try_insert!(conn, stmt, &[&self.name, &self.org]))
    }
}


// ------------------------------------------
// ----------- Querying things --------------
// ------------------------------------------
#[derive(Serialize)]
pub struct User {
    pub id: Option<i64>,
    pub email: Option<String>,
}
impl User {
    pub fn exists(conn: &Connection, email: &str) -> Result<bool> {
        let stmt = "select exists(select 1 from user where email = ?)";
        Ok(conn.query_row(stmt, &[&email], |row| {
            let i: u8 = row.get(0);
            i == 1
        })?)
    }
}


#[derive(Serialize)]
pub struct Linode {
    pub id: Option<i64>,
    pub name: Option<String>,
}
impl Linode {
    pub fn exists(conn: &Connection, name: &str) -> Result<bool> {
        let stmt = "select exists(select 1 from linode where name = ?)";
        Ok(conn.query_row(stmt, &[&name], |row| {
            let i: u8 = row.get(0);
            i == 1
        })?)
    }
}


#[derive(Serialize)]
pub struct OrgInfo {
    pub id: i64,
    pub name: String,
    pub users: Vec<User>,
    pub linodes: Vec<Linode>,
}
impl OrgInfo {
    fn add_row(orgs: &mut Vec<OrgInfo>, row: Row) {
        let user = User {
            id: row.get(2),
            email: row.get(3),
        };
        let linode = Linode {
            id: row.get(4),
            name: row.get(5),
        };
        let org = OrgInfo {
            id: row.get(0),
            name: row.get(1),
            users: if user.id.is_some() { vec![user] } else { vec![] },
            linodes: if linode.id.is_some() { vec![linode] } else { vec![] },
        };
        orgs.push(org);
    }

    fn extract_row(orgs: &mut Vec<OrgInfo>, row: Row) {
        if orgs.last().is_none() {
            Self::add_row(orgs, row);
            return;
        }
        let id: i64 = row.get(0);
        let add_row = orgs.last().map(|org| org.id != id).unwrap_or(true);
        if add_row { Self::add_row(orgs, row); }
        else {
            let prev = orgs.last_mut().unwrap();
            let user = User {
                id: row.get(2),
                email: row.get(3),
            };
            if user.id.is_some() {
                if prev.users.iter().find(|existing| existing.id == user.id).is_none() {
                    prev.users.push(user);
                }
            }
            let linode = Linode {
                id: row.get(4),
                name: row.get(5),
            };
            if linode.id.is_some() {
                if prev.linodes.iter().find(|existing| existing.id == linode.id).is_none() {
                    prev.linodes.push(linode);
                }
            }
        }
    }

    pub fn get_all_orgs(conn: &Connection) -> Result<Vec<OrgInfo>> {
        let stmt = "select org.id, org.name, user.id, user.email, linode.id, linode.name \
                        from org \
                        left outer join user_org on org.id=user_org.org \
                        left outer join user on user_org.user=user.id \
                        left outer join linode on user_org.org=linode.org \
                        order by org.id, user.id, linode.id";
        let mut stmt = conn.prepare(stmt)?;
        let mut rows = stmt.query(&[])?;
        let mut orgs = vec![];
        while let Some(row) = rows.next() {
            let row = row?;
            Self::extract_row(&mut orgs, row);
        }
        Ok(orgs)
    }
}


#[derive(Serialize)]
pub struct Org {
    id: Option<i64>,
    name: Option<String>,
}
impl Org {
    pub fn exists(conn: &Connection, name: &str) -> Result<bool> {
        let stmt = "select exists(select 1 from org where name = ?)";
        Ok(conn.query_row(stmt, &[&name], |row| {
            let i: u8 = row.get(0);
            i == 1
        })?)
    }
}


#[derive(Serialize)]
pub struct UserLinode {
    id: Option<i64>,
    name: Option<String>,
    org: Option<i64>,
}


#[derive(Serialize)]
pub struct UserInfo {
    id: i64,
    email: String,
    orgs: Vec<Org>,
    linodes: Vec<UserLinode>,
}
impl UserInfo {
    fn extract_row(user: &mut Option<UserInfo>, row: Row) {
        let org = Org {
            id: row.get(2),
            name: row.get(3),
        };
        let linode = UserLinode {
            id: row.get(4),
            name: row.get(5),
            org: row.get(6),
        };

        if user.is_none() {
            let userinfo = UserInfo {
                id: row.get(0),
                email: row.get(1),
                orgs: if org.id.is_some() { vec![org] } else { vec![] },
                linodes: if linode.id.is_some() { vec![linode] } else { vec![] },
            };
            user.get_or_insert(userinfo);
        } else {
            let user = user.get_or_insert_with(|| unreachable!());
            let org = Org {
                id: row.get(2),
                name: row.get(3),
            };
            if org.id.is_some() {
                if user.orgs.iter().find(|existing| existing.id == org.id).is_none() {
                    user.orgs.push(org);
                }
            }
            let linode = UserLinode {
                id: row.get(4),
                name: row.get(5),
                org: row.get(6),
            };
            if linode.id.is_some() { user.linodes.push(linode); }
        }
    }

    pub fn get_user(id: i64, conn: &Connection) -> Result<Option<UserInfo>> {
        let stmt = "select user.id, user.email, org.id, org.name, linode.id, linode.name, linode.org \
                        from user \
                        left outer join user_org on user_org.user=user.id \
                        left outer join org on user_org.org=org.id \
                        left outer join linode on user_org.org=linode.org \
                        where user.id = ? \
                        order by user.id, org.id, linode.id";
        let mut stmt = conn.prepare(stmt)?;
        let mut rows = stmt.query(&[&id])?;
        let mut user = None;
        while let Some(row) = rows.next() {
            let row = row?;
            Self::extract_row(&mut user, row);
        }
        Ok(user)
    }
}

