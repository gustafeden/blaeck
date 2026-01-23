//! Breadcrumbs component - Navigation path display.
//!
//! The Breadcrumbs component displays a hierarchical navigation path,
//! commonly used to show the current location in a file system or menu.
//!
//! ## When to use Breadcrumbs
//!
//! - File path display (Home > Documents > File.txt)
//! - Nested menu location
//! - Multi-step wizard showing current step
//!
//! ## See also
//!
//! - [`Tabs`](super::Tabs) — Horizontal navigation (not hierarchical)
//! - [`TreeView`](super::TreeView) — Full tree with expand/collapse

use crate::element::{Component, Element};
use crate::style::{Color, Modifier, Style};

/// A single item in the breadcrumb path.
#[derive(Debug, Clone)]
pub struct Crumb {
    /// The display label for this crumb.
    pub label: String,
    /// Whether this crumb is active/current.
    pub active: bool,
}

impl Crumb {
    /// Create a new breadcrumb item.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            active: false,
        }
    }

    /// Create an active (current) breadcrumb item.
    pub fn active(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            active: true,
        }
    }

    /// Set whether this crumb is active.
    #[must_use]
    pub fn set_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl<S: Into<String>> From<S> for Crumb {
    fn from(s: S) -> Self {
        Crumb::new(s)
    }
}

/// Separator styles for breadcrumbs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BreadcrumbSeparator {
    /// Slash: /
    #[default]
    Slash,
    /// Backslash: \
    Backslash,
    /// Arrow: >
    Arrow,
    /// Double arrow: >>
    DoubleArrow,
    /// Chevron: ›
    Chevron,
    /// Double chevron: »
    DoubleChevron,
    /// Bullet: •
    Bullet,
    /// Pipe: |
    Pipe,
    /// Custom separator string
    Custom(&'static str),
}

impl BreadcrumbSeparator {
    /// Get the separator string.
    pub fn as_str(&self) -> &str {
        match self {
            BreadcrumbSeparator::Slash => " / ",
            BreadcrumbSeparator::Backslash => " \\ ",
            BreadcrumbSeparator::Arrow => " > ",
            BreadcrumbSeparator::DoubleArrow => " >> ",
            BreadcrumbSeparator::Chevron => " › ",
            BreadcrumbSeparator::DoubleChevron => " » ",
            BreadcrumbSeparator::Bullet => " • ",
            BreadcrumbSeparator::Pipe => " | ",
            BreadcrumbSeparator::Custom(s) => s,
        }
    }
}

/// Properties for the Breadcrumbs component.
#[derive(Debug, Clone)]
pub struct BreadcrumbsProps {
    /// The breadcrumb items.
    pub crumbs: Vec<Crumb>,
    /// Separator between items.
    pub separator: BreadcrumbSeparator,
    /// Color for inactive items.
    pub inactive_color: Option<Color>,
    /// Color for the active/current item.
    pub active_color: Option<Color>,
    /// Color for the separator.
    pub separator_color: Option<Color>,
    /// Whether to bold the active item.
    pub bold_active: bool,
    /// Whether to dim inactive items.
    pub dim_inactive: bool,
    /// Maximum number of crumbs to show (0 = all).
    pub max_items: usize,
    /// Text shown when items are collapsed.
    pub ellipsis: String,
    /// Whether to show a root indicator at the start.
    pub show_root: bool,
    /// Root indicator text.
    pub root_text: String,
}

impl Default for BreadcrumbsProps {
    fn default() -> Self {
        Self {
            crumbs: Vec::new(),
            separator: BreadcrumbSeparator::Slash,
            inactive_color: Some(Color::DarkGray),
            active_color: None,
            separator_color: Some(Color::DarkGray),
            bold_active: true,
            dim_inactive: false,
            max_items: 0,
            ellipsis: "...".into(),
            show_root: false,
            root_text: "~".into(),
        }
    }
}

impl BreadcrumbsProps {
    /// Create new BreadcrumbsProps with items.
    pub fn new<I, T>(crumbs: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Crumb>,
    {
        let mut crumbs: Vec<Crumb> = crumbs.into_iter().map(Into::into).collect();
        // Mark the last item as active by default
        if let Some(last) = crumbs.last_mut() {
            last.active = true;
        }
        Self {
            crumbs,
            ..Default::default()
        }
    }

    /// Create from a path string (splits on /).
    pub fn from_path(path: &str) -> Self {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        Self::new(parts)
    }

    /// Add a crumb.
    #[must_use]
    pub fn crumb(mut self, label: impl Into<String>) -> Self {
        // Clear active from previous last item
        if let Some(last) = self.crumbs.last_mut() {
            last.active = false;
        }
        // Add new item as active
        self.crumbs.push(Crumb::active(label));
        self
    }

    /// Set the separator.
    #[must_use]
    pub fn separator(mut self, separator: BreadcrumbSeparator) -> Self {
        self.separator = separator;
        self
    }

    /// Set the inactive item color.
    #[must_use]
    pub fn inactive_color(mut self, color: Color) -> Self {
        self.inactive_color = Some(color);
        self
    }

    /// Set the active item color.
    #[must_use]
    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = Some(color);
        self
    }

