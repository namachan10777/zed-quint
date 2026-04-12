; Literals
(int_literal) @number
(string) @string
(string_fragment) @string
(escape_sequence) @string.escape
(bool_literal) @boolean

; Comments
(comment) @comment

; Identifiers
(identifier) @variable
(qualified_identifier) @variable

; Definitions
(operator_definition
  name: (qualified_identifier) @function)

(type_alias
  (identifier) @type)

(variable_definition
  (qualified_identifier) @variable)

(constant_declaration
  (qualified_identifier) @constant)

(variant_constructor
  (qualified_identifier) @type)

; Type names
(type
  (qualified_identifier) @type)

(polymorphic_type
  (qualified_identifier) @type)

(record_type
  (qualified_identifier) @property)

; Module / import structure
(module_definition
  (qualified_identifier) @module)

(import
  (qualified_identifier) @module)

(export
  (qualified_identifier) @module)

(import_segment
  (qualified_identifier) @module)

; Keywords
[
  "module"
  "import"
  "from"
  "export"
  "as"
  "const"
  "var"
  "def"
  "val"
  "assume"
  "type"
  "if"
  "else"
  "match"
  "action"
  "temporal"
  "nondet"
  "pure"
  "run"
] @keyword

; Boolean / logical operators written as keywords
[
  "and"
  "or"
  "iff"
  "implies"
  "all"
  "any"
] @keyword.operator

; Builtin primitive types / sets
[
  "Int"
  "Nat"
  "Bool"
] @type.builtin

[
  "true"
  "false"
] @boolean

; Punctuation
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  "."
  ":"
  "::"
  ";"
] @punctuation.delimiter

; Operators
[
  "="
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
  "+"
  "-"
  "*"
  "/"
  "%"
  "^"
  "->"
  "=>"
  "|"
] @operator
