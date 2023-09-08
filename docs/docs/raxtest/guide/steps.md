---
sidebar_position: 3
---

# Steps

Steps are the basic unit of raxtest.

A step is a request to the api server.

Here is an example of a step:

```yaml
- name: ApiUserMe(GET)
  path: /api/user/me
  method: GET
  ref_data: no_login/ApiUserMe(GET)
  option:
    query: false
    body: false
```

## name

The name of the step.  
Type: `string`

## path

The request path of the step.  
Type: `string`

## method

The request HTTP method of the step.  
Type: `string`

## ref_data

The reference data of the step.  
This points to a specific data in json file to be used in the step.  
Type: `string`

## option

The option of the step.
This specifies whether to include the query and body in the request.

### query
A boolean value that determines whether to include the query in the request.  
Type `boolean`
### body
A boolean value that determines whether to include the body in the request.  
Type `boolean`