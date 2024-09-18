use super::AMD64Generator;

impl AMD64Generator {
    pub(crate) fn postamble(&self) -> String {
        format!(
            r#";# <<<Begin postamble>>>
{}
;# <<<End postamble>>>"#,
            self.libc_funcs
                .iter()
                .map(|func| String::from("    .extern ") + func)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
