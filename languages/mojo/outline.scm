(decorator) @annotation

(class_definition
  keyword: [
    "class"
    "struct"
    "trait"
  ] @context
  name: (identifier) @name) @item

(function_definition
  keyword: [
    "def"
    "fn"
  ] @context
  name: (identifier) @name) @item
