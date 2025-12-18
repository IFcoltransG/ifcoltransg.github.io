use cap_std::{ambient_authority, fs_utf8::camino::Utf8Path as Path};
use chrono::{Datelike, DurationRound, TimeDelta, Utc};
use sailfish::{RenderResult, Template, TemplateSimple};

use std::error::Error;

mod output_dir;
mod output_file;
mod page;

use output_dir::create_output_dir;
use page::{Link, Navbar, Page, RenderStatic, Time, write_static_files};

#[derive(TemplateSimple, Debug, Copy, Clone, Default)]
#[template(path = "index.html")]
struct Home;

impl RenderStatic for Home {
    const TITLE: &str = "IFcoltransG's Spot";
    const PATH: &str = "index.html";
    const TEXT: &str = "Home";
}

// #[derive(TemplateSimple, Debug, Copy, Clone, Default)]
// #[template(path = "blog.html")]
// struct Blog;

// impl RenderStatic for Blog {
//     const TITLE: &str = "IFcoltransG's Blog";
//     const PATH: &str = "blog.html";
//     const TEXT: &str = "Blog";
// }

#[derive(TemplateSimple, Debug, Copy, Clone, Default)]
#[template(path = "about.html")]
struct About;

impl About {
    const LINKS: &[Link<'static>] = &[
        Link::new("BUUZA!! Wiki", "./buuza-wiki/"),
        Link::new("LEB 128", "./lebanon-leb128-converter/"),
        Link::new(
            "It Is An Innkeeper",
            "https://ifcoltransg.itch.io/innkeeper",
        ),
        Link::new(
            "Maze Gallery",
            "https://ifdb.org/viewgame?id=c9bc9vl2paxqa98d",
        ),
        Link::new(
            "Lettersize Sonnetling",
            "https://taper.badquar.to/14/lettersize_sonnetling.html",
        ),
        Link::new("Esoteric Python Guide", "./esoteric-python-guide/"),
        Link::new(
            "Code Guessing entries",
            "https://codeguessing.gay/stats/IFcoltransG",
        ),
    ];
}

impl RenderStatic for About {
    const TITLE: &str = "About IFcoltransG";
    const PATH: &str = "about.html";
    const TEXT: &str = "About";
}

#[derive(TemplateSimple, Debug, Copy, Clone, Default)]
#[template(path = "more-info.html")]
struct MoreInfo;

impl RenderStatic for MoreInfo {
    const TITLE: &str = "More Info";
    const PATH: &str = "more-info.html";
    const TEXT: &str = "More Info";
}

#[derive(TemplateSimple, Debug, Copy, Clone, Default)]
#[template(path = "rings.html")]
struct Rings;

impl RenderStatic for Rings {
    const TITLE: &str = "Webrings";
    const PATH: &str = "rings.html";
    const TEXT: &str = "Rings";
}

fn main() -> Result<(), Box<dyn Error>> {
    // This directory will be wiped if it exists
    eprintln!("Getting output directory");
    let output_dir = create_output_dir(Path::new("./docs"), ambient_authority())?;

    eprintln!("Collecting metadata");
    let now = Utc::now().duration_round(TimeDelta::hours(3)).unwrap();

    let year = format!("{}", now.year());
    let timestamp = now.to_rfc3339();
    let time_tooltip = now.to_rfc2822();

    let publication_time = &Time {
        year,
        timestamp,
        time_tooltip,
    };

    eprintln!("Constructing pages");
    let pages = &[
        Home::PAGE,
        MoreInfo::PAGE,
        Rings::PAGE,
        About::PAGE, /* Blog::PAGE*/
    ];
    let navbar = &pages.clone().map(|(_, item, _)| item);

    eprintln!("Rendering pages");
    for (title, item, content) in pages {
        let content: &dyn Fn() -> RenderResult = content;
        let navbar = &set_current(item.path, navbar);
        let page = Page {
            title,
            publication_time,
            navbar: Navbar {
                items: navbar.as_ref(),
            },
            content: &content().unwrap(),
        };
        output_dir.write(Path::new(&item.path), page.render().unwrap())?
    }

    eprintln!("Writing static files");
    write_static_files(&output_dir)?;

    Ok(())
}

fn set_current(current: &str, navbar: &[Link<'static>]) -> Box<[Link<'static>]> {
    navbar
        .iter()
        .cloned()
        .map(|mut item| {
            item.current = item.path == current;
            item
        })
        .collect()
}
