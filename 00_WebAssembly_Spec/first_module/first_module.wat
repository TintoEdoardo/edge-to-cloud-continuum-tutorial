
;; +-- PART 1 --+
;; Organisation of a module

;; In WebAssembly, the main deployment unit
;; is the *module*.
;; A module contains *functions*.

(module $the_module

    (func $sum_of_i32 (param i32 i32) (result i32)
        ;; WebAssembly bytecode executes as a stack machine:
        ;; instructions push and pop values to a *value stack*,
        ;; according to a last in, first out (LIFO) policy.

        local.get 0
        local.get 1
        i32.add
    )

    ;; Without further statements, functions defined
    ;; in the module (i.e., $sum_of_i32) cannot be
    ;; invoked by the host.
    ;; To do so, the function must be exported.
    (export "sum_of_i32" (func $sum_of_i32))
    
)



