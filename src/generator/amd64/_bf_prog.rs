use super::AMD64Generator;

impl AMD64Generator {
    pub(crate) fn bf_prog(&self) -> String {
        format!(
            r#";# <<<Begin BF Program>>>
bf_prog:
    movq %rdi, %r12
{}
    ret
;# <<<End BF Program>>>"#,
            self.bf_instrs
                .iter()
                .map(|inst| String::from("    ") + inst)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
