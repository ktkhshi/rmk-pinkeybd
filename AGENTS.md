# Commit Message Guidelines

You must follow the Conventional Commits specification for all git commits you propose or create.

## Rules

- **Format**: `<type>(<scope>): <short summary in English/Japanese>`
- **Case**: Use lowercase for the type and the summary.
- **Length**: Keep the subject line under 50 characters.
- **Body**: Provide a detailed explanation in the body if the change is complex. Wrap lines at 72 characters.
- **Co-authored-by**: Always append the `Co-authored-by` trailers at the end of the commit message to give proper credit to yourself (the AI assistant) and the user if applicable.

## Allowed Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools and libraries

## Example Output

feat(auth): add JWT validation logic

- Implement middleware for bearer token verification
- Add unit tests for expired tokens

Co-authored-by: github-workspace[bot] <github-workspace[bot]@://github.com>
