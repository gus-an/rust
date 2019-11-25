fn main() {
    let mut user1 = User {
        username: String::from("ajw"),
        active: true,
        sign_in_count: 1,
    };

    user1.sign_in_count = 2;
}

struct User{
    username: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String) -> User {
    User {
        username: username,
        active: true,
        sign_in_count: 1,
    }
}