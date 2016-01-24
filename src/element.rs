pub trait Renderable {
    fn render(&self) -> String;
}

pub struct Element<T: Renderable> {
    tag: String,
    attributes: Option<Vec<(String, String)>>,
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

impl<T: Renderable> Element<T> {
    pub fn new(tag: &str, attributes: Option<Vec<(String, String)>>, child: T) -> Element<T> {
        let tag_name = tag.to_owned();

        Element {
            tag: tag_name,
            attributes: attributes,
            child: child
        }
    }

    fn open_tag(&self) -> String {
        match self.attributes.clone() {
            Some(attributes) => {
                let attribute_list: Vec<String> = attributes.iter().map(|tuple| -> String {
                    format!("{}=\"{}\"", tuple.0, tuple.1)
                }).collect();

                format!("{} {}", self.tag.clone(), attribute_list.join(""))
            }

            None => {
                self.tag.clone()
            }
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
    let element = Element::new("div", None, "foo".to_string());
    assert_eq!(element.render(), "<div>foo</div>");
}

#[test]
fn it_can_render_vecs_of_elements() {
    let element = Element::new("div", None, vec![
        Element::new("h1", None, "Hello".to_owned()),
        Element::new("p", None, "This is neat, huh?".to_owned())
    ]);

    assert_eq!(element.render(), "<div><h1>Hello</h1><p>This is neat, huh?</p></div>");
}

#[test]
fn it_can_render_attributes() {
    let element = Element::new("div", Some(vec![("id".to_owned(), "awesome".to_owned())]), "foo".to_string());
    assert_eq!(element.render(), "<div id=\"awesome\">foo</div>");
}
