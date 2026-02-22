use crate::{core::widget::text::span::Span, style::Style};

#[derive(Clone, Debug)]
struct Line<'a, Spans: IntoIterator<Item = Span<'a>> = Vec<Span<'a>>> {
    spans: Spans,
    style: Style,
}
