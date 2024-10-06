use rand::prelude::*;
use derive_new::new;

#[derive(Debug)]
pub enum Role{
  Guest,
  Viewer,
  Creator,
  Admin
}

// rust doesn't have default constructors like cpp

#[derive(Debug)]
pub struct User{
    id: u32,
    pub username: String,
    pub role: Role,
}

impl User{
    pub fn new(username: String) -> Result<Self, String>{
        // check for same username
        if username == "raiden"{
            return Err("username already exist".to_owned());
        }

        Ok(Self{
            id: thread_rng().gen_range(0..9999),
            username,
            role: Role::Creator,
        })
    }
}

// to get a default constructor we have to impliment a default trait
impl Default for User{
    fn default() -> Self{
        let id = thread_rng().gen_range(0..9999);
        Self{
            id,
            username: format!("guest{id}"),
            role: Role::Guest
        }
    }
}


// we can use the derive new crate to automatically create a constructor

#[derive(Debug, Default, new)]
pub struct Post{
    content: String,
    #[new(value = "vec![\"raiden\".to_owned()]")]
    tags: Vec<String>,
    #[new(default)]
    likes: u32,
}