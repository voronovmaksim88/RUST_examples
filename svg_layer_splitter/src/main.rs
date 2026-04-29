//! Читает SVG (Inkscape) и выводит имена слоёв только верхнего уровня
//! (прямые потомки `<svg>`, элементы `<g inkscape:groupmode="layer">`).

use regex::Regex;
use roxmltree::Document;
use std::collections::HashSet;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const INKSCAPE_NS: &str = "http://www.inkscape.org/namespaces/inkscape";
const PREFIX_VSYO: &str = "всё";

#[derive(Clone, Debug)]
struct VersionedSvg {
    path: PathBuf,
    after_v: u32,
    after_r: u32,
}

fn scan_search_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(d) = std::env::current_dir() {
        dirs.push(d);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            dirs.push(parent.to_path_buf());
        }
    }
    dirs.sort();
    dirs.dedup();
    if dirs.is_empty() {
        dirs.push(PathBuf::from("."));
    }
    dirs
}

fn dedup_key(p: &Path) -> PathBuf {
    p.canonicalize().unwrap_or_else(|_| p.to_path_buf())
}

/// Сканируем каталоги: все `всё*.svg` и распознанный шаблон `всё_вNрM.svg`.
fn collect_files(
    re: &Regex,
) -> (Vec<PathBuf>, Vec<VersionedSvg>) {
    let mut dedup_seen: HashSet<PathBuf> = HashSet::new();

    let mut all_prefixed: Vec<PathBuf> = Vec::new();
    let mut structured: Vec<VersionedSvg> = Vec::new();

    for dir in scan_search_dirs() {
        let rd = match std::fs::read_dir(&dir) {
            Ok(rd) => rd,
            Err(_) => continue,
        };

        for ent in rd.filter_map(Result::ok) {
            let path = ent.path();
            if !path.is_file() {
                continue;
            }
            let name = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n,
                None => continue,
            };
            if !name.starts_with(PREFIX_VSYO) || !name.ends_with(".svg") {
                continue;
            }

            let key = dedup_key(&path);
            if !dedup_seen.insert(key.clone()) {
                continue;
            }

            all_prefixed.push(path.clone());

            if let Some(cap) = re.captures(name) {
                let after_v = cap.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
                let after_r = cap.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
                structured.push(VersionedSvg {
                    path,
                    after_v,
                    after_r,
                });
            }
        }
    }

    all_prefixed.sort_by_key(|p| file_name_utf8(p));
    structured.sort_by_key(|p| file_name_utf8(&p.path));
    (all_prefixed, structured)
}

fn file_name_utf8(p: &Path) -> String {
    p.file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "?".into())
}

fn choose_target(vs: &[VersionedSvg]) -> Option<&VersionedSvg> {
    if vs.is_empty() {
        return None;
    }
    let max_after_v = vs.iter().map(|x| x.after_v).max()?;
    let mut top: Vec<_> = vs.iter().filter(|x| x.after_v == max_after_v).collect();
    let max_after_r = top.iter().map(|x| x.after_r).max()?;
    top.retain(|x| x.after_r == max_after_r);
    top.sort_by_key(|x| x.path.display().to_string());
    top.into_iter().next()
}

fn list_layers(xml: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let doc = Document::parse(xml)?;

    let svg = doc
        .descendants()
        .find(|n| n.tag_name().name() == "svg")
        .ok_or("в документе нет корневого элемента <svg>")?;

    let mut layers = Vec::new();

    for child in svg.children().filter(|n| n.is_element()) {
        if child.tag_name().name() != "g" {
            continue;
        }
        let is_layer = child
            .attribute((INKSCAPE_NS, "groupmode"))
            .map(|v| v == "layer")
            .unwrap_or(false);
        if !is_layer {
            continue;
        }
        let name = child
            .attribute((INKSCAPE_NS, "label"))
            .map(str::to_string)
            .unwrap_or_else(|| "<без имени>".into());
        layers.push(name);
    }

    Ok(layers)
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"^всё_в([0-9]+)р([0-9]+)\.svg$")?;

    let (all_prefixed, structured) = collect_files(&re);

    println!("Файлы, имя которых начинается с «{PREFIX_VSYO}»:");
    if all_prefixed.is_empty() {
        println!("  (нет подходящих .svg файлов)");
    } else {
        for p in &all_prefixed {
            println!("  {}", file_name_utf8(p));
        }
    }
    println!();

    if structured.is_empty() {
        return Err(
            "нет файлов с именем по шаблону «всё_в<число>р<число>.svg» (в текущей папке или рядом с exe)."
                .into(),
        );
    }

    let max_after_v = structured.iter().map(|x| x.after_v).max().unwrap_or(0);
    let after_max_v: Vec<_> = structured
        .iter()
        .filter(|x| x.after_v == max_after_v)
        .collect();

    println!("Подмножество с наибольшим индексом после «в» (= {max_after_v}):");
    for x in after_max_v {
        println!(
            "  {}  (в{}р{})",
            file_name_utf8(&x.path),
            x.after_v,
            x.after_r,
        );
    }

    let Some(chosen) = choose_target(&structured) else {
        return Err("не удалось выбрать файл.".into());
    };

    println!();
    println!(
        "С наибольшим индексом после «р» среди них: {}",
        file_name_utf8(&chosen.path)
    );
    println!();
    println!("--- Скрипт обрабатывает файл: {} ---", chosen.path.display());

    let xml = std::fs::read_to_string(&chosen.path)?;
    let layers = list_layers(&xml)?;

    println!("Слои (только первый уровень):");
    for (i, name) in layers.iter().enumerate() {
        println!("{:4}. {}", i + 1, name);
    }
    println!();
    println!("Всего слоёв: {}", layers.len());

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Ошибка: {e}");
        }
    }
    io::stdout().flush().ok();
    println!("\nНажмите Enter для выхода...");
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
}
