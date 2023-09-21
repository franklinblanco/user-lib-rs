# User-lib
by Franklin Blanco

This library is my attempt at developing a recyclable utility for different projects, and not having to setup an authentication microservice each time I start a new project. 

### Must use Postgres!

## How to use?
Setup:
- Add this library to your Cargo.toml
- Copy the migrations from the migrations folder inside this library into your migrations
- Run the migrations
Usage:
- A user can have many credentials (Currently only 3, one for each CredentialType: Username, Email, PhoneNumber)
- Register a user with `register_user().await` This function returns a Token that holds an Auth token that's usable for 7 days and a Refresh token in case the auth expires.
- Authenticate a user with their id and auth_token using `authenticate_user().await`
- If that's expired use `refresh_token().await`
- If you want another token then use `password_login().await`
- `reset_password().await` To reset password with current password
- `force_reset_password().await` To reset password without knowing the password (YOU MUST IMPLEMENT YOUR OWN WAY OF VALIDATING THAT USER'S IDENTITY)