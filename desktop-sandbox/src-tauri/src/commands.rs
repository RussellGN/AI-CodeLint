use tauri::AppHandle;

#[tauri::command]
pub async fn lint_contents(_: AppHandle, contents: String) -> Result<Vec<String>, String> {
    let res = ai_codelint::linter::lint(true, "code_sample", &contents, None, None)
        .await?
        .iter()
        .map(|lint_res| {
            format!(
                "lines {}-{}: {}",
                lint_res.start_line, lint_res.end_line, lint_res.overview
            )
        })
        .collect();
    Ok(res)
}
