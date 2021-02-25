/// autofill struct body
#[macro_export]
macro_rules! struct_params {
    ($($param:ident),+) => {
        struct EffectParameters {
            $($param: AtomicFloat,)+
        }
    };
}

/// autofill default initializers, getters and setters
#[macro_export]
macro_rules! params_init_get_set {
    ($($param:ident),+) => {

    };
}