use std::ops::Deref;

struct Person {
    name: String,
}

impl Deref for Person {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

fn main(){
    let persons = vec![
        &Person{ name: "jing".to_string() },
        &Person{ name: "guo".to_string() },
    ];
}
