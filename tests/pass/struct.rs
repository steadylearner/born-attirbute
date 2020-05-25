// Move this to unit test and more heavier example here later.
// cargo test pass -- --nocapture

use born_attribute::{no_field, user_base, message_base};

#[no_field]
#[derive(Debug)]
struct NoField;

#[user_base]
#[derive(Debug)]
struct UserCreateRequest {
    name: String,
}

#[message_base]
#[derive(Debug)]
struct MessageCreateRequest {
    from: String,
    to: String
}

#[test]
fn pass_public_struct() {
    let no_field = NoField {
        useful: false,
    };
    println!("{:#?}", no_field);

    let user = UserCreateRequest {
        name: "www.steadylearner.com".into(),
        active: true,
    };

    let message_create_request = MessageCreateRequest {
        text: "You can do the same thing with functional macros from born.".into(),
        from: user.name,
        to: "Rust developers".into(),
    };
    println!("{:#?}", &message_create_request);
}
