use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub fn parse_sql(query: &str) -> Result<sqlparser::ast::Statement, sqlparser::parser::ParserError> {
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, query)?;
    Ok(ast[0].clone())
}