use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn impl_controller_ownership(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl ControllerOwnership for #name {
            fn own(&self, state: &RefCell<SharedState>) -> bool {
                state.borrow_mut().controller.try_lock(self.id())
            }

            fn disown(&self, state: &RefCell<SharedState>) {
                state.borrow_mut().controller.unlock(self.id());
            }
        }
    };

    gen.into()
}
