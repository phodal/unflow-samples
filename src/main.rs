fn main() {
    let data = r#"project: DesignDSL
feature: "design basic dsl"
type: web
width: 1080px

"#;

    let result = unflow::parse(data);
    println!("{:?}", result);
}