    /// Set the separator color.
    #[must_use]
    pub fn separator_color(mut self, color: Color) -> Self {
        self.separator_color = Some(color);
        self
    }

    /// Enable/disable bold for active item.
    #[must_use]
    pub fn bold_active(mut self, bold: bool) -> Self {
        self.bold_active = bold;
        self
    }

    /// Enable/disable dim for inactive items.
    #[must_use]
    pub fn dim_inactive(mut self, dim: bool) -> Self {
        self.dim_inactive = dim;
        self
    }

    /// Set maximum items to display (0 = all).
    #[must_use]
    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = max;
        self
    }

    /// Set the ellipsis text.
    #[must_use]
    pub fn ellipsis(mut self, text: impl Into<String>) -> Self {
        self.ellipsis = text.into();
        self
    }

    /// Enable/disable root indicator.
    #[must_use]
    pub fn show_root(mut self, show: bool) -> Self {
        self.show_root = show;
        self
    }

    /// Set the root indicator text.
    #[must_use]
    pub fn root_text(mut self, text: impl Into<String>) -> Self {
        self.root_text = text.into();
        self
    }

    /// Get the items to display (handles truncation).
    fn display_items(&self) -> Vec<&Crumb> {
        if self.max_items == 0 || self.crumbs.len() <= self.max_items {
            self.crumbs.iter().collect()
        } else {
            // Keep first item and last (max_items - 1) items
            let mut items = Vec::with_capacity(self.max_items + 1);
            if !self.crumbs.is_empty() {
                items.push(&self.crumbs[0]);
            }
            // Skip items in the middle, take last (max_items - 1)
            let skip = self.crumbs.len() - (self.max_items - 1);
            for crumb in self.crumbs.iter().skip(skip) {
                items.push(crumb);
            }
            items
        }
    }

    /// Check if truncation is needed.
    fn is_truncated(&self) -> bool {
        self.max_items > 0 && self.crumbs.len() > self.max_items
    }

    /// Render the breadcrumbs as a string.
    pub fn render_string(&self) -> String {
        if self.crumbs.is_empty() {
            if self.show_root {
                return self.root_text.clone();
            }
            return String::new();
        }

        let sep = self.separator.as_str();
        let items = self.display_items();
        let truncated = self.is_truncated();

        let mut parts = Vec::new();

        if self.show_root {
            parts.push(self.root_text.clone());
        }

        for (i, crumb) in items.iter().enumerate() {
            // Add ellipsis after first item if truncated
            if truncated && i == 1 {
                parts.push(self.ellipsis.clone());
            }
            parts.push(crumb.label.clone());
        }

        parts.join(sep)
    }
}

/// A component that displays a breadcrumb navigation path.
///
/// # Examples
///
/// ```ignore
/// // From path string
/// Element::node::<Breadcrumbs>(
///     BreadcrumbsProps::from_path("/home/user/documents/file.txt"),
///     vec![]
/// )
///
/// // With builder
/// Element::node::<Breadcrumbs>(
///     BreadcrumbsProps::new(["Home", "Projects", "Blaeck"])
///         .separator(BreadcrumbSeparator::Chevron)
///         .active_color(Color::Cyan),
///     vec![]
/// )
/// ```
pub struct Breadcrumbs;

impl Component for Breadcrumbs {
    type Props = BreadcrumbsProps;

    fn render(props: &Self::Props) -> Element {
        if props.crumbs.is_empty() {
            if props.show_root {
                let mut style = Style::new();
                if props.dim_inactive {
                    style = style.add_modifier(Modifier::DIM);
                }
                if let Some(color) = props.inactive_color {
                    style = style.fg(color);
                }
                return Element::styled_text(&props.root_text, style);
            }
            return Element::text("");
        }

        let sep = props.separator.as_str();
        let items = props.display_items();
        let truncated = props.is_truncated();

        let mut children = Vec::new();

        // Root indicator
        if props.show_root {
            let mut style = Style::new();
            if props.dim_inactive {
                style = style.add_modifier(Modifier::DIM);
            }
            if let Some(color) = props.inactive_color {
                style = style.fg(color);
            }
            children.push(Element::styled_text(&props.root_text, style));

            // Separator after root
            let mut sep_style = Style::new();
            if let Some(color) = props.separator_color {
                sep_style = sep_style.fg(color);
            }
            children.push(Element::styled_text(sep, sep_style));
        }

        for (i, crumb) in items.iter().enumerate() {
            // Add separator before item (except first)
            if i > 0 {
                let mut sep_style = Style::new();
                if let Some(color) = props.separator_color {
                    sep_style = sep_style.fg(color);
                }
                children.push(Element::styled_text(sep, sep_style));

                // Add ellipsis after first separator if truncated
                if truncated && i == 1 {
                    children.push(Element::styled_text(&props.ellipsis, sep_style));
                    children.push(Element::styled_text(sep, sep_style));
                }
            }

            // Add crumb
            let mut style = Style::new();
            if crumb.active {
                if props.bold_active {
                    style = style.add_modifier(Modifier::BOLD);
                }
                if let Some(color) = props.active_color {
                    style = style.fg(color);
                }
            } else {
                if props.dim_inactive {
                    style = style.add_modifier(Modifier::DIM);
                }
                if let Some(color) = props.inactive_color {
                    style = style.fg(color);
                }
            }
            children.push(Element::styled_text(&crumb.label, style));
        }

        // Return as Fragment for proper inline rendering
        Element::Fragment(children)
    }
}

