

(module $the_module

    (memory $linear_memory 1 1)

    (func $a_function (param $iteration_to_skip i32) (result i32 i32 i32 i32 i32 i32)

        ;; The function has three locals.
        (local $value i32)
        (local $addr  i32)
        (local $max_iter i32)

        ;; And consists of two blocks.
        (block
            ;; Initialisation of $value and $max_iter.
            i32.const 1
            local.set $value
            i32.const 5
            local.set $max_iter

            ;; Resume the value of $addr
            ;; from the linear memory.
            i32.const 10
            i32.load8_u
            local.set $addr

            ;; To simulate a checkpoint event,
            ;; decide how many iterations to complete
            ;; bedore suspension.
            local.get $max_iter
            local.get $iteration_to_skip
            i32.sub
            local.set $max_iter
        )
        ;; Append $value to a queue in
        ;; the linear memory, 10 times.
        (loop $loop_head

            ;; Make a checkpoint.
            ;; Save in memory the values of the
            ;; local variables.
            i32.const 10
            local.get $addr
            i32.store8

            ;; Write $value at the address $addr.
            local.get $addr
            local.get $value
            i32.store8

            ;; Increase $addr.
            i32.const 1
            local.get $addr
            i32.add
            local.set $addr

            ;; Decide to jump or continue:
            ;; if $addr is strictly less then
            ;; $max_iter, jump to the head
            ;; of the loop.
            local.get $addr
            local.get $max_iter
            i32.lt_s
            br_if $loop_head
        )

        ;; Return value.
        local.get $addr

        i32.const 0
        i32.load8_u

        i32.const 1
        i32.load8_u

        i32.const 2
        i32.load8_u

        i32.const 3
        i32.load8_u

        i32.const 4
        i32.load8_u

    )

    ;; (start $a_function)
    (export "function" (func $a_function))

    ;; Linear memory used by the computation.
    (data (i32.const 0) "\00\00\00\00\00\00\00\00\00\00")

    ;; Checkpoint of $addr.
    (data (i32.const 10) "\00")

)