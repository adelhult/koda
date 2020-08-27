" Vim syntax file
" Language: koda

if exists("b:current_syntax")
    finish
endif

syn keyword kodaConstant  nil sant falskt
syn keyword kodaLocal     lokal
syn keyword kodaOperator  och inte eller
syn keyword kodaStatement bryt ge
syn keyword kodaFunction  funktion
syn keyword kodaFlow      upprepa medan tills gör för i slut
syn keyword kodaCond      om utför annars annarsom

syn match kodaEscape contained /\\./
syn region kodaString start="\"" end="\"" contains=kodaEscape
syn region kodaString start="'" end="'" contains=kodaEscape

syn keyword kodaCommentTodo contained TODO FIXME
syn cluster kodaCommentGrp  contains=kodaCommentTodo
syn region  kodaComment  start="--" skip="\\$" end="$" keepend contains=@kodaCommentGrp
syn region  kodaComment  start="--\[" end="--]" contains=@kodaCommentGrp

syn match kodaFuncCall  "\I\+\s*\((\)\@="
syn match kodaIdent     "\I\+\s*\(=\s\)\@="
syn match kodaQuantPlus "==\|\*\|+\|-\(-\)\@!\|\/\|\.\."
syn match kodaDict      "\I\+\(\.\)\@="

hi def link kodaConstant    Boolean
hi def link kodaLocal       StorageClass
hi def link kodaOperator    Operator
hi def link kodaStatement   Statement
hi def link kodaFunction    Statement
hi def link kodaFlow        Repeat
hi def link kodaCond        Conditional
hi def link kodaEscape      Special
hi def link kodaString      String
hi def link kodaFuncCall    Function
hi def link kodaIdent       Structure
hi def link kodaQuantPlus   Special
hi def link kodaDict        Structure
hi def link kodaCommentTodo Todo
hi def link kodaComment     Comment

