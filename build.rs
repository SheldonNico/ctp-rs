use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

// ref: https://www.reddit.com/r/rust/comments/dym88t/rust_and_c_virtual_functions/
fn main() {
    let sdk_path = targeted_env_var("CROSS_CTP_SDK_PATH").expect("`CROSS_CTP_SDK_PATH` not set");
    let sdk_path = Path::new(&sdk_path);
    let target_os = std::env::var("CARGO_CFG_TARGET_OS")
        .expect("`CARGO_CFG_TARGET_OS` unknown? is cargo corrupt?");
    println!("CROSS_CTP_SDK_PATH = {}", sdk_path.display());
    println!("CARGO_CFG_TARGET_OS = {:?}", target_os);

    let fpathes = read_for_items(sdk_path).expect("fail to read files");
    let sdk_td_path = search_for_fname(&fpathes, "ThostFtdcTraderApi.h").unwrap();
    let sdk_md_path = search_for_fname(&fpathes, "ThostFtdcMdApi.h").unwrap();
    let rsfile = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    let cppfile = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.cpp");

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Create a new `Clang` instance
    let clang_ins = clang::Clang::new().unwrap();

    // Create a new `Index`
    let index = clang::Index::new(&clang_ins, false, false);

    let mut fh = std::fs::File::create(&cppfile).expect("fail to create cppfile");
    inherit_spi(
        sdk_td_path.as_ref(),
        &index,
        "CThostFtdcTraderSpi",
        "Rust_Td_",
        &mut fh,
    )
    .expect("fail to forward spi for trader");
    inherit_spi(
        sdk_md_path.as_ref(),
        &index,
        "CThostFtdcMdSpi",
        "Rust_Md_",
        &mut fh,
    )
    .expect("fail to forward spi for trader");
    drop(fh);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    search_and_println_dylib(&fpathes, "thosttraderapi", &target_os)
        .expect("fail to find lib for thosttraderapi");
    search_and_println_dylib(&fpathes, "thostmduserapi", &target_os)
        .expect("fail to find lib for thostmduserapi");

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Build
    cc::Build::new()
        .cpp(true)
        .file(&cppfile)
        .flag_if_supported("-std=c++17") // for gnu
        .flag_if_supported("/std:c++20") // for msvc
        .flag_if_supported("-w")
        .compile("bindings");

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Bindgen
    // ctp api header is clean enough, we will use blacklist instead whitelist
    bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(cppfile.display().to_string())
        .clang_args(["-x", "c++"])
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .derive_debug(true)
        // make output smaller
        .layout_tests(false)
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .rustified_enum("Rust_.*MsgType")
        .allowlist_item("Rust_.*")
        .allowlist_item("CThostFtdcTraderApi")
        .allowlist_item("CThostFtdcMdApi")
        // .allowlist_item("CThostFtdcTraderSpi")
        .vtable_generation(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(&rsfile)
        .expect("Couldn't write bindings!");
}

////////////////////////////////////////////////////////////////////////////////
pub fn read_for_items(dir: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    dir.as_ref()
        .read_dir()?
        .into_iter()
        .map(|entity| {
            entity.and_then(|entity| {
                let pth = entity.path();
                if pth.is_dir() {
                    read_for_items(&pth).map(|mut out| {
                        out.push(pth);
                        out
                    })
                } else {
                    Ok(vec![pth])
                }
            })
        })
        .collect::<io::Result<Vec<Vec<PathBuf>>>>()
        .map(|ps| ps.concat())
}

pub fn search_for_fname(fpathes: &[PathBuf], target: &str) -> Option<PathBuf> {
    let found: Vec<_> = fpathes
        .iter()
        .filter(|p| {
            p.is_file()
                && p.file_name()
                    .map(|fname| fname == target)
                    .unwrap_or_default()
        })
        .collect();
    if found.len() == 1 {
        Some(found[0].to_owned())
    } else {
        None
    }
}

pub fn search_for_dirname(fpathes: &[PathBuf], target: &str) -> Option<PathBuf> {
    let found: Vec<_> = fpathes
        .iter()
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .map(|fname| fname == target)
                    .unwrap_or_default()
        })
        .collect();
    if found.len() == 1 {
        Some(found[0].to_owned())
    } else {
        None
    }
}

