

struct User {
    username: String,
    age: i32,
    bio: String,
}


fn main() {
    
    let user_one = User {
        username: String::from("Bob"),
        age: 25,
        bio: String::from("University student"),
    };

    let user_two = User {
        username: String::from("Ben"),
        ..user_one
    };

    println!("{} is {} years old. His bio is: {}", user_two.username, user_one.age, user_two.bio);
}
