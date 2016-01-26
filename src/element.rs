use attribute::Attr;

pub trait Renderable {
    fn render(&self) -> String;
}

pub struct Element<T: Renderable> {
    tag: String,
    attributes: Vec<Attr>,
    child: T
}

impl<T: Renderable> Renderable for Element<T> {
    fn render(&self) -> String {
        format!("<{}>{}</{}>", self.open_tag(), self.child.render(), self.tag.clone())
    }
}

impl Renderable for String {
    fn render(&self) -> String {
        self.clone()
    }
}

impl<'a> Renderable for &'a str {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl<T: Renderable> Element<T> {
    pub fn new<S: Into<String>>(tag: S, attributes: Vec<Attr>, child: T) -> Element<T> {
        Element {
            tag: tag.into(),
            attributes: attributes,
            child: child
        }
    }

    fn open_tag(&self) -> String {
        let attribute_list: Vec<String> = self.clone().attributes
            .iter()
            .filter_map(|attribute| attribute.render())
            .collect();

        if attribute_list.is_empty() {
            self.tag.clone()
        } else {
            format!("{} {}", self.tag.clone(), attribute_list.join(""))
        }
    }
}

impl<T: Renderable> Renderable for Vec<Element<T>> {
    fn render(&self) -> String {
        let rendered: Vec<String> = self.into_iter().map(|element| -> String {
            element.render()
        }).collect();

        rendered.join("")
    }
}

#[test]
fn it_can_render_single_elements() {
    let element = Element::new("div", vec![], "foo");
    assert_eq!(element.render(), "<div>foo</div>");
}

#[test]
fn it_can_render_vecs_of_elements() {
    let element = Element::new("div", vec![], vec![
        Element::new("h1", vec![], "Hello"),
        Element::new("p", vec![], "This is neat, huh?")
    ]);

    assert_eq!(element.render(), "<div><h1>Hello</h1><p>This is neat, huh?</p></div>");
}

#[test]
fn it_can_render_attributes() {
    let element = Element::new("div", vec![Attr::id("awesome")], "foo".to_string());
    assert_eq!(element.render(), "<div id=\"awesome\">foo</div>");
}

#[test]
fn it_can_render_boolean_attributes() {
    let element = Element::new("div", vec![Attr::disabled(false)], "foo".to_string());
    assert_eq!(element.render(), "<div>foo</div>");

    let element = Element::new("div", vec![Attr::disabled(true)], "foo".to_string());
    assert_eq!(element.render(), "<div disabled>foo</div>");
}

#[test]
fn it_can_still_use_rust_reserved_keywords_for_attribites() {
    let element = Element::new("label", vec![Attr::key_value("for", "name")], "");
    assert_eq!(element.render(), "<label for=\"name\"></label>");
}
