use proc_macro::TokenStream;
use quote::quote;
use std::sync::atomic::{AtomicU8, Ordering};
use syn::{parse_macro_input, DeriveInput};

/// Generates an impl of the `CollisionLayersLabel` trait.
#[proc_macro_derive(CollisionLayersLabel)]
pub fn derive_collision_layers_label(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_collision_layers_label(&input)
}

fn impl_collision_layers_label(ast: &DeriveInput) -> TokenStream {
    static BIT_NUMBER: AtomicU8 = AtomicU8::new(0);

    let shift = BIT_NUMBER.fetch_add(1, Ordering::Relaxed);
    let bit = 1u64
        .checked_shl(shift.into())
        .expect("Too many collision layers - Exceeded the maximum limit of 64");

    let name = &ast.ident;
    let output = quote! {
        impl ::bevy_overlap_2d::prelude::CollisionLayersLabel for #name {
            fn into_layers(self) -> ::bevy_overlap_2d::prelude::CollisionLayerFlags {
                /// SAFETY: All bits are valid flags, so this can never produce an invalid value
                unsafe { ::bevy_overlap_2d::prelude::CollisionLayerFlags::from_bits_unchecked(#bit) }
            }
        }

        impl<T: ::bevy_overlap_2d::prelude::CollisionLayersLabel> ::std::ops::BitOr<T> for #name {
            type Output = ::bevy_overlap_2d::prelude::CollisionLayerFlags;

            fn bitor(self, rhs: T) -> Self::Output {
                self.into_layers() | rhs.into_layers()
            }
        }

        impl ::std::ops::BitOr<#name> for ::bevy_overlap_2d::prelude::CollisionLayerFlags {
            type Output = ::bevy_overlap_2d::prelude::CollisionLayerFlags;

            fn bitor(self, rhs: #name) -> Self::Output {
                self | rhs.into_layers()
            }
        }

        impl<T: ::bevy_overlap_2d::prelude::CollisionLayersLabel> ::std::ops::BitAnd<T> for #name {
            type Output = ::bevy_overlap_2d::prelude::CollisionLayerFlags;

            fn bitand(self, rhs: T) -> Self::Output {
                self.into_layers() & rhs.into_layers()
            }
        }

        impl ::std::ops::BitAnd<#name> for ::bevy_overlap_2d::prelude::CollisionLayerFlags {
            type Output = ::bevy_overlap_2d::prelude::CollisionLayerFlags;

            fn bitand(self, rhs: #name) -> Self::Output {
                self & rhs.into_layers()
            }
        }

        impl<T: ::bevy_overlap_2d::prelude::CollisionLayersLabel> ::std::ops::BitXor<T> for #name {
            type Output = ::bevy_overlap_2d::prelude::CollisionLayerFlags;

            fn bitxor(self, rhs: T) -> Self::Output {
                self.into_layers() ^ rhs.into_layers()
            }
        }

        impl ::std::ops::BitXor<#name> for ::bevy_overlap_2d::prelude::CollisionLayerFlags {
            type Output = ::bevy_overlap_2d::prelude::CollisionLayerFlags;

            fn bitxor(self, rhs: #name) -> Self::Output {
                self ^ rhs.into_layers()
            }
        }

        impl<T: ::bevy_overlap_2d::prelude::CollisionLayersLabel> ::std::ops::Sub<T> for #name {
            type Output = ::bevy_overlap_2d::prelude::CollisionLayerFlags;

            fn sub(self, rhs: T) -> Self::Output {
                self.into_layers() - rhs.into_layers()
            }
        }

        impl ::std::ops::Sub<#name> for ::bevy_overlap_2d::prelude::CollisionLayerFlags {
            type Output = ::bevy_overlap_2d::prelude::CollisionLayerFlags;

            fn sub(self, rhs: #name) -> Self::Output {
                self - rhs.into_layers()
            }
        }

        impl ::std::ops::Not for #name {
            type Output = ::bevy_overlap_2d::prelude::CollisionLayerFlags;

            fn not(self) -> Self::Output {
                !self.into_layers()
            }
        }
    };

    output.into()
}
