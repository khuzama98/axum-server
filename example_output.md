# Example Tracing Output

When you run the application, you'll see structured logs like this:

```
2024-01-09T15:30:01.123456Z  INFO ThreadId(01) main.rs:26: Connecting to database...
2024-01-09T15:30:01.234567Z  INFO ThreadId(01) main.rs:28: Database connection established
2024-01-09T15:30:01.345678Z  INFO ThreadId(01) main.rs:31: Running database migrations...
2024-01-09T15:30:01.456789Z  INFO ThreadId(01) main.rs:33: Database migrations completed successfully
2024-01-09T15:30:01.567890Z  INFO ThreadId(01) main.rs:41: Server starting on http://0.0.0.0:3000

# When someone creates a user:
2024-01-09T15:30:05.123456Z  INFO ThreadId(02) user.rs:24: Creating new user user_id=550e8400-e29b-41d4-a716-446655440000 username=john_doe
2024-01-09T15:30:05.234567Z  INFO ThreadId(02) user.rs:50: User created successfully user_id=550e8400-e29b-41d4-a716-446655440000 username=john_doe

# If there's an error:
2024-01-09T15:30:10.123456Z ERROR ThreadId(03) error.rs:45: Database error: connection closed
```

## Log Levels (from most to least verbose):

- `TRACE` - Very detailed, usually for debugging
- `DEBUG` - Detailed info for developers
- `INFO` - General information (what we're using)
- `WARN` - Something unexpected but not fatal
- `ERROR` - Something went wrong

## Key Benefits:

1. **Structured Data**: Fields like `user_id` and `username` can be searched/filtered
2. **Automatic Context**: Thread IDs, timestamps, file locations
3. **Performance**: Very low overhead in production
4. **Flexibility**: Can output to console, files, or external services (like Jaeger)

## The `#[tracing::instrument]` Macro:

```rust
#[tracing::instrument(skip(pool))]
pub async fn create_user(/* ... */) -> AppResult<User> {
    // Automatically creates a span for this function
    // Logs function entry/exit
    // Includes function parameters (except skipped ones)
}
```

This creates a "span" that tracks the entire function execution, making it easy to see:
- When the function started/ended
- How long it took
- What parameters it received
- Any logs that happened inside

## Production Benefits:

- **Debugging**: Trace requests across your application
- **Performance**: Find slow operations
- **Monitoring**: Export to tools like Jaeger, Prometheus
- **Alerting**: Set up alerts on error rates

Much better than scattered `println!` statements!