use crate::stack::Stack;

/// Checks whether parentheses, square brackets, braces, and chevrons are balanced in the input string.
///
/// To be balanced, not only must the number of opening and closing brackets of each type be equal,
/// they must also be closing in inverse order of opening.
pub fn is_balanced(s: &str) -> bool {
    let mut stack = Stack::new();

    for chr in s.chars() {
        match chr {
            '[' | '{' | '(' | '<' => stack.push(chr),
            ']' | '}' | ')' | '>' => match stack.pop() {
                Some(opening) => {
                    if (chr == ']' && opening != '[')
                        || (chr == '}' && opening != '{')
                        || (chr == ')' && opening != '(')
                        || (chr == '>' && opening != '<')
                    {
                        // Incorrect opening bracket left on the stack
                        return false;
                    }
                }
                // No opening bracket left on the stack at all
                None => return false,
            },
            _ => {}
        }
    }

    // If we got this far then there all closing brackets were preceded by an opening bracket in
    // the appropriate position.
    // If the stack is now empty then there were also no opening brackets which were never closed.
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_balanced() {
        assert!(is_balanced("(hello) (world)"));
        assert!(is_balanced("(<{()}>)[]"));

        assert!(!is_balanced("static public void main(String args[]) {"));
        assert!(!is_balanced("(<{()>)[}]"));
    }
}
