// Define the User struct
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub roles: Vec<String>,
}

// Define the Role struct
#[derive(Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub permissions: Vec<String>,
}

// Define the Permission struct
#[derive(Debug, Clone)]
pub struct Permission {
    pub id: String,
    pub name: String,
    pub description: String,
}

// Define the AccessControlSystem struct
#[derive(Debug, Clone)]
pub struct AccessControlSystem {
    users: Vec<User>,
    roles: Vec<Role>,
    permissions: Vec<Permission>,
}

impl AccessControlSystem {
    // Create a new AccessControlSystem
    pub fn new() -> Self {
        AccessControlSystem {
            users: Vec::new(),
            roles: Vec::new(),
            permissions: Vec::new(),
        }
    }

    // Add a user to the system
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    // Add a role to the system
    pub fn add_role(&mut self, role: Role) {
        self.roles.push(role);
    }

    // Add a permission to the system
    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.push(permission);
    }

    // Check if a user has a specific permission
    pub fn has_permission(&self, user_id: &str, permission_id: &str) -> bool {
        if let Some(user) = self.users.iter().find(|u| u.id == user_id) {
            for role_id in &user.roles {
                if let Some(role) = self.roles.iter().find(|r| r.id == *role_id) {
                    if role.permissions.contains(&permission_id.to_string()) {
                        return true;
                    }
                }
            }
        }
        false
    }

    // Check if a user has permission to access a specific API endpoint
    pub fn can_access_api(&self, user_id: &str, api_endpoint: &str) -> bool {
        // Example mapping of API endpoints to permission IDs
        let api_permissions = vec![
            ("/api/resource1", "perm1"),
            ("/api/resource2", "perm2"),
            // Add more mappings as needed
        ];

        // Find the permission ID for the given API endpoint
        if let Some(&(_, permission_id)) = api_permissions.iter().find(|&&(endpoint, _)| endpoint == api_endpoint) {
            // Check if the user has the required permission
            self.has_permission(user_id, permission_id)
        } else {
            // If the API endpoint is not found, deny access by default
            false
        }
    }

    // Initialize the system with example users, roles, and permissions
    pub fn initialize(&mut self) {
        // Create permissions
        let perm1 = Permission {
            id: "perm1".to_string(),
            name: "Access Resource 1".to_string(),
            description: "Permission to access /api/resource1".to_string(),
        };
        let perm2 = Permission {
            id: "perm2".to_string(),
            name: "Access Resource 2".to_string(),
            description: "Permission to access /api/resource2".to_string(),
        };

        // Add permissions to the system
        self.add_permission(perm1);
        self.add_permission(perm2);

        // Create roles
        let role1 = Role {
            id: "role1".to_string(),
            name: "Role 1".to_string(),
            permissions: vec!["perm1".to_string()],
        };
        let role2 = Role {
            id: "role2".to_string(),
            name: "Role 2".to_string(),
            permissions: vec!["perm2".to_string()],
        };

        // Add roles to the system
        self.add_role(role1);
        self.add_role(role2);

        // Create users
        let user1 = User {
            id: "user1".to_string(),
            name: "User 1".to_string(),
            roles: vec!["role1".to_string()],
        };
        let user2 = User {
            id: "user2".to_string(),
            name: "User 2".to_string(),
            roles: vec!["role2".to_string()],
        };

        // Add users to the system
        self.add_user(user1);
        self.add_user(user2);
    }
}

