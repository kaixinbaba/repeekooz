use std::ops::BitOr;

use crate::protocol::req::ACL;

pub enum Perms {
    Read = 1 << 0,
    Write = 1 << 1,
    Create = 1 << 2,
    Delete = 1 << 3,
    Acl = 1 << 4,
    All = Perms::Read as isize | Perms::Write as isize | Perms::Create as isize | Perms::Delete as isize | Perms::Acl as isize,
}

impl Perms {
    pub fn world_acl() -> Vec<ACL> {
        vec![ACL {
            perms: 31,
            scheme: WORLD.to_string(),
            id: ANYONE.to_string(),
        }]
    }
}

pub const WORLD: &str = "world";
pub const ANYONE: &str = "anyone";


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_static_constant() {
        assert_eq!("world", WORLD);
        assert_eq!("anyone", ANYONE);
    }

    #[test]
    fn test_perms() {
        assert_eq!(Perms::Read as isize, 1);
        assert_eq!(Perms::Write as isize, 2);
        assert_eq!(Perms::Create as isize, 4);
        assert_eq!(Perms::Delete as isize, 8);
        assert_eq!(Perms::Acl as isize, 16);
        assert_eq!(Perms::All as isize, 31);
    }
}

