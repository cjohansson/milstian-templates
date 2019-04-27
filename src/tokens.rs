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

    // TODO AdditionOne
    // TODO And
    // TODO Assign
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

    // TODO Division
    // TODO DoubleQuotedString

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

    // TODO EndForEach
    // TODO ElseIf
    // TODO Endif
    // TODO Equals
    // TODO ForEach
    // TODO If
    // TODO Multiplication

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

    // TODO Or
    // TODO SingleQuotedString
    // TODO StringConcatenation
    // TODO Subtraction
    // TODO SubtractOne

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
        pattern: LexerTokenMatchPattern::Regex(r"\$[a-zA-Z][a-zA-Z0-9_]*".to_string()),
        state: LexerState::Code,
    });

    items
}
