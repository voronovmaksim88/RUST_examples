//! Выбирает актуальный `всё_вNрM.svg`, режет верхний уровень слоёв Inkscape
//! в отдельные файлы `{имя_слоя}.svg` рядом с исходником.

use regex::Regex;
use roxmltree::Document;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

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

fn collect_files(re: &Regex) -> (Vec<PathBuf>, Vec<VersionedSvg>) {
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

/// Конец открывающего тега: позиция закрывающего `>` (вне строк в кавычках атрибутах).
fn end_of_opening_tag(xml: &str, open_lt: usize) -> Option<usize> {
    let b = xml.as_bytes();
    if open_lt >= b.len() || b[open_lt] != b'<' {
        return None;
    }
    let mut quote: Option<u8> = None;
    let mut i = open_lt + 1;
    while i < b.len() {
        let c = b[i];
        match quote {
            None => match c {
                b'>' => return Some(i),
                b'"' | b'\'' => quote = Some(c),
                _ => {}
            },
            Some(q) => {
                if c == q {
                    quote = None;
                }
            }
        }
        i += 1;
    }
    None
}

fn sanitize_for_filename(layer_name: &str) -> String {
    let mut s = layer_name.trim().to_string();
    if s.is_empty() {
        return "_layer_".into();
    }
    for ch in ['<', '>', ':', '"', '/', '\\', '|', '?', '*'] {
        s = s.replace(ch, "_");
    }
    s = s.trim_end_matches('.').trim().to_string();
    if s.is_empty() {
        "_layer_".into()
    } else {
        s
    }
}

/// Имя файла без `.svg`; при одинаковых имёнах слоёв: `Имя_2`, `Имя_3`, …
fn unique_export_stem(label: &str, per_base: &mut HashMap<String, u32>) -> String {
    let base = sanitize_for_filename(label);
    let cnt = per_base.entry(base.clone()).or_insert(0);
    *cnt += 1;
    if *cnt == 1 {
        base
    } else {
        format!("{}_{}", base, cnt)
    }
}

fn style_fragment_drop_for_layer_visibility(frag: &str) -> bool {
    let name_val: Vec<&str> = frag.trim().splitn(2, ':').collect();
    if name_val.len() != 2 {
        return false;
    }
    let name = name_val[0].trim().to_lowercase();
    let val = name_val[1].trim();
    match name.as_str() {
        "display" | "visibility" => true,
        "opacity" => val.parse::<f64>().map(|x| x == 0.0).unwrap_or(false),
        _ => false,
    }
}

/// Оставляет прочие декларации, в конце задаёт видимость корневой группы слоя.
fn merge_style_for_visible_layer(style: &str) -> String {
    let mut kept: Vec<String> = Vec::new();
    for part in style.split(';') {
        if part.trim().is_empty() {
            continue;
        }
        if style_fragment_drop_for_layer_visibility(part) {
            continue;
        }
        kept.push(part.trim().to_owned());
    }
    let mut out = kept.join("; ");
    if !out.is_empty() {
        out.push_str("; ");
    }
    out.push_str("display:inline; visibility:visible; opacity:1");
    out
}

fn xml_escape_double_quoted_attr_value(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}

static RE_DISPLAY_NONE_ATTR: OnceLock<Regex> = OnceLock::new();
static RE_STYLE_DQUOTE: OnceLock<Regex> = OnceLock::new();
static RE_STYLE_SQUOTE: OnceLock<Regex> = OnceLock::new();

/// Исходный SVG не меняется; правим только копию открывающего тега `<g>` в выходном файле.
fn patch_layer_root_g_open_tag_for_visibility(open_tag: &str) -> String {
    let re_attr =
        RE_DISPLAY_NONE_ATTR.get_or_init(|| Regex::new(r#"(?i)\sdisplay\s*=\s*["']none["']"#).unwrap());
    let s = re_attr.replace_all(open_tag, "").into_owned();

    let re_ds = RE_STYLE_DQUOTE.get_or_init(|| Regex::new(r#"(?i)(\sstyle\s*=\s*)("[^"]*")"#).unwrap());
    if let Some(cap) = re_ds.captures(&s) {
        let m = cap.get(0).expect("whole match");
        let inner_quote = cap.get(2).expect("quoted value").as_str();
        let inner = inner_quote
            .strip_prefix('"')
            .and_then(|x| x.strip_suffix('"'))
            .unwrap_or(inner_quote);
        let merged = merge_style_for_visible_layer(inner);
        let replacement = format!(
            "{}\"{}\"",
            cap.get(1).unwrap().as_str(),
            xml_escape_double_quoted_attr_value(&merged),
        );
        let mut out = String::with_capacity(s.len());
        out.push_str(&s[..m.start()]);
        out.push_str(&replacement);
        out.push_str(&s[m.end()..]);
        return out;
    }

    let re_ss = RE_STYLE_SQUOTE.get_or_init(|| Regex::new(r"(?i)(\sstyle\s*=\s*)('[^']*')").unwrap());
    if let Some(cap) = re_ss.captures(&s) {
        let m = cap.get(0).expect("whole match");
        let inner_quote = cap.get(2).expect("quoted").as_str();
        let inner = inner_quote
            .strip_prefix('\'')
            .and_then(|x| x.strip_suffix('\''))
            .unwrap_or(inner_quote);
        let merged = merge_style_for_visible_layer(inner);
        let replacement = format!(
            " style=\"{}\"",
            xml_escape_double_quoted_attr_value(&merged),
        );
        let mut out = String::with_capacity(s.len());
        out.push_str(&s[..m.start()]);
        out.push_str(&replacement);
        out.push_str(&s[m.end()..]);
        return out;
    }

    let inject = r#" style="display:inline; visibility:visible; opacity:1""#;
    if let Some(pos) = s.rfind('>') {
        let mut out = String::with_capacity(s.len() + inject.len());
        out.push_str(&s[..pos]);
        out.push_str(inject);
        out.push_str(&s[pos..]);
        out
    } else {
        s
    }
}

fn build_layer_document(
    xml: &str,
    svg: roxmltree::Node<'_, '_>,
    layer: roxmltree::Node<'_, '_>,
) -> Result<String, Box<dyn std::error::Error>> {
    let open_start = svg.range().start;
    let open_end_gt = end_of_opening_tag(xml, open_start)
        .ok_or("не удалось определить конец открывающего тега <svg>")?;

    let open_svg = &xml[open_start..=open_end_gt];

    let layer_start = layer.range().start;
    let layer_open_end = end_of_opening_tag(xml, layer_start)
        .ok_or("не удалось определить конец открывающего тега группы слоя <g>")?;
    let open_g = patch_layer_root_g_open_tag_for_visibility(&xml[layer_start..=layer_open_end]);
    let layer_tail = &xml[(layer_open_end + 1)..layer.range().end];

    Ok(format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n\
{open_svg}\n\
{open_g}{layer_tail}\n\
</svg>\n"
    ))
}

#[derive(Clone, Debug)]
struct CreatedFile {
    file_name: String,
    layer_label: String,
    renamed_duplicates: bool,
}

fn extract_and_write_layers(src_path: &Path, xml: &str) -> Result<Vec<CreatedFile>, Box<dyn std::error::Error>> {
    let doc = Document::parse(xml)?;

    let svg = doc
        .descendants()
        .find(|n| n.tag_name().name() == "svg")
        .ok_or("в документе нет корневого элемента <svg>")?;

    let out_dir = src_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut per_base = HashMap::new();
    let mut created = Vec::new();

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

        let label = child
            .attribute((INKSCAPE_NS, "label"))
            .unwrap_or("<без имени>")
            .to_string();

        let sane = sanitize_for_filename(&label);
        let stem = unique_export_stem(&label, &mut per_base);

        let out_path = out_dir.join(format!("{stem}.svg"));
        let content = build_layer_document(xml, svg, child)?;
        std::fs::write(&out_path, content)?;

        let renamed_duplicates = sane != stem;

        created.push(CreatedFile {
            file_name: file_name_utf8(&out_path),
            layer_label: label,
            renamed_duplicates,
        });
    }

    Ok(created)
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
    let created = extract_and_write_layers(&chosen.path, &xml)?;

    println!("Созданные файлы (слой → файл):");
    for c in &created {
        if c.renamed_duplicates {
            println!(
                "  {} ← слой «{}» (повтор имени файла добавлен счётчик)",
                c.file_name, c.layer_label
            );
        } else {
            println!("  {} ← слой «{}»", c.file_name, c.layer_label);
        }
    }

    println!();
    println!("Всего создано файлов: {}", created.len());

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