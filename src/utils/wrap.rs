pub fn wrap<To, From: Into<To>>(value: From) -> To {
    Into::into(value)
}