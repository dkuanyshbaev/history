use axum_login::{secrecy::SecretVec, AuthUser};

#[derive(Debug, Clone)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub password_hash: String,
    pub role: Role,
}

impl User {
    pub fn new(secret: String) -> Self {
        Self {
            id: 42,
            name: "Admin".to_string(),
            password_hash: secret,
            role: Role::Admin,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Role {
    _User,
    Admin,
}

impl AuthUser<usize, Role> for User {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }

    fn get_role(&self) -> Option<Role> {
        Some(self.role.clone())
    }
}
