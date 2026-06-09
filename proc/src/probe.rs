#![allow(clippy::use_self)]

use self::keywords::{
    bookmarks, by, combobox, frozen, inlined, multiline, name, new, range, rgb, rgba,
    rgba_premultiplied, rgba_unmultiplied, skip, tags, toggle_switch, transparent, with,
};
use crate::name_display::NameDisplay as _;
use proc_easy::EasyArgument as _;
use syn::{parse::Parse, spanned::Spanned as _};

macro_rules! validate {
    ($condition:expr; !$attributes:expr => [ $($attribute:ident),+ ]) => {
        (|| -> syn::Result<Option<proc_macro2::TokenStream>> {
            if $condition {
                $(
                    if let Some(attribute) = &$attributes.$attribute {
                        return Err(syn::Error::new(
                            attribute.name_span(),
                            format!("Cannot use {} attribute for skipped field", attribute.name_display())
                        ));
                    }
                )+
            }
            Ok(None)
        })()
    };
}

// Tokens
mod keywords {
    proc_easy::easy_token!(bookmarks);
    proc_easy::easy_token!(by);
    proc_easy::easy_token!(combobox);
    proc_easy::easy_token!(frozen);
    proc_easy::easy_token!(inlined);
    proc_easy::easy_token!(multiline);
    proc_easy::easy_token!(name);
    proc_easy::easy_token!(new);
    proc_easy::easy_token!(range);
    proc_easy::easy_token!(rgb);
    proc_easy::easy_token!(rgba_premultiplied);
    proc_easy::easy_token!(rgba_unmultiplied);
    proc_easy::easy_token!(rgba);
    proc_easy::easy_token!(skip);
    proc_easy::easy_token!(tags);
    proc_easy::easy_token!(toggle_switch);
    proc_easy::easy_token!(transparent);
    proc_easy::easy_token!(with);
}

// Parse

proc_easy::easy_argument! {
    struct With {
        with: with,

        /// Expression type must implement `FnOnce(&mut FieldType, &mut
        /// egui::Ui, &::egui_probe::Style) -> egui::Response`
        expr: syn::Expr,
    }
}

proc_easy::easy_argument! {
    struct As {
        probe_as: syn::Token![as],

        /// Expression type must implement `FnOnce(&mut FieldType) -> R`
        /// and R must implement `EguiProbeWrapper`
        expr: syn::Expr,
    }
}

proc_easy::easy_parse! {
    struct RangeStep {
        by: by,

        /// Expr type must match field type.
        expr: syn::Expr,
    }
}

struct RangeArg {
    /// `EguiProbeRange<FieldType, ExprType>` must implement `EguiProbeWrapper`.
    range: Option<syn::Expr>,

    step: Option<RangeStep>,
}

impl Parse for RangeArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let range = if input.peek(by) {
            None
        } else {
            Some(input.parse()?)
        };

        let step = if input.peek(by) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self { range, step })
    }
}

proc_easy::easy_argument_group! {
    pub(super) enum FieldKind {
        As(As),
        Range(Range),
        With(With),
        Frozen(frozen),
        Multiline(multiline),
        Rgb(rgb),
        Rgba(rgba),
        RgbaPremultiplied(rgba_premultiplied),
        RgbaUnmultiplied(rgba_unmultiplied),
        ToggleSwitch(toggle_switch),
    }
}

proc_easy::easy_argument! {
    struct WhereClause {
        where_token: syn::Token![where],
        predicates: proc_easy::EasyTerminated<syn::WherePredicate, syn::Token![,]>,
    }
}

proc_easy::easy_argument_group! {
    enum TagsKind {
        ComboBox(combobox),
        Inlined(inlined),
    }
}

proc_easy::easy_argument! {
    struct EnumTags {
        tags: tags,
        kind: TagsKind,
    }
}

// Argument value

proc_easy::easy_argument_value! {
    struct New {
        new: new,
        expr: syn::Expr,
    }
}

proc_easy::easy_argument_value! {
    struct Bookmarks {
        bookmarks: bookmarks,
        expr: syn::ExprArray,
    }
}

proc_easy::easy_argument_value! {
    struct Name {
        name: name,
        expr: syn::Expr,
    }
}

proc_easy::easy_argument_value! {
    struct Range {
        range: range,
        /// `EguiProbeRange<FieldType, ExprType>` must implement `EguiProbeWrapper`.
        arg: RangeArg,
    }
}

