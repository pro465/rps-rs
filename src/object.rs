use std::convert::Infallible;
use std::convert::TryFrom;

const OBJ_ARR: [Object; 3] = [Object::Rock, Object::Paper, Object::Scissor];

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Object {
    Rock,
    Paper,
    Scissor,
}

impl TryFrom<usize> for Object {
    type Error = Infallible;

    fn try_from(obj: usize) -> Result<Object, Self::Error> {
        Ok(OBJ_ARR[obj])
    }
}

impl TryFrom<&str> for Object {
    type Error = &'static str;

    fn try_from(obj: &str) -> Result<Object, Self::Error> {
        match obj {
             "rock" | "r" => Ok(Object::Rock),
             "paper" | "p" => Ok(Object::Paper),
             "scissor" | "s" => Ok(Object::Scissor),
             _ => Err("unexpected string.\nTry again using rock, paper or scissor;\n or their shortened forms: r,p or s")
         }
    }
}
