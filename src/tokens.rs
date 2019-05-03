use LexerElement;
use LexerPosition;
use LexerState;
use LexerToken;
use LexerTokenMatcher;
use LexerTokenMatchPattern;

// Setup lexer patterns here
pub fn get_lexer_items() -> Vec<LexerTokenMatcher>
{
    let mut items: Vec<LexerTokenMatcher> = Vec::new();

    // Addition
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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

    // TODO Call

    // CloseTag
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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

    // Division
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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

    // TODO DoubleQuotedString
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {

                // Search string forward until next un-escaped double quote
                let mut index: usize = *char_index + 1;
                let mut index_end: Option<usize> = None;
                let mut previous_was_escape = false;
                while index < buffer.len() {
                    if previous_was_escape {
                        previous_was_escape = false;
                    } else {
                        if let Some(character) = buffer.chars().nth(index) {
                            if character == '\\' {
                                previous_was_escape = true;
                            } else if character == '"' {
                                index_end = Some(index);
                                break;
                            }
                        }
                    }
                    index = index + 1;
                }

                if index_end.is_some() {
                    let contents = &buffer[(char_index + 1)..(index_end.unwrap())];
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

    // Echo
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Echo,
                });
                (*state) = LexerState::Code;
            },
        ),
        pattern: LexerTokenMatchPattern::Literal("echo".to_string()),
        state: LexerState::Code,
    });

    // EndForEach
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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

    // ForEach
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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

    // If
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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

    // Multiplication
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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

    // OpenTag
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                let new_buffer: &str = &buffer[*char_start..(char_end + 1)];
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: *char_end,
                        char_start: *char_start,
                        line_end: *line_end,
                        line_start: *line_start,
                    },
                    token: LexerToken::Inline(new_buffer.to_string()),
                });
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                let new_buffer: &str = &buffer[*char_start..(char_end + 1)];
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: *char_end,
                        char_start: *char_start,
                        line_end: *line_end,
                        line_start: *line_start,
                    },
                    token: LexerToken::Inline(new_buffer.to_string()),
                });
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
    
    // TODO SingleQuotedString
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::StringConcatenation,
                });
            },
        ),
        pattern: LexerTokenMatchPattern::Literal(".".to_string()),
        state: LexerState::Code,
    });

    // Subtraction
    items.push(LexerTokenMatcher {
        logic: Box::new(
            |_buffer: &str,
            char_index: &usize,
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
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
            char_start: &usize,
            char_end: &usize,
            length: &usize,
            line_index: &usize,
            line_start: &usize,
            line_end: &usize,
            elements: &mut Vec<LexerElement>,
            state: &mut LexerState| {
                let variable_name = &buffer[(char_index+1)..(char_index + length)];
                elements.push(LexerElement {
                    position: LexerPosition {
                        char_end: (char_index + length),
                        char_start: (*char_index),
                        line_end: (*line_end),
                        line_start: (*line_start),
                    },
                    token: LexerToken::Variable(variable_name.to_string()),
                });
                (*state) = LexerState::Code;
            },
        ),
        pattern: LexerTokenMatchPattern::Regex(r"[a-zA-Z][a-zA-Z0-9_]*".to_string()),
        state: LexerState::Code,
    });

    items
}
