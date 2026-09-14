use super::configure::{bounds, finish, host, locus, root, spelling, target, tool};
use super::types::Host;
use macroonz::harness::oracle::{
    CompilationVerdict, DeclaredCompilation, DiagnosticAnchor, PrimarySourceSpan, RustcErrorCode,
    SourcePosition,
};
use macroonz::native_compiler::{self, CargoFixture, CargoTarget, CompilerRequest};
use std::io::Write;
use std::path::Path;

const LAWFUL: &str = concat!(
    "bakery::recipe! {\n",
    "    pub mod sample {\n",
    "        pub enum Phase { Ready }\n",
    "        pub const COUNT: u64 = 6 * 7;\n",
    "        bake! {\n",
    "            vocabularies { Phase; };\n",
    "            projections { companions; };\n",
    "        }\n",
    "    }\n",
    "}\n",
    "use std::io::Write;\n",
    "fn main() -> std::io::Result<()> {\n",
    "    writeln!(std::io::stdout(), \"{}\", sample::COUNT)\n",
    "}\n",
);

#[test]
fn registry_recipe_retains_authored_primary_span_and_a_lawful_read_back_twin() -> Result<(), String>
{
    let root = root()?;
    let host = host(&root)?;
    let (package, manifest) = super::real::package(
        &root,
        "[[bin]]\nname = \"registry-reader\"\npath = \"fixture.rs\"\n",
    )?;
    std::fs::OpenOptions::new().append(true).open(&manifest).map_err(|error| error.to_string())?
        .write_all(b"[dependencies]\nbakery = { package = \"macroonz\", version = \"=0.2.0\", default-features = false }\n").map_err(|error| error.to_string())?;
    std::fs::write(root.join("fixture.rs"), LAWFUL).map_err(|error| error.to_string())?;
    registry_graph(&root, &host, &manifest)?;
    let fixture = CargoFixture::informed(
        manifest,
        target()?,
        package,
        CargoTarget::Binary("registry-reader".to_owned()),
        host.triple.clone(),
    )
    .map_err(|error| error.to_string())?;
    let request = CompilerRequest::cargo(
        &tool(&host, &host.cargo, &root, bounds()?)?,
        fixture,
        locus()?,
    )
    .map_err(|error| error.to_string())?;
    let lawful = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    assert_eq!(
        lawful.compared(&DeclaredCompilation::compiles()),
        Ok(CompilationVerdict::Conforms),
        "{lawful:?}"
    );
    super::real::read_count(&lawful, &host, &root)?;
    std::fs::write(
        root.join("fixture.rs"),
        LAWFUL.replace("6 * 7", "\"wrong\""),
    )
    .map_err(|error| error.to_string())?;
    let refused = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    let code = RustcErrorCode::informed("E0308").map_err(|error| format!("{error:?}"))?;
    let start = SourcePosition::informed(4, 32).map_err(|error| format!("{error:?}"))?;
    let end = SourcePosition::informed(4, 39).map_err(|error| format!("{error:?}"))?;
    let primary =
        PrimarySourceSpan::informed(locus()?, start, end).map_err(|error| format!("{error:?}"))?;
    let expected = DiagnosticAnchor::at(code, primary);
    assert_eq!(
        refused.observed().map(|value| value.refusal()),
        Ok(Some(&expected)),
        "{refused:?}"
    );
    Ok(())
}

pub(crate) fn registry_graph(
    root: &Path,
    host: &Host,
    manifest: &Path,
) -> Result<Vec<(&'static str, std::path::PathBuf)>, String> {
    let selected = tool(host, &host.cargo, root, bounds()?)?;
    let manifest = spelling(manifest)?;
    for arguments in [
        vec!["generate-lockfile", "--manifest-path", &manifest],
        vec![
            "fetch",
            "--locked",
            "--manifest-path",
            &manifest,
            "--target",
            &host.triple,
        ],
    ] {
        let preparation = selected
            .invocation(arguments.into_iter().map(str::to_owned).collect())
            .map_err(|error| error.to_string())?;
        let prepared = super::super::process::completed(
            macroonz::native_process::run(&preparation, None).map_err(|error| error.to_string())?,
        )?;
        if !prepared.status().success() {
            return Err(format!("registry fixture preparation failed: {prepared:?}"));
        }
    }
    let metadata_request = selected
        .invocation(
            [
                "metadata",
                "--locked",
                "--offline",
                "--format-version=1",
                "--manifest-path",
                &manifest,
            ]
            .map(str::to_owned)
            .to_vec(),
        )
        .map_err(|error| error.to_string())?;
    let metadata = super::super::process::completed(
        macroonz::native_process::run(&metadata_request, None)
            .map_err(|error| error.to_string())?,
    )?;
    if !metadata.status().success() {
        return Err(format!("registry metadata failed: {metadata:?}"));
    }
    let value: serde_json::Value =
        serde_json::from_slice(metadata.stdout().bytes()).map_err(|error| error.to_string())?;
    let packages = value
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or("no Cargo package roster")?;
    let mut roots = Vec::new();
    for name in ["macroonz", "macroonz-compiler", "macroonz-macros"] {
        let package = packages
            .iter()
            .filter(|package| package.get("name").and_then(serde_json::Value::as_str) == Some(name))
            .collect::<Vec<_>>();
        let [package] = package.as_slice() else {
            return Err(format!(
                "registry package roster for {name} was not singular"
            ));
        };
        assert_eq!(
            package.get("version").and_then(serde_json::Value::as_str),
            Some("0.2.0")
        );
        assert_eq!(
            package.get("source").and_then(serde_json::Value::as_str),
            Some("registry+https://github.com/rust-lang/crates.io-index")
        );
        let package_manifest = package
            .get("manifest_path")
            .and_then(serde_json::Value::as_str)
            .ok_or("missing registry source manifest")?;
        roots.push((
            name,
            Path::new(package_manifest)
                .parent()
                .ok_or("missing registry source directory")?
                .to_path_buf(),
        ));
    }
    Ok(roots)
}
