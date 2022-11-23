// slugs.rs will include getSlug and setSlug for setting values to our link shortener
use salvo::__private::tracing::error;
use names::Generator;

pub async fn get_slug(slug: &str) -> String {
    let db: sled::Db = sled::open("my_db").unwrap();
    match db.get(slug) {
        Ok(t) => {
            let owned_edition = t.to_owned();
            if owned_edition == None {
                return "".to_string()
            }
            let unwrapped_owned_edition = owned_edition.unwrap();
            let re = std::str::from_utf8(&unwrapped_owned_edition).unwrap();
            return re.to_owned()
        },
        Err(e) => {
            error!("Error: {e}");
        }
    }
    return "".to_string()
}

pub async fn set_slug(domain: String) -> String {
    let key = Generator::default().next().unwrap();
    let db: sled::Db = sled::open("my_db").unwrap();
    db.insert(key.clone().into_bytes(), domain.into_bytes()).unwrap(); // key has to be cloned here because the borrow checker
    key
}