# 用 Rust 写 OJ

## [RestFul 接口](src/router.rs)
所有请求均需要 cookie。

### 1. 获取用于简单测试的代码
#### request：
|方法|路径|参数|
|:----:|:----:|:----:|
|GET|/test_code/<题目 id>|所用语言（lang）

#### response：[TestCode](src/resp/test_code.rs)
获取成功返回代码。e.g.
``` json
{
    "status": "Success",
    "func_code": "code for function that will be impl",
    "simple_test": "code for simple test"
}
```
失败返回 [SomeException](src/resp/some_exception.rs) :
``` json
{
    "status": "Exception",
    "code": 0,
    "message": "SomeException test"
}
```

<br/>

### 2. 提交答案
#### request：[Solution](src/req/solution.rs)
|方法|路径|参数|
|:----:|:----:|:----:|
|POST|/j/submit|solution 表单

#### response：[JudgeSubmit](src/resp/judge_submit.rs)
提交成功返回本次提交信息，失败返回 SomeException 。e.g.
``` json
{
    "id": "<提交 id: usize>"
}
```

<br/>

### 3. 获取答案 judge 状态
#### request：
|方法|路径|参数|
|:----:|:----:|:----:|
|GET|/j/judge_state/<提交 id>|

#### response： [JudgeState](src/resp/judge_state.rs)
获取成功返回当前判题进程，失败返回 SomeException 。e.g.
``` json
{
    "status": "Success",
    "stage": {
        "type": "Exit",
        "case_idx": 6,
        "state": {
            "type": "Error",
            "error": {
                "type": "TimeLimitExceeded",
                "time_used": 120000000
            }
        }
    },
    "message": "judge state test"
}
```

<br/>

### 4. 获取 judge 结果
#### request：
|方法|路径|参数|
|:----:|:----:|:----:|
|GET|/j/judge_result/<提交 id>|

#### response： [JudgeResult](src/resp/judge_result.rs)
获取成功返回判题结果，失败返回 SomeException 。e.g.
``` json
{
    "status": "Success",
    "type": "WrongAnswer",
    "cases_result": [
        [
            0,
            {
                "type": "Pass",
                "time_used": 100,
                "mem_used": 10240
            }
        ],
        [
            1,
            {
                "type": "Pass",
                "time_used": 120,
                "mem_used": 23333
            }
        ],
        [
            2,
            {
                "type": "WrongAnswer",
                "expect": "1 + 1 = 2",
                "got": "1 + 1 = 3"
            }
        ]
    ]
}
```