---
sidebar_position: 4
---

# Data Structure


Data file is a json file that contains the data used in the test cases.  
Each data supports multiple test cases.

```json
{
  "no_login/ApiUserMe(GET)": [
    {
      "body": {
        "handle": "johndoe2",
        "password": "Password123@"
      },
      "query": {
        "handle": "johndoe2"
      },
      "expect_status": 200
    }
  ],
  (...)
}
```

### body
The body of the request.  
Type: `object`

### query
The query of the request.  
Type: `object`

### expect_status
The expected status code of the response.  
This status code is used to determine whether the test case is passed or failed.  
Type: `number`