/// autofill struct body
#[macro_export]
macro_rules! declare_params {
    ($($param:ident),+) => {
        $(
            $param: AtomicFloat,
        )+
    };
}

/// autofill default initializers, getters and setters
#[macro_export]
macro_rules! params_init_get_set {
    ($($param:ident),+) => {

    };
}