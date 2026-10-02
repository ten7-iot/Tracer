use anyhow::{bail, Context, Result};
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
    fs::copy(root().join("target").join(target).join("release").join(krate), out.join(name))?;
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