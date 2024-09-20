//! Generates the postamble (assembly after the body) for AARCH64

use super::AARCH64Generator;

impl AARCH64Generator {
    /// Generates the postamble (assembly after the body) for AARCH64
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
