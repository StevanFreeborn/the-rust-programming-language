fn main() {
  let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
  };

  user1.email = String::from("anotheremail@example.com");

  let user2 = build_user(
    String::from("test@test.com"), 
    String::from("tester")
  );

  let user3 = User {
    email: String::from("another@test.com"),
    ..user2
  };
}

fn build_user(email: String, username: String) -> User {
  User {
      active: true,
      username,
      email,
      sign_in_count: 1,
  }
}

struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
