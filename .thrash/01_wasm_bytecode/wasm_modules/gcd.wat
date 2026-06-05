(module
  (func $gcd (param i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        local.get 1
        local.set 2
        br 1 (;@1;)
      end
      loop  ;; label = @2
        local.get 1
        local.get 0
        local.tee 2
        i32.rem_u
        local.set 0
        local.get 2
        local.set 1
        local.get 0
        br_if 0 (;@2;)
      end
    end
    local.get 2
  )
  (func $loop (result i32)
    (local i32)
    ;; i32.const 0
    ;; local.set 0
    (loop
       i32.const 1
       local.get 0
       i32.add
       local.tee 0
       i32.const 10
       i32.lt_s
       br_if 0
    )
    local.get 0
  )
  (func $start_f
    ;; i32.const 10
    ;; i32.const 40
    call $loop
    drop
  )
  (start $start_f)
  (export "loop" (func $loop))
  (export "gcd" (func $gcd))
)