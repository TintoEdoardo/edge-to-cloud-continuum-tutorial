
;; +-- PART 1 --+
;; Interfacing with the host environment

;; Linear memories are contiguous portions
;; of the host memory address space.
;; A module accesses its linear memory using
;; an offset.

(module $the_module

    (memory $memory 1 1)

    (func $get_from_offset (param i32) (result i32)

        ;; Put the parameter on the stack,
        ;; it will be used as a memory offset.
        local.get 0

        ;; Load an i32 value from $memory
        ;; using the value on top of the stack
        ;; as an offset.
        i32.load
    )

    (func $set_to_offset (param i32 i32)

        ;; Same as before: write the first
        ;; parameter value on top of the stack.
        local.get 0

        ;; Write also the second parameter,
        ;; it will be written into $memory.
        local.get 1

        ;; Finally, perform the store operation.
        i32.store
    )

    (export "module_lin_mem" (memory $memory))
    (export "get_from_offset" (func $get_from_offset))
    (export "set_to_offset" (func $set_to_offset))

    ;; The linear memory can be initialised using
    ;; the data segment construct.
    (data (i32.const 0) "\00\00\01\00\00\00\03\00")
)