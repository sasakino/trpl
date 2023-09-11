fn main() {
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1, 
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        username: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        username: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}