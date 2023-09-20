# User-lib
by Franklin Blanco

This library is my attempt at developing a recyclable utility for different projects, and not having to setup an authentication microservice each time I start a new project. 

### Must use Postgres!

## How to use?
Setup:
- Add this library to your Cargo.toml
- Copy the migrations from the migrations folder inside this library into your migrations
- Add the user_lib::setup() function to your main. Make sure to pass it a PgPool
- Add the user_lib::routes to your actix_web server (register, authenticate, change_password, refresh_token)
Usage: