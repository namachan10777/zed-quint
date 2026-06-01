; operator 定義を function として扱う
(operator_definition
  rhs: (_) @function.inside
) @function.around

; module を class として扱う
(module_definition
  "{"
  (_)* @class.inside
  "}"
) @class.around

; comments
(comment) @comment.around
