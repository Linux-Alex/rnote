// Modules
pub mod brush;
pub mod eraser;
pub mod penbehaviour;
pub mod penholder;
pub mod penmode;
pub mod pensconfig;
pub mod selector;
pub mod shaper;
pub mod shortcuts;
pub mod tools;
pub mod typewriter;

// Re-exports
pub use brush::Brush;
pub use eraser::Eraser;
pub use penbehaviour::PenBehaviour;
pub use penholder::PenHolder;
pub use penmode::PenMode;
pub use pensconfig::PensConfig;
pub use selector::Selector;
pub use shaper::Shaper;
pub use shortcuts::Shortcuts;
pub use tools::Tools;
pub use typewriter::Typewriter;

// Imports
use self::penbehaviour::PenProgress;
use crate::engine::{EngineView, EngineViewMut};
use crate::{DrawOnDocBehaviour, WidgetFlags};
use piet_cairo::CairoRenderContext;
use rnote_compose::penevents::PenEvent;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug)]
pub enum Pen {
    Brush(Brush),
    Shaper(Shaper),
    Typewriter(Typewriter),
    Eraser(Eraser),
    Selector(Selector),
    Tools(Tools),
}

impl Default for Pen {
    fn default() -> Self {
        Self::Brush(Brush::default())
    }
}

impl PenBehaviour for Pen {
    fn init(&mut self, engine_view: &EngineView) -> WidgetFlags {
        match self {
            Pen::Brush(brush) => brush.init(engine_view),
            Pen::Shaper(shaper) => shaper.init(engine_view),
            Pen::Typewriter(typewriter) => typewriter.init(engine_view),
            Pen::Eraser(eraser) => eraser.init(engine_view),
            Pen::Selector(selector) => selector.init(engine_view),
            Pen::Tools(tools) => tools.init(engine_view),
        }
    }

    fn deinit(&mut self) -> WidgetFlags {
        match self {
            Pen::Brush(brush) => brush.deinit(),
            Pen::Shaper(shaper) => shaper.deinit(),
            Pen::Typewriter(typewriter) => typewriter.deinit(),
            Pen::Eraser(eraser) => eraser.deinit(),
            Pen::Selector(selector) => selector.deinit(),
            Pen::Tools(tools) => tools.deinit(),
        }
    }

    fn style(&self) -> PenStyle {
        match self {
            Pen::Brush(brush) => brush.style(),
            Pen::Shaper(shaper) => shaper.style(),
            Pen::Typewriter(typewriter) => typewriter.style(),
            Pen::Eraser(eraser) => eraser.style(),
            Pen::Selector(selector) => selector.style(),
            Pen::Tools(tools) => tools.style(),
        }
    }

    fn update_state(&mut self, engine_view: &mut EngineViewMut) -> WidgetFlags {
        match self {
            Pen::Brush(brush) => brush.update_state(engine_view),
            Pen::Shaper(shaper) => shaper.update_state(engine_view),
            Pen::Typewriter(typewriter) => typewriter.update_state(engine_view),
            Pen::Eraser(eraser) => eraser.update_state(engine_view),
            Pen::Selector(selector) => selector.update_state(engine_view),
            Pen::Tools(tools) => tools.update_state(engine_view),
        }
    }

    fn handle_event(
        &mut self,
        event: PenEvent,
        now: Instant,
        engine_view: &mut EngineViewMut,
    ) -> (PenProgress, WidgetFlags) {
        match self {
            Pen::Brush(brush) => brush.handle_event(event, now, engine_view),
            Pen::Shaper(shaper) => shaper.handle_event(event, now, engine_view),
            Pen::Typewriter(typewriter) => typewriter.handle_event(event, now, engine_view),
            Pen::Eraser(eraser) => eraser.handle_event(event, now, engine_view),
            Pen::Selector(selector) => selector.handle_event(event, now, engine_view),
            Pen::Tools(tools) => tools.handle_event(event, now, engine_view),
        }
    }

    fn fetch_clipboard_content(
        &self,
        engine_view: &EngineView,
    ) -> anyhow::Result<(Option<(Vec<u8>, String)>, WidgetFlags)> {
        match self {
            Pen::Brush(brush) => brush.fetch_clipboard_content(engine_view),
            Pen::Shaper(shaper) => shaper.fetch_clipboard_content(engine_view),
            Pen::Typewriter(typewriter) => typewriter.fetch_clipboard_content(engine_view),
            Pen::Eraser(eraser) => eraser.fetch_clipboard_content(engine_view),
            Pen::Selector(selector) => selector.fetch_clipboard_content(engine_view),
            Pen::Tools(tools) => tools.fetch_clipboard_content(engine_view),
        }
    }

