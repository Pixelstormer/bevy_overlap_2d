use proc_macro::TokenStream;
use quote::quote;
use std::sync::atomic::AtomicU8;
use syn::{parse_macro_input, DeriveInput};

/// Generates an impl of the `CollisionLayerLabel` trait.
#[proc_macro_derive(CollisionLayerLabel, attributes(collision_layer_label))]
pub fn derive_collision_layer_label(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_collision_layer_label(&input)
}

fn impl_collision_layer_label(ast: &DeriveInput) -> TokenStream {
    static BIT_NUMBER: AtomicU8 = AtomicU8::new(0);

    let name = &ast.ident;
    let bit = 1u64 << BIT_NUMBER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    let output = quote! {
        impl ::bevy_overlap_2d::CollisionLayersLabel for #name {
            fn into_layers(self) -> ::bevy_overlap_2d::CollisionLayerFlags {
                /// SAFETY: All bits are valid flags, so this can never produce an invalid value
                unsafe { ::bevy_overlap_2d::CollisionLayerFlags::from_bits_unchecked(#bit) }
            }
        }

        impl<T: ::bevy_overlap_2d::CollisionLayersLabel> ::std::ops::BitOr<T> for #name {
            type Output = ::bevy_overlap_2d::CollisionLayerFlags;

            fn bitor(self, rhs: T) -> Self::Output {
                self.into_layers() | rhs.into_layers()
            }
        }

        impl ::std::ops::BitOr<#name> for ::bevy_overlap_2d::CollisionLayerFlags {
            type Output = ::bevy_overlap_2d::CollisionLayerFlags;

            fn bitor(self, rhs: #name) -> Self::Output {
                self | rhs.into_layers()
            }
        }
    };

    output.into()
}
