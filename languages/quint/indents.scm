; 括弧ペアを @indent し、閉じトークンを @end でマークして揃える。
; @outdent 単体だと閉じ括弧が二重にデデントされ 1 段浅くなるため使わない。
(_
  "("
  ")" @end
) @indent

(_
  "["
  "]" @end
) @indent

(_
  "{"
  "}" @end
) @indent
