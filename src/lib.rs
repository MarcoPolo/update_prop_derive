extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(UpdateProp, attributes(update_type))]
pub fn update_prop_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_update_prop(&ast)
}

fn impl_update_prop(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
      impl UpdateProp<f32> for #name {
        fn update_prop(&mut self, k: &str, v: f32) -> Result<(), Box<dyn std::error::Error>> {
            self.inner.update_prop(k, v)?;
            Ok(())
        }
      }
      impl UpdateProp<String> for #name {
        fn update_prop(&mut self, k: &str, v: String) -> Result<(), Box<dyn std::error::Error>> {
            self.inner.update_prop(k, v)?;
            Ok(())
        }
      }
      impl UpdateProp<Callback> for #name {
        fn update_prop(&mut self, k: &str, v: Callback) -> Result<(), Box<dyn std::error::Error>> {
            self.inner.update_prop(k, v)?;
            Ok(())
        }
      }


      impl UpdatePropSignal<f32> for #name {
        fn update_prop_signal<S>(&mut self, k: &'static str, s: S) -> Result<(), Box<dyn Error>>
        where
            S: 'static + Signal<Item = f32> + Send,
        {
            let mut platform_view = self.inner.clone();
            let f = s.for_each(move |i| {
            platform_view.update_prop(k, i).expect("view is there");
            ready(())
            });

            self.after_remove.push(spawn_future(f));
            Ok(())
        }
      }

      impl UpdatePropSignal<String> for #name {
        fn update_prop_signal<S>(&mut self, k: &'static str, s: S) -> Result<(), Box<dyn Error>>
        where
            S: 'static + Signal<Item = String> + Send,
        {
            let mut platform_view = self.inner.clone();
            let f = s.for_each(move |i| {
            platform_view.update_prop(k, i).expect("view is there");
            ready(())
            });

            self.after_remove.push(spawn_future(f));
            Ok(())
        }
      }

      impl UpdatePropSignal<Callback> for #name {
        fn update_prop_signal<S>(&mut self, k: &'static str, s: S) -> Result<(), Box<dyn Error>>
        where
            S: 'static + Signal<Item = Callback> + Send,
        {
            let mut platform_view = self.inner.clone();
            let f = s.for_each(move |i| {
            platform_view.update_prop(k, i).expect("view is there");
            ready(())
            });

            self.after_remove.push(spawn_future(f));
            Ok(())
        }
      }
    };
    gen.into()
}
