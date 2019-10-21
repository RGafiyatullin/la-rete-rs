use json_utils::json::JsNumber;

pub trait IntoJsNumber {
    fn into_js_number(self) -> JsNumber;
}

impl IntoJsNumber for JsNumber {
    fn into_js_number(self) -> JsNumber {
        self
    }
}

impl IntoJsNumber for usize {
    fn into_js_number(self) -> JsNumber {
        self.into()
    }
}
impl IntoJsNumber for u8 {
    fn into_js_number(self) -> JsNumber {
        self.into()
    }
}
impl IntoJsNumber for u16 {
    fn into_js_number(self) -> JsNumber {
        self.into()
    }
}
impl IntoJsNumber for u32 {
    fn into_js_number(self) -> JsNumber {
        self.into()
    }
}
impl IntoJsNumber for u64 {
    fn into_js_number(self) -> JsNumber {
        self.into()
    }
}

impl IntoJsNumber for i8 {
    fn into_js_number(self) -> JsNumber {
        self.into()
    }
}
impl IntoJsNumber for i16 {
    fn into_js_number(self) -> JsNumber {
        self.into()
    }
}
impl IntoJsNumber for i32 {
    fn into_js_number(self) -> JsNumber {
        self.into()
    }
}
impl IntoJsNumber for i64 {
    fn into_js_number(self) -> JsNumber {
        self.into()
    }
}
