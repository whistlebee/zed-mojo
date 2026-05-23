(comment)+ @comment.around

(function_definition
  body: (_) @function.inside) @function.around

(struct_definition
  body: (_) @class.inside) @class.around

(trait_definition
  body: (_) @class.inside) @class.around
