/// Generates wasm getters and setters for all the Node primitive types listed within this macro invocation
macro_rules! wasm_node_primitive_getters_setters {
    ( $( ($field:ident, $setter:ident, $type:ty) ),* $(,)? ) => {
        #[wasm_bindgen]
        impl Node {
            $(
                #[wasm_bindgen(getter)]
                pub fn $field(&self) -> $type {
                    self.inner.borrow().$field
                }

                #[wasm_bindgen(setter)]
                pub fn $setter(&self, value: $type) {
                    self.inner.borrow_mut().$field = value;
                }
            )*
        }
    };
}
pub(crate) use wasm_node_primitive_getters_setters;
