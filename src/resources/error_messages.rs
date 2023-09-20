//  This file stores all the error messages
//  Template:   pub const ERROR_KEY_OR_NAME: (&str, &str) = ("ERROR.KEY", "ERROR VALUE");

pub type ErrorResource<'a> = (&'a str, &'a str);
pub const ERROR_INVALID_EMAIL: (&str, &str) = ("ERROR.INVALID_EMAIL", "Invalid email. Needs to be at least 4 characters, at most 254 and correctly formatted.");

pub const ERROR_INVALID_PHONE_NUMBER: (&str, &str) = ("ERROR.INVALID_PHONE_NUMBER", "Invalid Phone number. Needs to be 10 characters.");

pub const ERROR_INVALID_USERNAME: (&str, &str) = ("ERROR.INVALID_USERNAME", "Invalid Username. ");

pub const ERROR_INVALID_NAME: (&str, &str) = ("ERROR.INVALID_NAME", "Invalid name. Names should have at least 4 characters in length and at most 254.");

pub const ERROR_INVALID_PASSWORD: (&str, &str) = ("ERROR.INVALID_PASSWORD", "Invalid password. Password should have at least 8 characters and at most 128.");

pub const ERROR_USER_ALREADY_EXISTS: (&str, &str) = ("ERROR.USER_ALREADY_EXISTS", "A user with that email already exists.");

pub const ERROR_USER_DOES_NOT_EXIST: (&str, &str) = ("ERROR.USER_DOES_NOT_EXIST", "User with this email does not exist.");

pub const ERROR_PASSWORD_INCORRECT: (&str, &str) = ("ERROR.PASSWORD_INCORRECT", "The password you have entered is incorrect.");

pub const ERROR_INVALID_TOKEN: (&str, &str) = ("ERROR.INVALID_TOKEN", "The token you have supplied is not formattable.");

pub const ERROR_INCORRECT_TOKEN: (&str, &str) = ("ERROR.INCORRECT_TOKEN", "The token you have supplied does not belong to this user.");

pub const ERROR_MISSING_TOKEN: (&str, &str) = ("ERROR.MISSING_TOKEN", "No token supplied.");

pub const ERROR_EXPIRED_TOKEN: (&str, &str) = ("ERROR.EXPIRED_TOKEN", "The token you have supplied is expired.");

pub const ERROR_CREATING_TOKEN: (&str, &str) = ("ERROR.CREATING_TOKEN", "The server had an error creating the auth tokens.");