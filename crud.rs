use std::collections::HashMap;

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
}

struct UserStore {
    users: HashMap<u32, User>,
    next_id: u32,
}

impl UserStore {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    fn create_user(&mut self, name: String) -> u32 {
        let user = User {
            id: self.next_id,
            name,
        };
        self.users.insert(self.next_id, user);
        self.next_id += 1;
        self.next_id - 1
    }

    fn get_user(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    fn update_user(&mut self, id: u32, new_name: String) -> bool {
        if let Some(user) = self.users.get_mut(&id) {
            user.name = new_name;
            true
        } else {
            false
        }
    }

    fn delete_user(&mut self, id: u32) -> bool {
        self.users.remove(&id).is_some()
    }

    fn list_users(&self) {
        for user in self.users.values() {
            println!("{:?}", user);
        }
    }
}

fn main() {
    let mut store = UserStore::new();

    let user1_id = store.create_user("Alice".to_string());
    let user2_id = store.create_user("Bob".to_string());

    println!("Created: {:?}", user1_id);
    println!("Created: {:?}", user2_id);

    println!("\nAll users:");
    store.list_users();

    println!("\nUpdating user 1:");
    store.update_user(1, "Alicia".to_string());
    store.list_users();

    println!("\nDeleting user 2:");
    store.delete_user(2);
    store.list_users();
}
