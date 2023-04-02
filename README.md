# rstwitter
- A toy project to learn rust

# Todo
- [x] implement create account
    - implement ok pattern
    - implement error handling

# Features
- Create account
    - attributes
        - username
- Post tweet
    - attributes
        - text
- Get timeline
- Follow other account
    - attributes
        - user id

# Models
## User
### Attributes
- id: int
- username: varchar

## Tweet
- id: int
- text: text

## FollowRelation
- following_id: int
- followed_id: int
