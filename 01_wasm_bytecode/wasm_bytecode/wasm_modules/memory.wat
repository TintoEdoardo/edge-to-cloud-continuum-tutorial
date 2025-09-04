(module
  (memory (export "memory") 2 3)
  (func $size (result i32)
    memory.size
  )
  (func $load (param i32) (result i32)
    local.get 0
    i32.load8_s
  )
  (func $store (param i32 i32)
    local.get 0
    local.get 1
    i32.store8
  )
  (export "size" (func $size))
  (export "load" (func $load))
  (export "store" (func $store))
  (data (i32.const 0x1000) "\01\02\03\04")
)
