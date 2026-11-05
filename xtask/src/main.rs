use anyhow::{bail, Context, Result};
use goblin::elf::{program_header::PT_LOAD, Elf};
use std::collections::BTreeMap;
use serde::Deserialize;
use std::{fs, path::{Path, PathBuf}, process::Command};

const PREFIX: &str = "tracer";

#[derive(Deserialize)]
struct Index { categories: Vec<String>, component: Vec<Entry> }

#[derive(Deserialize)]
struct Entry { id: String, category: String, enabled: bool, data: PathBuf }

#[derive(Deserialize, Clone)]
struct Data {
    #[serde(rename = "crate")]
    crate_name: Option<String>,
    target: Option<String>,
    feature: Option<String>,
    uf2_family: Option<u32>,
    #[serde(default)]
    incompatible_with: Vec<String>,
}

#[derive(Clone)]
struct Comp { id: String, category: String, data: Data }

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}

fn load_combos() -> Result<Vec<Vec<Comp>>> {
    let index: Index = toml::from_str(&fs::read_to_string(root().join("combos.toml"))?)?;
    let mut groups: Vec<Vec<Comp>> = Vec::new();
    for cat in &index.categories {
        let mut group = Vec::new();
        for e in index.component.iter().filter(|e| &e.category == cat && e.enabled) {
            let path = root().join(&e.data);
            let data: Data = toml::from_str(&fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?)?;
            group.push(Comp { id: e.id.clone(), category: cat.clone(), data });
        }
        if group.is_empty() { bail!("no enabled components in category '{cat}'"); }
        groups.push(group);
    }
    let all = groups.iter().fold(vec![vec![]], |acc: Vec<Vec<Comp>>, g| {
        acc.iter().flat_map(|c| g.iter().map(move |x| {
            let mut n = c.clone(); n.push(x.clone()); n
        })).collect()
    });
    // an incompatibility declared on either side excludes the pair
    Ok(all.into_iter().filter(|c| {
        !c.iter().any(|a| c.iter().any(|b| a.data.incompatible_with.contains(&b.id)))
    }).collect())
}

fn ids(c: &[Comp]) -> Vec<&str> { c.iter().map(|x| x.id.as_str()).collect() }

fn build(combo: &[Comp]) -> Result<()> {
    let cpu = combo.iter().find(|c| c.category == "processor").context("no processor")?;
    let krate = cpu.data.crate_name.as_deref().context("processor needs `crate`")?;
    let target = cpu.data.target.as_deref().context("processor needs `target`")?;
    let features: Vec<&str> = combo.iter().filter_map(|c| c.data.feature.as_deref()).collect();

    let status = Command::new("cargo")
        .current_dir(root())
        .args(["build", "--release", "-p", krate, "--target", target,
               "--no-default-features", "--features", &features.join(",")])
        .status()?;
    if !status.success() { bail!("build failed for {}", ids(combo).join("-")); }

    let out = root().join("dist");
    fs::create_dir_all(&out)?;
    let name = format!("{PREFIX}-{}", ids(combo).join("-"));
    let elf = root().join("target").join(target).join("release").join(krate);
    fs::copy(&elf, out.join(&name))?;

    if let Some(family) = cpu.data.uf2_family {
        elf_to_uf2(&elf, &out.join(format!("{name}.uf2")), family)?;
    }
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let combos = load_combos()?;
    match args.first().map(String::as_str) {
        Some("list") => combos.iter().for_each(|c| println!("{PREFIX}-{}", ids(c).join("-"))),
        Some("build-all") => for c in &combos { build(c)?; },
        Some("build") => {
            let want = &args[1..];
            let c = combos.iter().find(|c| ids(c) == want).context("no such valid combo")?;
            build(c)?;
        }
        _ => eprintln!("usage: cargo xtask <list | build-all | build <id>...>"),
    }
    Ok(())
}

fn elf_to_uf2(elf_path: &Path, uf2_path: &Path, family: u32) -> Result<()> {
    let bytes = fs::read(elf_path)?;
    let elf = Elf::parse(&bytes)?;

    // Gather every loadable byte at its flash (load) address, in 256-byte pages
    let mut pages: BTreeMap<u32, [u8; 256]> = BTreeMap::new();
    for ph in elf.program_headers.iter().filter(|p| p.p_type == PT_LOAD && p.p_filesz > 0) {
        let start = ph.p_offset as usize;
        let data = &bytes[start..start + ph.p_filesz as usize];
        for (i, b) in data.iter().enumerate() {
            let addr = ph.p_paddr as u32 + i as u32;
            pages.entry(addr & !0xFF).or_insert([0xFF; 256])[(addr & 0xFF) as usize] = *b;
        }
    }

    let total = pages.len() as u32;
    let mut out = Vec::with_capacity(pages.len() * 512);
    for (n, (addr, data)) in pages.iter().enumerate() {
        // magic0, magic1, flags (family ID present), address, payload size, block no, block count, family
        for w in [0x0A32_4655u32, 0x9E5D_5157, 0x0000_2000, *addr, 256, n as u32, total, family] {
            out.extend_from_slice(&w.to_le_bytes());
        }
        out.extend_from_slice(data);
        out.extend_from_slice(&[0u8; 476 - 256]); // pad payload area to 476 bytes
        out.extend_from_slice(&0x0AB1_6F30u32.to_le_bytes()); // magic end
    }
    fs::write(uf2_path, out)?;
    Ok(())
}