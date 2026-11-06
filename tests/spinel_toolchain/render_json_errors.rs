//! The native half of `emit_and_run::render_json_errors_is_keyed_by_attribute`:
//! the scaffold's JSON failure arm, `format.json { render json:
//! @article.errors, status: :unprocessable_content }`, on the blog built
//! with `spin build` and driven over HTTP.

use super::{emit_and_run, native_http};

/// A create and an update that fail validation answer 422 with the errors
/// keyed by attribute, as Rails 8.1.4 does on the same fixture, not the
/// html arm's form.
#[test]
#[ignore = "requires the Spinel toolchain, run in its CI lane"]
fn a_failed_json_save_answers_its_errors_natively() {
    const FORM: &str = "application/x-www-form-urlencoded";
    const ERRORS: &str = r#"{"title":["can't be blank"],"body":["is too short (minimum is 10 characters)"]}"#;
    let tree = emit_and_run::real_blog().native_http_tree();
    let mut server = native_http::Server::start(&tree);
    server.take_session("/articles/new");
    let invalid = "article%5Btitle%5D=&article%5Bbody%5D=x";

    let created = server.post("/articles.json", FORM, invalid);
    assert_eq!(created.status, 422, "create: {}\n{}", created.body, server.log());
    assert_eq!(created.body, ERRORS, "create\n{}", server.log());
    let content_type = created.headers.get("content-type").map(String::as_str).unwrap_or("");
    assert!(content_type.starts_with("application/json"), "create: {content_type}");

    let valid = server.post("/articles.json", FORM, "article%5Btitle%5D=Kept&article%5Bbody%5D=A+body+long+enough");
    assert_eq!(valid.status, 201, "{}\n{}", valid.body, server.log());
    let id: String = valid.body.split(r#""id":"#).nth(1).expect("an id").chars().take_while(char::is_ascii_digit).collect();
    let updated = server.post(&format!("/articles/{id}.json"), FORM, &format!("_method=patch&{invalid}"));
    assert_eq!(updated.status, 422, "update: {}\n{}", updated.body, server.log());
    assert_eq!(updated.body, ERRORS, "update\n{}", server.log());
}
