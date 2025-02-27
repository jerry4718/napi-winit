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

#[macro_export] macro_rules! handle_res {
    ($result: ident) => {
        match $result {
            Ok(_) => (),
            Err(napi::bindgen_prelude::Error { status, reason, .. }) => {
                println!("status: {}, reason: {}", status, reason);
            }
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
            Err(err) => { println!("handle_result error {:?}", err); return },
        }
    };
}

#[macro_export] macro_rules! pool_rop {
    ($result: ident) => {
        $crate::THREAD_POOL.execute(move || {
            let result = $crate::flat_rop!($result: Some(promise) => pollster::block_on(promise));
            $crate::handle_res!(result);
        })
    };
}