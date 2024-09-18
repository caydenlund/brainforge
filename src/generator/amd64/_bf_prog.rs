use super::AMD64Generator;

impl AMD64Generator {
    pub(crate) fn bf_prog(&self) -> String {
        format!(
            r#";# <<<Begin BF Program>>>
bf_prog:
    movq %rdi, %rbx
    movq %rdi, %rcx
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
