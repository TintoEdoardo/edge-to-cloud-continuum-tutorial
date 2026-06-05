
;; +-- PART 1 --+
;; Wasm properties

(module $the_module

    ;; Wasm provides some form of
    ;; Control-Flow Integrity (CFI).
    (func $a_function (result i32)
        (local $i i32)

        ;; Initialise the local $i.
        i32.const 0
        local.set $i

        (block $third_level
            (block $second_level
                (loop $first_level

                    ;; Increment $i.
                    i32.const 1
                    local.get $i
                    i32.add
                    local.set $i

                    ;; Loop while $i is < 10.
                    local.get $i
                    i32.const 10
                    i32.lt_s
                    br_if $first_level

                    ;; Illegal instruction.
                    ;; br $non_outer_block

                )
            )
            (block $non_outer_block
                nop
            )
        )

        ;; Return the value of $i.
        local.get $i

    )

    (export "function" (func $a_function))

)