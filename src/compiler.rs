pub fn compile(input: i32) -> String {
    format!(
        "(module
        (export \"_start\" (func $main))
        (func  $main
        (result i32)

        (i32.const {})
        return

    ))",
        input
    )
}
