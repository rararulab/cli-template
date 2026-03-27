use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, Attribute, Meta, Expr, ExprLit, Lit};

/// Derive `AgentDescribe` for a Clap `Subcommand` enum.
///
/// Generates `fn agent_schema() -> serde_json::Value` that introspects
/// enum variants and produces an agent-cli/1 protocol schema.
#[proc_macro_derive(AgentDescribe, attributes(agent))]
pub fn derive_agent_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let cli_type = extract_cli_type(&input.attrs)
        .expect("#[agent(cli = CliType)] is required on the enum");

    let data = match &input.data {
        Data::Enum(data) => data,
        _ => panic!("AgentDescribe can only be derived for enums"),
    };

    let mut command_tokens = Vec::new();

    for variant in &data.variants {
        if has_agent_attr(&variant.attrs, "skip") {
            continue;
        }

        let variant_name = &variant.ident;
        let kebab_name = to_kebab_case(&variant_name.to_string());
        let doc = extract_doc_comment(&variant.attrs);

        // Check if this variant has a #[command(subcommand)] field
        if has_clap_subcommand(&variant.fields) {
            // Subcommand variant: flatten via Clap runtime introspection
            command_tokens.push(quote! {
                {
                    let root_cmd = <#cli_type as ::clap::CommandFactory>::command();
                    if let Some(parent) = root_cmd.get_subcommands()
                        .find(|c| c.get_name() == #kebab_name)
                    {
                        for sub in parent.get_subcommands() {
                            let sub_name = sub.get_name().to_string();
                            let sub_desc = sub.get_about()
                                .map(|s| s.to_string())
                                .unwrap_or_default();
                            let sub_args = ::agent_describe::args_from_clap_command(sub);
                            commands.push(::agent_describe::CommandSchema {
                                name: format!("{} {}", #kebab_name, sub_name),
                                description: sub_desc,
                                args: sub_args,
                                output: None,
                            });
                        }
                    }
                }
            });
        } else {
            // Regular variant: extract fields as args from macro analysis
            let output_type = extract_output_type(&variant.attrs).unwrap_or_else(|| {
                let result_name = syn::Ident::new(
                    &format!("{}Result", variant_name),
                    variant_name.span(),
                );
                quote! { #result_name }
            });

            let mut arg_tokens = Vec::new();

            if let Fields::Named(fields) = &variant.fields {
                for field in &fields.named {
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    let field_doc = extract_doc_comment(&field.attrs);
                    let is_long = has_clap_long(&field.attrs);
                    let is_bool = is_bool_type(&field.ty);
                    let is_option = is_option_type(&field.ty);

                    let display_name = if is_long {
                        let kebab = to_kebab_case(&field_name);
                        format!("--{}", kebab)
                    } else {
                        field_name.clone()
                    };

                    let type_str = if is_bool { "bool" } else { "string" };
                    let required = !is_bool && !is_option;

                    arg_tokens.push(quote! {
                        ::agent_describe::ArgSchema {
                            name: #display_name.to_string(),
                            r#type: #type_str.to_string(),
                            required: #required,
                            description: #field_doc.to_string(),
                            r#enum: None,
                        }
                    });
                }
            }

            command_tokens.push(quote! {
                {
                    let output_schema = {
                        let schema = ::schemars::generate::SchemaSettings::draft2020_12()
                            .into_generator()
                            .into_root_schema_for::<#output_type>();
                        Some(::serde_json::to_value(schema).unwrap())
                    };
                    commands.push(::agent_describe::CommandSchema {
                        name: #kebab_name.to_string(),
                        description: #doc.to_string(),
                        args: vec![#(#arg_tokens),*],
                        output: output_schema,
                    });
                }
            });
        }
    }

    let expanded = quote! {
        impl #enum_name {
            /// Generate the agent-cli/1 protocol schema for this command enum.
            pub fn agent_schema() -> ::serde_json::Value {
                let root_cmd = <#cli_type as ::clap::CommandFactory>::command();
                let name = root_cmd.get_name().to_string();
                let version = root_cmd.get_version()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let description = root_cmd.get_about()
                    .map(|s| s.to_string())
                    .unwrap_or_default();

                let mut commands: Vec<::agent_describe::CommandSchema> = Vec::new();
                #(#command_tokens)*

                let schema = ::agent_describe::AgentSchema {
                    protocol: "agent-cli/1",
                    name,
                    version,
                    description,
                    commands,
                    error_format: ::agent_describe::AgentSchema::default_error_format(),
                };

                ::serde_json::to_value(schema).unwrap()
            }
        }
    };

    TokenStream::from(expanded)
}

/// Collect `#[doc = "..."]` attributes into a single trimmed string.
fn extract_doc_comment(attrs: &[Attribute]) -> String {
    let lines: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if !attr.path().is_ident("doc") {
                return None;
            }
            if let Meta::NameValue(nv) = &attr.meta
                && let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value
            {
                return Some(s.value().trim().to_string());
            }
            None
        })
        .collect();
    lines.join(" ")
}

