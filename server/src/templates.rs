#[cfg(debug_assertions)]
mod render_impl {
    use crate::config;
    use tera::{Context, Result, Tera};

    pub fn render(template: &str, ctx: &Context) -> Result<String> {
        let inst = Tera::new(config::ENV.templates())?;
        inst.render(template, ctx)
    }
}

#[cfg(not(debug_assertions))]
mod render_impl {
    use crate::config;
    use once_cell::sync::Lazy;
    use tera::{Context, Result, Tera};

    const TEMPLATES: Lazy<Tera> = Lazy::new(|| {
        let inst = Tera::new(config::ENV.templates());
        match inst {
            Ok(inst) => inst,
            Err(err) => {
                log::error!("Failed to load templates: {}", err);
                panic!("Failed to load templates.");
            }
        }
    });

    pub fn render(template: &str, ctx: &Context) -> Result<String> {
        Ok(TEMPLATES.render(template, ctx)?)
    }
}

/// Render a template from the templates
/// directory. The location of the directory
/// is specified in the environment config.
///
/// This function behaves differently in debug
/// builds: When in debug mode, the templates
/// are re-loaded before each render for an
/// improved developer experience.
pub use render_impl::render;