/// Helper function to create breadcrumbs from a path.
pub fn breadcrumbs_path(path: &str) -> String {
    BreadcrumbsProps::from_path(path).render_string()
}

/// Helper function to create breadcrumbs from items.
pub fn breadcrumbs<I, T>(items: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Into<Crumb>,
{
    BreadcrumbsProps::new(items).render_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crumb_new() {
        let crumb = Crumb::new("Home");
        assert_eq!(crumb.label, "Home");
        assert!(!crumb.active);
    }

    #[test]
    fn test_crumb_active() {
        let crumb = Crumb::active("Current");
        assert_eq!(crumb.label, "Current");
        assert!(crumb.active);
    }

    #[test]
    fn test_crumb_from_string() {
        let crumb: Crumb = "Test".into();
        assert_eq!(crumb.label, "Test");
    }

    #[test]
    fn test_separator_slash() {
        assert_eq!(BreadcrumbSeparator::Slash.as_str(), " / ");
    }

    #[test]
    fn test_separator_chevron() {
        assert_eq!(BreadcrumbSeparator::Chevron.as_str(), " › ");
    }

    #[test]
    fn test_separator_custom() {
        let sep = BreadcrumbSeparator::Custom(" -> ");
        assert_eq!(sep.as_str(), " -> ");
    }

    #[test]
    fn test_breadcrumbs_props_new() {
        let props = BreadcrumbsProps::new(["Home", "Projects", "Blaeck"]);
        assert_eq!(props.crumbs.len(), 3);
        // Last item should be active
        assert!(props.crumbs[2].active);
        assert!(!props.crumbs[0].active);
    }

    #[test]
    fn test_breadcrumbs_props_from_path() {
        let props = BreadcrumbsProps::from_path("/home/user/docs");
        assert_eq!(props.crumbs.len(), 3);
        assert_eq!(props.crumbs[0].label, "home");
        assert_eq!(props.crumbs[1].label, "user");
        assert_eq!(props.crumbs[2].label, "docs");
    }

    #[test]
    fn test_breadcrumbs_props_builder() {
        let props = BreadcrumbsProps::new(Vec::<&str>::new())
            .crumb("Home")
            .crumb("Projects")
            .separator(BreadcrumbSeparator::Chevron)
            .active_color(Color::Cyan);

        assert_eq!(props.crumbs.len(), 2);
        assert_eq!(props.separator, BreadcrumbSeparator::Chevron);
        assert_eq!(props.active_color, Some(Color::Cyan));
    }

    #[test]
    fn test_breadcrumbs_render_string() {
        let props = BreadcrumbsProps::new(["Home", "Projects", "Blaeck"]);
        let result = props.render_string();
        assert_eq!(result, "Home / Projects / Blaeck");
    }

    #[test]
    fn test_breadcrumbs_render_string_chevron() {
        let props = BreadcrumbsProps::new(["A", "B", "C"]).separator(BreadcrumbSeparator::Chevron);
        let result = props.render_string();
        assert_eq!(result, "A › B › C");
    }

    #[test]
    fn test_breadcrumbs_render_empty() {
        let props = BreadcrumbsProps::new(Vec::<&str>::new());
        assert_eq!(props.render_string(), "");
    }

    #[test]
    fn test_breadcrumbs_render_with_root() {
        let props = BreadcrumbsProps::new(["home", "user"]).show_root(true);
        let result = props.render_string();
        assert_eq!(result, "~ / home / user");
    }

    #[test]
    fn test_breadcrumbs_truncation() {
        let props =
            BreadcrumbsProps::new(["a", "b", "c", "d", "e", "f"]).max_items(3);
        let result = props.render_string();
        // Should show: a / ... / e / f
        assert!(result.contains("a"));
        assert!(result.contains("..."));
        assert!(result.contains("f"));
    }

    #[test]
    fn test_breadcrumbs_helper_path() {
        let result = breadcrumbs_path("/home/user/docs");
        assert!(result.contains("home"));
        assert!(result.contains("docs"));
    }

    #[test]
    fn test_breadcrumbs_helper() {
        let result = breadcrumbs(["Home", "Docs"]);
        assert_eq!(result, "Home / Docs");
    }

    #[test]
    fn test_breadcrumbs_component_render() {
        let props = BreadcrumbsProps::new(["A", "B"]);
        let elem = Breadcrumbs::render(&props);
        assert!(elem.is_fragment());
    }

    #[test]
    fn test_breadcrumbs_component_render_empty() {
        let props = BreadcrumbsProps::new(Vec::<&str>::new());
        let elem = Breadcrumbs::render(&props);
        assert!(elem.is_text());
    }
}
