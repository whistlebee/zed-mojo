; Catch-all for variables
(identifier) @variable

; Special backticked identifiers treated as string literals
((identifier) @string.special
  (#match? @string.special "^`.*`$"))

; Special variables
((identifier) @variable.special
  (#match? @variable.special "^(self|Self)$"))

; Properties (field access)
(attribute
  attribute: (identifier) @property)

; Highlight uppercase attribute names as types/constructors
((attribute
   attribute: (identifier) @type)
  (#match? @type "^[A-Z]"))

; Highlight all-caps attribute names as constants (e.g. SWIZZLE_NONE)
((attribute
   attribute: (identifier) @constant)
  (#match? @constant "^_*[A-Z][A-Z\\d_]*$"))

; Struct fields (var declarations inside struct body)
(struct_definition
  body: (expression_statement
    (assignment
      left: (var_pattern
        pattern: (identifier) @property))))

; Type definitions
(struct_definition
  name: (identifier) @type)

(trait_definition
  name: (identifier) @type)

(extension_definition
  name: (identifier) @type)

; Generic/Compile-time type parameters (e.g. [T: Type, a_type: DType])
(meta_parameter
  name: (identifier) @variable.parameter)

((meta_parameter
   name: (identifier) @type)
  (#match? @type "^[A-Z]"))

; Dunder/magic methods as constructors (e.g. __init__, __del__, __str__)
((function_definition
   name: (identifier) @constructor)
  (#match? @constructor "^__.*__$"))

; Function definitions
(function_definition
  name: (identifier) @function)

; Function calls
(call
  function: [
    (identifier) @function
    (attribute
      attribute: (identifier) @function)
  ])

(call
  function: (subscript
    value: [
      (identifier) @function
      (attribute
        attribute: (identifier) @function)
    ]))

; Constructor/type instantiation calls starting with uppercase letters
((call
   function: (identifier) @type)
  (#match? @type "^[A-Z]"))

((call
   function: (subscript
     value: (identifier) @type))
  (#match? @type "^[A-Z]"))

((call
   function: (attribute
     attribute: (identifier) @type))
  (#match? @type "^[A-Z]"))

((call
   function: (subscript
     value: (attribute
       attribute: (identifier) @type)))
  (#match? @type "^[A-Z]"))

; Identifier conventions
; Assume uppercase names are types/enum-constructors
((identifier) @type
  (#match? @type "^[A-Z]"))

; Assume all-caps names are constants
((identifier) @constant
  (#match? @constant "^_*[A-Z][A-Z\\d_]*$"))

; Compile-time constant declarations
(comptime_declaration
  name: (identifier) @constant)

(comptime_member_declaration
  name: (identifier) @constant)

; Imports
(dotted_name
  (identifier) @type)

(aliased_import
  alias: (identifier) @type)

; Brackets
[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

; Delimiters
[
  "."
  ";"
  ","
  ":"
] @punctuation.delimiter

; Decorators (highlighted like Rust's attributes)
"@" @punctuation.special

(decorator
  (identifier) @attribute)

(decorator
  (call
    function: (identifier) @attribute))

; String interpolation brackets (f-string {expr})
(interpolation
  "{" @punctuation.special
  "}" @punctuation.special)

; Keywords
[
  "as"
  "async"
  "comptime"
  "def"
  "fn"
  "import"
  "from"
  "struct"
  "trait"
  "type"
  "var"
  "ref"
  "where"
  "read"
  "mut"
  "out"
  "deinit"
  "__extension"
] @keyword

; Control flow keywords
[
  "await"
  "else"
  "elif"
  "if"
  "in"
  "return"
  "with"
  "while"
  "for"
  "try"
  "except"
  "finally"
  "raise"
  "assert"
  "__comptime_assert"
] @keyword

(break_statement) @keyword
(continue_statement) @keyword
(pass_statement) @keyword

; Word operators
[
  "and"
  "or"
  "not"
  "is"
] @keyword

; Docstrings (bare string expressions in function/struct/trait bodies)
(function_definition
  body: (expression_statement
    (string) @comment.doc))

(struct_definition
  body: (expression_statement
    (string) @comment.doc))

(trait_definition
  body: (expression_statement
    (string) @comment.doc))

; Literals
[
  (string)
  (concatenated_string)
] @string

(escape_sequence) @string.escape

[
  (integer)
  (float)
] @number

(true) @boolean
(false) @boolean
(none) @constant.builtin

; Ellipsis literal (...)
(ellipsis) @constant.builtin

; Comments
(comment) @comment

; MLIR inline regions
(mlir_region_statement
  "__mlir_region" @embedded)

; Symbolic operators
[
  "+"
  "-"
  "*"
  "/"
  "//"
  "%"
  "**"
  "|"
  "&"
  "^"
  "<<"
  ">>"
  "@"
  "+="
  "-="
  "*="
  "/="
  "//="
  "%="
  "**="
  "<<="
  ">>="
  "&="
  "|="
  "^="
  "@="
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
  "<>"
  "->"
  ":="
  "="
] @operator

(unary_operator) @operator

; Ownership transfer operator
(transfer_expression
  "^" @operator)

; Parameters
(typed_parameter
  name: (identifier) @variable.parameter)

(default_parameter
  name: (identifier) @variable.parameter)

(typed_default_parameter
  name: (identifier) @variable.parameter)

(convention_parameter
  name: (identifier) @variable.parameter)

(parameters
  (identifier) @variable.parameter)

; Ensure self/Self is always @variable.special, even inside parameters
((typed_parameter
   name: (identifier) @variable.special)
  (#match? @variable.special "^(self|Self)$"))

((convention_parameter
   name: (identifier) @variable.special)
  (#match? @variable.special "^(self|Self)$"))

((parameters
   (identifier) @variable.special)
  (#match? @variable.special "^(self|Self)$"))

; Function modifiers/effects
(function_modifier
  [
    "raises"
    "capturing"
    "thin"
    "register_passable"
  ] @keyword)

(abi_effect
  "abi" @keyword)

; Keyword / Label arguments
(keyword_argument
  name: [
    (identifier)
    (string)
  ] @property)

(argument_convention
  (identifier) @label)