// NOTE: build.rs runs on host not on target, so cfg!(target_os = ...) not work
// ref to: https://github.com/cross-rs/cross/issues/1627
pub fn search_and_println_dylib(fpathes: &[PathBuf], target: &str, target_os: &str) -> Option<()> {
    let pth = match target_os {
        "macos" => search_for_dirname(fpathes, &format!("{target}.framework")).or(
            search_for_dirname(fpathes, &format!("{target}_se.framework")),
        ),
        "windows" => search_for_fname(fpathes, &format!("{target}.lib"))
            .or(search_for_fname(fpathes, &format!("{target}_se.lib"))),
        "linux" => search_for_fname(fpathes, &format!("lib{target}.so"))
            .or(search_for_fname(fpathes, &format!("lib{target}_se.so")))
            .or(search_for_fname(fpathes, &format!("{target}.so")))
            .or(search_for_fname(fpathes, &format!("{target}_se.so"))),
        _ => None,
    };
    let pth = pth?;

    println!("cargo:rustc-link-search={}", pth.parent()?.display());
    println!(
        "cargo:rustc-link-lib=dylib:+verbatim={}",
        pth.file_name()?.display()
    );
    Some(())
}

pub fn inherit_spi(
    hfile: &Path,
    index: &clang::Index,
    spi: &str,
    prefix: &str,
    mut output: impl Write,
) -> io::Result<()> {
    let mut s_include: String = format!("#include \"{}\"", std::path::absolute(hfile)?.display());
    let mut s_struct_decl: Vec<String> = vec![];
    let mut s_union_decl: String = "".into();
    let mut s_enum_decl: String = "".into();
    let mut s_typedef_decl: Vec<String> = vec![];
    let mut s_class_decl: String = "".into();
    ////////////////////////////////////////////////////////////////////////////////
    const SPACE: &'static str = "    ";
    let s_union_name = format!("{prefix}MsgBody");
    let s_class_name = format!("{prefix}{spi}");
    let s_enum_name = format!("{prefix}MsgType");

    let s_callback_name = format!("F_On_{s_enum_name}_T");
    s_typedef_decl.push(format!("typedef void (*{s_callback_name})({s_enum_name} msg_type, {s_union_name} msg_body, void* params);"));

    // Parse a source file into a translation unit
    let tu = index
        .parser(hfile)
        .arguments(&["-x", "c++"])
        .parse()
        .map_err(io::Error::other)?;

    let mut union_fields = vec![];
    let mut enum_fields = vec![];
    let mut class_methods_override: Vec<String> = vec![];
    let mut arg_len_to_args: HashMap<usize, (Vec<String>, Vec<String>, String)> =
        Default::default();
    for ety in tu.get_entity().get_children().into_iter() {
        if let Some(name) = ety.get_name() {
            if name == spi {
                assert_eq!(ety.get_kind(), clang::EntityKind::ClassDecl);
                for mtd in ety.get_children() {
                    if mtd.get_kind() != clang::EntityKind::Method {
                        continue;
                    }
                    let Some(func_name) = mtd.get_name() else {
                        continue;
                    };
                    let Some(arguments) = mtd.get_arguments() else {
                        continue;
                    };
                    let mut args_name: Vec<_> =
                        arguments.iter().flat_map(|arg| arg.get_name()).collect();
                    let mut args_type_name: Vec<_> = arguments
                        .iter()
                        .flat_map(|arg| arg.get_type().map(|t| t.get_display_name()))
                        .collect();
                    let return_type_name = mtd.get_result_type().unwrap().get_display_name();
                    let is_first_pointer = arguments.len() > 0
                        && arguments[0]
                            .get_type()
                            .map(|a| a.get_pointee_type().is_some())
                            .unwrap_or_default();

                    enum_fields.push(func_name.clone());
                    if !is_first_pointer {
                        let s_struct_name = format!("{prefix}{func_name}");
                        let s_struct_fields_decl = to_argument_list(&args_type_name, &args_name)
                            .iter()
                            .map(|s| format!("{SPACE}{s};"))
                            .collect::<Vec<_>>()
                            .join("\n");
                        s_struct_decl.push(format!(
                            "struct {s_struct_name} {{\n{s_struct_fields_decl}\n}};"
                        ));

                        let s_union_field_name = camel_to_snake(&func_name);
                        union_fields.push((s_struct_name.clone(), s_union_field_name.clone()));

                        let s_method_arguments_decl =
                            to_argument_list(&args_type_name, &args_name).join(", ");
                        let s_method_arguments = indent(&args_name.join(",\n"), 5);
                        class_methods_override.push(format!(
                            "{return_type_name} {func_name}({s_method_arguments_decl}) override {{ 
    return m_callback(
            {s_enum_name}::{func_name},
            {s_union_name} {{
                .{s_union_field_name} = {s_struct_name} {{
{s_method_arguments}
                }}
            }},
            m_callback_params
    );
}}"
                        ));
                    } else {
                        let arg_len = args_type_name.len();
                        let s_struct_name = format!("{prefix}Arg{arg_len}");
                        let s_struct_fields_decl =
                            vec![(&"void*".to_string(), &"body".to_string())]
                                .into_iter()
                                .chain(args_type_name.iter().zip(args_name.iter()).skip(1))
                                .map(|(t, n)| format!("{SPACE} {t} {n};"))
                                .collect::<Vec<_>>()
                                .join("\n");
                        let s_union_field_name = format!("arg{arg_len}");

                        if let Some((args_type_name_prev, _, return_type_name_prev)) =
                            arg_len_to_args.get(&arg_len)
                        {
                            assert_eq!(
                                args_type_name_prev,
                                &args_type_name.iter().skip(1).cloned().collect::<Vec<_>>(),
                            );
                            assert_eq!(return_type_name_prev, &return_type_name);
                        } else {
                            s_struct_decl.push(format!(
                                "struct {s_struct_name} {{\n{s_struct_fields_decl}\n}};"
                            ));

                            union_fields.push((s_struct_name.clone(), s_union_field_name.clone()));

                            arg_len_to_args.insert(
                                args_name.len(),
                                (
                                    args_type_name.iter().skip(1).cloned().collect(),
                                    args_name.iter().skip(1).cloned().collect(),
                                    return_type_name.clone(),
                                ),
                            );
                        }

                        let s_method_arguments_decl =
                            to_argument_list(&args_type_name, &args_name).join(", ");
                        let s_method_arguments = indent(&args_name.join(",\n"), 5);
                        class_methods_override.push(format!(
                            "{return_type_name} {func_name}({s_method_arguments_decl}) override {{ 
    return m_callback(
            {s_enum_name}::{func_name},
            {s_union_name} {{
                .{s_union_field_name} = {s_struct_name} {{
{s_method_arguments}
                }}
            }},
            m_callback_params
    );
}}"
                        ));
                    }
                }
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    let class_fields = vec![
        (s_callback_name.clone(), "m_callback".to_string()),
        ("void *".to_string(), "m_callback_params".to_string()),
    ];
    let s_union_decl = format!(
        "union {s_union_name} {{\n{}\n}};",
        union_fields
            .iter()
            .map(|(t, n)| format!("{SPACE} {t} {n};"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let s_enum_decl = format!(
        "enum class {s_enum_name} {{\n{}\n}};",
        indent(
            &enum_fields
                .iter()
                .map(|e| format!("{e},"))
                .collect::<Vec<_>>()
                .join("\n"),
            1
        )
    );
    let s_class_arguments_list = class_fields
        .iter()
        .map(|(t, n)| format!("{t} {}", n.strip_prefix("m_").unwrap()))
        .collect::<Vec<_>>()
        .join(", ");
    let s_class_arguments_list_init = class_fields
        .iter()
        .map(|(t, n)| format!("{}({})", n, n.strip_prefix("m_").unwrap()))
        .collect::<Vec<_>>()
        .join(", ");
    let s_class_constructor_sig = format!("{s_class_name}({s_class_arguments_list});");
    let s_class_destructor_sig = format!("~{s_class_name}();");
    let s_class_constructor_decl = format!(
        "{s_class_name}::{s_class_name}({s_class_arguments_list}): {s_class_arguments_list_init} {{}}"
    );
    let s_class_destructor_decl = format!("{s_class_name}::~{s_class_name}() {{}}");

    let s_class_decl = format!(
        "class {s_class_name} : {spi} {{
public:
    {s_class_constructor_sig}
    {s_class_destructor_sig}
{}
{}
}};",
        indent(
            &class_fields
                .iter()
                .map(|(t, n)| format!("{t} {n};"))
                .collect::<Vec<_>>()
                .join("\n"),
            1
        ),
        indent(&class_methods_override.join("\n"), 1),
    );

    output.write_all(
        vec![
            s_include,
            s_struct_decl.join("\n"),
            s_union_decl,
            s_enum_decl,
            s_typedef_decl.join("\n"),
            s_class_decl,
            s_class_constructor_decl,
            s_class_destructor_decl,
            // "int main() {{}}".to_string(),
        ]
        .join("\n\n")
        .as_bytes(),
    )?;
    writeln!(output, "")?;

    Ok(())
}

