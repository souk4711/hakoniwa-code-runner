use handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext, RenderError,
};

pub fn render_to_string_helper(
    h: &Helper,
    _: &Handlebars,
    _c: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let v = match h.param(0).map(|v| v.value()) {
        Some(v) => v,
        None => return Err(RenderError::new("param not found")),
    };
    match super::fs::read_to_string(&v.render()) {
        Ok(v) => out.write(&v)?,
        Err(e) => return Err(RenderError::new(e.to_string())),
    };
    Ok(())
}
