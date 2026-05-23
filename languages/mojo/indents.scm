; Mojo indentation queries
; Based on Python indentation with Mojo extensions

[
  (function_definition)
  (struct_definition)
  (if_statement)
  (elif_clause)
  (else_clause)
  (for_statement)
  (while_statement)
  (try_statement)
  (except_clause)
  (finally_clause)
  (with_statement)
] @indent.begin

[
  ")"
  "]"
  "}"
] @indent.branch

[
  (return_statement)
  (break_statement)
  (continue_statement)
  (raise_statement)
  (pass_statement)
] @indent.dedent

(comment) @indent.ignore