fn to_argument_list(tylist: &[String], nmlist: &[String]) -> Vec<String> {
    tylist
        .iter()
        .zip(nmlist.iter())
        .map(|(t, n)| format!("{t} {n}"))
        .collect()
}

fn camel_to_snake(name: &str) -> String {
    lazy_static! {
        static ref PATTERN1: Regex = Regex::new(r"(.)([A-Z][a-z]+)").unwrap();
        static ref PATTERN2: Regex = Regex::new(r"([a-z0-9])([A-Z])").unwrap();
    }
    PATTERN2
        .replace_all(
            PATTERN1.replace_all(name, r"${1}_${2}").as_ref(),
            r"${1}_${2}",
        )
        .to_lowercase()
}

fn indent(ori: &str, size: usize) -> String {
    let space = vec!["    "; size].join("");
    ori.lines()
        .map(|line| format!("{space}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

// ref to: https://github.com/rust-lang/pkg-config-rs/blob/057321c21329ead3ec7d502a8e730a5fd7a271e9/src/lib.rs#L638
fn targeted_env_var(var_base: &str) -> Option<std::ffi::OsString> {
    match (env::var("TARGET"), env::var("HOST")) {
        (Ok(target), Ok(host)) => {
            let kind = if host == target { "HOST" } else { "TARGET" };
            let target_u = target.replace('-', "_");

            env_var_os(&format!("{}_{}", var_base, target))
                .or_else(|| env_var_os(&format!("{}_{}", var_base, target_u)))
                .or_else(|| env_var_os(&format!("{}_{}", kind, var_base)))
                .or_else(|| env_var_os(var_base))
        }
        (Err(env::VarError::NotPresent), _) | (_, Err(env::VarError::NotPresent)) => {
            env_var_os(var_base)
        }
        (Err(env::VarError::NotUnicode(s)), _) | (_, Err(env::VarError::NotUnicode(s))) => {
            panic!(
                "HOST or TARGET environment variable is not valid unicode: {:?}",
                s
            )
        }
    }
}

fn env_var_os(name: &str) -> Option<std::ffi::OsString> {
    println!("cargo:rerun-if-env-changed={}", name);
    std::env::var_os(name)
}
