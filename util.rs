
pub fn ident_to_str<'a>(ident: ::libsyn::Ident) -> &'a str {
    // get_ident returns an InternedString
    ::libsyn::get_ident(ident).get()
}
