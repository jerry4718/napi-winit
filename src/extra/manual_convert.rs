use std::path::PathBuf;

pub trait ManualFrom<T> {
    fn manual_from(other: T) -> Self;
}

pub trait ManualInto<T> {
    fn manual_into(self) -> T;
}

impl ManualFrom<String> for PathBuf {
    fn manual_from(other: String) -> Self {
        Self::from(other)
    }
}

impl ManualInto<String> for PathBuf {
    fn manual_into(self) -> String {
        String::from(self.to_str().unwrap())
    }
}

fn manual_from<From, Into: ManualFrom<From>>(from: From) -> Into {
    Into::manual_from(from)
}

fn manual_into<From: ManualInto<Into>, Into>(from: From) -> Into {
    From::manual_into(from)
}