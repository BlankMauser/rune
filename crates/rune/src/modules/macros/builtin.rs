//! Built-in macros.

use crate as rune;
#[cfg(feature = "macros")]
use crate::compile;
#[cfg(feature = "macros")]
use crate::macros::{quote, MacroContext, TokenStream};
#[cfg(feature = "macros")]
use crate::parse::Parser;
use crate::{ContextError, Module};

/// Built-in macros.
#[rune::module(::std::macros::builtin)]
pub fn module() -> Result<Module, ContextError> {
    #[cfg_attr(not(feature = "macros"), allow(unused_mut))]
    let mut builtins = Module::from_meta(self::module_meta)?.with_unique("std::macros::builtin");
    #[cfg(feature = "macros")]
    builtins.macro_meta(file)?;
    #[cfg(feature = "macros")]
    builtins.macro_meta(line)?;
    Ok(builtins)
}

/// Return the line in the current file.
///
/// # Examples
///
/// ```rune
/// println!("{}:{}: Something happened", file!(), line!());
/// ```
#[cfg(feature = "macros")]
#[rune::macro_]
pub(crate) fn line(
    cx: &mut MacroContext<'_, '_, '_>,
    stream: &TokenStream,
) -> compile::Result<TokenStream> {
    use crate as rune;

    let mut parser = Parser::from_token_stream(stream, cx.input_span());
    parser.eof()?;

    let stream = quote!(
        #[builtin]
        line!()
    );

    Ok(stream.into_token_stream(cx)?)
}

/// Return the name of the current file.
///
/// # Examples
///
/// ```rune
/// println!("{}:{}: Something happened", file!(), line!());
/// ```
#[cfg(feature = "macros")]
#[rune::macro_]
pub(crate) fn file(
    cx: &mut MacroContext<'_, '_, '_>,
    stream: &TokenStream,
) -> compile::Result<TokenStream> {
    use crate as rune;

    let mut parser = Parser::from_token_stream(stream, cx.input_span());
    parser.eof()?;

    let stream = quote!(
        #[builtin]
        file!()
    );

    Ok(stream.into_token_stream(cx)?)
}
