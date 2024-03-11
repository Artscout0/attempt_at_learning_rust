fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@email.com");

    // this
    /*
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    */
    // is equivalent to
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    /*
        in other terms, to copy from 1 instance of a struct to the other, you can use ..struct1Name, but be sure to first set the values you need to change.
     */
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email, // If the field and the input var are the same name, you don't have to write that email:email, you can just write email, .
        sign_in_count: 1,
    }
}
