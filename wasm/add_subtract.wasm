(module

    (import "math" "callback" (func $callback))

    (export "add" (func $add))
    (export "subtract" (func $subtract))

    (func $add (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        i32.add
    )

    (func $subtract (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        i32.sub
    )
)