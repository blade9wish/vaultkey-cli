use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::template::TemplateRenderer;
use crate::error::VaultError;

/// CLI arguments for the `template` subcommand.
#[derive(Debug)]
pub struct TemplateCmdArgs {
    /// Path to the template file containing `{{{KEY}}}` placeholders.
    pub template_path: PathBuf,
    /// Inline key=value pairs to use for rendering (overrides bundle values).
    pub overrides: Vec<(String, String)>,
    /// Output path; if None, prints to stdout.
    pub output_path: Option<PathBuf>,
    /// Only list keys referenced in the template without rendering.
    pub list_keys: bool,
}

/// Render a template file using the provided secrets map, applying any CLI overrides.
pub fn run_template_cmd(
    args: &TemplateCmdArgs,
    bundle_secrets: &HashMap<String, String>,
) -> Result<String, VaultError> {
    let template_content = fs::read_to_string(&args.template_path).map_err(|e| {
        VaultError::Generic(format!(
            "Failed to read template file '{}': {e}",
            args.template_path.display()
        ))
    })?;

    let renderer = TemplateRenderer::default();

    if args.list_keys {
        let keys = renderer.extract_keys(&template_content);
        return Ok(keys.join("\n"));
    }

    let mut secrets = bundle_secrets.clone();
    for (k, v) in &args.overrides {
        secrets.insert(k.clone(), v.clone());
    }

    let rendered = renderer.render(&template_content, &secrets)?;

    if let Some(out_path) = &args.output_path {
        fs::write(out_path, &rendered).map_err(|e| {
            VaultError::Generic(format!(
                "Failed to write output to '{}': {e}",
                out_path.display()
            ))
        })?;
    }

    Ok(rendered)
}

/// Parse `KEY=VALUE` strings into a Vec of tuples.
pub fn parse_overrides(raw: &[String]) -> Result<Vec<(String, String)>, VaultError> {
    raw.iter()
        .map(|s| {
            let mut parts = s.splitn(2, '=');
            let key = parts
                .next()
                .filter(|k| !k.is_empty())
                .ok_or_else(|| VaultError::Generic(format!("Invalid override '{s}': missing key")))?;
            let val = parts
                .next()
                .ok_or_else(|| VaultError::Generic(format!("Invalid override '{s}': missing '='")))?;
            Ok((key.to_string(), val.to_string()))
        })
        .collect()
}
