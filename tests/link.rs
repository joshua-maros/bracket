use bracket::{
    helper::*,
    parser::ast::Node,
    render::{Context, Render, Type},
    Registry, Result,
};
use serde_json::json;

static NAME: &str = "link.rs";

pub struct LinkHelper;
impl Helper for LinkHelper {
    fn call<'render, 'call>(
        &self,
        rc: &mut Render<'render>,
        ctx: &Context<'call>,
        _template: Option<&'render Node<'render>>,
    ) -> HelperValue {
        let href = ctx.try_get(0, &[Type::String])?.as_str().unwrap();
        let label = ctx.try_get(1, &[Type::String])?.as_str().unwrap();
        let link = format!(
            r#"<a href="{}">{}</a>"#,
            rc.escape(href),
            rc.escape(label));
        rc.write(&link)?;
        Ok(None)
    }
}

#[test]
fn link_noop() -> Result<()> {
    let registry = Registry::new();
    let value = r"[[/some/target]]";
    let data = json!({});
    let result = registry.once(NAME, value, &data)?;
    assert_eq!("[[/some/target]]", &result);
    Ok(())
}

#[test]
fn link_href() -> Result<()> {
    let mut registry = Registry::new();
    registry.helpers_mut().insert("link", Box::new(LinkHelper {}));
    let value = r"[[SomeTarget|Label & Info]]";
    let data = json!({});
    let result = registry.once(NAME, value, &data)?;
    println!("Result {}", result);
    assert_eq!(r#"<a href="SomeTarget">Label &amp; Info</a>"#, &result);
    Ok(())
}

#[test]
fn link_escaped_pipe() -> Result<()> {
    let mut registry = Registry::new();
    registry.helpers_mut().insert("link", Box::new(LinkHelper {}));
    let value = r"[[Some\|Target|Label & Info]]";
    let data = json!({});
    let result = registry.once(NAME, value, &data)?;
    assert_eq!(r#"<a href="Some|Target">Label &amp; Info</a>"#, &result);
    Ok(())
}

#[test]
fn link_escaped_pipe_label() -> Result<()> {
    let mut registry = Registry::new();
    registry.helpers_mut().insert("link", Box::new(LinkHelper {}));
    let value = r"[[Some\|Target|Label\|Info]]";
    let data = json!({});
    let result = registry.once(NAME, value, &data)?;
    assert_eq!(r#"<a href="Some|Target">Label|Info</a>"#, &result);
    Ok(())
}

#[test]
fn link_escaped_bracket() -> Result<()> {
    let mut registry = Registry::new();
    registry.helpers_mut().insert("link", Box::new(LinkHelper {}));
    let value = r"[[Some\]Target|Label & Info]]";
    let data = json!({});
    let result = registry.once(NAME, value, &data)?;
    assert_eq!(r#"<a href="Some]Target">Label &amp; Info</a>"#, &result);
    Ok(())
}

#[test]
fn link_escaped_bracket_label() -> Result<()> {
    let mut registry = Registry::new();
    registry.helpers_mut().insert("link", Box::new(LinkHelper {}));
    let value = r"[[Some\]Target|Label\]Info]]";
    let data = json!({});
    let result = registry.once(NAME, value, &data)?;
    assert_eq!(r#"<a href="Some]Target">Label]Info</a>"#, &result);
    Ok(())
}

#[test]
fn link_escaped_newline() -> Result<()> {
    let mut registry = Registry::new();
    registry.helpers_mut().insert("link", Box::new(LinkHelper {}));
    let value = r"[[Some\nTarget|Label & Info]]";
    let data = json!({});
    let result = registry.once(NAME, value, &data)?;
    assert_eq!(r#"<a href="Some
Target">Label &amp; Info</a>"#, &result);
    Ok(())
}

#[test]
fn link_escaped_newline_label() -> Result<()> {
    let mut registry = Registry::new();
    registry.helpers_mut().insert("link", Box::new(LinkHelper {}));
    let value = r"[[Some\nTarget|Label\nInfo]]";
    let data = json!({});
    let result = registry.once(NAME, value, &data)?;
    assert_eq!(r#"<a href="Some
Target">Label
Info</a>"#, &result);
    Ok(())
}