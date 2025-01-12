use rocket::http::Header;

pub type Hxh = HXResponder<&'static str>;

#[derive(Responder)]
pub struct HXResponder<T> {
    inner: T,
    header: Header<'static>,
}   

impl Hxh  {
    pub fn new(inner: &'static str) -> Self {
        HXResponder {
            inner,
            header: HXHeader(inner.to_string()).into(),
        }
    }
}

struct HXHeader(String);

impl From<HXHeader> for Header<'static> {
    #[inline(always)]
    fn from(hx: HXHeader) -> Self {
        Header::new("HX-Trigger", hx.0)
    }
}