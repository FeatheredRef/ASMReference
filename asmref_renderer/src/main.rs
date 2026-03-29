use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    process::Command,
};

use crate::{
    rendering::logic::{Dir, pre_render},
    structuring::logic::{Details, derive_structure},
};

mod rendering;
mod structuring;

fn write_dir(dir: &Dir, parent: &Path) {
    let current = parent.join(&dir.name);
    fs::create_dir_all(&current).unwrap();
    println!("  📁 {}", current.display());
    for (name, content) in &dir.files {
        let file_path = current.join(name);
        fs::write(&file_path, content).unwrap();
        println!("     ✍  {}", file_path.display());
    }

    for sub in &dir.sub {
        write_dir(sub, &current);
    }
}

fn main() {
    println!("\n🔧 ASMREF builder starting...\n");

    // ── Load details.json ────────────────────────────────────────────────────
    println!("📄 Reading details.json...");
    let mut s = String::new();
    {
        let mut f = File::open("details.json").unwrap();
        f.read_to_string(&mut s).unwrap();
    }
    println!("   ✅ Loaded.\n");

    // ── Parse & derive structure ─────────────────────────────────────────────
    println!("🗂  Deriving structure...");
    let details: Details = serde_json::from_str(&s).unwrap();
    let structure = derive_structure(&details).expect("❌ No root category found.");
    println!("   ✅ Structure ready.\n");

    // ── Pre-render ───────────────────────────────────────────────────────────
    println!("🖊  Pre-rendering pages...");
    let root: Dir = pre_render(&structure, &details);
    println!("   ✅ Pre-render complete.\n");

    // ── Write to build/ ──────────────────────────────────────────────────────
    println!("💾 Writing to build/...");
    let build = Path::new("build");
    fs::create_dir_all(build).unwrap();
    write_dir(&root, build);
    for i in details.dependencies.iter() {
        Command::new("cp")
            .args(["-aL", i, &format!("build/{}/", root.name)])
            .status()
            .unwrap();
    }
    println!("\n✨ Done! Output is in build/{}/\n", root.name);
}
