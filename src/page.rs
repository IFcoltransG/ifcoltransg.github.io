use sailfish::{RenderResult, Template, TemplateSimple};

use std::io::Result as IOResult;

use super::output_file::{OutputFile, static_byte_file, static_file};

#[derive(Template, Debug)]
#[template(path = "footer.html")]
pub(crate) struct Footer<'data> {
    pub(crate) time: &'data Time,
}

#[derive(Debug)]
pub(crate) struct Time {
    pub(crate) year: String,
    pub(crate) timestamp: String,
    pub(crate) time_tooltip: String,
}

#[derive(Template, Clone, Debug)]
#[template(path = "link.html")]
pub(crate) struct Link<'data> {
    pub(crate) text: &'data str,
    pub(crate) path: &'data str,
    pub(crate) current: bool,
}

impl<'data> Link<'data> {
    pub(crate) const fn new(text: &'data str, path: &'data str) -> Self {
        Link {
            text,
            path,
            current: false,
        }
    }
}

#[derive(Template, Debug)]
#[template(path = "navbar.html")]
pub(crate) struct Navbar<'data> {
    pub(crate) items: &'data [Link<'data>],
}

#[derive(Template, Debug)]
#[template(path = "page.html")]
pub(crate) struct Page<'data> {
    pub(crate) title: &'data str,
    pub(crate) publication_time: &'data Time,
    pub(crate) navbar: Navbar<'data>,
    pub(crate) content: &'data str,
}

impl<'data> Page<'data> {
    pub(crate) fn footer(&'_ self) -> Footer<'_> {
        Footer {
            time: self.publication_time,
        }
    }
}

pub(crate) trait RenderStatic: TemplateSimple + Default {
    const RENDER: &dyn Fn() -> RenderResult = &|| Self::default().render_once();
    const TITLE: &'static str;
    const PATH: &'static str;
    const TEXT: &'static str;
    const PAGE: (&'static str, Link<'static>, &dyn Fn() -> RenderResult) = (
        Self::TITLE,
        Link::new(Self::TEXT, Self::PATH),
        &Self::RENDER,
    );
}

pub fn write_static_files(output_dir: &cap_std::fs_utf8::Dir) -> IOResult<()> {
    let favicon = static_byte_file!("favicon.ico");
    let robots = static_file!("robots.txt");
    let css = [static_file!("style.css"), static_file!("simple.min.css"), static_file!("webring.css")];
    favicon.write(output_dir)?;
    robots.write(output_dir)?;
    for css_file in css {
        css_file.write(output_dir)?
    }
    Ok(())
}
