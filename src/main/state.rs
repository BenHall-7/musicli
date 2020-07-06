use musiclib::midi::File;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{List, Paragraph, Text};
use tui::Frame;

// TODO: slice instead of vec.
// We want all areas to be pre-compiled. Right?
pub enum Area {
    Widget(WidgetType),
    // todo: Combine Layout and Vec<Area> into a single struct
    // to better handle possible length mismatches
    SubArea(Vec<(Rect, Area)>),
}

impl Area {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        match self {
            Area::Widget(widget_type) => widget_type.render(frame, rect),
            Area::SubArea(sub_areas) => sub_areas
                .iter()
                .for_each(|(rect, sub_area)| sub_area.render(frame, *rect)),
        }
    }
}

pub enum WidgetType {
    List(Vec<String>, Option<Style>),
    Paragraph(Vec<String>, Option<Style>),
    // more...
}

impl WidgetType {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        match self {
            WidgetType::List(text, style) => {
                frame.render_widget(
                    List::new(text.iter().map(Into::into).map(|s| {
                        if let Some(style) = style {
                            Text::Styled(s, *style)
                        } else {
                            Text::Raw(s)
                        }
                    })),
                    rect,
                );
            }
            WidgetType::Paragraph(text, style) => {
                frame.render_widget(
                    Paragraph::new(
                        text.iter()
                            .map(Into::into)
                            .map(|s| {
                                if let Some(style) = style {
                                    Text::Styled(s, *style)
                                } else {
                                    Text::Raw(s)
                                }
                            })
                            .collect::<Vec<Text>>() // this is dumb but apparently necessary
                            .iter(),
                    ),
                    rect,
                )
            }
        }
    }
}

fn test() {
    let iterator = [0, 1, 2].iter();
}
