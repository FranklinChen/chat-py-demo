// -*-text-*-

%start Chat
%expect-unused Unmatched "UNMATCHED"

%%

Chat -> Chat<'input>:
    StarTop { Chat { tops: $1 } }
;

StarTop -> Vec<Top<'input>>:
    { vec![] }
  | StarTop Top "NEWLINE" {
    let mut result = $1;
    result.push($2);
    result
  }
;

Top -> Top<'input>:
    "HEADER" { Top::Header($lexer.span_str($1.unwrap().span())) }
  | "MAIN_TIER" { Top::MainTier($lexer.span_str($1.unwrap().span())) }
  | "DEPENDENT_TIER" { Top::DependentTier($lexer.span_str($1.unwrap().span())) }
;

Unmatched -> ():
    "UNMATCHED" {}
;

%%

use crate::ast::*;
