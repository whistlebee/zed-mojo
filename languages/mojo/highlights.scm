(type
  [
    (identifier) @type
    (soft_keyword_identifier) @type
  ])

(type
  (attribute
    attribute: (identifier) @type))

(type
  (subscript
    value: [
      (identifier) @type
      (soft_keyword_identifier) @type
    ]))

(type
  (subscript
    value: (attribute
      attribute: (identifier) @type)))

(type
  [
    (identifier) @type.builtin
    (soft_keyword_identifier) @type.builtin
  ]
  (#match? @type.builtin "^(__mlir_attr|__mlir_op|__mlir_type|Bool|DType|Dict|Float16|Float32|Float64|Int|List|None|Optional|Pointer|SIMD|Set|String|Tuple|UnsafePointer|bool|dict|float|int|list|object|set|str|tuple|type)$"))

(meta_parameter
  name: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(meta_parameter
  type: (type
    [
      (identifier) @type.interface
      (soft_keyword_identifier) @type.interface
    ]))

(class_definition
  name: (identifier) @type)

(class_definition
  keyword: "trait"
  name: (identifier) @type.interface)

(class_definition
  superclasses: (argument_list
    [
      (identifier) @type.interface
      (attribute
        attribute: (identifier) @type.interface)
    ]))

(function_definition
  name: (identifier) @function.definition)

(function_definition
  keyword: [
    "def"
    "fn"
  ] @keyword)

(function_type
  keyword: [
    "def"
    "fn"
  ] @keyword)

(class_definition
  keyword: [
    "class"
    "struct"
    "trait"
  ] @keyword)

(decorator
  "@" @punctuation.special)

(decorator
  (expression
    [
      (identifier) @attribute
      (soft_keyword_identifier) @attribute
    ]))

(decorator
  (expression
    (attribute
      attribute: (identifier) @attribute)))

(decorator
  (expression
    (call
      function: [
        (identifier) @attribute
        (soft_keyword_identifier) @attribute
      ])))

(decorator
  (expression
    (call
      function: (attribute
        attribute: (identifier) @attribute))))

(call
  function: [
    (identifier) @function
    (soft_keyword_identifier) @function
  ])

(call
  function: (attribute
    attribute: (identifier) @function))

(call
  function: (subscript
    value: [
      (identifier) @function
      (soft_keyword_identifier) @function
    ]))

(call
  function: (subscript
    value: (attribute
      attribute: (identifier) @function)))

(binary_operator
  left: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ]
  (#match? @variable "^[a-z_][A-Za-z0-9_]*$"))

(binary_operator
  right: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ]
  (#match? @variable "^[a-z_][A-Za-z0-9_]*$"))

(boolean_operator
  left: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ]
  (#match? @variable "^[a-z_][A-Za-z0-9_]*$"))

(boolean_operator
  right: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ]
  (#match? @variable "^[a-z_][A-Za-z0-9_]*$"))

(comparison_operator
  [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ]
  (#match? @variable "^[a-z_][A-Za-z0-9_]*$"))

(return_statement
  [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(expression_list
  [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(transfer_expression
  value: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(await
  [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(attribute
  object: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ]
  (#match? @variable "^[a-z_][A-Za-z0-9_]*$"))

(call
  function: (subscript
    value: [
      (identifier)
      (soft_keyword_identifier)
    ]
    subscript: [
      (identifier) @variable
      (soft_keyword_identifier) @variable
    ]))

(call
  function: (subscript
    value: (attribute
      attribute: (identifier) @function)))

(call
  function: (subscript
    value: (attribute
      attribute: (identifier) @function)
    subscript: [
      (identifier) @variable
      (soft_keyword_identifier) @variable
    ]))

(typed_parameter
  name: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(convention_parameter
  name: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(default_parameter
  name: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(typed_default_parameter
  name: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(aliased_import
  alias: (identifier) @label)

(assignment
  left: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(assignment
  right: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(augmented_assignment
  left: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(ref_pattern
  pattern: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(comptime_declaration
  name: [
    (identifier) @variable
    (soft_keyword_identifier) @variable
  ])

(call
  arguments: (argument_list
    [
      (identifier) @variable
      (soft_keyword_identifier) @variable
    ]))

((identifier) @variable.special
  (#eq? @variable.special "self"))

((identifier) @variable.special
  (#eq? @variable.special "cls"))

((soft_keyword_identifier) @variable.special
  (#eq? @variable.special "self"))

((soft_keyword_identifier) @variable.special
  (#eq? @variable.special "cls"))

(import_statement
  "import" @keyword)

(import_from_statement
  [
    "from"
    "import"
  ] @keyword)

(comptime_declaration
  "comptime" @keyword)

(comptime_modifier) @keyword

(ref_pattern
  "ref" @keyword)

(await
  "await" @keyword)

(aliased_import
  "as" @keyword)

(as_pattern
  "as" @keyword)

(type_alias_statement
  "type" @keyword)

[
  "async"
  "raises"
  "capturing"
  "thin"
] @keyword

[
  "if"
  "elif"
  "else"
  "for"
  "while"
  "try"
  "except"
  "finally"
  "with"
  "match"
  "case"
  "return"
  "yield"
  "raise"
  "assert"
  "__comptime_assert"
  "where"
] @keyword.control

(pass_statement) @keyword.control
(break_statement) @keyword.control
(continue_statement) @keyword.control

[
  "and"
  "or"
  "not"
  "in"
  "is"
  "is not"
  "not in"
] @keyword.control

(assignment
  "var" @keyword)

(abi_effect
  "abi" @keyword)

(capture_default) @keyword
(capture_convention) @keyword

(argument_convention
  [
    "read"
    "mut"
    "var"
    "out"
    "deinit"
    "ref"
  ] @keyword)

(true) @boolean
(false) @boolean
(none) @constant.builtin
(ellipsis) @constant.builtin

[
  (integer)
  (float)
] @number

(string) @string
(string_start) @string
(string_end) @string
(escape_sequence) @string.escape
(type_conversion) @string.escape

(interpolation
  [
    "{"
    "}"
  ] @punctuation.special)

(comment) @comment

(binary_operator
  operator: [
    "+"
    "-"
    "*"
    "/"
    "//"
    "%"
    "**"
    "<<"
    ">>"
    "&"
    "|"
    "@"
    "^"
  ] @operator)

(unary_operator
  operator: [
    "+"
    "-"
    "~"
  ] @operator)

(comparison_operator
  operators: [
    "=="
    "!="
    "<"
    "<="
    ">"
    ">="
    "<>"
  ] @operator)

(comparison_operator
  operators: [
    "in"
    "not in"
    "is"
    "is not"
  ] @keyword.control)

(boolean_operator
  operator: [
    "and"
    "or"
  ] @keyword.control)

(not_operator
  "not" @keyword.control)

(augmented_assignment
  operator: [
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
  ] @operator)

(transfer_expression
  "^" @operator)

(assignment
  "=" @operator)

(named_expression
  ":=" @operator)

(list_splat
  "*" @operator)

(list_splat_pattern
  "*" @operator)

(dictionary_splat
  "**" @operator)

(dictionary_splat_pattern
  "**" @operator)

(splat_pattern
  "*" @operator)

(keyword_separator
  "*" @operator)

(positional_separator
  "/" @operator)

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "."
  ","
  ":"
  ";"
] @punctuation.delimiter

[
  "->"
  "@"
] @punctuation.special
