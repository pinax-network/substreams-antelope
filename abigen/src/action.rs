use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;

use super::ty::Type;

#[derive(Debug, Clone)]
pub struct Action {
    pub name: String,
    pub ty: Type,
}

impl From<(String, Type)> for Action {
    fn from(tuple: (String, Type)) -> Self {
        Action {
            name: tuple.0,
            ty: tuple.1,
        }
    }
}

impl Action {
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let camel_name = syn::Ident::new(&self.name.to_upper_camel_case(), Span::call_site());
        let ty = self.ty.generate();

        quote! {
            #ty
            impl substreams_antelope::Action for #camel_name {
                const NAME: &'static str = #name;
                fn decode(
                    trace: &substreams_antelope::pb::ActionTrace,
                ) -> Result<Self, substreams_antelope::Error> {
                    Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
                }
            }
        }
    }
}
