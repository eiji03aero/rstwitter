# rstwitter
- A toy project to learn rust

# CLI commands
- Generate migration file
    ```sh
    sea-orm-cli migrate generate NAME_OF_MIGRATION [--local-time]
    ```

- Run migrations
    ```sh
    sea-orm-cli migrate up
    ```

- Generate entity
    ```sh
    sea-orm-cli generate entity -o entity/src
    ```

# Features
- Create account
    - attributes
        - username
- List accounts
- Post tweet
    - attributes
        - content
        - user id
- Follow other account
    - attributes
        - following id
        - followed id
- Get timeline

# Models
## User
### Attributes
- id: int
- username: varchar

## Tweet
- id: int
- text: text
- user_id: int

## FollowRelation
- following_id: int
- followed_id: int

# Todo
- [ ] implement create tweet
- [x] implement list accounts
- [x] implement create account
    - implement ok pattern
    - implement error handling
