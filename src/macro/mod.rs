use std::fmt::format;

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