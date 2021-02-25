/// autofill struct body
#[macro_export]
macro_rules! struct_params {
    ($($param:ident),+) => {
        struct EffectParameters {
            $($param: AtomicFloat,)+
        }

        impl Default for EffectParameters {
            fn default() -> EffectParameters {
                EffectParameters {
                    $($param: AtomicFloat::new(0.0),)+
                }
            }
        }
    };
}

/// autofill default initializers, getters and setters
#[macro_export]
macro_rules! params_init_get_set {
    ($($param:ident),+) => {

    };
}