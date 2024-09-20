//! Generates the postamble (assembly after the body) for AArch64

use super::AArch64Generator;

impl AArch64Generator {
    /// Generates the postamble (assembly after the body) for AArch64
    ///
    /// Lists used functions from the C standard library
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
