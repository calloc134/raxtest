---
sidebar_position: 5
---

# Output

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
    (...)
  ]
}
```

## base_url

The base url of the api server.  
Type: `string`

## results

An array that stores the test results.  
Type: `array`

Each element in the results array is as follows.

### name

The name of the step.  
Type: `string`

### category

The category to which the step belongs.  
Type: `string`

### status

Indicates the result of the step.  
Takes one of the values `success` or `failure`.  
Type: `string`

### duration

Indicates the execution time of the step.  
Type: `float`

### message

Indicates the message of the step.  
Type: `string`