// Attributes

proc_easy::easy_attributes! {
    @(egui_probe)
    struct FieldAttributes {
        bookmarks: Option<Bookmarks>,
        kind: Option<FieldKind>,
        name: Option<Name>,
        new: Option<New>,
        // If `skip` is present, the field will be skipped.
        // Error will be generated if other attributes are present together with
        // `skip`.
        skip: Option<skip>,
    }
}

proc_easy::easy_attributes! {
    @(egui_probe)
    struct TypeAttributes {
        name: Option<Name>,
        new: Option<new>,
        tags: Option<EnumTags>,
        transparent: Option<transparent>,
        where_clause: Option<WhereClause>,
    }
}

proc_easy::easy_attributes! {
    @(egui_probe)
    struct VariantAttributes {
        name: Option<Name>,
        new: Option<new>,
        transparent: Option<transparent>,
    }
}

fn is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn make_name(name: Option<Name>, ident: Option<&syn::Ident>) -> proc_macro2::TokenStream {
    match name {
        Some(name) => {
            let expr = name.expr;
            quote::quote!(#expr)
        }
        None => match ident {
            None => quote::quote!(""),
            Some(ident) => {
                let name = ident.to_string();
                quote::quote!(#name)
            }
        },
    }
}

fn field_name(field: &syn::Field) -> syn::Result<Option<proc_macro2::TokenStream>> {
    let attributes: FieldAttributes = proc_easy::EasyAttributes::parse(&field.attrs, field.span())?;

    validate!(attributes.skip.is_some(); !attributes => [bookmarks, new, kind, name])?;

    let name = make_name(attributes.name, field.ident.as_ref());

    Ok(Some(name))
}

fn field_default(field: &syn::Field) -> syn::Result<proc_macro2::TokenStream> {
    let expr = unnamed_field_default(field)?;

    Ok(match &field.ident {
        Some(ident) => quote::quote!(#ident: #expr),
        None => quote::quote!(#expr),
    })
}

fn unnamed_field_default(field: &syn::Field) -> syn::Result<proc_macro2::TokenStream> {
    let attributes: FieldAttributes = proc_easy::EasyAttributes::parse(&field.attrs, field.span())?;

    let mut expr = match attributes.new {
        Some(New { expr, .. }) => quote::quote!(#expr),
        _ => quote::quote!(::core::default::Default::default()),
    };
    if is_option(&field.ty) {
        expr = quote::quote!(::core::option::Option::Some(#expr));
    }
    Ok(expr)
}

fn field_probe(idx: usize, field: &syn::Field) -> syn::Result<Option<proc_macro2::TokenStream>> {
    let attributes: FieldAttributes = proc_easy::EasyAttributes::parse(&field.attrs, field.span())?;

    validate!(attributes.skip.is_some(); !attributes => [bookmarks, new, kind, name])?;

    let binding = quote::format_ident!("__{}", idx);

    let mut tokens = match attributes.kind {
        None => {
            if is_option(&field.ty) {
                let default = unnamed_field_default(field)?;
                quote::quote_spanned! {field.span() =>
                    &mut probe_option(#binding, || #default)
                }
            } else {
                quote::quote_spanned! {field.span() =>
                    #binding
                }
            }
        }
        Some(FieldKind::With(with)) => {
            let expr = with.expr;
            quote::quote_spanned! {field.span() =>
                &mut probe_with(#expr, #binding)
            }
        }
        Some(FieldKind::As(probe_as)) => {
            let expr = probe_as.expr;
            quote::quote_spanned! {field.span() =>
                &mut probe_as(#expr, #binding)
            }
        }
        Some(FieldKind::Range(range)) => match (range.arg.range, range.arg.step) {
            (None, None) => {
                unreachable!()
            }
            (Some(range), None) => {
                quote::quote_spanned! {field.span() =>
                    &mut probe_range(#range, #binding)
                }
            }
            (None, Some(step)) => {
                let step = step.expr;
                quote::quote_spanned! {field.span() =>
                    &mut probe_step(#step, #binding)
                }
            }
            (Some(range), Some(step)) => {
                let step = step.expr;
                quote::quote_spanned! {field.span() =>
                    &mut probe_range_step(#range, #step, #binding)
                }
            }
        },
        Some(FieldKind::Multiline(_)) => {
            quote::quote_spanned! {field.span() =>
                &mut probe_multiline(#binding)
            }
        }
        Some(FieldKind::ToggleSwitch(_)) => {
            quote::quote_spanned! {field.span() =>
                &mut probe_toggle_switch(#binding)
            }
        }
        Some(FieldKind::Frozen(_)) => {
            quote::quote_spanned! {field.span() =>
                &mut probe_frozen(#binding)
            }
        }
        Some(FieldKind::Rgb(_)) => {
            quote::quote_spanned! {field.span() =>
                &mut probe_rgb(#binding)
            }
        }
        Some(FieldKind::Rgba(_)) => {
            quote::quote_spanned! {field.span() =>
                &mut probe_rgba(#binding)
            }
        }
        Some(FieldKind::RgbaPremultiplied(_)) => {
            quote::quote_spanned! {field.span() =>
                &mut probe_rgba_premultiplied(#binding)
            }
        }
        Some(FieldKind::RgbaUnmultiplied(_)) => {
            quote::quote_spanned! {field.span() =>
                &mut probe_rgba_unmultiplied(#binding)
            }
        }
    };

    if let Some(bookmarks) = attributes.bookmarks {
        let buttons = bookmarks.expr.elems.iter().map(|expr| {
            let text = quote::quote!(#expr).to_string();
            quote::quote! {
                if ui.selectable_value(#binding, #expr, #text).clicked() {
                    changed = true;
                    ui.close_menu();
                }
            }
        });
        tokens = quote::quote_spanned! {field.span() =>
            &mut probe_with(|#binding, ui, style| {
                ui.horizontal(|ui| {
                    let mut response = ::egui_probe::EguiProbe::probe(#tokens, ui, style);
                    let mut changed = false;
                    ui.menu_button(::egui_phosphor::regular::BOOKMARK, |ui| {
                        #(#buttons)*
                    });
                    if changed {
                        response.mark_changed();
                    }
                    response
                }).inner
            }, #binding)
        };
    }

    Ok(Some(tokens))
}

fn variant_new(variant: &syn::Variant) -> syn::Result<Option<proc_macro2::TokenStream>> {
    let attributes: VariantAttributes =
        proc_easy::EasyAttributes::parse(&variant.attrs, variant.span())?;
    if attributes.new.is_some() {
        Ok(Some(quote::quote!()))
    } else {
        Ok(None)
    }
}

/// Генерирует ветку match, которая возвращает строковое имя варианта, если он
/// сейчас выбран.  
/// Когда enum отображается в виде выпадающего списка (ComboBox), на самой
/// кнопке списка должно быть написано имя текущего активного варианта.
fn variant_selected(variant: &syn::Variant) -> syn::Result<proc_macro2::TokenStream> {
    let attributes: VariantAttributes =
        proc_easy::EasyAttributes::parse(&variant.attrs, variant.span())?;

    let ident = &variant.ident;

    let name = make_name(attributes.name, Some(ident));

    let pattern = match variant.fields {
        syn::Fields::Unit => quote::quote!(Self::#ident),
        syn::Fields::Unnamed(_) => quote::quote!(Self::#ident (..)),
        syn::Fields::Named(_) => quote::quote!(Self::#ident {..}),
    };

    let tokens = quote::quote_spanned! {variant.ident.span() =>
        #pattern => #name
    };

    Ok(tokens)
}

/// Генерирует UI-элемент (кнопку в выпадающем списке или selectable_label в
/// линию), который позволяет пользователю выбрать этот вариант.  
/// Это сама логика переключения. Функция проверяет, выбран ли этот вариант
/// сейчас (checked). Если пользователь кликает по кнопке, а вариант был не
/// выбран, макрос мутирует self, заменяя текущее значение на этот новый вариант
/// (заполняя его поля дефолтными значениями).
fn variant_probe(variant: &syn::Variant) -> syn::Result<proc_macro2::TokenStream> {
    let attributes: VariantAttributes =
        proc_easy::EasyAttributes::parse(&variant.attrs, variant.span())?;

    let ident = &variant.ident;

    let name = make_name(attributes.name, Some(ident));

    let pattern = match variant.fields {
        syn::Fields::Unit => quote::quote!(Self::#ident),
        syn::Fields::Unnamed(_) => quote::quote!(Self::#ident (..)),
        syn::Fields::Named(_) => quote::quote!(Self::#ident {..}),
    };

    let new_self = match &variant.fields {
        syn::Fields::Unit => quote::quote!(Self::#ident),
        syn::Fields::Unnamed(fields) => {
            let new_fields = fields
                .unnamed
                .iter()
                .map(field_default)
                .collect::<syn::Result<Vec<_>>>()?;
            quote::quote!(Self::#ident ( #(#new_fields,)* ))
        }
        syn::Fields::Named(fields) => {
            let new_fields = fields
                .named
                .iter()
                .map(field_default)
                .collect::<syn::Result<Vec<_>>>()?;
            quote::quote!(Self::#ident { #(#new_fields,)* })
        }
    };

    let tokens = quote::quote_spanned! {variant.ident.span() =>
        #[allow(unreachable_patterns)]
        let checked = match self { #pattern => true, _ => false };
        if _ui.selectable_label(checked, #name).clicked() && !checked {
            *self = #new_self;
        }
        // if _ui.selectable_label(checked, #name).clicked() {
        //     if !checked {
        //         *self = #construct;
        //     }
        //     if _in_cbox {
        //         _ui.close_menu();
        //     }
        // }
    };

    Ok(tokens)
}

/// Генерирует код для отрисовки полей варианта прямо рядом с кнопкой выбора (в
/// ту же горизонтальную линию), но только если вариант помечен атрибутом
/// #[egui_probe(transparent)].  
/// Обычно поля варианта рисуются ниже, в виде таблицы/дерева свойств. Но если у
/// варианта всего одно поле (например, Color(Rgb)), вы можете захотеть, чтобы
/// виджет цвета рисовался прямо в той же строке, что и выпадающий список. Если
/// атрибута transparent нет, функция генерирует пустой блок {}.
fn variant_inline_probe(variant: &syn::Variant) -> syn::Result<proc_macro2::TokenStream> {
    let attributes: VariantAttributes =
        proc_easy::EasyAttributes::parse(&variant.attrs, variant.span())?;

    let ident = &variant.ident;

    if attributes.transparent.is_some() {
        let pattern = match variant.fields {
            syn::Fields::Unit => quote::quote!(Self::#ident),
            syn::Fields::Unnamed(ref fields) => {
                let fields = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(idx, _)| quote::format_ident!("__{}", idx));
                quote::quote!(Self::#ident ( #(#fields,)* ))
            }
            syn::Fields::Named(ref fields) => {
                let fields = fields.named.iter().enumerate().map(|(idx, field)| {
                    let binding = quote::format_ident!("__{}", idx);
                    let ident = field.ident.as_ref().unwrap();
                    quote::quote!(#ident: #binding)
                });
                quote::quote!(Self::#ident { #(#fields,)* })
            }
        };

        let all_fields_probe: Vec<_> = variant
            .fields
            .iter()
            .enumerate()
            .filter_map(|(idx, field)| field_probe(idx, field).transpose())
            .collect::<syn::Result<_>>()?;

        if all_fields_probe.len() != 1 {
            return Err(syn::Error::new_spanned(
                attributes.transparent.unwrap(),
                "Transparent variant must have exactly one non-skipped field",
            ));
        }

        let field_probe = &all_fields_probe[0];

        let tokens = quote::quote_spanned! {variant.ident.span() =>
            #pattern => {
                ::egui_probe::EguiProbe::probe(#field_probe, _ui, _style);
            }
        };

        Ok(tokens)
    } else {
        let pattern = match variant.fields {
            syn::Fields::Unit => quote::quote!(Self::#ident),
            syn::Fields::Unnamed(_) => quote::quote!(Self::#ident (..)),
            syn::Fields::Named(_) => quote::quote!(Self::#ident {..}),
        };

        Ok(quote::quote!( #pattern => {} ))
    }
}

/// Генерирует ветку match для метода iterate_inner. Она берет все поля текущего
/// активного варианта и передает их в специальное замыкание _f, которое строит
/// таблицу свойств (Property Grid).  
/// Когда вариант выбран, ниже должна появиться таблица с его полями (например,
/// если выбран Num { value: usize }, должна появиться строка "value" и поле
/// ввода числа). Эта функция "скармливает" поля структуре egui-probe, чтобы та
/// их красиво отрисовала с отступами.
fn variant_iterate_inner(variant: &syn::Variant) -> syn::Result<proc_macro2::TokenStream> {
    let attributes: VariantAttributes =
        proc_easy::EasyAttributes::parse(&variant.attrs, variant.span())?;

    let ident = &variant.ident;

    let pattern = match variant.fields {
        syn::Fields::Unit => quote::quote!(Self::#ident),
        syn::Fields::Unnamed(ref fields) => {
            let fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(idx, _)| quote::format_ident!("__{}", idx));
            quote::quote!(Self::#ident ( #(#fields,)* ))
        }
        syn::Fields::Named(ref fields) => {
            let fields = fields.named.iter().enumerate().map(|(idx, field)| {
                let binding = quote::format_ident!("__{}", idx);
                let ident = field.ident.as_ref().unwrap();
                quote::quote!(#ident: #binding)
            });
            quote::quote!(Self::#ident { #(#fields,)* })
        }
    };

    if attributes.transparent.is_some() {
        let all_fields_probe: Vec<_> = variant
            .fields
            .iter()
            .enumerate()
            .filter_map(|(idx, field)| field_probe(idx, field).transpose())
            .collect::<syn::Result<_>>()?;

        if all_fields_probe.len() != 1 {
            return Err(syn::Error::new_spanned(
                attributes.transparent.unwrap(),
                "Transparent variant must have exactly one non-skipped field",
            ));
        }

        let field_probe = &all_fields_probe[0];

        let tokens = quote::quote_spanned! {variant.ident.span() =>
            #pattern => ::egui_probe::EguiProbe::iterate_inner(#field_probe, _ui, _f),
        };

        Ok(tokens)
    } else {
        let fields_name: Vec<_> = variant
            .fields
            .iter()
            .filter_map(|field| field_name(field).transpose())
            .collect::<syn::Result<_>>()?;

        let fields_probe: Vec<_> = variant
            .fields
            .iter()
            .enumerate()
            .filter_map(|(idx, field)| field_probe(idx, field).transpose())
            .collect::<syn::Result<_>>()?;

        assert_eq!(fields_name.len(), fields_probe.len());

        let tokens = quote::quote_spanned! {variant.ident.span() =>
            #pattern => {
                #(_f(::core::convert::AsRef::<str>::as_ref(&(#fields_name)), _ui, #fields_probe);)*
            },
        };

        Ok(tokens)
    }
}

#[allow(clippy::too_many_lines)]
pub fn derive(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;
    let generics = &input.generics;

    let attributes: TypeAttributes = proc_easy::EasyAttributes::parse(&input.attrs, ident.span())?;

    let type_name = make_name(attributes.name, Some(ident));

    let (impl_generics, ty_generics, mut where_clause) = generics.split_for_impl();

    let mut extended_where_clause;
    if let Some(derive_where_clause) = attributes.where_clause {
        extended_where_clause = where_clause.cloned().unwrap_or_else(|| syn::WhereClause {
            where_token: derive_where_clause.where_token,
            predicates: syn::punctuated::Punctuated::new(),
        });
        for predicate in derive_where_clause.predicates.iter() {
            extended_where_clause.predicates.push(predicate.clone());
        }
        where_clause = Some(&extended_where_clause);
    }

    match input.data {
        syn::Data::Struct(data) => {
            if attributes.tags.is_some() {
                return Err(syn::Error::new_spanned(
                    attributes.tags.unwrap().tags,
                    "Tags may be specified only for enums",
                ));
            }

            let pattern = match data.fields {
                syn::Fields::Unit => quote::quote!(Self),
                syn::Fields::Unnamed(ref fields) => {
                    let fields = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(idx, _)| quote::format_ident!("__{}", idx));
                    quote::quote!(Self ( #(#fields,)* ))
                }
                syn::Fields::Named(ref fields) => {
                    let fields = fields.named.iter().enumerate().map(|(idx, field)| {
                        let binding = quote::format_ident!("__{}", idx);
                        let ident = field.ident.as_ref().unwrap();
                        quote::quote!(#ident: #binding)
                    });
                    quote::quote!(Self { #(#fields,)* })
                }
            };

            let all_fields_probe: Vec<_> = data
                .fields
                .iter()
                .enumerate()
                .filter_map(|(idx, field)| field_probe(idx, field).transpose())
                .collect::<syn::Result<_>>()?;

            let mut r#impl = if attributes.transparent.is_some() {
                if all_fields_probe.len() != 1 {
                    return Err(syn::Error::new_spanned(
                        attributes.transparent.unwrap(),
                        "Transparent struct must have exactly one non-skipped field",
                    ));
                }

                let field_probe = &all_fields_probe[0];

                quote::quote! {
                    impl #impl_generics ::egui_probe::EguiProbe for #ident #ty_generics
                    #where_clause
                    {
                        fn probe(&mut self, ui: &mut ::egui_probe::egui::Ui, style: &::egui_probe::Style) -> ::egui_probe::egui::Response {
                            use ::egui_probe::private::*;

                            let #pattern = self;

                            ::egui_probe::EguiProbe::probe(#field_probe, ui, style)
                        }

                        fn iterate_inner(&mut self, ui: &mut ::egui_probe::egui::Ui, f: &mut dyn FnMut(&str, &mut ::egui_probe::egui::Ui, &mut dyn ::egui_probe::EguiProbe)) {
                            use ::egui_probe::private::*;

                            let #pattern = self;

                            ::egui_probe::EguiProbe::iterate_inner(#field_probe, ui, f)
                        }
                    }
                }
            } else {
                let fields_name: Vec<_> = data
                    .fields
                    .iter()
                    .filter_map(|field| field_name(field).transpose())
                    .collect::<syn::Result<_>>()?;

                quote::quote! {
                    impl #impl_generics ::egui_probe::EguiProbe for #ident #ty_generics
                    #where_clause
                    {
                        fn probe(&mut self, _ui: &mut ::egui_probe::egui::Ui, _style: &::egui_probe::Style) -> ::egui_probe::egui::Response {
                            _ui.weak(#type_name)
                        }

                        fn iterate_inner(&mut self, _ui: &mut ::egui_probe::egui::Ui, _f: &mut dyn FnMut(&str, &mut ::egui_probe::egui::Ui, &mut dyn ::egui_probe::EguiProbe)) {
                            use ::egui_probe::private::*;

                            let #pattern = self;

                            #(
                                _f(::core::convert::AsRef::<str>::as_ref(&(#fields_name)), _ui, #all_fields_probe);
                            )*
                        }
                    }
                }
            };

            if attributes.new.is_some() {
                let new_impl = {
                    let new_fields = data
                        .fields
                        .iter()
                        .map(field_default)
                        .collect::<syn::Result<Vec<_>>>()?;
                    let new_self = match &data.fields {
                        syn::Fields::Named(_) => quote::quote!( Self { #(#new_fields),* } ),
                        syn::Fields::Unnamed(_) => quote::quote!( Self ( #(#new_fields),* ) ),
                        syn::Fields::Unit => quote::quote!(Self),
                    };
                    quote::quote! {
                        impl ::egui_probe::New for #ident #ty_generics #where_clause {
                            fn new() -> Self { #new_self }
                        }
                    }
                };
                r#impl = quote::quote! {
                    #r#impl
                    #new_impl
                };
            }
            Ok(r#impl)
        }
        syn::Data::Enum(data) => {
            if let Some(transparent) = attributes.transparent {
                return Err(syn::Error::new_spanned(
                    transparent,
                    "Transparent may be specified only for structs or enum variants with exactly one non-skipped field",
                ));
            }

            let variants_selected = data
                .variants
                .iter()
                .map(|variant| variant_selected(variant))
                .collect::<syn::Result<Vec<_>>>()?;

            let variants_probe = data
                .variants
                .iter()
                .map(|variant| variant_probe(variant))
                .collect::<syn::Result<Vec<_>>>()?;

            let variants_inline_probe = data
                .variants
                .iter()
                .map(variant_inline_probe)
                .collect::<syn::Result<Vec<_>>>()?;

            let variants_iterate_inner = data
                .variants
                .iter()
                .map(|variant| variant_iterate_inner(variant))
                .collect::<syn::Result<Vec<_>>>()?;

            let variants_style = match attributes.tags {
                None => quote::quote!(_style.variants),
                Some(EnumTags {
                    kind: TagsKind::Inlined(_),
                    ..
                }) => quote::quote!(::egui_probe::VariantsStyle::Inlined),
                Some(EnumTags {
                    kind: TagsKind::ComboBox(_),
                    ..
                }) => quote::quote!(::egui_probe::VariantsStyle::ComboBox),
            };

            let mut r#impl = quote::quote! {
                impl #impl_generics ::egui_probe::EguiProbe for #ident #ty_generics
                    #where_clause
                    {
                        fn probe(&mut self, ui: &mut ::egui_probe::egui::Ui, _style: &::egui_probe::Style) -> ::egui_probe::egui::Response {
                            use ::egui_probe::private::*;

                            ui.horizontal(|_ui| {
                                match #variants_style {
                                    ::egui_probe::VariantsStyle::Inlined => {
                                        let _in_cbox = false;
                                        #(
                                            #variants_probe
                                        )*
                                    }
                                    ::egui_probe::VariantsStyle::ComboBox => {
                                        let selected_variant = match self { #(#variants_selected,)* };
                                        let cbox = ::egui_probe::egui::ComboBox::from_id_salt(_ui.make_persistent_id("cbox")).selected_text(selected_variant);
                                        let _in_cbox = true;
                                        cbox.show_ui(_ui, |_ui| {
                                            #(
                                                #variants_probe;
                                            )*
                                        });
                                    }
                                }

                                match self {#(
                                    #variants_inline_probe
                                )*}
                            }).response
                        }

                        fn iterate_inner(&mut self, _ui: &mut egui_probe::egui::Ui, _f: &mut dyn FnMut(&str, &mut egui_probe::egui::Ui, &mut dyn ::egui_probe::EguiProbe)) {
                            use ::egui_probe::private::*;

                            match self {#(
                                #variants_iterate_inner
                            )*}
                        }
                    }
            };

            // let variant_new = data
            //     .variants
            //     .iter()
            //     .filter_map(|variant| variant_new(variant).transpose())
            //     .collect::<syn::Result<Vec<_>>>()?;
            // if variant_new.len() != 1 {
            //     return Err(syn::Error::new_spanned(
            //         attributes.new.unwrap(),
            //         "Enum must only one new field",
            //     ));
            // }
            if attributes.new.is_some() {
                let mut new_impl = quote::quote!();
                for variant in &data.variants {
                    let attributes: VariantAttributes =
                        proc_easy::EasyAttributes::parse(&variant.attrs, variant.span())?;
                    if attributes.new.is_some() {
                        let variant_ident = &variant.ident;
                        let new_fields = variant
                            .fields
                            .iter()
                            .map(field_default)
                            .collect::<syn::Result<Vec<_>>>()?;
                        let new_self = match &variant.fields {
                            syn::Fields::Named(_) => {
                                quote::quote!(Self::#variant_ident { #(#new_fields),* } )
                            }
                            syn::Fields::Unnamed(_) => {
                                quote::quote!(Self::#variant_ident ( #(#new_fields),* ))
                            }
                            syn::Fields::Unit => quote::quote!(Self),
                        };
                        new_impl = quote::quote! {
                            impl ::egui_probe::New for #ident #ty_generics #where_clause {
                                fn new() -> Self { #new_self }
                            }
                        };
                        // let construct = match &variant.fields {
                        //     syn::Fields::Unit => quote::quote!(Self::#variant_ident),
                        //     syn::Fields::Unnamed(fields) => {
                        //         let defaults = fields
                        //             .unnamed
                        //             .iter()
                        //             .map(|_| quote::quote!(::core::default::Default::default()));
                        //         quote::quote!(Self::#variant_ident(#(#defaults),*))
                        //     }
                        //     syn::Fields::Named(fields) => {
                        //         let defaults = fields.named.iter().map(|f| {
                        //             let f_ident = f.ident.as_ref().unwrap();
                        //             quote::quote!(#f_ident: ::core::default::Default::default())
                        //         });
                        //         quote::quote!(Self::#variant_ident { #(#defaults),* })
                        //     }
                        // };
                        // new_impl = quote::quote! {
                        //     impl #impl_generics ::core::default::Default for #ident #ty_generics #where_clause {
                        //         fn default() -> Self { #construct }
                        //     }
                        // };
                        break;
                    }
                }
                r#impl = quote::quote! {
                    #r#impl
                    #new_impl
                };
            }
            Ok(r#impl)
        }
        syn::Data::Union(_) => Err(syn::Error::new_spanned(
            input,
            "EguiProbe can only be derived for structs and enums",
        )),
    }
}
