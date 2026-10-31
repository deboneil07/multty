pub(super) fn boundary_forward(text: &str, index: usize) -> usize {
    let mut index = index.min(text.len());
    while index < text.len() && !text.is_char_boundary(index) {
        index += 1;
    }
    index
}

pub(super) fn boundary_backward(text: &str, index: usize) -> usize {
    let mut index = index.min(text.len());
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

pub(super) fn fit_width(text: &str, width: usize) -> &str {
    if text.len() <= width {
        return text;
    }
    &text[..boundary_backward(text, width)]
}
