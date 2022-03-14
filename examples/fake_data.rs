#[tokio::main]
async fn main() {}

#[cfg(test)]
mod test {

    use super::*;
    use fake::{Dummy, Fake, Faker};
    use uuid::Uuid;

    #[derive(Debug, Dummy)]
    pub struct Foo {
        #[dummy(faker = "1000..2000")]
        order_id: usize,
        customer: String,
        paid: bool,
    }

    #[test]
    fn generate_uuid() {
        let uuid = Uuid::new_v4().to_string();
        println!("{}", uuid);
        assert_eq!(true, uuid.len() > 0, "uuid must not be blank");
    }

    #[test]
    fn generate_list_of_fake_data() {
        let f: Foo = Faker.fake();
        println!("{:?}", f);

        use fake::faker::internet::en::*;

        let email: String = SafeEmail().fake();
        println!("the fake email: {:?}", email);

        use fake::faker::name::raw::*;
        use fake::locales::*;

        let name: String = Name(EN).fake();
        println!("name {:?}", name);

        let name: String = Name(ZH_CN).fake();
        println!("name {:?}", name);

        // using convenient function without providing locale
        use fake::faker::lorem::en::*;
        let words: Vec<String> = Words(3..5).fake();
        println!("words {:?}", words);
    }
}
