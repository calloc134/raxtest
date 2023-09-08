---
sidebar_position: 3
---

# Quick Start

## Create a config file
Here is a complete example of a raxtest config file.

### config file (yaml)
```yaml
base_url: http://localhost
data: json://out.json
init:
- name: LoginStep
  path: /api/auth/login
  method: POST
  ref_data: LoginData
  option:
    query: false
    body: true
categories:
  LoginCategory
    login: LoginStep
    steps:
    - name: UpdateMyProfile
      path: /api/profile/me
      method: PUT
      ref_data: MyProfileData
      option:
        query: false
        body: true
```

### data file (json)

```json
{
  "LoginStep": [
    {
      "body": {
        "password": "dummy",
        "screenName": "dummy"
      },
      "expect_status": 0
    }
  ],
  "UpdateMyProfile": [
    {
      "body": {
        "password": "dummy",
        "screenName": "dummy",
        "userName": "dummy"
      },
      "expect_status": 200
    }
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

![console](/img/console.svg)