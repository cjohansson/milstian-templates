use LexerElement;
use LexerPosition;
use LexerState;
use LexerToken;
use LexerTokenMatchPattern;
use LexerTokenMatcher;

// Setup lexer patterns here
pub fn get_lexer_items() -> Vec<LexerTokenMatcher> {
    let mut items: Vec<LexerTokenMatcher> = Vec::new();

    // Addition
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Addition,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("+".to_string()),
        state: LexerState::Code,
    });

    // AddOne
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::AddOne,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("++".to_string()),
        state: LexerState::Code,
    });

    // And
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::And,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("and".to_string()),
        state: LexerState::Code,
    });

    // Assign
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::And,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("=".to_string()),
        state: LexerState::Code,
    });

    // Call
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                let variable_name = &buffer[(*char_index)..(char_index + *length - 1)];
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length - 1),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Call(variable_name.to_string()),
                });
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (char_index + *length - 1),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::OpenParenthesis,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Regex(r"[a-zA-Z][a-zA-Z0-9_]*\(".to_string()),
        state: LexerState::Code,
    });

    // CloseCurlyBracket
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::CloseCurlyBracket,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("}".to_string()),
        state: LexerState::Code,
    });

    // CloseParenthesis
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::CloseParenthesis,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal(")".to_string()),
        state: LexerState::Code,
    });

    // CloseTag
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::CloseTag,
                });
                (*state) = LexerState::Initial;
            },
        ),
        pattern: LexerTokenMatchPattern::Literal(" %}".to_string()),
        state: LexerState::Code,
    });

    // CloseTagWithEcho
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::CloseTagWithEcho,
                });
                (*state) = LexerState::Initial;
            },
        ),
        pattern: LexerTokenMatchPattern::Literal(" }}".to_string()),
        state: LexerState::Code,
    });

    // Comma
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Comma,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal(",".to_string()),
        state: LexerState::Code,
    });

    // Division
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Division,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("/".to_string()),
        state: LexerState::Code,
    });

    // DoubleQuotedString
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                // Search string forward until next un-escaped double quote
                let index_start: usize = *char_index + 1;
                let mut index: usize = 0;
                let mut index_end: Option<usize> = None;
                let mut previous_was_escape = false;
                for character in buffer.chars() {
                    if index > index_start {
                        if previous_was_escape {
                            previous_was_escape = false;
                        } else {
                            if character == '\\' {
                                previous_was_escape = true;
                            } else if character == '"' {
                                index_end = Some(index);
                                break;
                            } else if character == '\n' {
                                *line_end = *line_end + 1;
                            }
                        }
                    }
                    index = index + 1;
                }

                if index_end.is_some() {
                    let contents = &buffer[(char_index + 1)..(index_end.unwrap())];
                    *length = index_end.unwrap() - char_index;
                    elements.push(LexerElement {
                        position: LexerPosition {
                            char_end: index_end.unwrap(),
                            char_start: (*char_index),
                            line_end: (*line_end),
                            line_start: (*line_start),
                        },
                        token: LexerToken::DoubleQuotedString(contents.to_string()),
                    });
                }
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("\"".to_string()),
        state: LexerState::Code,
    });

    // EndForEach
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::EndForEach,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("endforeach".to_string()),
        state: LexerState::Code,
    });

    // ElseIf
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::ElseIf,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("elseif".to_string()),
        state: LexerState::Code,
    });

    // EndIf
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::EndIf,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("endif".to_string()),
        state: LexerState::Code,
    });

    // Equals
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Equals,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("equals".to_string()),
        state: LexerState::Code,
    });

    // Float
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                let float_value = &buffer[(*char_index)..(char_index + *length)];
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Float(float_value.to_string()),
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Regex(r"[0-9]\.[0-9]+".to_string()),
        state: LexerState::Code,
    });

    // ForEach
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::ForEach,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("foreach".to_string()),
        state: LexerState::Code,
    });

    // GreaterThan
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::GreaterThan,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal(">".to_string()),
        state: LexerState::Code,
    });

    // GreaterOrEqualThan
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::GreaterOrEqualThan,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal(">=".to_string()),
        state: LexerState::Code,
    });

    // If
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::If,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("if".to_string()),
        state: LexerState::Code,
    });

    // Integer
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                let integer_value = &buffer[(*char_index)..(char_index + *length)];
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Integer(integer_value.to_string()),
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Regex(r"[1-9][0-9]*".to_string()),
        state: LexerState::Code,
    });

    // LesserThan
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::LesserThan,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("<".to_string()),
        state: LexerState::Code,
    });

    // LesserOrEqualThan
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::LesserOrEqualThan,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("<=".to_string()),
        state: LexerState::Code,
    });

    // Multiplication
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Multiplication,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("*".to_string()),
        state: LexerState::Code,
    });

    // OpenCurlyBracket
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::OpenCurlyBracket,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("{".to_string()),
        state: LexerState::Code,
    });

    // OpenParenthesis
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::OpenParenthesis,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("(".to_string()),
        state: LexerState::Code,
    });

    // OpenTag
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                // Only add inline if it's not empty
                if char_end < char_index {
                    let new_buffer: &str = &buffer[*char_end..*char_index];
                    elements.push(LexerElement {
                        position: LexerPosition {
                            char_end: *char_index,
                            char_start: *char_end,
                            line_end: *line_end,
                            line_start: *line_start,
                        },
                        token: LexerToken::Inline(new_buffer.to_string()),
                    });
                }
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::OpenTag,
                });
                (*state) = LexerState::Code;
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("{% ".to_string()),
        state: LexerState::Initial,
    });

    // OpenTagWithEcho
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             state: &mut LexerState| {
                 // Only add inline if it's not empty
                 if *char_end < *char_index {
                    let new_buffer: &str = &buffer[*char_end..*char_index];
                    elements.push(LexerElement {
                        position: LexerPosition {
                            char_end: *char_index,
                            char_start: *char_end,
                            line_end: *line_end,
                            line_start: *line_start,
                        },
                        token: LexerToken::Inline(new_buffer.to_string()),
                    });
                }
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (*char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::OpenTagWithEcho,
                });
                (*state) = LexerState::Code;
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("{{ ".to_string()),
        state: LexerState::Initial,
    });

    // Or
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Or,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("or".to_string()),
        state: LexerState::Code,
    });

    // Semicolon
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Semicolon,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal(";".to_string()),
        state: LexerState::Code,
    });

    // SingleQuotedString
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                // Search string forward until next un-escaped single quote
                let index_start: usize = *char_index + 1;
                let mut index: usize = 0;
                let mut index_end: Option<usize> = None;
                let mut previous_was_escape = false;
                for character in buffer.chars() {
                    if index > index_start {
                        if previous_was_escape {
                            previous_was_escape = false;
                        } else {
                            if character == '\\' {
                                previous_was_escape = true;
                            } else if character == '\'' {
                                index_end = Some(index);
                                break;
                            } else if character == '\n' {
                                *line_end = *line_end + 1;
                            }
                        }
                    }
                    index = index + 1;
                }

                if index_end.is_some() {
                    let contents = &buffer[(char_index + 1)..(index_end.unwrap())];
                    *length = index_end.unwrap() - char_index;
                    elements.push(LexerElement {
                        position: LexerPosition {
                            char_end: index_end.unwrap(),
                            char_start: (*char_index),
                            line_end: (*line_end),
                            line_start: (*line_start),
                        },
                        token: LexerToken::SingleQuotedString(contents.to_string()),
                    });
                }
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("\"".to_string()),
        state: LexerState::Code,
    });

    // Subtraction
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Subtraction,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("-".to_string()),
        state: LexerState::Code,
    });

    // SubtractOne
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::SubtractOne,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("--".to_string()),
        state: LexerState::Code,
    });

    // Variable
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
             char_index: &usize,
             _char_start: &usize,
             _char_end: &usize,
             length: &mut usize,
             _line_index: &usize,
             line_start: &usize,
             line_end: &mut usize,
             elements: &mut Vec<LexerElement>,
             _state: &mut LexerState| {
                let variable_name = &buffer[(*char_index)..(char_index + *length)];
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + *length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Variable(variable_name.to_string()),
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Regex(r"[a-zA-Z][a-zA-Z0-9_]*".to_string()),
        state: LexerState::Code,
    });

    items
}