    fn cut_clipboard_content(
        &mut self,
        engine_view: &mut EngineViewMut,
    ) -> anyhow::Result<(Option<(Vec<u8>, String)>, WidgetFlags)> {
        match self {
            Pen::Brush(brush) => brush.cut_clipboard_content(engine_view),
            Pen::Shaper(shaper) => shaper.cut_clipboard_content(engine_view),
            Pen::Typewriter(typewriter) => typewriter.cut_clipboard_content(engine_view),
            Pen::Eraser(eraser) => eraser.cut_clipboard_content(engine_view),
            Pen::Selector(selector) => selector.cut_clipboard_content(engine_view),
            Pen::Tools(tools) => tools.cut_clipboard_content(engine_view),
        }
    }
}

impl DrawOnDocBehaviour for Pen {
    fn bounds_on_doc(&self, engine_view: &EngineView) -> Option<p2d::bounding_volume::Aabb> {
        match self {
            Pen::Brush(brush) => brush.bounds_on_doc(engine_view),
            Pen::Shaper(shaper) => shaper.bounds_on_doc(engine_view),
            Pen::Typewriter(typewriter) => typewriter.bounds_on_doc(engine_view),
            Pen::Eraser(eraser) => eraser.bounds_on_doc(engine_view),
            Pen::Selector(selector) => selector.bounds_on_doc(engine_view),
            Pen::Tools(tools) => tools.bounds_on_doc(engine_view),
        }
    }

    fn draw_on_doc(
        &self,
        cx: &mut CairoRenderContext,
        engine_view: &EngineView,
    ) -> anyhow::Result<()> {
        match self {
            Pen::Brush(brush) => brush.draw_on_doc(cx, engine_view),
            Pen::Shaper(shaper) => shaper.draw_on_doc(cx, engine_view),
            Pen::Typewriter(typewriter) => typewriter.draw_on_doc(cx, engine_view),
            Pen::Eraser(eraser) => eraser.draw_on_doc(cx, engine_view),
            Pen::Selector(selector) => selector.draw_on_doc(cx, engine_view),
            Pen::Tools(tools) => tools.draw_on_doc(cx, engine_view),
        }
    }
}

#[derive(
    Eq,
    PartialEq,
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
    Hash,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
)]
#[serde(rename = "pen_style")]
pub enum PenStyle {
    #[serde(rename = "brush")]
    Brush,
    #[serde(rename = "shaper")]
    Shaper,
    #[serde(rename = "typewriter")]
    Typewriter,
    #[serde(rename = "eraser")]
    Eraser,
    #[serde(rename = "selector")]
    Selector,
    #[serde(rename = "tools")]
    Tools,
}

impl Default for PenStyle {
    fn default() -> Self {
        Self::Brush
    }
}

impl TryFrom<u32> for PenStyle {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_u32(value)
            .ok_or_else(|| anyhow::anyhow!("PenStyle try_from::<u32>() for value {} failed", value))
    }
}

impl std::str::FromStr for PenStyle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "brush" => Ok(Self::Brush),
            "shaper" => Ok(Self::Shaper),
            "typewriter" => Ok(Self::Typewriter),
            "eraser" => Ok(Self::Eraser),
            "selector" => Ok(Self::Selector),
            "tools" => Ok(Self::Tools),
            s => Err(anyhow::anyhow!(
                "PenStyle::from_str() failed, invalid name {s}"
            )),
        }
    }
}

impl std::string::ToString for PenStyle {
    fn to_string(&self) -> String {
        match self {
            PenStyle::Brush => String::from("brush"),
            PenStyle::Shaper => String::from("shaper"),
            PenStyle::Typewriter => String::from("typewriter"),
            PenStyle::Eraser => String::from("eraser"),
            PenStyle::Selector => String::from("selector"),
            PenStyle::Tools => String::from("tools"),
        }
    }
}

impl PenStyle {
    pub fn icon_name(self) -> String {
        match self {
            Self::Brush => String::from("pen-brush-symbolic"),
            Self::Shaper => String::from("pen-shaper-symbolic"),
            Self::Typewriter => String::from("pen-typewriter-symbolic"),
            Self::Eraser => String::from("pen-eraser-symbolic"),
            Self::Selector => String::from("pen-selector-symbolic"),
            Self::Tools => String::from("pen-tools-symbolic"),
        }
    }
}
