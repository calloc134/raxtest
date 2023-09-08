---
sidebar_position: 2
---

# Categories

The config file is divided into categories.

## Init Category

The init category is used to run initialization steps before running the test cases.  
Each init step runs independently.

Here is an example of init category:

```yaml
init:
- name: LoginStep
  path: /api/auth/login
  method: POST
  ref_data: LoginData
  option:
    query: false
    body: true
```

## Test Categories

Test categories are used to run test cases.  
Each test category has a name and a list of steps.

### Login Field
The field is optional.  
If this field is specified, the step run with the credentials(cookies) of the init step with the same name.  
Type: `string`

Here is an example of test categories:

```yaml
categories:
  LoginCategory:
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
