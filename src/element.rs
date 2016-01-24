pub trait Renderable {
    fn render(&self) -> String;
}

pub struct Element<T: Renderable> {
    tag: String,
    child: T
}

impl<T: Renderable> Renderable for Element<T> {
    fn render(&self) -> String {
        format!("<{}>{}</{}>", self.tag.clone(), self.child.render(), self.tag.clone())
    }
}

impl Renderable for String {
    fn render(&self) -> String {
        self.clone()
    }
}

impl<T: Renderable> Element<T> {
    pub fn new(tag: &str, child: T) -> Element<T> {
        let tag_name = tag.to_owned();

        Element {
            tag: tag_name,
            child: child
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
    let element = Element::new("div", "foo".to_string());
    assert_eq!(element.render(), "<div>foo</div>");
}

#[test]
fn it_can_render_vecs_of_elements() {
    let element = Element::new("div", vec![
        Element::new("h1", "Hello".to_owned()),
        Element::new("p", "This is neat, huh?".to_owned())
    ]);

    assert_eq!(element.render(), "<div><h1>Hello</h1><p>This is neat, huh?</p></div>");
}
