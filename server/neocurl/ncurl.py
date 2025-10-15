import neocurl as nc
import json

SERVER_URL = "http://localhost:3000"

JWT_VERIFY_URL = f"{SERVER_URL}/jwt/verify"

USERS_ME = f"{SERVER_URL}/users/me"

DEBUG_USER_URL = f"{SERVER_URL}/debug/user"

@nc.on_init
def main():
    if not nc.check_version("2.0.4"):
        nc.fatal(f"This version of neocurl is not compatible with this script: {nc.version()}")

    logger_config = nc.get_logger_config()
    if nc.env("LOG") == "DEBUG":
        logger_config.level = nc.LogLevel.Debug
    else:
        logger_config.level = nc.LogLevel.Info
    logger_config.datetime_format = "%H:%M:%S%.3f"
    logger_config.use_colors = True
    nc.set_logger_config(logger_config)

def create_user_and_jwt(client):
    user = {
        "uuid": "497dcba3-ecbf-4587-a2dd-5eb0665e6880",
        "username": "testuser",
        "nickname": "Tester",
        "email": "testuser@commeator.org",
    }

    response = client.post(
        DEBUG_USER_URL,
        body = json.dumps(user),
        headers = {
            "Content-Type": "application/json"
        }
    )
    assert response.status_code == 200, f"Failed to create user:\\n{repr(response.dump())}"

    jwt_token = json.loads(response.body)
    nc.info(f"JWT token generated")
    return (user, jwt_token)

@nc.define
def verify_jwt(client):
    (user, jwt) = create_user_and_jwt(client)

    response = client.get(
        JWT_VERIFY_URL,
        headers = {
            "Authorization": f"Bearer {jwt}"
        }
    )
    assert response.status_code == 200, f"Failed to verify JWT: {response.body}"
    assert response.body == "true", f"JWT verification failed: {response.body}"

    nc.info("JWT verification succeeded")

@nc.define
def get_my_user(client):
    (user, jwt) = create_user_and_jwt(client)

    response = client.get(
        USERS_ME,
        headers = {
            "Authorization": f"Bearer {jwt}"
        }
    )
    assert response.status_code == 200, f"Failed to verify JWT: {response.body}"

    user_info = json.loads(response.body)

    assert user_info["username"] == user["username"], "Username does not match"
    assert user_info["nickname"] == user["nickname"], "Nickname does not match"
    assert user_info["email"].split("-")[0] == user["email"], "Email does not match"

    nc.info("User info matches")
