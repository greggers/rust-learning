struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}


fn main()
{
    let mut user1 = User{
        email:String::from("someone@example.com"),
        username:String::from("someusername123"),
        active:true,
        sign_in_count:1,
    };

    user1.email = String::from("anotheremail@example.com");
    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}