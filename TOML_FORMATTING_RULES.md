# TOML Configuration Formatting Rules

## Maps/Tables

1. **Inline maps**: If a map contains less than 5 elements, all elements are scalar (not map/array), and none have comments, format the map inline: `{ key1 = "value1", key2 = "value2" }`

2. **Block maps**: Otherwise, format as a table block with `[section]` headers

3. **No dotted keys by default**: By default, never use `key1.key2 = value` syntax. Always display fields on separate lines within their table section. Use `#[format]` attribute to override.

## Arrays

4. **Inline arrays**: If an array contains only scalar types (no nested map/array) and has less than 5 elements, format inline: `[1, 2, 3]`

5. **Array of tables**: For arrays of maps/structs, use `[[item]]` syntax with each element as a separate block

6. **Multi-line arrays**: For long scalar arrays (5+ elements), use multi-line format with one element per line

## Format Attributes

21. **Custom formatting**: Use `#[format = "..."]` attribute on struct fields to override default formatting:
    - `"inline"` - Force inline table: `field = { key = value }`
    - `"dotted"` - Flatten one level: `field.key = value`
    - `"dotted-nested"` - Recursively flatten all levels: `field.key.subkey = value`
    - `"*dotted"` - Keep `[field]` section, flatten children: `[field]` then `key.subkey = value`
    - `"*dotted-nested"` - Keep `[field]` section, recursively flatten all children
    - `"multiline"` - Force multiline array format even for <5 elements

## Comments

7. **Comment placement**: Place comments on the line immediately above the key they describe

8. **Empty line before sections**: Add one empty line before each `[section]` header (except the first one) for readability

9. **Preserve comment formatting**: Keep original comment formatting including indentation and line breaks from doc comments

10. **No trailing comments**: Avoid inline comments after values; always place comments above the key

## Spacing

11. **Consistent indentation**: Use spaces around `=` for assignments: `key = value`

12. **No trailing whitespace**: Remove trailing spaces from all lines

13. **Single empty line between groups**: Use one empty line to separate logical groups of related keys within a section

14. **File ending**: Always end the file with a single newline character

## Ordering

15. **Scalar fields first**: Within a table, list scalar fields before nested tables or arrays of tables

16. **Preserve definition order**: Maintain the order of fields as defined in the struct

## Special Cases

17. **Optional fields**: For `Option<T>` fields with `None` value, render them as commented-out lines to show they're available but optional

18. **Empty collections**: Show empty arrays as `[]` and empty inline tables as `{}`

19. **String escaping**: Use basic strings (`"..."`) by default; use literal strings (`'...'`) only when avoiding excessive escaping

20. **Multi-line strings**: Always use multi-line strings (`"""..."""`) for string values containing newlines (`\n`)
