use lib_core::AccessControlSystem;

fn main() {
    // Create a new AccessControlSystem
    let mut acs = AccessControlSystem::new();

    // Initialize the system with example users, roles, and permissions
    acs.initialize();

    // Check if user1 has access to /api/resource1
    let user1_access = acs.can_access_api("user1", "/api/resource1");
    println!("User 1 has access to /api/resource1: {}", user1_access);

    // Check if user1 has access to /api/resource2
    let user1_access_resource2 = acs.can_access_api("user1", "/api/resource2");
    println!("User 1 has access to /api/resource2: {}", user1_access_resource2);

    // Check if user2 has access to /api/resource1
    let user2_access_resource1 = acs.can_access_api("user2", "/api/resource1");
    println!("User 2 has access to /api/resource1: {}", user2_access_resource1);

    // Check if user2 has access to /api/resource2
    let user2_access = acs.can_access_api("user2", "/api/resource2");
    println!("User 2 has access to /api/resource2: {}", user2_access);
}

