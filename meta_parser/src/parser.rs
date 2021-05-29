use nom::branch::{alt, permutation};
use nom::bytes::streaming::{tag_no_case, take_till};
use nom::character::streaming::{char, multispace0, multispace1};
use nom::combinator::not;
use nom::sequence::{delimited, preceded};
use nom::{error::ParseError, Err, IResult};

#[derive(Debug, Clone, Copy)]
pub struct MetaTag<'a> {
    pub(super) name: &'a [u8],
    pub(super) content: &'a [u8],
}

#[derive(Debug, Clone, Copy)]
pub struct TitleTag<'a> {
    pub(super) title: &'a [u8],
}

#[derive(Debug, Clone, Copy)]
pub enum Tag<'a> {
    Meta(MetaTag<'a>),
    Title(TitleTag<'a>),
}

/// matches `<`
fn open_bracket<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = char('<')(input)?;
    Ok((rest, ()))
}

/// matches `>`
fn close_bracket<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = char('>')(input)?;
    Ok((rest, ()))
}

/// matches `/`
fn slash<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = char('/')(input)?;
    Ok((rest, ()))
}

/// matches `=`
fn equals<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = char('=')(input)?;
    Ok((rest, ()))
}

/// matches `'`
fn single_quote<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = char('\'')(input)?;
    Ok((rest, ()))
}

/// matches `"`
fn double_quote<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = char('"')(input)?;
    Ok((rest, ()))
}

/// matches `meta` (case insensitive)
fn keyword_meta<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = tag_no_case("meta")(input)?;
    Ok((rest, ()))
}

/// matches `name` (case insensitive)
fn keyword_name<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = tag_no_case("name")(input)?;
    Ok((rest, ()))
}

/// matches `property` (case insensitive)
fn keyword_property<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = tag_no_case("property")(input)?;
    Ok((rest, ()))
}

/// matches `content` (case insensitive)
fn keyword_content<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = tag_no_case("content")(input)?;
    Ok((rest, ()))
}

/// matches `title` (case insensitive)
fn keyword_title<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let (rest, _) = tag_no_case("title")(input)?;
    Ok((rest, ()))
}

/// matches `< *{meta}`
fn open_meta_tag<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    preceded(preceded(open_bracket, multispace0), keyword_meta)(input)
}

/// matches `/>` or `>`
fn close_tag<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    alt((preceded(slash, close_bracket), close_bracket))(input)
}

/// matches single quoted text
fn singel_quoted_text<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], &'a [u8], E> {
    let text = take_till(|i| i == b'\'');
    delimited(single_quote, text, single_quote)(input)
}

/// matches double quoted text
fn double_quoted_text<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], &'a [u8], E> {
    let text = take_till(|i| i == b'"');
    delimited(double_quote, text, double_quote)(input)
}

/// matches single or double quoted text
fn quoted_text<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], &'a [u8], E> {
    alt((singel_quoted_text, double_quoted_text))(input)
}

/// matches unquoted text
fn unqouted_text<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], &[u8], E> {
    take_till(|i| i == b' ')(input)
}

/// matches either quoted or unquoted text
fn text<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], &[u8], E> {
    alt((quoted_text, unqouted_text))(input)
}

/// matches an un-interesting key-value pair, i.e. `xxx=yyy`
fn uninteresting_kw_pair<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let not_name = not(keyword_name);
    let not_property = not(keyword_property);
    let not_content = not(keyword_content);
    let key = take_till(|i| i == b'=' || i == b'>');
    let uninteresting_key = preceded(not_name, preceded(not_property, preceded(not_content, key)));
    delimited(uninteresting_key, equals, text)(input)
}

/// matches and discards white-space (and uninteresting key value pairs)
fn tag_white_space<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], (), E> {
    let ws =
        |i: &'a [u8]| -> IResult<&'a [u8], (), E> { multispace0(i).map(|(rest, _)| (rest, ())) };
    let ws_pair = preceded(multispace0, uninteresting_kw_pair);
    let mut ws_or_kw = alt((ws_pair, ws));
    let mut rest = input;
    loop {
        match ws_or_kw(rest) {
            Ok((r, ())) => {
                if rest == r {
                    return Ok((rest, ()));
                } else {
                    rest = r
                }
            }
            Err(_) => return Ok((rest, ())),
        }
    }
}

/// Parses a single `<meta ... />` tag.
fn meta_tag<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], MetaTag<'a>, E> {
    let name_kw = alt((keyword_name, keyword_property));
    let name = preceded(preceded(name_kw, equals), text);
    let content = preceded(preceded(keyword_content, equals), text);

    let name_ws = preceded(tag_white_space, name);
    let content_ws = preceded(tag_white_space, content);

    let name_content = permutation((name_ws, content_ws));

    let open = preceded(open_meta_tag, multispace1);
    let close = preceded(multispace0, close_tag);

    let (rest, (name, content)) = delimited(open, name_content, close)(input)?;
    Ok((rest, MetaTag { name, content }))
}

/// Parses a single `<title>...</title>` tag.
fn title_tag<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], TitleTag<'a>, E> {
    let title_open_ws = preceded(keyword_title, tag_white_space);
    let open_tag = delimited(open_bracket, title_open_ws, close_bracket);

    let title_close_ws = delimited(multispace0, keyword_title, multispace0);
    let close_tag = delimited(preceded(open_bracket, slash), title_close_ws, close_bracket);

    let child_text = take_till(|i| i == b'<');

    let (rest, title) = delimited(open_tag, child_text, close_tag)(input)?;
    Ok((rest, TitleTag { title }))
}

/// Parses either a meta, or a title tag.
pub fn run<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&[u8], Tag<'a>, E> {
    meta_tag(input)
        .map(|(rest, tag)| (rest, Tag::Meta(tag)))
        .or_else(|_: Err<E>| title_tag(input).map(|(rest, tag)| (rest, Tag::Title(tag))))
}
