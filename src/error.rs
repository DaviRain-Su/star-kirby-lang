use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknow expression: `{0}`")]
    UnknownExpression(String),
    #[error("unknow statement: `{0}`")]
    UnknowStatement(String),
    #[error("no found build in function: `{0}`")]
    NoFoundBuildInFunction(String),
    #[error("downcast_ref Program Error")]
    DownCastRefProgramError,
    #[error("downcast_ref Statement Error")]
    DownCastRefStatementError,
    #[error("downcast_ref Expression Statement Error")]
    DownCastRefExpressionStatementError,
    #[error("downcast_ref Return Statement Error")]
    DownCastRefReturnStatementError,
    #[error("downcast_ref LetStatement Error")]
    DownCastRefLetStatementError,
    #[error("downcast_ref Expression Error")]
    DownCastRefExpressionError,
    #[error("downcast_ref Prefix Expression Error")]
    DownCastRefPrefixExpressionError,
    #[error("downcast_ref Infix Expression Error")]
    DownCastRefInfixExpressionError,
    #[error("downcast_ref Ast Integer Literal Error")]
    DownCastRefAstIntegerLiteralError,
    #[error("downcast_ref Function Literal Error")]
    DownCastRefFunctionLiteralError,
    #[error("downcast_ref Ast Boolean Error")]
    DownCastRefAstBooleanError,
    #[error("downcast_ref Block Statement Error")]
    DownCastRefBlockStatementError,
    #[error("downcast_ref If Expression Error")]
    DownCastRefIfExpressionError,
    #[error("downcast_ref Identifier Error")]
    DownCastRefIdentifierError,
    #[error("downcast_ref CallExpression Error")]
    DownCastRefCallExpressionError,
    #[error("downcast_ref StringLiteral Error")]
    DownCastRefStringLiteralError,
    #[error("downcast_ref ArrayLiteral Error")]
    DownCastRefArrayLiteralError,
    #[error("downcast_ref IndexExpression Error")]
    DownCastRefIndexExpressionError,
    #[error("downcast_ref HashLiteral Error")]
    DownCastRefHashLiteralError,
    #[error("downcast_ref Boolean Error")]
    DownCastRefBooleanError,
    #[error("Unknown Type Error,  This type_id is (`{0}`)")]
    UnknownTypeError(String),
    #[error("downcast_ref Object Error")]
    DownCastRefObjectError,
    #[error("not a function: `{0}`")]
    NoFunction(String),
    #[error("unknown operator: left(`{left}`),  operator(`{operator}`), right(`{right}`)")]
    UnknownOperator {
        left: String,
        operator: String,
        right: String,
    },
    #[error("index operator not supported: `{0}`")]
    IndexOperatorNotSupported(String),
    #[error("Not Array Type")]
    NotArrayType,
    #[error("Not Integer Type")]
    NotIntegerType,
    #[error("identifier not found: `{0}`")]
    IdentifierNotFound(String),
    #[error("read char error")]
    ReadCharError,
    #[error("read identifier error")]
    ReadIdentifierError,
    #[error("read number error")]
    ReadNumberError,
    #[error("unknown Object type")]
    UnknownObjectType,
    #[error("wrong number of arguments. got=`{got}`, want=`{want}`")]
    WrongNumberOfArguments { got: usize, want: usize },
    #[error("argument to `len` not supported, got `{got}`")]
    ArgumentNotSupported { got: String },
    #[error("argument to `first` must ARRAY, got `{got}`")]
    ArgumentFirstMustArray { got: String },
    #[error("Cannot find `{ty}` token type")]
    CannotFindTokenType { ty: String },
    #[error("no prefix parse function for `{0}` found")]
    NoPrefixParseFunctionFound(String),
    #[error("Expect COLON Error")]
    ExpectColonError,
    #[error("Expect RBRACE and COMMA Error")]
    ExpectBraceAndCommaError,
    #[error("Expect RBRACE Error")]
    ExpectRbraceError,
    #[error("expected next token be `{expected}`, got `{got}` instead")]
    ExpectNextToken { expected: String, got: String },
}
