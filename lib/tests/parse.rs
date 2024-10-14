use pretty_assertions::assert_str_eq;
use usage::parse;
use usage::Spec;

macro_rules! tests {
    ($($name:ident: spec=$spec:expr, args=$args:expr, expected=$expected:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let spec: Spec = $spec.parse().unwrap();
            let mut args = shell_words::split($args).unwrap();
            args.insert(0, "test".to_string());
            match parse(&spec, &args) {
                Ok(env) => assert_str_eq!(format!("{:?}", env.as_env()).trim(), $expected.trim()),
                Err(e) => assert_str_eq!(format!("{e}").trim(), $expected.trim()),
            }
        }
    )*
    }
}

tests! {
    required_arg:
        spec=r#"arg "<name>""#,
        args="",
        expected=r#"Missing required arg: <name>"#,

    required_flag:
        spec=r#"flag "--name <name>" required=true"#,
        args="",
        expected=r#"Missing required flag: --name <name>"#,

    negate:
        spec=r#"flag "--force" negate="--no-force""#,
        args="--no-force",
        expected=r#"{"usage_force": "false"}"#,

    flag_short_next:
        spec=r#"flag "-s <shell>""#,
        args="-sbash",
        expected=r#"{"usage_s": "bash"}"#,

    flag_short_space:
        spec=r#"flag "-s <shell>""#,
        args="-s bash",
        expected=r#"{"usage_s": "bash"}"#,

    flag_choices_ok:
        spec=r#"flag "--shell <shell>" {
    choices "bash" "fish" "zsh"
}"#,
        args="--shell bash",
        expected=r#"{"usage_shell": "bash"}"#,

    flag_choices_err:
        spec=r#"flag "-s --shell <shell>" {
    choices "bash" "fish" "zsh"
}"#,
        args="-s invalid",
        expected=r#"Invalid choice for option shell: invalid, expected one of bash, fish, zsh"#,

    arg_choices_ok:
        spec=r#"arg "<shell>" {
    choices "bash" "fish" "zsh"
}"#,
        args="bash",
        expected=r#"{"usage_shell": "bash"}"#,

    arg_choices_err:
        spec=r#"arg "<shell>" {
    choices "bash" "fish" "zsh"
}"#,
        args="invalid",
        expected=r#"Invalid choice for arg shell: invalid, expected one of bash, fish, zsh"#,

    arg_choices_help:
        spec=r#"arg "<shell>" {
    choices "bash" "fish" "zsh"
}"#,
        args="--help",
        expected=r#"Usage: <shell>

Arguments:
  <shell>
"#,
}
