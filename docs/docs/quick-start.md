---
sidebar_position: 3
---

# Quick Start

## Create a config file
Here is a complete example of a raxtest config file.

### config file (yaml)
```yaml
base_url: http://localhost
data: json://data.json
init:
- name: ApiAuthLogin(POST)
  path: /api/auth/login
  method: POST
  ref_data: ApiAuthLogin(POST)
  option:
    query: false
    body: true

categories:
  no_loginStep:
    - name: ApiUserMe(GET)
      path: /api/user/me
      method: GET
      ref_data: no_login/ApiUserMe(GET)
      option:
        query: false
        body: false

  loginStep:
    login: ApiAuthLogin(POST)
    - name: ApiUserMe(PUT)
      path: /api/user/me
      method: PUT
      ref_data: ApiAuthLogin(POST)/ApiUserMe(PUT)
      option:
        query: false
        body: true
    - name: ApiUserMe(DELETE)
      path: /api/user/me
      method: DELETE
      ref_data: ApiAuthLogin(POST)/ApiUserMe(DELETE)
      option:
        query: false
        body: false
    - name: ApiProfileScreenName_GET
      path: /api/profile/@{screenName}
      method: GET
      ref_data: ApiAuthLogin(POST)/ApiUserMe(DELETE)
      option:
        query: true
        body: false
```

### data file (json)

```json
{
  "ApiAuthLogin(POST)": [
    {
      "body": {
        "handle": "johndoe2",
        "password": "Password123@"
      },
      "expect_status": 0
    }
  ],
  "ApiAuthLogin(POST)/ApiUserMe(GET)": [
    {
      "expect_status": 200
    }
  ],
  "ApiAuthLogin(POST)/ApiUserMe(PUT)": [
    {
      "body": {
        "bio": "john doe bio",
      },
      "expect_status": 200
    },
    {
      "body": {
        "screen_name": "jondoe@@2"
      },
      "expect_status": 200
    },
    {
      "body": {
        "hidden_comment": "john doe hidden comment"
      },
      "expect_status": 400
    },
    (...)
  ],
}
```

## Run raxtest
```sh
raxtest -i (config file path) -d (data file path)
```

## Output
```json
{
  "base_url": "http://localhost",
  "results": [
    {
      "name": "no_login/ApiUserMe(DELETE)[0]",
      "category": "no_login",
      "status": "success",
      "duration": 0.0126284,
      "message": "success (status: 401 Unauthorized, expect status: 401)"
    },
    {
      "name": "no_login/ApiUserMe(GET)[0]",
      "category": "no_login",
      "status": "success",
      "duration": 0.0099064,
      "message": "success (status: 401 Unauthorized, expect status: 401)"
    },
    {
      "name": "no_login/ApiUserMe(PUT)[0]",
      "category": "no_login",
      "status": "success",
      "duration": 0.007595,
      "message": "success (status: 401 Unauthorized, expect status: 401)"
    },
    ...
  ]
}
```