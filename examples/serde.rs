use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
    dob: DateTime<Utc>,
    skills: Vec<String>,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("user", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("age", &self.age)?;
        state.serialize_field("dob", &self.dob)?;
        state.serialize_field("skills", &self.skills)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("User", &["name", "age", "dob", "skills"], UserVisitor)
    }
}

struct UserVisitor;

impl<'de> Visitor<'de> for UserVisitor {
    type Value = User;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct User")
    }

    fn visit_seq<A>(self, mut seq: A) -> std::prelude::v1::Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let name = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

        let age = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

        let dob = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;

        let skills = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;

        Ok(User {
            name,
            age,
            dob,
            skills,
        })
    }

    fn visit_map<A>(self, mut map: A) -> std::prelude::v1::Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut name = None;
        let mut age = None;
        let mut dob = None;
        let mut skills = None;

        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "name" => {
                    name = Some(map.next_value()?);
                }
                "age" => {
                    age = Some(map.next_value()?);
                }
                "dob" => {
                    dob = Some(map.next_value()?);
                }
                "skills" => {
                    skills = Some(map.next_value()?);
                }
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }
        let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
        let age = age.ok_or_else(|| serde::de::Error::missing_field("age"))?;
        let dob = dob.ok_or_else(|| serde::de::Error::missing_field("dob"))?;
        let skills = skills.ok_or_else(|| serde::de::Error::missing_field("skills"))?;

        Ok(User {
            name,
            age,
            dob,
            skills,
        })
    }
}

fn main() -> Result<()> {
    let user = User {
        name: "John".to_string(),
        age: 42,
        dob: Utc::now(),
        skills: vec!["Rust".to_string(), "Web Development".to_string()],
    };
    let json = serde_json::to_string(&user)?;
    println!("{:?}", json);
    println!("{}", json);

    let user1: User = serde_json::from_str(&json)?;
    println!("{:?}", user1);
    assert_eq!(user, user1);
    Ok(())
}
