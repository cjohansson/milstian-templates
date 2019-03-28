# Lexer algorithm

Look-ahead state-based lexer, inspired by re2c.

Psuedo-code:

```
tokens = [];
buffer-position := buffer-start;
state = initial-state;
while buffer-position < buffer-end:
    best-match-logic := false;
    best-match-length := 0;
    foreach pattern in state-patterns(state):
        if pattern-matches(pattern, buffer, buffer-position):
            if best-match-length = 0 || best-match-length < match-length:
                best-match-logic := match-pattern;
                best-match-length := match-length;
            endif;
        endif;
    endforeach;
    if best-match-length > 0:
        tokens.append(execute-logic(best-match-logic, buffer, buffer-position, state));
        buffer-position += best-match-length;
    else:
        buffer-position++;
    endif;
endwhile;
return tokens;
```
