use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    generate_taxonomy_aliases();
    generate_embedded_locales();
}

fn generate_taxonomy_aliases() {
    const TAXONOMY_PATH: &str = "data/contracts/lens-taxonomy-v1.json";
    println!("cargo::rerun-if-changed={TAXONOMY_PATH}");

    let source = std::fs::read_to_string(TAXONOMY_PATH).expect("read wallpaper taxonomy");
    let taxonomy: serde_json::Value =
        serde_json::from_str(&source).expect("parse wallpaper taxonomy");
    assert_eq!(taxonomy["format"], 1);
    assert_eq!(taxonomy["id"], "skwd-wallpaper");
    assert_eq!(taxonomy["version"], "taxonomy-v1");
    let categories = taxonomy["categories"].as_object().expect("taxonomy categories");
    let mut aliases = BTreeMap::new();
    for concepts in categories.values() {
        for concept in concepts.as_array().expect("taxonomy category concepts") {
            let tag = concept["tag"].as_str().expect("taxonomy concept tag");
            let Some(concept_aliases) = concept["aliases"].as_array() else {
                continue;
            };
            for alias in concept_aliases {
                let normalized =
                    alias.as_str().expect("taxonomy alias").to_lowercase().replace([' ', '_'], "-");
                if let Some(existing) = aliases.insert(normalized.clone(), tag.to_string()) {
                    assert_eq!(existing, tag, "alias {normalized:?}");
                }
            }
        }
    }

    let mut generated = String::from("const TAXONOMY_ALIASES: &[(&str, &str)] = &[\n");
    for (alias, tag) in aliases {
        writeln!(generated, "    ({alias:?}, {tag:?}),").expect("write generated alias");
    }
    generated.push_str("];\n");

    let output = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR"))
        .join("wallpaper_taxonomy_aliases.rs");
    std::fs::write(output, generated).expect("write wallpaper taxonomy aliases");
}

const LOCALE_CONSTANTS: &[(&str, &str)] = &[
    ("en-US", "EN_US_RESOURCES"),
    ("sv-SE", "SV_SE_RESOURCES"),
    ("es-ES", "ES_ES_RESOURCES"),
    ("pt-BR", "PT_BR_RESOURCES"),
    ("ru-RU", "RU_RU_RESOURCES"),
    ("zh-CN", "ZH_CN_RESOURCES"),
    ("ja-JP", "JA_JP_RESOURCES"),
    ("ar-SA", "AR_SA_RESOURCES"),
    ("fr-FR", "FR_FR_RESOURCES"),
    ("bn-BD", "BN_BD_RESOURCES"),
    ("ur-PK", "UR_PK_RESOURCES"),
    ("hi-IN", "HI_IN_RESOURCES"),
    ("fa-IR", "FA_IR_RESOURCES"),
    ("tr-TR", "TR_TR_RESOURCES"),
];

fn generate_embedded_locales() {
    println!("cargo:rerun-if-changed=locales");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo must provide OUT_DIR"));
    let generated = out_dir.join("embedded_locales.rs");
    let locales = Path::new("locales");

    let mut source = String::new();
    for (locale, constant) in LOCALE_CONSTANTS {
        let files = fluent_files(&locales.join(locale))
            .unwrap_or_else(|error| panic!("failed to scan {locale} Fluent files: {error}"));
        assert!(!files.is_empty(), "no Fluent files for {locale}");
        writeln!(source, "pub const {constant}: &[&str] = &{};", include_array(&files))
            .expect("write generated locale list");
    }
    fs::write(generated, source).expect("failed to generate embedded Fluent resource list");
}

fn fluent_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    fn visit(directory: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
        if !directory.exists() {
            return Ok(());
        }
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path = entry.path();
            if file_type.is_dir() {
                visit(&path, files)?;
            } else if file_type.is_file()
                && path.extension().is_some_and(|extension| extension == "ftl")
            {
                files.push(path);
            }
        }
        Ok(())
    }

    let mut files = Vec::new();
    visit(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn include_array(files: &[PathBuf]) -> String {
    let entries = files
        .iter()
        .map(|path| {
            let relative = path.to_string_lossy().replace('\\', "/");
            format!("include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{relative}\"))")
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{entries}]")
}
