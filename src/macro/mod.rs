use std::fmt::format;

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
            fn from($inner: $origin) -> Self {
                Self { $inner }
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

#[macro_export] macro_rules! flat_rop {
    ($result: ident: Some($promise: ident) => $($tt: tt)*) => {
        match $result {
            Ok(Some($promise)) => { $($tt)* },
            Ok(None) => Ok(()),
            Err(err) => Err(err)
        }
    };
}

#[macro_export] macro_rules! code_point {
    () => (format!("{}:{}:{}", file!(), line!(), column!()))
}

#[macro_export] macro_rules! print_err {
    ($err: ident) => (eprintln!("handle_res:Error [{}]\n  {:?}", $crate::code_point!(), $err))
}

#[macro_export] macro_rules! handle_res {
    ($result: ident) => {
        match $result {
            Ok(_) => (),
            Err(err) => $crate::print_err!(err),
        }
    };
}

#[macro_export] macro_rules! handle_rop {
    ($run: ident ( Some($promise: ident) @ $result: ident)) => {
        match $result {
            Ok(Some($promise)) => napi::bindgen_prelude::$run(async {
                let result = $promise.await;
                $crate::handle_res!(result);
            }),
            Ok(None) => return,
            Err(err) => return $crate::print_err!(err),
        }
    };
}

#[macro_export] macro_rules! ok_or_reason {
    ($from: expr) => {
        ok_or_reason!($from; "{}")
    };
    ($from: expr; $display: literal) => {
        match $from {
            Ok(temp) => temp,
            Err(err) => return Err(napi_reason!($display, err)),
        }
    };
}

#[macro_export] macro_rules! napi_reason {
    ($display: literal $(, $args: expr )*) => {
        napi::Error::from_reason(format!($display $(, $args)*))
    };
}