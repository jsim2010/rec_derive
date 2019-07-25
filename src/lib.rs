//! Implements custom derives for [`rec`].
//!
//! Derives implementations of the following operations with all other `Element`s:
//! - `Add`
//! - `BitOr`
//! - `PartialEq`
#![warn(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bare_trait_objects,
    box_pointers,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    private_doc_tests,
    question_mark_macro_sep,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
// Rustc lints that are not warned:
// single_use_lifetimes: there are issues with derived traits
// variant_size_differences: generally there is not much that can be done about this
#![allow(
    clippy::suspicious_op_assign_impl,
    clippy::suspicious_arithmetic_impl,
    clippy::fallible_impl_from, // Above lints are not always correct; issues should be detected by tests or other lints.
    clippy::implicit_return, // Omitting the return keyword is idiomatic Rust code.
    clippy::missing_inline_in_public_items, // Generally not bad and there are issues with derived traits.
)]
#![no_std]
#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

/// Implements all operations for the `Component` defined by `item`.
#[proc_macro_derive(ComponentOps)]
pub fn derive_component_ops(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let element_ops = element_ops(name);

    TokenStream::from(quote! {
        #element_ops

        impl<Rhs: Element> ::core::ops::BitOr<Rhs> for #name {
            type Output = Rec;

            fn bitor(self, rhs: Rhs) -> Self::Output {
                self.alternate(&rhs)
            }
        }

        impl ::core::ops::BitOr<#name> for char {
            type Output = Rec;

            fn bitor(self, rhs: #name) -> Self::Output {
                self.alternate(&rhs)
            }
        }
    })
}

/// Implements all operations for the `Atom` defined by `item`.
#[proc_macro_derive(AtomOps)]
pub fn derive_atom_ops(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let element_ops = element_ops(name);

    TokenStream::from(quote! {
        #element_ops

        impl<Rhs: Atom> ::core::ops::BitOr<Rhs> for #name {
            type Output = Ch;

            fn bitor(self, rhs: Rhs) -> Self::Output {
                self.union(&rhs)
            }
        }

        impl ::core::ops::BitOr<Rec> for #name {
            type Output = Rec;

            fn bitor(self, rhs: Rec) -> Self::Output {
                self.alternate(&rhs)
            }
        }

        impl ::core::ops::BitOr<&str> for #name {
            type Output = Rec;

            fn bitor(self, rhs: &str) -> Self::Output {
                self.alternate(&rhs)
            }
        }

        impl ::core::ops::BitOr<String> for #name {
            type Output = Rec;

            fn bitor(self, rhs: String) -> Self::Output {
                self.alternate(&rhs)
            }
        }

        impl ::core::ops::BitOr<#name> for char {
            type Output = Ch;

            fn bitor(self, rhs: #name) -> Self::Output {
                self.union(&rhs)
            }
        }
    })
}

/// Returns an implementation of all operations shared by `Atom` and `Component` involving `name`.
fn element_ops(name: &Ident) -> TokenStream2 {
    quote! {
        impl<Rhs: Element> ::core::ops::Add<Rhs> for #name {
            type Output = Rec;

            fn add(self, rhs: Rhs) -> Self::Output {
                self.concatenate(&rhs)
            }
        }

        impl<Rhs: Element> PartialEq<Rhs> for #name {
            fn eq(&self, other: &Rhs) -> bool {
                self.is_equal(other)
            }
        }

        impl ::core::ops::Add<#name> for char {
            type Output = Rec;

            fn add(self, rhs: #name) -> Self::Output {
                self.concatenate(&rhs)
            }
        }

        impl PartialEq<#name> for char {
            fn eq(&self, other: &#name) -> bool {
                self.is_equal(other)
            }
        }

        impl ::core::ops::Add<#name> for &str {
            type Output = Rec;

            fn add(self, rhs: #name) -> Self::Output {
                self.concatenate(&rhs)
            }
        }

        impl ::core::ops::BitOr<#name> for &str {
            type Output = Rec;

            fn bitor(self, rhs: #name) -> Self::Output {
                self.alternate(&rhs)
            }
        }

        impl PartialEq<#name> for &str {
            fn eq(&self, other: &#name) -> bool {
                self.is_equal(other)
            }
        }

        impl ::core::ops::Add<#name> for String {
            type Output = Rec;

            fn add(self, rhs: #name) -> Self::Output {
                self.concatenate(&rhs)
            }
        }

        impl ::core::ops::BitOr<#name> for String {
            type Output = Rec;

            fn bitor(self, rhs: #name) -> Self::Output {
                self.alternate(&rhs)
            }
        }

        impl PartialEq<#name> for String {
            fn eq(&self, other: &#name) -> bool {
                self.is_equal(other)
            }
        }
    }
}
