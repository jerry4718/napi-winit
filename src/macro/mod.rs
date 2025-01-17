#[macro_export] macro_rules! wrap_struct {
    ($(#[$attr: meta])* struct $name: ident ($origin: path)) => {
        #[napi]
        $(#[$attr])*
        pub struct $name(pub(crate) $origin);

        impl From<$origin> for $name {
            fn from(origin: $origin) -> Self {
                Self(origin)
            }
        }

        impl Into<$origin> for $name {
            fn into(self) -> $origin {
                self.0
            }
        }
    };
    ($(#[$attr: meta])* struct $name: ident { $inner: ident: $origin: path }) => {
        #[napi]
        $(#[$attr])*
        pub struct $name {
            pub(crate) $inner: $origin,
        }

        impl From<$origin> for $name {
            fn from(origin: $origin) -> Self {
                Self { $inner: origin }
            }
        }

        impl Into<$origin> for $name {
            fn into(self) -> $origin {
                self.$inner
            }
        }
    }
}

#[macro_export] macro_rules! string_enum {
    ($(#[$attr: meta])* enum $name: ident => $origin: path { $($variant: ident),* $(,)? } $($never: literal)?) => {
        #[napi(string_enum)]
        $(#[$attr])*
        pub enum $name {
            $($variant,)*
        }

        impl From<$origin> for $name {
            fn from(origin: $origin) -> Self {
                match origin {
                    $(<$origin>::$variant => Self::$variant,)*
                    $(_ => unreachable!($never),)?
                }
            }
        }

        impl Into<$origin> for $name {
            fn into(self) -> $origin {
                match self {
                    $($name::$variant => <$origin>::$variant,)*
                }
            }
        }
    }
}