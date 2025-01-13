use maud::Markup;
use rocket::{http::{ContentType, Header}, response::{content::{self, RawHtml}, Responder}};

pub type Hxh = HXResponder<String>;

#[derive(Responder)]
pub struct HXResponder<T> {
    inner: content::RawHtml<T>,
    header: Header<'static>,
}

impl Hxh {
    pub fn new(header: impl Into<String>, html: Option<content::RawHtml<String>>) -> Self {
        let header: String = header.into();
        let inner_html = html.unwrap_or(content::RawHtml("".to_string()));    
        HXResponder {
            inner: inner_html,
            header: HXHeader(header).into(),
        }
    }
    
    pub fn only_header(header: impl Into<String>) -> Self {
        let header: String = header.into();
        Hxh::new(header, None)
    }
    
    pub fn many(many: Vec<impl Into<String>>, html: Option<content::RawHtml<String>>) -> Self {
        let s: Vec<String> = many
            .into_iter()
            .map(|s| {
                let s: String = s.into();
                s
            })
            .collect();

        let pure = s.join(",");
        Self::new(pure, html)
    }
}

impl From<Hxh> for RawHtml<String> {
    fn from(hx: Hxh) -> Self {
        RawHtml(hx.inner.0)
    }
}


struct HXHeader(String);

impl From<HXHeader> for Header<'static> {
    #[inline(always)]
    fn from(hx: HXHeader) -> Self {
        Header::new("HX-Trigger", hx.0)
    }
}

pub fn hx_event(event: impl Into<String>) -> String {
   format!("{} from:body", event.into())
}
 