pub enum Attr {
    KeyValue(String, String),
    Bool(String, bool),
}

macro_rules! key_value {
    ($x:ident) => {
        #[allow(non_snake_case)]
        pub fn $x<S: Into<String>>(value: S) -> Attr {
            Attr::KeyValue(stringify!($x).to_owned(), value.into())
        }
    };

    ( $( $x:ident ),* ) => {
        $(
            key_value!($x);
        )*
    }
}

macro_rules! bool_attribute {
    ($x:ident) => {
        #[allow(non_snake_case)]
        pub fn $x(conditional: bool) -> Attr {
            Attr::Bool(stringify!($x).to_owned(), conditional)
        }
    };

    ( $( $x:ident ),* ) => {
        $(
            bool_attribute!($x);
        )*
    }
}

impl Attr {
    // loop, for, type are keywords and can't be defined as functions
    bool_attribute!(
        async,
        autofocus,
        checked,
        compact,
        declare,
        default,
        defer,
        disabled,
        formnovalidate,
        hidden,
        ismap,
        multiple,
        noresize,
        novalidate,
        readonly,
        required,
        reversed,
        seamless,
        selected
    );

    key_value!(
        accesskey,
        align,
        attribute,
        challenge,
        charset,
        cite,
        colspan,
        content,
        contenteditable,
        contextmenu,
        datetime,
        dir,
        draggable,
        dropzone,
        headers,
        httpEquiv,
        itemprop,
        keytype,
        kind,
        lang,
        language,
        manifest,
        poster,
        property,
        pubdate,
        rowspan,
        sandbox,
        scope,
        scoped,
        spellcheck,
        srcdoc,
        srclang,
        start,
        tabindex,
        preload,
        alt,
        autoplay,
        cols,
        controls,
        coords,
        download,
        downloadAs,
        height,
        href,
        hreflang,
        max,
        media,
        min,
        ping,
        rel,
        rows,
        shape,
        src,
        step,
        target,
        usemap,
        width,
        wrap,
        form,
        accept,
        acceptCharset,
        action,
        autocomplete,
        autosave,
        class,
        classList,
        enctype,
        formaction,
        id,
        key,
        list,
        maxlength,
        method,
        minlength,
        name,
        pattern,
        placeholder,
        size,
        style,
        title,
        value
    );

    pub fn key_value<S: Into<String>>(key: S, value: S) -> Attr {
        Attr::KeyValue(key.into(), value.into())
    }

    pub fn boolean<S: Into<String>>(key: S, conditional: bool) -> Attr {
        Attr::Bool(key.into(), conditional)
    }

    pub fn render(&self) -> Option<String> {
        match *self {
            Attr::KeyValue(ref key, ref value) => {
                Some(format!("{}=\"{}\"", key, value))
            },
            Attr::Bool(ref key, ref conditional) => {
                if *conditional {
                    Some(key.clone())
                } else {
                    None
                }
            },
        }
    }
}
