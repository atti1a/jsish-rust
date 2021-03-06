use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    BopPlus,
    BopMinus,
    BopTimes,
    BopDivide,
    BopMod,
    BopEq,
    BopNe,
    BopLt,
    BopGt,
    BopGe,
    BopLe,
    BopAnd,
    BopOr,
    BopComma
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::BinaryOperator::*;
        let c = match *self {
            BopPlus => "+",
            BopMinus => "-",
            BopTimes => "*",
            BopDivide => "/",
            BopMod => "%",
            BopEq => "==",
            BopNe => "!=",
            BopLt => "<",
            BopLe => "<=",
            BopGt => ">",
            BopGe => ">=",
            BopAnd => "&&",
            BopOr => "||",
            BopComma => ","
        };

        write!(f, "{}", c)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    UopNot,
    UopTypeof,
    UopMinus
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::UnaryOperator::*;
        let c = match *self {
            UopNot => "!",
            UopTypeof => "typeof ",
            UopMinus => "-"
        };

        write!(f, "{}", c)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclInitData {
    pub id: String,
    pub src: Box<Expression>
}

#[derive(Clone, Debug, PartialEq)]
pub enum Declaration {
    DeclId(String),
    DeclInit(DeclInitData)
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Declaration::*;
        match *self {
            DeclId(ref s) => write!(f, "{}", s),
            DeclInit(DeclInitData { ref id, ref src }) =>
                write!(f, "{} = {}", id, src)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpBinaryData {
    pub opr: BinaryOperator,
    pub lft: Box<Expression>,
    pub rht: Box<Expression>
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpUnaryData {
    pub opr: UnaryOperator,
    pub opnd: Box<Expression>
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpCondData {
    pub guard: Box<Expression>,
    pub then_exp: Box<Expression>,
    pub else_exp: Box<Expression>
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpAssignData {
    pub lft: Box<Expression>,
    pub rht: Box<Expression>
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    ExpId(String),
    ExpNum(i64),
    ExpString(String),
    ExpTrue,
    ExpFalse,
    ExpUndefined,
    ExpBinary(ExpBinaryData),
    ExpUnary(ExpUnaryData),
    ExpCond(ExpCondData),
    ExpAssign(ExpAssignData)
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::Expression::*;
        match *self {
            ExpId(ref s) => write!(f, "{}", s),
            ExpNum(ref n) => write!(f, "{}", n),
            ExpString(ref s) => write!(f, "{:?}", s),
            ExpTrue => write!(f, "true"),
            ExpFalse => write!(f, "false"),
            ExpUndefined => write!(f, "undefined"),
            ExpBinary(ExpBinaryData {ref opr, ref lft, ref rht}) =>
                write!(f, "({} {} {})", lft, opr, rht),
            ExpUnary(ExpUnaryData {ref opr, ref opnd}) =>
                write!(f, "({}{})", opr, opnd),
            ExpCond(ExpCondData {ref guard, ref then_exp, ref else_exp}) =>
                write!(f, "({} ? {} : {})", guard, then_exp, else_exp),
            ExpAssign(ExpAssignData {ref lft, ref rht}) =>
                write!(f, "({} = {})", lft, rht),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StIfData {
    pub guard: Expression,
    pub th: Box<Statement>,
    pub el: Box<Statement>
}

#[derive(Clone, Debug, PartialEq)]
pub struct StWhileData {
    pub guard: Expression,
    pub body: Box<Statement>
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    StExp(Expression),
    StPrint(Expression),
    StBlock(Vec<Statement>),
    StIf(StIfData),
    StWhile(StWhileData)
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::Statement::*;
        match *self {
            StExp(ref exp) => write!(f, "{};", exp),
            StPrint(ref exp) => write!(f, "print {};", exp),
            StBlock(ref list) => {
                write!(f, "{}", "{\n")?;
                for stmt in list {
                    write!(f, "{}\n", stmt)?;
                }
                write!(f, "{}", "}")
            }
            StIf(StIfData { ref guard, ref th, ref el }) =>
                write!(f, "if ({})\n{}\nelse\n{}", guard, th, el),
            StWhile(StWhileData { ref guard, ref body }) =>
                write!(f, "while ({})\n{}", guard, body),
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum SourceElement {
    Stmt(Statement),
    VarDecl(Vec<Declaration>)
}

impl fmt::Display for SourceElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::SourceElement::*;
        match *self {
            Stmt(ref s) => write!(f, "{}", s),
            VarDecl(ref decls) => {
                write!(f, "var ")?;
                for decl in decls {
                    write!(f, "{}, ", decl)?;
                }
                write!(f, "\n")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Program {
    Prog(Vec<SourceElement>)
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Program::*;
        let Prog(ref se_list) = *self;

        for ref se in se_list {
            write!(f, "{}\n", se)?;
        }

        Ok(())
    }
}
