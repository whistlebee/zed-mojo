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

; Type definitions
(struct_definition
  name: (identifier) @type)

(trait_definition
  name: (identifier) @type.interface)

(extension_definition
  name: (identifier) @type)

; Generic/Compile-time type parameters (e.g. [T: Type])
(meta_parameter
  name: (identifier) @type)

; Function definitions
(function_definition
  name: (identifier) @function.definition)

; Function calls
(call
  function: [
    (identifier) @function
    (attribute
      attribute: (identifier) @function.method)
  ])

(call
  function: (subscript
    value: [
      (identifier) @function
      (attribute
        attribute: (identifier) @function.method)
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
] @keyword.control

(break_statement) @keyword.control
(continue_statement) @keyword.control
(pass_statement) @keyword.control

; Word operators
[
  "and"
  "or"
  "not"
  "is"
] @keyword.operator

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
(none) @constant

; Comments
(comment) @comment

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
