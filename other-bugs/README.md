# DISCLAMER
**It is important to note that this code is for demonstration purposes only and should not be used in a production environment.**

# Description

# **Inconsistent rounding:** 
The inconsistent_rounding() function takes two f64 values as input and computes their ratio, then stores the result in an account. However, because of the way that floating-point arithmetic works, the result can be affected by inconsistent rounding. For example if `amount1 = 0.1` and `amount2 = 0.3` when dividing amount1 by amount2, the result will be a floating point number that cannot be represented exactly in binary. The division will produce a result like: `0.33333333333333337`. When storing this result in the MyData struct, it will have to round the number to the nearest representable value. Depending on the rounding mode used, this could result in either:
0.333333 (round to nearest, ties to even)
0.333334 (round up)

This vulnerability could be exploited to create subtle bugs or security vulnerabilities in financial or scientific applications.

To avoid this, you should either:
- Store the result with higher precision, using a decimal data type
- Perform the division and rounding in the same way each time, using a consistent rounding mode

# **Incorrect calculation:** 
The incorrect_calculation() function takes two u64 values as input and computes their product, then stores the result in an account. However, the implementation mistakenly computes the product instead of the sum. This vulnerability could lead to unexpected or incorrect behavior in applications that rely on the correct computation of arithmetic operations.

## Example
For example:
```
amount1 = 0.1
amount2 = 3.0

// Correct calculation:
0.1 * 3.0 = 0.3
```

However, due to floating point error, storing 0.1 and performing the calculation in binary could result in a slightly incorrect value, for example:
```
Stored value of amount1: 
      0.10000000000000001  

Calculating:
0.10000000000000001 * 3 = 0.30000000000000004
```

Storing this incorrect result of 0.30000000000000004 instead of the exact 0.3 would produce an incorrect calculation.
The issue here is that floating point numbers cannot represent some values exactly in binary. This leads to small rounding errors that can compound when performing multiple calculations.

To fix this, you have a few options:
- Use a decimal data type with higher precision instead of f64
- Perform the calculation directly on the stored decimal string values, without converting to float
- Round the result to an acceptable level of precision after calculating

# **Exponential complexity**
The exponential_complexity() function takes a single u64 value as input and computes its factorial using a recursive function. However, the recursive implementation has exponential time complexity, meaning that it becomes very slow for large inputs. This vulnerability could lead to **denial-of-service** attacks or other performance issues in applications that rely on the correct computation of factorial or other recursive functions.

## Example 
The factorial function has exponential complexity with respect to its input. This means the runtime grows exponentially as the input increases.
For the given code, passing in a very large amount value (converting to a large u64) will result in the factorial function taking a very long time to complete.
For example:
```
amount = 1e20   // 1 with 20 zeros
x = amount as u64 // x is a very large number   

// Calling factorial(x) will take a very long time!
let result = factorial(x);
```

Computing the factorial of such a large number using recursion will result in an extremely deep call stack and long runtime.

To avoid this, you have a few options:
- Limit the maximum input value to avoid very large factorials
- Use iteration instead of recursion to compute the factorial
- Memoize the factorial values to reuse previously computed results