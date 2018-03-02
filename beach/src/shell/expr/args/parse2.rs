use std::str::FromStr;
use std::marker::PhantomData;

use frunk::hlist::*;
use frunk::coproduct::*;
use void::Void;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Err<E> {
    MissingArgument,
    Other(E)
}

impl<E> Err<E> {
    pub fn map<F, O>(self, f: F) -> Err<O>
    where F : FnOnce(E) -> O {
        match self {
            Err::MissingArgument => Err::MissingArgument,
            Err::Other(err) => Err::Other(f(err))
        }
    }
}

impl<E> From<E> for Err<E> {
    fn from(err: E) -> Err<E> {
        Err::Other(err)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Args<'a> {
    ptr: usize,
    vec: Vec<&'a str>
}

impl<'a> Args<'a> {
    pub fn new<S>(args: &'a [S]) -> Args<'a>
    where S: AsRef<str>
    {
        Args {
            ptr: 0,
            vec: args.iter().map(|s| s.as_ref()).collect()
        }
    }

    pub fn pop<E>(&mut self) -> Result<&'a str, Err<E>> {
        if self.ptr < self.vec.len() {
            let res = Ok(self.vec[self.ptr]);
            self.ptr += 1;
            res
        } else {
            Err(Err::MissingArgument)
        }
    }
}

pub trait ParseArg {
    type Arg;
    type Err;
    fn parse_arg(args: &mut Args) -> Result<Self::Arg, Err<Self::Err>>;
}

pub enum Nat {}

impl ParseArg for Nat {
    type Arg = u8;
    type Err = <u8 as FromStr>::Err;
    fn parse_arg(args: &mut Args) -> Result<Self::Arg, Err<Self::Err>> {
        let arg = args.pop()?;
        let res = u8::from_str(arg)?;
        Ok(res)
    }
}

pub enum Text {}

impl ParseArg for Text {
    type Arg = String;
    type Err = Void;
    fn parse_arg(args: &mut Args) -> Result<Self::Arg, Err<Self::Err>> {
        args.pop().map(String::from)
    }
}

pub struct Optional<A> {
    _phantom: PhantomData<A>
}

impl<A: ParseArg> ParseArg for Optional<A> {
    type Arg = Option<A::Arg>;
    type Err = A::Err;
    fn parse_arg(args: &mut Args) -> Result<Self::Arg, Err<Self::Err>> {
        let old_ptr = args.ptr;
        let res = match A::parse_arg(args) {
            Err(_) => {
                args.ptr = old_ptr;
                None
            }
            Ok(arg) => Some(arg)
        };
        Ok(res)
    }
}


pub trait Parse: HList {
    type List;
    type Err;
    fn parse(args: Args) -> Result<Self::List, Err<Self::Err>>;
}

impl Parse for HNil {
    type List = HNil;
    type Err  = CNil;
    fn parse(_: Args) -> Result<Self::List, Err<Self::Err>> {
        Ok(HNil)
    }
}

impl<H: ParseArg, T: Parse> Parse for HCons<H, T> {
    type List = HCons<H::Arg, T::List>;
    type Err  = Coproduct<H::Err, T::Err>;
    fn parse(mut args: Args) -> Result<Self::List, Err<Self::Err>> {
        let head = H::parse_arg(&mut args).map_err(|e| e.map(Coproduct::Inl))?;
        let tail = T::parse(args).map_err(|err| err.map(Coproduct::Inr))?;
        Ok(HCons { head, tail })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_none() {
        let vec : Vec<&str> = vec![];
        let args = Args::new(&vec);
        assert_eq!(<Hlist![]>::parse(args), Ok(hlist![]));
    }

    #[test]
    fn parse_many() {
        let vec = vec!["foobar", "2", "not-a-number"];
        let args = Args::new(&vec);
        assert_eq!(
            <Hlist![Text, Nat, Optional<Nat>, Text]>::parse(args),
            Ok(hlist!["foobar".to_string(), 2, None, "not-a-number".to_string()])
        );
    }
}
