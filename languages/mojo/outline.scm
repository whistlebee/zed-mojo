(decorator) @annotation

(struct_definition
  keyword: "struct" @context
  name: (identifier) @name) @item

(trait_definition
  keyword: "trait" @context
  name: (identifier) @name) @item

(function_definition
  keyword: [
    "def"
    "fn"
  ] @context
  name: (identifier) @name) @item