/// Convert PascalCase to kebab-case (e.g., "DeployApp" → "deploy-app").
fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                // Only add hyphen if previous char was lowercase or next is lowercase
                result.push('-');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            // Convert underscores to hyphens
            if c == '_' {
                result.push('-');
            } else {
                result.push(c);
            }
        }
    }
    result
}

/// Check whether `#[agent(name)]` is present in attributes.
fn has_agent_attr(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("agent") {
            return false;
        }
        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(name) {
                found = true;
            }
            Ok(())
        });
        found
    })
}

/// Extract `Type` from `#[agent(cli = Type)]`.
fn extract_cli_type(attrs: &[Attribute]) -> Option<proc_macro2::TokenStream> {
    for attr in attrs {
        if !attr.path().is_ident("agent") {
            continue;
        }
        let mut result = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("cli") {
                let value = meta.value()?;
                let ty: Type = value.parse()?;
                result = Some(quote! { #ty });
            }
            Ok(())
        });
        if result.is_some() {
            return result;
        }
    }
    None
}

/// Extract `Type` from `#[agent(output = Type)]`.
fn extract_output_type(attrs: &[Attribute]) -> Option<proc_macro2::TokenStream> {
    for attr in attrs {
        if !attr.path().is_ident("agent") {
            continue;
        }
        let mut result = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("output") {
                let value = meta.value()?;
                let ty: Type = value.parse()?;
                result = Some(quote! { #ty });
            }
            Ok(())
        });
        if result.is_some() {
            return result;
        }
    }
    None
}

/// Check whether any field attribute has `#[arg(long)]`.
fn has_clap_long(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("arg") {
            return false;
        }
        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("long") {
                found = true;
            }
            Ok(())
        });
        found
    })
}

/// Check if a type is `bool`.
fn is_bool_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty
        && let Some(seg) = tp.path.segments.last()
    {
        return seg.ident == "bool";
    }
    false
}

/// Check if a type is `Option<T>`.
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty
        && let Some(seg) = tp.path.segments.last()
    {
        return seg.ident == "Option";
    }
    false
}

/// Check if any field in the variant has `#[command(subcommand)]`.
fn has_clap_subcommand(fields: &Fields) -> bool {
    match fields {
        Fields::Named(named) => named.named.iter().any(|f| {
            f.attrs.iter().any(|attr| {
                if !attr.path().is_ident("command") {
                    return false;
                }
                let mut found = false;
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("subcommand") {
                        found = true;
                    }
                    Ok(())
                });
                found
            })
        }),
        Fields::Unnamed(unnamed) => unnamed.unnamed.iter().any(|f| {
            f.attrs.iter().any(|attr| {
                if !attr.path().is_ident("command") {
                    return false;
                }
                let mut found = false;
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("subcommand") {
                        found = true;
                    }
                    Ok(())
                });
                found
            })
        }),
        Fields::Unit => false,
    }
}
