pub struct LintResult {
    pub overview: String,
}

pub fn lint(text: &str) -> Result<LintResult, String> {
    Ok(LintResult{
   overview: format!("linting_____________________________________________________________\n\n{text}\n\ndone linting________________________________________________________")
})
}
